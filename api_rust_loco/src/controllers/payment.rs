#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::payments::{ActiveModel, Entity};
use crate::models::_entities::payment_methods;

#[derive(Debug, Deserialize)]
pub struct ProcessPaymentParams {
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount: Decimal,
}

#[derive(Debug, Serialize)]
pub struct PaymentMethodJson {
    pub id: i32,
    pub name: Option<String>,
    pub code: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PaymentJson {
    pub id: i32,
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub status: Option<i32>,
    pub transaction_id: Option<String>,
    pub processed_at: Option<String>,
}

#[debug_handler]
pub async fn list_methods(State(ctx): State<AppContext>) -> Result<Response> {
    let methods = payment_methods::Entity::find()
        .filter(payment_methods::Column::Active.eq(true))
        .all(&ctx.db)
        .await?;

    let result: Vec<PaymentMethodJson> = methods
        .into_iter()
        .map(|m| PaymentMethodJson {
            id: m.id,
            name: m.name,
            code: m.code,
            active: m.active,
        })
        .collect();

    format::json(result)
}

#[debug_handler]
pub async fn process(
    State(ctx): State<AppContext>,
    Json(params): Json<ProcessPaymentParams>,
) -> Result<Response> {
    let order = crate::models::_entities::orders::Entity::find_by_id(params.order_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    payment_methods::Entity::find_by_id(params.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let now = chrono::Utc::now();
    let transaction_id = format!("TXN-{}", Uuid::new_v4());

    let payment = ActiveModel {
        order_id: Set(params.order_id),
        payment_method_id: Set(params.payment_method_id),
        amount: Set(Some(params.amount)),
        currency: Set(Some("BRL".to_string())),
        status: Set(Some(1)),
        transaction_id: Set(Some(transaction_id)),
        processed_at: Set(Some(now.naive_utc())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    };

    let saved = payment.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to process payment");
        Error::InternalServerError
    })?;

    let mut order_active: crate::models::_entities::orders::ActiveModel = order.into();
    order_active.payment_status = Set(Some(2));
    order_active.updated_at = Set(now.into());
    order_active.update(&ctx.db).await?;

    format::json(PaymentJson {
        id: saved.id,
        order_id: saved.order_id,
        payment_method_id: saved.payment_method_id,
        amount: saved.amount,
        currency: saved.currency,
        status: saved.status,
        transaction_id: saved.transaction_id,
        processed_at: saved.processed_at.map(|dt| dt.and_utc().to_rfc3339()),
    })
}

#[debug_handler]
pub async fn get_by_order(
    Path(order_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let payment = Entity::find()
        .filter(crate::models::_entities::payments::Column::OrderId.eq(order_id))
        .one(&ctx.db)
        .await?;

    match payment {
        Some(p) => format::json(PaymentJson {
            id: p.id,
            order_id: p.order_id,
            payment_method_id: p.payment_method_id,
            amount: p.amount,
            currency: p.currency,
            status: p.status,
            transaction_id: p.transaction_id,
            processed_at: p.processed_at.map(|dt| dt.and_utc().to_rfc3339()),
        }),
        None => Err(Error::NotFound),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/payments/")
        .add("process", post(process))
        .add("order/{order_id}", get(get_by_order))
        .add("methods", get(list_methods))
}
