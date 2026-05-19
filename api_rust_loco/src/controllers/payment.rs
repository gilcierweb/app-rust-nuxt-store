#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::_entities::payment_gateways;
use crate::models::_entities::payment_methods;
use crate::models::_entities::payments::Entity;
use crate::models::order_status::PaymentStatus;
use crate::models::payment_gateway_status::PaymentAttemptStatus;
use crate::payment_gateways::{create_payment_attempt, CreatePaymentAttemptInput};

#[derive(Debug, Deserialize)]
pub struct ProcessPaymentParams {
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount: Decimal,
    pub payment_gateway_payload: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct PaymentMethodJson {
    pub id: i32,
    pub name: Option<String>,
    pub code: Option<String>,
    pub active: Option<bool>,
    pub method_type: i16,
    pub gateway_driver: Option<String>,
    pub requires_gateway_payload: bool,
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
        .find_also_related(payment_gateways::Entity)
        .all(&ctx.db)
        .await?;

    let result: Vec<PaymentMethodJson> = methods
        .into_iter()
        .map(|(method, gateway)| {
            let gateway_driver = gateway.map(|gateway| gateway.driver);
            let requires_gateway_payload =
                matches!(gateway_driver.as_deref(), Some("braintree" | "getnet"));

            PaymentMethodJson {
                id: method.id,
                name: method.name,
                code: method.code,
                active: method.active,
                method_type: method.method_type,
                gateway_driver,
                requires_gateway_payload,
            }
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

    let payment_method = payment_methods::Entity::find_by_id(params.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let now = chrono::Utc::now();
    let saved = create_payment_attempt(
        &ctx.db,
        CreatePaymentAttemptInput {
            order_id: params.order_id,
            payment_method,
            amount: params.amount,
            currency: "BRL".to_string(),
            gateway_payload: params.payment_gateway_payload,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "failed to process payment");
        e
    })?;

    let mut order_active: crate::models::_entities::orders::ActiveModel = order.into();
    order_active.payment_status = Set(Some(order_payment_status(saved.status).to_i32()));
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
    routes_with_prefix("api/payments/")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/payments/")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("process", post(process))
        .add("order/{order_id}", get(get_by_order))
        .add("methods", get(list_methods))
}

fn order_payment_status(status: Option<i32>) -> PaymentStatus {
    match status
        .and_then(|value| i16::try_from(value).ok())
        .and_then(PaymentAttemptStatus::from_i16)
    {
        Some(PaymentAttemptStatus::Captured) => PaymentStatus::Paid,
        Some(PaymentAttemptStatus::Refunded) => PaymentStatus::Refunded,
        Some(PaymentAttemptStatus::PartiallyRefunded) => PaymentStatus::PartiallyRefunded,
        _ => PaymentStatus::Unpaid,
    }
}
