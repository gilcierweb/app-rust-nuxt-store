#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;

use crate::models::_entities::wishlists::{ActiveModel, Entity};

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
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let items = Entity::find()
        .filter(crate::models::_entities::wishlists::Column::UserId.eq(1))
        .order_by_desc(crate::models::_entities::wishlists::Column::CreatedAt)
        .all(&ctx.db)
        .await?;
    format::json(items)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Query(params): Query<AddParams>,
) -> Result<Response> {
    let existing = Entity::find()
        .filter(crate::models::_entities::wishlists::Column::UserId.eq(1))
        .filter(crate::models::_entities::wishlists::Column::ProductId.eq(params.product_id))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(Error::BadRequest("Product already in wishlist".into()));
    }

    let now = chrono::Utc::now().into();
    let item = ActiveModel {
        user_id: Set(1),
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
    State(ctx): State<AppContext>,
    Query(params): Query<RemoveParams>,
) -> Result<Response> {
    let item = Entity::find_by_id(params.id)
        .one(&ctx.db)
        .await?;
    let item = item.ok_or_else(|| Error::NotFound)?;
    item.delete(&ctx.db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to remove wishlist item");
        Error::InternalServerError
    })?;
    format::json(serde_json::json!({ "removed": true }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/wishlists/")
        .add("/", get(index))
        .add("list", get(list))
        .add("add", get(add))
        .add("remove", get(remove))
}
