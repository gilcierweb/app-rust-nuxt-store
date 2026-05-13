#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::models::_entities::users::{Entity, Model};
use crate::models::users;

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

fn to_list_item(user: Model) -> UserListItem {
    let role = if user.id == 1 || user.email.starts_with("admin@") {
        "Admin".to_string()
    } else {
        "User".to_string()
    };

    UserListItem {
        id: user.id,
        email: user.email,
        name: user.name,
        role,
        active: user.email_verified_at.is_some(),
        created_at: user.created_at,
    }
}

fn to_detail(user: Model) -> UserDetail {
    let role = if user.id == 1 || user.email.starts_with("admin@") {
        "Admin".to_string()
    } else {
        "User".to_string()
    };

    UserDetail {
        id: user.id,
        pid: user.pid.to_string(),
        email: user.email,
        name: user.name,
        role,
        active: user.email_verified_at.is_some(),
        email_verified_at: user.email_verified_at,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

fn is_admin(user: &Model) -> bool {
    user.id == 1 || user.email.starts_with("admin@")
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if !is_admin(&current_user) {
        return unauthorized(t!("auth.unauthorized"));
    }

    let items = Entity::find()
        .order_by_desc(crate::models::_entities::users::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let response: Vec<UserListItem> = items.into_iter().map(to_list_item).collect();
    format::json(response)
}

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if !is_admin(&current_user) {
        return unauthorized(t!("auth.unauthorized"));
    }

    format::json(to_detail(load_item(&ctx, id).await?))
}

#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if !is_admin(&current_user) {
        return unauthorized(t!("auth.unauthorized"));
    }

    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/users/")
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
}
