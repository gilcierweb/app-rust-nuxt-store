#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use std::collections::HashMap;

use axum::debug_handler;
use axum::extract::{Extension, Query};
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, Statement,
};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AdminSession;
use crate::models::_entities::{roles, users, users_roles};
use crate::services::admin_audit_logs;
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize)]
pub struct RoleJson {
    pub id: i32,
    pub name: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<i32>,
    pub assignment_count: usize,
    pub protected: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Clone, Debug, Serialize)]
pub struct RbacUserJson {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub roles: Vec<RoleJson>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RbacSummaryJson {
    pub roles: Vec<RoleJson>,
    pub users: Vec<RbacUserJson>,
    pub total_users: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PermissionGroupJson {
    pub subject: String,
    pub actions: Vec<String>,
    pub source: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PermissionsJson {
    pub groups: Vec<PermissionGroupJson>,
    pub role_matrix: Vec<RolePermissionJson>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RolePermissionJson {
    pub role: String,
    pub sections: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SectionsJson {
    pub sections: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RoleParams {
    pub name: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssignmentParams {
    pub user_id: i32,
    pub role_id: i32,
}

fn normalized_name(name: &str) -> String {
    name.trim().to_lowercase()
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_lowercase())
        .filter(|item| !item.is_empty())
}

fn validate_role_name(name: &str) -> Result<()> {
    let is_valid = !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_');

    if is_valid {
        Ok(())
    } else {
        Err(Error::BadRequest(
            "role name must use letters, numbers, and underscores".into(),
        ))
    }
}

fn is_protected_admin_role(role: &roles::Model) -> bool {
    role.name == "admin" && role.resource_type.is_none() && role.resource_id.is_none()
}

fn to_role_json(role: &roles::Model, assignment_count: usize) -> RoleJson {
    RoleJson {
        id: role.id,
        name: role.name.clone(),
        resource_type: role.resource_type.clone(),
        resource_id: role.resource_id,
        assignment_count,
        protected: is_protected_admin_role(role),
        created_at: role.created_at,
        updated_at: role.updated_at,
    }
}

async fn load_user(db: &DatabaseConnection, id: i32) -> Result<users::Model> {
    users::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

async fn load_role(db: &DatabaseConnection, id: i32) -> Result<roles::Model> {
    roles::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn summary(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let page_size = pagination.page_size().min(100);
    let page_index = pagination.page_index();

    let roles_fut = roles::Entity::find()
        .order_by_asc(roles::Column::Name)
        .order_by_asc(roles::Column::ResourceType)
        .order_by_asc(roles::Column::ResourceId)
        .all(&ctx.db);

    let users_query = users::Entity::find().order_by_asc(users::Column::Email);
    let total_users_fut = users_query.clone().count(&ctx.db);
    let paginator = users_query.paginate(&ctx.db, page_size);
    let users_fut = paginator.fetch_page(page_index);

    let (roles_res, total_users, users) = tokio::try_join!(roles_fut, total_users_fut, users_fut)
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to load rbac summary in parallel");
            Error::InternalServerError
        })?;

    let roles = roles_res;
    let role_ids: Vec<i32> = roles.iter().map(|role| role.id).collect();
    let paginated_user_ids: Vec<i32> = users.iter().map(|u| u.id).collect();

    // Round 2 (parallel): both queries are now independent of each other
    // and run concurrently with a single roundtrip.
    let role_counts_fut = async {
        if role_ids.is_empty() {
            return Ok::<_, Error>(Vec::new());
        }
        #[derive(Debug, FromQueryResult)]
        struct RoleAssignmentCount {
            role_id: i32,
            cnt: i64,
        }
        let counts_sql = "SELECT role_id, COUNT(*)::BIGINT AS cnt FROM users_roles \
                          WHERE role_id = ANY($1) GROUP BY role_id";
        RoleAssignmentCount::find_by_statement(Statement::from_sql_and_values(
            ctx.db.get_database_backend(),
            counts_sql,
            vec![role_ids.clone().into()],
        ))
        .all(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to count role assignments");
            Error::InternalServerError
        })
    };

    let user_role_rows_fut = async {
        if paginated_user_ids.is_empty() || role_ids.is_empty() {
            return Ok::<_, Error>(Vec::new());
        }
        users_roles::Entity::find()
            .filter(users_roles::Column::UserId.is_in(paginated_user_ids.clone()))
            .filter(users_roles::Column::RoleId.is_in(role_ids.clone()))
            .select_only()
            .column(users_roles::Column::UserId)
            .column(users_roles::Column::RoleId)
            .into_tuple::<(i32, i32)>()
            .all(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to load user role rows");
                Error::InternalServerError
            })
    };

    let (role_counts, user_role_rows) =
        tokio::try_join!(role_counts_fut, user_role_rows_fut).map_err(|e| {
            tracing::error!(error = ?e, "failed to load rbac summary second wave");
            Error::InternalServerError
        })?;

    let mut assignment_counts = HashMap::<i32, usize>::new();
    for row in role_counts {
        assignment_counts.insert(row.role_id, row.cnt as usize);
    }

    let role_lookup = roles
        .iter()
        .map(|role| {
            (
                role.id,
                to_role_json(role, assignment_counts.get(&role.id).copied().unwrap_or(0)),
            )
        })
        .collect::<HashMap<_, _>>();

    let role_items = roles
        .iter()
        .map(|role| to_role_json(role, assignment_counts.get(&role.id).copied().unwrap_or(0)))
        .collect::<Vec<_>>();

    let mut role_ids_by_user = HashMap::<i32, Vec<i32>>::new();
    for (user_id, role_id) in user_role_rows {
        role_ids_by_user.entry(user_id).or_default().push(role_id);
    }

    let user_items = users
        .into_iter()
        .map(|user| {
            let user_roles = role_ids_by_user
                .remove(&user.id)
                .unwrap_or_default()
                .into_iter()
                .filter_map(|role_id| role_lookup.get(&role_id).cloned())
                .collect();

            RbacUserJson {
                id: user.id,
                email: user.email,
                name: user.name,
                roles: user_roles,
            }
        })
        .collect();

    format::json(RbacSummaryJson {
        roles: role_items,
        users: user_items,
        total_users,
        page: pagination.page.max(1) as u64,
        page_size,
    })
}

#[debug_handler]
pub async fn permissions() -> Result<Response> {
    let role_matrix = vec![
        RolePermissionJson {
            role: "admin".to_string(),
            sections: vec![
                "dashboard", "products", "categories", "orders", "reviews",
                "banners", "coupons", "shippings", "shipments", "profiles",
                "addresses", "posts", "variants", "payments", "inventory",
                "settings", "audit_logs", "emails", "rbac", "customers",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        RolePermissionJson {
            role: "store_manager".to_string(),
            sections: vec![
                "dashboard", "products", "categories", "orders", "reviews",
                "banners", "coupons", "shippings", "shipments", "variants",
                "inventory", "payments", "profiles", "addresses",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        RolePermissionJson {
            role: "editor".to_string(),
            sections: vec![
                "dashboard", "products", "categories", "reviews", "banners",
                "posts", "variants", "payments", "shippings", "shipments",
                "profiles", "addresses", "customers",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        RolePermissionJson {
            role: "support".to_string(),
            sections: vec![
                "dashboard", "products", "categories", "orders", "payments",
                "shipments", "shippings", "profiles", "addresses", "customers",
                "reviews", "coupons", "variants",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        RolePermissionJson {
            role: "viewer".to_string(),
            sections: vec![
                "dashboard", "products", "categories", "orders", "reviews",
                "banners", "coupons", "shippings", "shipments", "profiles",
                "addresses", "posts", "variants", "payments", "inventory",
                "audit_logs", "customers",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
    ];

    format::json(PermissionsJson {
        groups: vec![
            PermissionGroupJson {
                subject: "Admin".to_string(),
                actions: vec!["manage".to_string()],
                source: "Ability::define_admin_rules".to_string(),
            },
            PermissionGroupJson {
                subject: "User".to_string(),
                actions: vec![
                    "read".to_string(),
                    "create".to_string(),
                    "update".to_string(),
                    "destroy".to_string(),
                ],
                source: "Ability::define_admin_rules".to_string(),
            },
            PermissionGroupJson {
                subject: "Own User".to_string(),
                actions: vec!["read".to_string(), "update".to_string()],
                source: "Ability::define_authenticated_user_rules".to_string(),
            },
        ],
        role_matrix,
    })
}

#[debug_handler]
pub async fn sections(
    Extension(session): Extension<AdminSession>,
) -> Result<Response> {
    let sections = session.ability.admin_sections();
    format::json(SectionsJson {
        sections: sections.into_iter().map(String::from).collect(),
    })
}

#[debug_handler]
pub async fn create_role(
    admin_session: Extension<AdminSession>,
    State(ctx): State<AppContext>,
    Json(params): Json<RoleParams>,
) -> Result<Response> {
    let name = normalized_name(&params.name);
    validate_role_name(&name)?;
    let resource_type = normalize_optional_text(params.resource_type);
    let resource_id = params.resource_id;

    if resource_id.is_some() && resource_type.is_none() {
        return Err(Error::BadRequest(
            "resource_type is required when resource_id is set".into(),
        ));
    }

    let mut query = roles::Entity::find().filter(roles::Column::Name.eq(&name));
    query = match &resource_type {
        Some(resource_type) => query.filter(roles::Column::ResourceType.eq(resource_type)),
        None => query.filter(roles::Column::ResourceType.is_null()),
    };
    query = match resource_id {
        Some(resource_id) => query.filter(roles::Column::ResourceId.eq(resource_id)),
        None => query.filter(roles::Column::ResourceId.is_null()),
    };

    if query.one(&ctx.db).await?.is_some() {
        return Err(Error::BadRequest("role already exists".into()));
    }

    let now = chrono::Utc::now().into();
    let role = roles::ActiveModel {
        name: Set(name),
        resource_type: Set(resource_type),
        resource_id: Set(resource_id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    admin_audit_logs::record(
        &ctx.db,
        admin_session.current_user_id,
        "role.create",
        "role",
        Some(role.id),
        Some(role.name.clone()),
        Some("Created RBAC role".to_string()),
    )
    .await?;

    format::json(to_role_json(&role, 0))
}

#[debug_handler]
pub async fn assign_role(
    admin_session: Extension<AdminSession>,
    State(ctx): State<AppContext>,
    Json(params): Json<AssignmentParams>,
) -> Result<Response> {
    let user = load_user(&ctx.db, params.user_id).await?;
    let role = load_role(&ctx.db, params.role_id).await?;

    let exists = users_roles::Entity::find()
        .filter(users_roles::Column::UserId.eq(params.user_id))
        .filter(users_roles::Column::RoleId.eq(params.role_id))
        .one(&ctx.db)
        .await?
        .is_some();

    if !exists {
        users_roles::ActiveModel {
            user_id: Set(params.user_id),
            role_id: Set(params.role_id),
        }
        .insert(&ctx.db)
        .await?;

        admin_audit_logs::record(
            &ctx.db,
            admin_session.current_user_id,
            "role.assign",
            "role_assignment",
            Some(role.id),
            Some(format!("{} -> {}", role.name, user.email)),
            Some("Assigned RBAC role to user".to_string()),
        )
        .await?;
    }

    format::empty()
}

#[debug_handler]
pub async fn remove_assignment(
    admin_session: Extension<AdminSession>,
    Path((user_id, role_id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let role = load_role(&ctx.db, role_id).await?;

    if admin_session.current_user_id == user_id && is_protected_admin_role(&role) {
        return Err(Error::BadRequest(
            "current admin user must keep the admin role".into(),
        ));
    }

    users_roles::Entity::delete_many()
        .filter(users_roles::Column::UserId.eq(user_id))
        .filter(users_roles::Column::RoleId.eq(role_id))
        .exec(&ctx.db)
        .await?;

    admin_audit_logs::record(
        &ctx.db,
        admin_session.current_user_id,
        "role.unassign",
        "role_assignment",
        Some(role.id),
        Some(format!("{} -> user:{user_id}", role.name)),
        Some("Removed RBAC role from user".to_string()),
    )
    .await?;

    format::empty()
}

#[debug_handler]
pub async fn delete_role(
    admin_session: Extension<AdminSession>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let role = load_role(&ctx.db, id).await?;
    if is_protected_admin_role(&role) {
        return Err(Error::BadRequest("admin role cannot be deleted".into()));
    }

    let assigned = users_roles::Entity::find()
        .filter(users_roles::Column::RoleId.eq(role.id))
        .one(&ctx.db)
        .await?
        .is_some();
    if assigned {
        return Err(Error::BadRequest("role has assignments".into()));
    }

    let role_id = role.id;
    let role_name = role.name.clone();
    role.delete(&ctx.db).await?;
    admin_audit_logs::record(
        &ctx.db,
        admin_session.current_user_id,
        "role.delete",
        "role",
        Some(role_id),
        Some(role_name),
        Some("Deleted RBAC role".to_string()),
    )
    .await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/rbac")
        .add("summary", get(summary))
        .add("permissions", get(permissions))
        .add("sections", get(sections))
        .add("roles", post(create_role))
        .add("roles/{id}", delete(delete_role))
        .add("assignments", post(assign_role))
        .add("assignments/{user_id}/{role_id}", delete(remove_assignment))
}
