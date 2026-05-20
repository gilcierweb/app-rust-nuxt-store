#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    payment_gateway_events, payment_gateway_logs, payment_gateways, payment_methods,
    payment_refunds, payment_sessions, payments,
};
use crate::payment_gateways::registry::gateway_for_driver;
use crate::payment_gateways::types::{CapturePaymentInput, RefundPaymentInput, VoidPaymentInput};

#[derive(Serialize)]
pub struct PaymentDetailJson {
    pub payment: payments::Model,
    pub sessions: Vec<payment_sessions::Model>,
    pub refunds: Vec<payment_refunds::Model>,
    pub logs: Vec<payment_gateway_logs::Model>,
    pub events: Vec<payment_gateway_events::Model>,
}

#[derive(Deserialize)]
pub struct RefundParams {
    pub amount: Option<rust_decimal::Decimal>,
    pub reason: Option<String>,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let items = payments::Entity::find()
        .order_by_desc(payments::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(items)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let payment = payments::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let logs = payment_gateway_logs::Entity::find()
        .filter(payment_gateway_logs::Column::PaymentId.eq(id))
        .order_by_desc(payment_gateway_logs::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let sessions = payment_sessions::Entity::find()
        .filter(payment_sessions::Column::PaymentId.eq(id))
        .order_by_desc(payment_sessions::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let refunds = payment_refunds::Entity::find()
        .filter(payment_refunds::Column::PaymentId.eq(id))
        .order_by_desc(payment_refunds::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    // To get events we find the payment's gateway id
    let method = payment_methods::Entity::find_by_id(payment.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut events = Vec::new();
    if let Some(gateway_id) = method.payment_gateway_id {
        events = payment_gateway_events::Entity::find()
            .filter(payment_gateway_events::Column::PaymentGatewayId.eq(gateway_id))
            .order_by_desc(payment_gateway_events::Column::CreatedAt)
            .limit(50) // Assuming we only want recent events or we'd filter by external_payment_id if we parsed the payload
            .all(&ctx.db)
            .await?;
    }

    format::json(PaymentDetailJson {
        payment,
        sessions,
        refunds,
        logs,
        events,
    })
}

#[debug_handler]
pub async fn list_gateway_events(State(ctx): State<AppContext>) -> Result<Response> {
    let items = payment_gateway_events::Entity::find()
        .order_by_desc(payment_gateway_events::Column::CreatedAt)
        .limit(200)
        .all(&ctx.db)
        .await?;

    format::json(items)
}

#[debug_handler]
pub async fn list_gateway_logs(State(ctx): State<AppContext>) -> Result<Response> {
    let items = payment_gateway_logs::Entity::find()
        .order_by_desc(payment_gateway_logs::Column::CreatedAt)
        .limit(200)
        .all(&ctx.db)
        .await?;

    format::json(items)
}

#[debug_handler]
pub async fn capture(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let payment = payments::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let method = payment_methods::Entity::find_by_id(payment.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let gateway = payment_gateways::Entity::find_by_id(method.payment_gateway_id.unwrap())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let driver = gateway_for_driver(&gateway.driver).map_err(|e| Error::Message(e.to_string()))?;

    let result = driver
        .capture_payment(CapturePaymentInput {
            payment_id: payment.id,
            amount: payment.amount.unwrap_or_default(),
            currency: payment.currency.clone().unwrap_or_default(),
            external_payment_id: payment.external_payment_id.clone().unwrap_or_default(),
            idempotency_key: payment
                .idempotency_key
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        })
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    // Update status based on result...
    let mut active_payment: payments::ActiveModel = payment.into();
    active_payment.status = sea_orm::ActiveValue::Set(Some(result.status.to_i16() as i32));
    active_payment.transaction_id = sea_orm::ActiveValue::Set(result.external_payment_id);
    use sea_orm::ActiveModelTrait;
    let updated = active_payment.update(&ctx.db).await?;

    format::json(updated)
}

#[debug_handler]
pub async fn void(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let payment = payments::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let method = payment_methods::Entity::find_by_id(payment.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let gateway = payment_gateways::Entity::find_by_id(method.payment_gateway_id.unwrap())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let driver = gateway_for_driver(&gateway.driver).map_err(|e| Error::Message(e.to_string()))?;

    let result = driver
        .void_payment(VoidPaymentInput {
            payment_id: payment.id,
            external_payment_id: payment.external_payment_id.clone().unwrap_or_default(),
            idempotency_key: payment
                .idempotency_key
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        })
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    let mut active_payment: payments::ActiveModel = payment.into();
    active_payment.status = sea_orm::ActiveValue::Set(Some(result.status.to_i16() as i32));
    use sea_orm::ActiveModelTrait;
    let updated = active_payment.update(&ctx.db).await?;

    format::json(updated)
}

#[debug_handler]
pub async fn refund(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<RefundParams>,
) -> Result<Response> {
    let payment = payments::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let method = payment_methods::Entity::find_by_id(payment.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let gateway = payment_gateways::Entity::find_by_id(method.payment_gateway_id.unwrap())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let driver = gateway_for_driver(&gateway.driver).map_err(|e| Error::Message(e.to_string()))?;

    let result = driver
        .refund_payment(RefundPaymentInput {
            payment_id: payment.id,
            amount: params
                .amount
                .unwrap_or_else(|| payment.amount.unwrap_or_default()),
            currency: payment.currency.clone().unwrap_or_default(),
            external_payment_id: payment.external_payment_id.clone().unwrap_or_default(),
            idempotency_key: payment
                .idempotency_key
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        })
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    let mut active_payment: payments::ActiveModel = payment.into();
    active_payment.status = sea_orm::ActiveValue::Set(Some(result.status.to_i16() as i32));
    use sea_orm::ActiveModelTrait;
    let updated = active_payment.update(&ctx.db).await?;

    format::json(updated)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payments/")
        .add("/", get(list))
        .add("{id}", get(get_one))
        .add("{id}/capture", post(capture))
        .add("{id}/void", post(void))
        .add("{id}/refund", post(refund))
}

pub fn gateway_event_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-gateway-events/")
        .add("/", get(list_gateway_events))
}

pub fn gateway_log_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-gateway-logs/")
        .add("/", get(list_gateway_logs))
}
