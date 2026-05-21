#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::models::_entities::admin_audit_logs;

#[derive(Debug, Deserialize)]
pub struct AuditLogParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub actor_user_id: Option<i32>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogsResponse {
    pub items: Vec<admin_audit_logs::Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<AuditLogParams>,
) -> Result<Response> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(25).clamp(1, 100);

    let mut query = admin_audit_logs::Entity::find()
        .order_by_desc(admin_audit_logs::Column::CreatedAt)
        .order_by_desc(admin_audit_logs::Column::Id);

    if let Some(action) = params.action.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        query = query.filter(admin_audit_logs::Column::Action.eq(action));
    }

    if let Some(resource_type) = params
        .resource_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        query = query.filter(admin_audit_logs::Column::ResourceType.eq(resource_type));
    }

    if let Some(actor_user_id) = params.actor_user_id {
        query = query.filter(admin_audit_logs::Column::ActorUserId.eq(actor_user_id));
    }

    if let Some(search) = params.search.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        query = query.filter(
            Condition::any()
                .add(admin_audit_logs::Column::ActorName.contains(search))
                .add(admin_audit_logs::Column::ActorEmail.contains(search))
                .add(admin_audit_logs::Column::Action.contains(search))
                .add(admin_audit_logs::Column::ResourceType.contains(search))
                .add(admin_audit_logs::Column::ResourceLabel.contains(search))
                .add(admin_audit_logs::Column::Message.contains(search)),
        );
    }

    let total = query.clone().count(&ctx.db).await?;
    let items = query.paginate(&ctx.db, page_size).fetch_page(page - 1).await?;

    format::json(AuditLogsResponse {
        items,
        total,
        page,
        page_size,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/audit-logs/")
        .add("/", get(list))
}
