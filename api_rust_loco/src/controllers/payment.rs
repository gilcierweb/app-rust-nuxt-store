#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::middleware::auth::CookieJWT;
use crate::models::_entities::payment_gateways;
use crate::models::_entities::payment_methods;
use crate::models::_entities::payments::Entity;
use crate::models::_entities::{orders, payments};
use crate::payment_gateways::{
    create_payment_attempt, latest_payment_session_json, order_payment_status,
    CreatePaymentAttemptInput,
};
use crate::utils::auth::current_user_id;

#[derive(Debug, Deserialize)]
pub struct ProcessPaymentParams {
    pub order_id: i32,
    pub payment_method_id: i32,
    #[serde(with = "crate::utils::decimal")]
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
    pub payment_session: Option<Value>,
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
    let payment_session = latest_payment_session_json(&ctx.db, saved.id).await?;

    format::json(PaymentJson {
        id: saved.id,
        order_id: saved.order_id,
        payment_method_id: saved.payment_method_id,
        amount: saved.amount,
        currency: saved.currency,
        status: saved.status,
        transaction_id: saved.transaction_id,
        processed_at: saved.processed_at.map(|dt| dt.and_utc().to_rfc3339()),
        payment_session,
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
            payment_session: latest_payment_session_json(&ctx.db, p.id).await?,
        }),
        None => Err(Error::NotFound),
    }
}

pub fn routes() -> Routes {
    routes_with_prefix("api/payments/")
}

#[debug_handler]
pub async fn bulk_export(
    State(ctx): State<AppContext>,
    Json(params): Json<crate::utils::bulk_export::BulkExportParams>,
) -> Result<Response> {
    if params.ids.is_empty() {
        return Err(Error::BadRequest(t!("payment.no_ids_provided").into()));
    }

    let (zip_bytes, filename) =
        crate::services::bulk_pdf::build_payments_zip(&ctx.db, &params.ids)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to build payments ZIP");
                loco_rs::Error::string(&format!("Bulk export error: {e}"))
            })?;

    Ok(axum::response::Response::builder()
        .header("content-type", "application/zip")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(zip_bytes))
        .unwrap())
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/payments/")
        .add("bulk-export", post(bulk_export))
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("process", post(process))
        .add("order/{order_id}", get(get_by_order))
        .add("methods", get(list_methods))
}

pub fn account_routes() -> Routes {
    routes_with_prefix("api/account/payments/")
        .add("/", get(account_list))
        .add("{id}", get(account_get_one))
        .add("{id}/receipt", get(account_receipt))
}

#[derive(Debug, Serialize)]
pub struct AccountPaymentListItem {
    pub id: i32,
    pub order_id: i32,
    pub order_number: Option<String>,
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    pub status: Option<i32>,
    pub transaction_id: Option<String>,
    pub payment_method_name: Option<String>,
    pub processed_at: Option<String>,
    pub created_at: String,
}

#[debug_handler]
pub async fn account_list(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let uid = current_user_id(&ctx, &auth).await?;

    let payments = Entity::find()
        .find_also_related(orders::Entity)
        .filter(orders::Column::UserId.eq(uid))
        .order_by_desc(payments::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    let mut result: Vec<AccountPaymentListItem> = Vec::with_capacity(payments.len());
    for (payment, order) in payments {
        let payment_method_name = payment_methods::Entity::find_by_id(payment.payment_method_id)
            .one(&ctx.db)
            .await
            .ok()
            .flatten()
            .and_then(|pm| pm.name);

        result.push(AccountPaymentListItem {
            id: payment.id,
            order_id: payment.order_id,
            order_number: order.and_then(|o| o.order_number),
            amount: payment.amount,
            currency: payment.currency,
            status: payment.status,
            transaction_id: payment.transaction_id,
            payment_method_name,
            processed_at: payment.processed_at.map(|dt| dt.and_utc().to_rfc3339()),
            created_at: payment.created_at.to_rfc3339(),
        });
    }

    format::json(result)
}

#[derive(Debug, Serialize)]
pub struct AccountPaymentDetail {
    pub payment: payments::Model,
    pub order_number: Option<String>,
    pub payment_method_name: Option<String>,
    pub gateway_name: Option<String>,
}

#[debug_handler]
pub async fn account_get_one(
    auth: CookieJWT,
    Path(payment_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|v| v.as_i64())
        .and_then(|v| i32::try_from(v).ok())
        .ok_or_else(|| loco_rs::Error::string(&t!("payment.unauthorized")))?;

    let payment = Entity::find_by_id(payment_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let order = orders::Entity::find_by_id(payment.order_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if order.user_id != current_user_id {
        return Err(loco_rs::Error::string(&t!("payment.unauthorized")));
    }

    let payment_method =
        payment_methods::Entity::find_by_id(payment.payment_method_id)
            .one(&ctx.db)
            .await?;

    let gateway_name = if let Some(pm) = &payment_method {
        if let Some(gw_id) = pm.payment_gateway_id {
            payment_gateways::Entity::find_by_id(gw_id)
                .one(&ctx.db)
                .await
                .ok()
                .flatten()
                .map(|gw| gw.name)
        } else {
            None
        }
    } else {
        None
    };

    format::json(AccountPaymentDetail {
        order_number: order.order_number,
        payment_method_name: payment_method.and_then(|pm| pm.name),
        gateway_name,
        payment,
    })
}

pub fn admin_receipt_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payments/")
        .add("{id}/receipt", get(admin_receipt))
}

#[debug_handler]
pub async fn account_receipt(
    auth: CookieJWT,
    Path(payment_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|v| v.as_i64())
        .and_then(|v| i32::try_from(v).ok())
        .ok_or_else(|| loco_rs::Error::string(&t!("payment.unauthorized")))?;

    let data = crate::services::receipt::load_receipt_data(&ctx.db, payment_id, Some(current_user_id))
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to load receipt data");
            e
        })?;

    let pdf_bytes = crate::services::receipt::generate_receipt_pdf(&data)
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to generate receipt PDF");
            loco_rs::Error::string(&format!("PDF generation error: {e}"))
        })?;

    let payment_number = data.payment.number.as_deref().unwrap_or("payment");
    let filename = format!("receipt-{payment_number}.pdf");

    Ok(axum::response::Response::builder()
        .header("content-type", "application/pdf")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(pdf_bytes))
        .unwrap())
}

#[debug_handler]
pub async fn admin_receipt(
    Path(payment_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let data = crate::services::receipt::load_receipt_data(&ctx.db, payment_id, None)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to load receipt data");
            e
        })?;

    let pdf_bytes = crate::services::receipt::generate_receipt_pdf(&data)
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to generate receipt PDF");
            loco_rs::Error::string(&format!("PDF generation error: {e}"))
        })?;

    let payment_number = data.payment.number.as_deref().unwrap_or("payment");
    let filename = format!("receipt-{payment_number}.pdf");

    Ok(axum::response::Response::builder()
        .header("content-type", "application/pdf")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(pdf_bytes))
        .unwrap())
}
