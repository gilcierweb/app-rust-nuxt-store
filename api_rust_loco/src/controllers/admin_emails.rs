use axum::debug_handler;
use axum::extract::{Path, Query, State};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};
use serde::{Deserialize, Serialize};

use crate::mailers::auth::AuthMailer;
use crate::mailers::email_service::EmailService;
use crate::models::email_logs;

#[derive(Debug, Deserialize)]
pub struct LogsParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub recipient: Option<String>,
    pub status: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct LogsResponse {
    pub items: Vec<email_logs::Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
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
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    
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

    let total = query.clone().count(&ctx.db).await?;
    
    let paginator = query.paginate(&ctx.db, page_size);
    let items = paginator.fetch_page(page - 1).await?;

    format::json(LogsResponse {
        items,
        total,
        page,
        page_size,
    })
}

#[debug_handler]
pub async fn resend_log(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    EmailService::resend(&ctx, id).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/emails/")
        .add("templates", get(list_templates))
        .add("logs", get(list_logs))
        .add("logs/{id}/resend", post(resend_log))
}
