#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::models::_entities::payment_gateways;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGatewayParams {
    pub status: Option<i16>,
    pub environment: Option<i16>,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let gateways = payment_gateways::Entity::find()
        .order_by_asc(payment_gateways::Column::Name)
        .all(&ctx.db)
        .await?;

    format::json(gateways)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let gateway = payment_gateways::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(gateway)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateGatewayParams>,
) -> Result<Response> {
    let gateway = payment_gateways::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_model: payment_gateways::ActiveModel = gateway.into();

    if let Some(status) = params.status {
        active_model.status = Set(status);
    }

    if let Some(env) = params.environment {
        active_model.environment = Set(env);
    }

    active_model.updated_at = Set(chrono::Utc::now().into());

    let updated = active_model.update(&ctx.db).await?;

    format::json(updated)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-gateways")
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}", put(update))
}
