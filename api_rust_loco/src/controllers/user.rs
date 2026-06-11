#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::debug_handler;
use axum::extract::{Extension, Query};
use loco_rs::hash;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::{AdminSession, CookieJWT};
use crate::models::_entities::roles;
use crate::models::_entities::users::{ActiveModel, Entity, Model};
use crate::models::_entities::users_roles;
use crate::models::ability::{Ability, Action, Resource, Subject};
use crate::models::users::validate_password;
use crate::services::admin_audit_logs;
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams, PaginationParams};

const ALLOWED_GLOBAL_ROLES: &[&str] = &[
    "admin",
    "user",
    "store_manager",
    "editor",
    "support",
    "viewer",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub id: i32,
    pub pid: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    pub email: String,
    pub name: String,
    pub password: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AdminUserListParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
}

impl UserParams {
    fn normalized_email(&self) -> String {
        self.email.trim().to_lowercase()
    }

    fn normalized_name(&self) -> String {
        self.name.trim().to_string()
    }

    fn normalized_role(&self) -> Result<String> {
        let role = self.role.as_deref().unwrap_or("user").trim().to_lowercase();

        if ALLOWED_GLOBAL_ROLES.contains(&role.as_str()) {
            Ok(role)
        } else {
            Err(Error::BadRequest(t!("user.invalid_role").into()))
        }
    }

    fn validate_for_create(&self) -> Result<()> {
        self.validate_common()?;
        match self.password.as_deref().map(str::trim) {
            Some(password) => validate_password(password).map_err(|e| Error::Message(e.to_string())),
            None => Err(Error::BadRequest(t!("user.password_required").into())),
        }
    }

