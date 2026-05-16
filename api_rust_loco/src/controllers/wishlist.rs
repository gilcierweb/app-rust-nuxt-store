#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use crate::middleware::auth::CookieJWT;

use crate::models::_entities::wishlists::{ActiveModel, Entity};
use crate::models::users;

#[derive(Debug, Deserialize)]
pub struct AddParams {
    pub product_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct RemoveParams {
    pub id: i32,
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn list(auth: CookieJWT, State(ctx): State<AppContext>) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let items = Entity::find()
        .filter(crate::models::_entities::wishlists::Column::UserId.eq(current_user.id))
        .order_by_desc(crate::models::_entities::wishlists::Column::CreatedAt)
        .all(&ctx.db)
        .await?;
    format::json(items)
}

#[debug_handler]
pub async fn add(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Query(params): Query<AddParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let existing = Entity::find()
        .filter(crate::models::_entities::wishlists::Column::UserId.eq(current_user.id))
        .filter(crate::models::_entities::wishlists::Column::ProductId.eq(params.product_id))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(Error::BadRequest(t!("wishlist.already_exists").into()));
    }

    let now = chrono::Utc::now().into();
    let item = ActiveModel {
        user_id: Set(current_user.id),
        product_id: Set(params.product_id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let saved = item.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to add wishlist item");
        Error::InternalServerError
    })?;
    format::json(saved)
}

#[debug_handler]
pub async fn remove(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Query(params): Query<RemoveParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = Entity::find_by_id(params.id).one(&ctx.db).await?;
    let item = item.ok_or_else(|| Error::NotFound)?;
    if item.user_id != current_user.id {
        return unauthorized(t!("auth.unauthorized"));
    }
    item.delete(&ctx.db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to remove wishlist item");
        Error::InternalServerError
    })?;
    format::json(serde_json::json!({ "removed": true }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/wishlists")
        .add("/", get(index))
        .add("/list", get(list))
        .add("/add", get(add))
        .add("/remove", get(remove))
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/wishlist")
        .add("/", get(list))
        .add("/list", get(list))
        .add("/add", get(add))
        .add("/remove", get(remove))
}
