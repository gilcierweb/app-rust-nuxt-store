#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::auth::CookieJWT;
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::models::users;
use crate::services::cart;

async fn current_user_id(ctx: &AppContext, auth: &CookieJWT) -> Result<i32> {
    if let Some(user_id) = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
    {
        return Ok(user_id);
    }

    Ok(users::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await?
        .id)
}

#[derive(Debug, Deserialize)]
pub struct AddItemParams {
    pub product_id: i32,
    pub product_variant_id: Option<i32>,
    pub quantity: i32,
    #[serde(with = "crate::utils::decimal")]
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemParams {
    pub item_id: i32,
    pub quantity: i32,
}

#[debug_handler]
pub async fn list(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let result = cart::get_cart_with_items_for_user(&ctx.db, user_id).await?;
    format::json(result)
}

#[debug_handler]
pub async fn add_item(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AddItemParams>,
) -> Result<Response> {
    if params.quantity <= 0 {
        return Err(Error::BadRequest(t!("cart.quantity_required").into()));
    }

    let user_id = current_user_id(&ctx, &auth).await?;
    let user_cart = cart::get_or_create_cart(&ctx.db, user_id).await?;
    let result = cart::add_item(
        &ctx.db,
        user_cart.id,
        params.product_id,
        params.product_variant_id,
        params.quantity,
        params.price,
    )
    .await?;
    format::json(result)
}

#[debug_handler]
pub async fn update_item(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateItemParams>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let user_cart = cart::get_or_create_cart(&ctx.db, user_id).await?;
    let result = cart::update_item_quantity(&ctx.db, params.item_id, user_cart.id, params.quantity).await?;
    format::json(result)
}

#[debug_handler]
pub async fn remove_item(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Path(item_id): Path<i32>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let user_cart = cart::get_or_create_cart(&ctx.db, user_id).await?;
    let result = cart::remove_item(&ctx.db, item_id, user_cart.id).await?;
    format::json(result)
}

#[debug_handler]
pub async fn clear(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let user_cart = cart::get_or_create_cart(&ctx.db, user_id).await?;
    cart::clear_cart(&ctx.db, user_cart.id).await?;
    format::json(serde_json::json!({ "cleared": true }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/carts/")
        .add("/", get(list))
        .add("list", get(list))
        .add("add_item", post(add_item))
        .add("remove_item/{item_id}", delete(remove_item))
        .add("update_item", put(update_item))
        .add("clear", delete(clear))
}
