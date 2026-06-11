#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

use crate::models::_entities::admin_audit_logs;
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams};

#[derive(Debug, Deserialize)]
pub struct AuditLogParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub actor_user_id: Option<i32>,
    pub search: Option<String>,
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<AuditLogParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let page_size = pagination.page_size();

    let mut query = admin_audit_logs::Entity::find()
        .order_by_desc(admin_audit_logs::Column::CreatedAt)
        .order_by_desc(admin_audit_logs::Column::Id);

    if let Some(action) = params
        .action
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
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

    if let Some(search) = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let mut condition = Condition::any()
            .add(admin_audit_logs::Column::ActorName.contains(search))
            .add(admin_audit_logs::Column::ActorEmail.contains(search))
            .add(admin_audit_logs::Column::ResourceLabel.contains(search));

        if let Ok(identifier) = search.parse::<i32>() {
            condition = condition
                .add(admin_audit_logs::Column::Id.eq(identifier))
                .add(admin_audit_logs::Column::ActorUserId.eq(identifier))
                .add(admin_audit_logs::Column::ResourceId.eq(identifier));
        }

        query = query.filter(condition);
    }

    let total_fut = query.clone().count(&ctx.db);
    let paginator = query.paginate(&ctx.db, page_size);
    let items_fut = paginator.fetch_page(pagination.page_index());

    let (total, items) = tokio::try_join!(total_fut, items_fut).map_err(|e| {
        tracing::error!(error = ?e, "failed to load admin audit logs in parallel");
        Error::InternalServerError
    })?;

    format::json(AdminPaginatedResponse::new(items, total, pagination))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/audit-logs")
        .add("/", get(list))
}
