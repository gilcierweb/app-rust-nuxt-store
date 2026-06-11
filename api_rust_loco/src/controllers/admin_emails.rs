use axum::debug_handler;
use axum::extract::{Path, Query, State};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

use crate::mailers::auth::AuthMailer;
use crate::mailers::order::OrderMailer;
use crate::mailers::email_service::EmailService;
use crate::models::email_logs;
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams};

#[derive(Debug, Deserialize)]
pub struct LogsParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub recipient: Option<String>,
    pub status: Option<i16>,
}

#[debug_handler]
pub async fn list_templates(State(_ctx): State<AppContext>) -> Result<Response> {
    let cache_key = "email_templates:all";
    if let Some(cached) = crate::cache::json_cache().get(cache_key) {
        return format::json(cached);
    }

    let mut templates = AuthMailer::get_all_templates();
    templates.extend(OrderMailer::get_all_templates());

    let value = std::sync::Arc::new(serde_json::to_value(&templates).unwrap_or_default());
    crate::cache::json_cache().insert(cache_key.to_string(), value);

    format::json(templates)
}

#[debug_handler]
pub async fn list_logs(
    State(ctx): State<AppContext>,
    Query(params): Query<LogsParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let page_size = pagination.page_size();

    let cache_key = format!("email_logs:admin:p{}:s{}:r{:?}:st{:?}", 
        pagination.page_index(), page_size, params.recipient, params.status);
    if let Some(cached) = crate::cache::json_cache().get(&cache_key) {
        return format::json(cached);
    }

    let mut query = email_logs::Entity::find();

    if let Some(ref recipient) = params.recipient {
        let recipient_trimmed = recipient.trim();
        if !recipient_trimmed.is_empty() {
            query = query.filter(email_logs::Column::Recipient.contains(recipient_trimmed));
        }
    }

    if let Some(status) = params.status {
        query = query.filter(email_logs::Column::Status.eq(status));
    }

    // Order by newest logs first
    query = query.order_by_desc(email_logs::Column::Id);

    let total_fut = query.clone().count(&ctx.db);
    let paginator = query.paginate(&ctx.db, page_size);
    let items_fut = paginator.fetch_page(pagination.page_index());

    let (total, items) = tokio::try_join!(total_fut, items_fut).map_err(|e| {
        tracing::error!(error = ?e, "failed to load admin email logs in parallel");
        Error::InternalServerError
    })?;

    let response = AdminPaginatedResponse::new(items, total, pagination);
    let value = std::sync::Arc::new(serde_json::to_value(&response).unwrap_or_default());
    crate::cache::json_cache().insert(cache_key, value);

    format::json(response)
}

#[debug_handler]
pub async fn resend_log(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    EmailService::resend(&ctx, id).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/emails")
        .add("templates", get(list_templates))
        .add("logs", get(list_logs))
        .add("logs/{id}/resend", post(resend_log))
}
