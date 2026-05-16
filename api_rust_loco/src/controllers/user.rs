#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::middleware::auth::CookieJWT;
use crate::models::_entities::users::{Entity, Model};
use crate::models::ability::{Ability, Action, Resource, Subject};

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

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(auth: CookieJWT, State(ctx): State<AppContext>) -> Result<Response> {
    let (current_user, ability) = Ability::for_user_pid(&ctx.db, &auth.claims.pid).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let items = ability
        .accessible_users_query(Action::Read, current_user.id)
        .order_by_desc(crate::models::_entities::users::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    if items.is_empty() {
        return format::json(Vec::<UserListItem>::new());
    }

    // Optimization: Fetch all roles for all displayed users in one go
    let user_ids: Vec<i32> = items.iter().map(|u| u.id).collect();
    
    use crate::models::_entities::roles::Entity as RoleEntity;
    use crate::models::_entities::users_roles::Entity as UserRoleEntity;

    let all_users_roles = UserRoleEntity::find()
        .filter(crate::models::_entities::users_roles::Column::UserId.is_in(user_ids))
        .find_also_related(RoleEntity)
        .all(&ctx.db)
        .await?;

    let mut response = Vec::with_capacity(items.len());
    for item in items {
        let user_id = item.id;
        let roles: Vec<String> = all_users_roles
            .iter()
            .filter(|(ur, _)| ur.user_id == user_id)
            .filter_map(|(_, r)| r.as_ref().map(|role| role.name.clone()))
            .collect();
        
        response.push(to_list_item(item, roles));
    }
    format::json(response)
}

#[debug_handler]
pub async fn get_one(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (_, ability) = Ability::for_user_pid(&ctx.db, &auth.claims.pid).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let item = load_item(&ctx, id).await?;
    ability.authorize_resource(Action::Read, Resource::User { id: item.id })?;

    let roles = item.roles(&ctx.db).await?;
    format::json(to_detail(item, roles))
}

#[debug_handler]
pub async fn remove(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let (_, ability) = Ability::for_user_pid(&ctx.db, &auth.claims.pid).await?;
    ability.authorize(Action::Read, Subject::Admin)?;

    let item = load_item(&ctx, id).await?;
    ability.authorize_resource(Action::Destroy, Resource::User { id: item.id })?;

    item.delete(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    routes_with_prefix("api/users/")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/users/")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
}