    fn validate_for_update(&self) -> Result<()> {
        self.validate_common()?;
        if let Some(password) = self.password.as_deref().map(str::trim) {
            if !password.is_empty() {
                validate_password(password).map_err(|e| Error::Message(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn validate_common(&self) -> Result<()> {
        if self.normalized_name().len() < 2 {
            return Err(Error::BadRequest(
                t!("user.name_min_length").into(),
            ));
        }

        let email = self.normalized_email();
        if !email.contains('@') || email.len() < 5 {
            return Err(Error::BadRequest(t!("user.email_invalid").into()));
        }

        self.normalized_role()?;
        Ok(())
    }
}

fn display_role(roles: &[String]) -> String {
    if roles.iter().any(|role| role == "admin") {
        "Admin".to_string()
    } else if let Some(role) = roles.first() {
        role.to_string()
    } else {
        "User".to_string()
    }
}

fn to_list_item(user: Model, roles: Vec<String>) -> UserListItem {
    UserListItem {
        id: user.id,
        email: user.email,
        name: user.name,
        role: display_role(&roles),
        active: user.email_verified_at.is_some(),
        created_at: user.created_at,
    }
}

fn to_detail(user: Model, roles: Vec<String>) -> UserDetail {
    UserDetail {
        id: user.id,
        pid: user.pid.to_string(),
        email: user.email,
        name: user.name,
        role: display_role(&roles),
        active: user.email_verified_at.is_some(),
        email_verified_at: user.email_verified_at,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

async fn build_list_items(ctx: &AppContext, items: Vec<Model>) -> Result<Vec<UserListItem>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let user_ids: Vec<i32> = items.iter().map(|user| user.id).collect();

    use crate::models::_entities::roles::Entity as RoleEntity;
    use crate::models::_entities::users_roles::Entity as UserRoleEntity;

    let all_users_roles = UserRoleEntity::find()
        .filter(crate::models::_entities::users_roles::Column::UserId.is_in(user_ids))
        .find_also_related(RoleEntity)
        .all(&ctx.db)
        .await?;

    let mut roles_by_user: HashMap<i32, Vec<String>> = HashMap::new();
    for (user_role, role) in all_users_roles {
        if let Some(role) = role {
            roles_by_user
                .entry(user_role.user_id)
                .or_default()
                .push(role.name);
        }
    }

    let mut response = Vec::with_capacity(items.len());
    for item in items {
        let roles = roles_by_user.remove(&item.id).unwrap_or_default();
        response.push(to_list_item(item, roles));
    }

    Ok(response)
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

async fn ensure_email_available(
    ctx: &AppContext,
    email: &str,
    current_id: Option<i32>,
) -> Result<()> {
    let existing = Entity::find()
        .filter(crate::models::_entities::users::Column::Email.eq(email))
        .one(&ctx.db)
        .await?;

    if existing
        .as_ref()
        .is_some_and(|user| Some(user.id) != current_id)
    {
        return Err(Error::BadRequest(t!("user.email_in_use").into()));
    }

    Ok(())
}

async fn find_or_create_global_role<C>(db: &C, name: &str) -> Result<roles::Model>
where
    C: ConnectionTrait,
{
    let role = roles::Entity::find()
        .filter(roles::Column::Name.eq(name))
        .filter(roles::Column::ResourceType.is_null())
        .filter(roles::Column::ResourceId.is_null())
        .one(db)
        .await?;

    if let Some(role) = role {
        return Ok(role);
    }

    let now = chrono::Utc::now().into();
    Ok(roles::ActiveModel {
        name: Set(name.to_string()),
        resource_type: Set(None),
        resource_id: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

async fn replace_global_role<C>(db: &C, user_id: i32, role_name: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    let global_roles = roles::Entity::find()
        .filter(roles::Column::Name.is_in(ALLOWED_GLOBAL_ROLES.iter().copied()))
        .filter(roles::Column::ResourceType.is_null())
        .filter(roles::Column::ResourceId.is_null())
        .all(db)
        .await?;
    let global_role_ids = global_roles.iter().map(|role| role.id).collect::<Vec<_>>();

    if !global_role_ids.is_empty() {
        users_roles::Entity::delete_many()
            .filter(users_roles::Column::UserId.eq(user_id))
            .filter(users_roles::Column::RoleId.is_in(global_role_ids))
            .exec(db)
            .await?;
    }

    let role = find_or_create_global_role(db, role_name).await?;
    users_roles::ActiveModel {
        user_id: Set(user_id),
        role_id: Set(role.id),
    }
    .insert(db)
    .await?;

    Ok(())
}

fn set_active(active_model: &mut ActiveModel, active: bool) {
    active_model.email_verified_at = if active {
        Set(Some(chrono::Utc::now().into()))
    } else {
        Set(None)
    };
}

async fn resolve_user_and_ability(
    ctx: &AppContext,
    pid: &str,
    admin_session: Option<Extension<AdminSession>>,
) -> Result<(i32, Ability)> {
    if let Some(Extension(admin_session)) = admin_session {
        return Ok((admin_session.current_user_id, admin_session.ability));
    }

    let (current_user, ability) = Ability::for_user_pid(&ctx.db, pid).await?;
    Ok((current_user.id, ability))
}

#[debug_handler]
pub async fn list(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let (current_user_id, ability) =
        resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let items = ability
        .accessible_users_query(Action::Read, current_user_id)
        .order_by_desc(crate::models::_entities::users::Column::CreatedAt)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    let response = build_list_items(&ctx, items).await?;
    format::json(response)
}

#[debug_handler]
pub async fn admin_list(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    State(ctx): State<AppContext>,
    Query(params): Query<AdminUserListParams>,
) -> Result<Response> {
    let (current_user_id, ability) =
        resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let mut query = ability
        .accessible_users_query(Action::Read, current_user_id)
        .order_by_desc(crate::models::_entities::users::Column::CreatedAt)
        .order_by_desc(crate::models::_entities::users::Column::Id);

    if let Some(search) = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let mut condition = Condition::any()
            .add(crate::models::_entities::users::Column::Email.contains(search))
            .add(crate::models::_entities::users::Column::Name.contains(search));

        if let Ok(user_id) = search.parse::<i32>() {
            condition = condition.add(crate::models::_entities::users::Column::Id.eq(user_id));
        }

        query = query.filter(condition);
    }

    let total_fut = query.clone().count(&ctx.db);
    let paginator = query.paginate(&ctx.db, pagination.page_size());
    let items_fut = paginator.fetch_page(pagination.page_index());

    let (total, items) = tokio::try_join!(total_fut, items_fut).map_err(|e| {
        tracing::error!(error = ?e, "failed to load admin users list in parallel");
        Error::InternalServerError
    })?;

    let response = build_list_items(&ctx, items).await?;

    format::json(AdminPaginatedResponse::new(response, total, pagination))
}

#[debug_handler]
pub async fn get_one(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (_, ability) = resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let item = load_item(&ctx, id).await?;
    ability.authorize_resource(Action::Read, Resource::User { id: item.id })?;

    let roles = item.roles(&ctx.db).await?;
    format::json(to_detail(item, roles))
}

#[debug_handler]
pub async fn add(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    State(ctx): State<AppContext>,
    Json(params): Json<UserParams>,
) -> Result<Response> {
    let (current_user_id, ability) =
        resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;
    ability.authorize(Action::Create, Subject::User)?;

    params.validate_for_create()?;
    let email = params.normalized_email();
    ensure_email_available(&ctx, &email, None).await?;
    let role = params.normalized_role()?;
    let password = params.password.as_deref().unwrap_or_default().trim();
    let password_hash = hash::hash_password(password)?;
    let now = chrono::Utc::now().into();

    let txn = ctx.db.begin().await?;
    let mut active_model = ActiveModel {
        email: Set(email),
        password: Set(password_hash),
        name: Set(params.normalized_name()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    set_active(&mut active_model, params.active.unwrap_or(true));

    let item = active_model.insert(&txn).await?;
    replace_global_role(&txn, item.id, &role).await?;
    admin_audit_logs::record(
        &txn,
        current_user_id,
        "user.create",
        "user",
        Some(item.id),
        Some(item.email.clone()),
        Some(format!("Created admin-managed user with role {role}")),
    )
    .await?;
    txn.commit().await?;

    let roles = item.roles(&ctx.db).await?;
    format::json(to_detail(item, roles))
}

#[debug_handler]
pub async fn update(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UserParams>,
) -> Result<Response> {
    let (current_user_id, ability) =
        resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    params.validate_for_update()?;
    let item = load_item(&ctx, id).await?;
    ability.authorize_resource(Action::Update, Resource::User { id: item.id })?;

    let role = params.normalized_role()?;
    if item.id == current_user_id && role != "admin" {
        return Err(Error::BadRequest(
            t!("user.admin_role_required").into(),
        ));
    }

    let email = params.normalized_email();
    ensure_email_available(&ctx, &email, Some(item.id)).await?;

    let txn = ctx.db.begin().await?;
    let mut active_model = item.into_active_model();
    active_model.email = Set(email);
    active_model.name = Set(params.normalized_name());
    active_model.updated_at = Set(chrono::Utc::now().into());
    set_active(&mut active_model, params.active.unwrap_or(true));

    if let Some(password) = params.password.as_deref().map(str::trim) {
        if !password.is_empty() {
            active_model.password = Set(hash::hash_password(password)?);
        }
    }

    let item = active_model.update(&txn).await?;
    replace_global_role(&txn, item.id, &role).await?;
    admin_audit_logs::record(
        &txn,
        current_user_id,
        "user.update",
        "user",
        Some(item.id),
        Some(item.email.clone()),
        Some(format!("Updated admin-managed user role to {role}")),
    )
    .await?;
    txn.commit().await?;

    let roles = item.roles(&ctx.db).await?;
    format::json(to_detail(item, roles))
}

#[debug_handler]
pub async fn remove(
    auth: CookieJWT,
    admin_session: Option<Extension<AdminSession>>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (current_user_id, ability) =
        resolve_user_and_ability(&ctx, &auth.claims.pid, admin_session).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let item = load_item(&ctx, id).await?;
    ability.authorize_resource(Action::Destroy, Resource::User { id: item.id })?;
    let resource_id = item.id;
    let resource_label = item.email.clone();
    let txn = ctx.db.begin().await?;
    item.delete(&txn).await?;
    admin_audit_logs::record(
        &txn,
        current_user_id,
        "user.delete",
        "user",
        Some(resource_id),
        Some(resource_label),
        Some("Deleted admin-managed user".to_string()),
    )
    .await?;
    txn.commit().await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/users")
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/users")
        .add("/", get(admin_list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", put(update))
        .add("{id}", patch(update))
        .add("{id}", delete(remove))
}
