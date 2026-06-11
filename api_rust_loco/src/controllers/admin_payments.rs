#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    payment_gateway_events, payment_gateway_logs, payment_gateways, payment_methods,
    payment_refunds, payment_sessions, payments,
};
use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentGatewayEventStatus, PaymentSessionStatus,
};
use crate::payment_gateways::registry::gateway_for_driver;
use crate::payment_gateways::types::{CapturePaymentInput, RefundPaymentInput, VoidPaymentInput};
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams};

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
    #[serde(with = "crate::utils::decimal::opt")]
    pub amount: Option<rust_decimal::Decimal>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GatewayEventsParams {
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct AdminPaymentListParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    pub status: Option<i32>,
    pub gateway_id: Option<i32>,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminPaymentListResponse {
    pub items: Vec<payments::Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub currencies: Vec<String>,
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<AdminPaymentListParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let mut query = payments::Entity::find()
        .order_by_desc(payments::Column::CreatedAt)
        .order_by_desc(payments::Column::Id);

    if let Some(status) = params.status {
        query = query.filter(payments::Column::Status.eq(status));
    }

    if let Some(currency) = params
        .currency
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        query = query.filter(payments::Column::Currency.eq(currency.to_uppercase()));
    }

    if let Some(gateway_id) = params.gateway_id {
        let method_ids = payment_methods::Entity::find()
            .select_only()
            .column(payment_methods::Column::Id)
            .filter(payment_methods::Column::PaymentGatewayId.eq(gateway_id))
            .into_tuple::<i32>()
            .all(&ctx.db)
            .await?;

        if method_ids.is_empty() {
            return format::json(AdminPaymentListResponse {
                items: Vec::new(),
                total: 0,
                page: pagination.page(),
                page_size: pagination.page_size(),
                currencies: Vec::new(),
            });
        }

        query = query.filter(payments::Column::PaymentMethodId.is_in(method_ids));
    }

    if let Some(search) = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let mut condition = Condition::any()
            .add(payments::Column::Number.contains(search))
            .add(payments::Column::TransactionId.contains(search))
            .add(payments::Column::ExternalPaymentId.contains(search))
            .add(payments::Column::ExternalStatus.contains(search))
            .add(payments::Column::IdempotencyKey.contains(search))
            .add(payments::Column::FailureCode.contains(search))
            .add(payments::Column::FailureMessage.contains(search));

        if let Ok(identifier) = search.parse::<i32>() {
            condition = condition
                .add(payments::Column::Id.eq(identifier))
                .add(payments::Column::OrderId.eq(identifier))
                .add(payments::Column::PaymentMethodId.eq(identifier));
        }

        query = query.filter(condition);
    }

    let total_fut = query.clone().count(&ctx.db);
    let currencies_fut = query
        .clone()
        .select_only()
        .column(payments::Column::Currency)
        .into_tuple::<Option<String>>()
        .all(&ctx.db);
    let paginator = query.paginate(&ctx.db, pagination.page_size());
    let items_fut = paginator.fetch_page(pagination.page_index());

    let (total_res, currencies_res, items_res) =
        tokio::try_join!(total_fut, currencies_fut, items_fut).map_err(|e| {
            tracing::error!(error = ?e, "failed to load admin payments list in parallel");
            Error::InternalServerError
        })?;

    let total = total_res;
    let currencies = currencies_res
        .into_iter()
        .flatten()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let items = items_res;

    let response = AdminPaginatedResponse::new(items, total, pagination);

    format::json(AdminPaymentListResponse {
        items: response.items,
        total: response.total,
        page: response.page,
        page_size: response.page_size,
        currencies,
    })
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let payment = payments::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let logs_fut = payment_gateway_logs::Entity::find()
        .filter(payment_gateway_logs::Column::PaymentId.eq(id))
        .order_by_desc(payment_gateway_logs::Column::CreatedAt)
        .all(&ctx.db);

    let sessions_fut = payment_sessions::Entity::find()
        .filter(payment_sessions::Column::PaymentId.eq(id))
        .order_by_desc(payment_sessions::Column::CreatedAt)
        .all(&ctx.db);

    let refunds_fut = payment_refunds::Entity::find()
        .filter(payment_refunds::Column::PaymentId.eq(id))
        .order_by_desc(payment_refunds::Column::CreatedAt)
        .all(&ctx.db);

    let method_fut = payment_methods::Entity::find_by_id(payment.payment_method_id).one(&ctx.db);

    let (logs, sessions, refunds, method) =
        tokio::try_join!(logs_fut, sessions_fut, refunds_fut, method_fut)
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to load payment detail in parallel");
                Error::InternalServerError
            })?;

    let method = method.ok_or_else(|| Error::NotFound)?;

    let events = if let Some(gateway_id) = method.payment_gateway_id {
        payment_gateway_events::Entity::find()
            .filter(payment_gateway_events::Column::PaymentGatewayId.eq(gateway_id))
            .order_by_desc(payment_gateway_events::Column::CreatedAt)
            .limit(50)
            .all(&ctx.db)
            .await?
    } else {
        Vec::new()
    };

    format::json(PaymentDetailJson {
        payment,
        sessions,
        refunds,
        logs,
        events,
    })
}

