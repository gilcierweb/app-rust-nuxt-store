#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::models::_entities::payment_methods;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMethodParams {
    pub active: Option<bool>,
    pub auto_capture: Option<bool>,
    pub position: Option<i32>,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let methods = payment_methods::Entity::find()
        .order_by_asc(payment_methods::Column::Position)
        .all(&ctx.db)
        .await?;

    format::json(methods)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let method = payment_methods::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(method)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateMethodParams>,
) -> Result<Response> {
    let method = payment_methods::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_model: payment_methods::ActiveModel = method.into();

    if let Some(active) = params.active {
        active_model.active = Set(Some(active));
    }

    if let Some(auto_capture) = params.auto_capture {
        active_model.auto_capture = Set(auto_capture);
    }

    if let Some(position) = params.position {
        active_model.position = Set(position);
    }

    active_model.updated_at = Set(chrono::Utc::now().into());

    let updated = active_model.update(&ctx.db).await?;

    format::json(updated)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-methods/")
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}", put(update))
}
