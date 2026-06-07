use axum::debug_handler;
use axum::extract::{Path, Query, State};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

use crate::mailers::auth::AuthMailer;
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
    let templates = AuthMailer::get_all_templates();
    format::json(templates)
}

#[debug_handler]
pub async fn list_logs(
    State(ctx): State<AppContext>,
    Query(params): Query<LogsParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let page_size = pagination.page_size();

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

    format::json(AdminPaginatedResponse::new(items, total, pagination))
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