#[debug_handler]
pub async fn list_gateway_events(
    State(ctx): State<AppContext>,
    Query(params): Query<GatewayEventsParams>,
) -> Result<Response> {
    let limit = params.limit.unwrap_or(200).clamp(1, 500);
    let items = payment_gateway_events::Entity::find()
        .order_by_desc(payment_gateway_events::Column::CreatedAt)
        .limit(limit)
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

#[derive(Debug, Serialize)]
pub struct PaymentHealthSummary {
    pub failed_events_24h: u64,
    pub failed_payments_24h: u64,
    pub stale_processing_count: u64,
    pub expired_sessions_count: u64,
    pub total_events_30d: u64,
    pub total_logs_30d: u64,
    pub gateways_with_recent_failures: Vec<GatewayFailureInfo>,
}

#[derive(Debug, Serialize)]
pub struct GatewayFailureInfo {
    pub gateway_id: i32,
    pub gateway_name: String,
    pub gateway_driver: String,
    pub failed_event_count: u64,
}

#[debug_handler]
pub async fn health_summary(State(ctx): State<AppContext>) -> Result<Response> {
    let now = chrono::Utc::now();
    let h24 = now - chrono::Duration::hours(24);
    let d30 = now - chrono::Duration::days(30);

    let failed_events_24h_fut = payment_gateway_events::Entity::find()
        .filter(
            payment_gateway_events::Column::Status
                .eq(PaymentGatewayEventStatus::Failed.to_i16()),
        )
        .filter(payment_gateway_events::Column::CreatedAt.gt(h24.naive_utc()))
        .count(&ctx.db);

    let failed_payments_24h_fut = payments::Entity::find()
        .filter(
            payments::Column::Status
                .eq(Some(PaymentAttemptStatus::Failed.to_i16() as i32)),
        )
        .filter(payments::Column::CreatedAt.gt(h24.naive_utc()))
        .count(&ctx.db);

    let stale_processing_fut = payments::Entity::find()
        .filter(
            payments::Column::Status.is_in([
                PaymentAttemptStatus::Processing.to_i16() as i32,
                PaymentAttemptStatus::Pending.to_i16() as i32,
            ]),
        )
        .filter(payments::Column::CreatedAt.lt(h24.naive_utc()))
        .count(&ctx.db);

    let expired_sessions_fut = payment_sessions::Entity::find()
        .filter(
            payment_sessions::Column::Status
                .eq(PaymentSessionStatus::Expired.to_i16()),
        )
        .filter(payment_sessions::Column::CreatedAt.gt(h24.naive_utc()))
        .count(&ctx.db);

    let total_events_30d_fut = payment_gateway_events::Entity::find()
        .filter(payment_gateway_events::Column::CreatedAt.gt(d30.naive_utc()))
        .count(&ctx.db);

    let total_logs_30d_fut = payment_gateway_logs::Entity::find()
        .filter(payment_gateway_logs::Column::CreatedAt.gt(d30.naive_utc()))
        .count(&ctx.db);

    let (
        failed_events_24h,
        failed_payments_24h,
        stale_processing_count,
        expired_sessions_count,
        total_events_30d,
        total_logs_30d,
    ) = tokio::try_join!(
        failed_events_24h_fut,
        failed_payments_24h_fut,
        stale_processing_fut,
        expired_sessions_fut,
        total_events_30d_fut,
        total_logs_30d_fut,
    )
    .map_err(|e| {
        tracing::error!(error = ?e, "failed to load payment health summary");
        Error::InternalServerError
    })?;

    // Find gateways with recent failures
    let failed_events_with_gateway = payment_gateway_events::Entity::find()
        .filter(
            payment_gateway_events::Column::Status
                .eq(PaymentGatewayEventStatus::Failed.to_i16()),
        )
        .filter(payment_gateway_events::Column::CreatedAt.gt(h24.naive_utc()))
        .all(&ctx.db)
        .await?;

    let mut gateway_failure_map: std::collections::HashMap<i32, u64> =
        std::collections::HashMap::new();
    for event in &failed_events_with_gateway {
        *gateway_failure_map
            .entry(event.payment_gateway_id)
            .or_insert(0) += 1;
    }

    let mut gateways_with_recent_failures = Vec::new();
    for (gateway_id, count) in &gateway_failure_map {
        if let Ok(Some(gateway)) = payment_gateways::Entity::find_by_id(*gateway_id)
            .one(&ctx.db)
            .await
        {
            gateways_with_recent_failures.push(GatewayFailureInfo {
                gateway_id: *gateway_id,
                gateway_name: gateway.name.clone(),
                gateway_driver: gateway.driver,
                failed_event_count: *count,
            });
        }
    }
    gateways_with_recent_failures.sort_by(|a, b| b.failed_event_count.cmp(&a.failed_event_count));

    format::json(PaymentHealthSummary {
        failed_events_24h,
        failed_payments_24h,
        stale_processing_count,
        expired_sessions_count,
        total_events_30d,
        total_logs_30d,
        gateways_with_recent_failures,
    })
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
    let gateway = payment_gateways::Entity::find_by_id(
        method
            .payment_gateway_id
            .ok_or_else(|| Error::BadRequest("payment method has no gateway".to_string()))?,
    )
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
    let gateway = payment_gateways::Entity::find_by_id(
        method
            .payment_gateway_id
            .ok_or_else(|| Error::BadRequest("payment method has no gateway".to_string()))?,
    )
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
    let gateway = payment_gateways::Entity::find_by_id(
        method
            .payment_gateway_id
            .ok_or_else(|| Error::BadRequest("payment method has no gateway".to_string()))?,
    )
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
        .prefix("api/admin/payments")
        .add("/", get(list))
        .add("/health", get(health_summary))
        .add("{id}", get(get_one))
        .add("{id}/capture", post(capture))
        .add("{id}/void", post(void))
        .add("{id}/refund", post(refund))
}

pub fn gateway_event_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-gateway-events")
        .add("/", get(list_gateway_events))
}

pub fn gateway_log_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-gateway-logs")
        .add("/", get(list_gateway_logs))
}
