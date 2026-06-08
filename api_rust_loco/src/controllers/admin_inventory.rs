#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, FromQueryResult, Statement, Value};
use serde::{Deserialize, Serialize};

use crate::controllers::variant::invalidate_variants_cache;
use crate::models::_entities::product_variants;
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams};

#[derive(Debug, FromQueryResult, Serialize)]
pub struct InventoryItemJson {
    pub variant_id: i32,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub variant_name: Option<String>,
    pub sku: Option<String>,
    pub active: Option<bool>,
    pub inventory_quantity: i32,
    pub reserved_quantity: i32,
    pub available_quantity: i32,
    pub track_inventory: bool,
    pub allow_backorder: bool,
    pub low_stock_threshold: i32,
    pub stock_status: String,
}

#[derive(Debug, Default, FromQueryResult, Serialize)]
pub struct InventorySummaryRow {
    pub total_variants: i64,
    pub total_stock: i64,
    pub total_reserved: i64,
    pub total_available: i64,
    pub alert_count: i64,
    pub out_of_stock_count: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct InventorySummaryJson {
    pub total_variants: i64,
    pub total_stock: i64,
    pub total_reserved: i64,
    pub total_available: i64,
    pub alert_count: i64,
    pub out_of_stock_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct AdminInventoryListParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub low_stock_threshold: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct InventoryListResponse {
    pub items: Vec<InventoryItemJson>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub summary: InventorySummaryJson,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryParams {
    pub inventory_quantity: Option<i32>,
    pub track_inventory: Option<bool>,
    pub allow_backorder: Option<bool>,
    pub low_stock_threshold: Option<i32>,
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<AdminInventoryListParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let low_stock_threshold = params.low_stock_threshold.unwrap_or(5).max(0);
    let backend = ctx.db.get_database_backend();
    let page_size = pagination.page_size();
    let offset = pagination.page_index() * page_size;

    let (filters_sql, mut filter_values) = build_inventory_filters(&params, low_stock_threshold, 1);
    filter_values.push((page_size as i64).into());
    let limit_placeholder = filter_values.len();
    filter_values.push((offset as i64).into());
    let offset_placeholder = filter_values.len();

    let items_sql = format!(
        "WITH inventory_rows AS ({base})
        SELECT *
        FROM inventory_rows
        WHERE 1=1{filters}
        ORDER BY product_name ASC, variant_id ASC
        LIMIT ${limit_placeholder} OFFSET ${offset_placeholder}",
        base = inventory_base_sql(),
        filters = filters_sql,
    );
    let count_sql = format!(
        "WITH inventory_rows AS ({base})
        SELECT COUNT(*) AS total FROM inventory_rows
        WHERE 1=1{filters}",
        base = inventory_base_sql(),
        filters = filters_sql,
    );
    let summary_sql = format!(
        "WITH inventory_rows AS ({base}),
        filtered AS (
            SELECT * FROM inventory_rows WHERE 1=1{filters}
        )
        SELECT
            COUNT(*) AS total_variants,
            COALESCE(SUM(inventory_quantity), 0)::BIGINT AS total_stock,
            COALESCE(SUM(reserved_quantity), 0)::BIGINT AS total_reserved,
            COALESCE(SUM(available_quantity), 0)::BIGINT AS total_available,
            COALESCE(SUM(
                CASE WHEN
                    track_inventory = true
                    AND available_quantity > 0
                    AND available_quantity <= low_stock_threshold
                THEN 1 ELSE 0 END
            ), 0)::BIGINT AS alert_count,
            COALESCE(SUM(
                CASE WHEN
                    track_inventory = true
                    AND available_quantity <= 0
                    AND allow_backorder = false
                THEN 1 ELSE 0 END
            ), 0)::BIGINT AS out_of_stock_count
        FROM filtered",
        base = inventory_base_sql(),
        filters = filters_sql,
    );

    let items_fut = InventoryItemJson::find_by_statement(Statement::from_sql_and_values(
        backend,
        &items_sql,
        filter_values.clone(),
    ))
    .all(&ctx.db);
    let count_fut = InventoryCountRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        &count_sql,
        filter_values,
    ))
    .one(&ctx.db);
    let summary_fut =
        InventorySummaryRow::find_by_statement(Statement::from_sql_and_values(
            backend,
            &summary_sql,
            vec![low_stock_threshold.into()],
        ))
        .one(&ctx.db);

    let (items_res, count_res, summary_res) =
        tokio::try_join!(items_fut, count_fut, summary_fut).map_err(|e| {
            tracing::error!(error = ?e, "failed to load admin inventory in parallel");
            Error::InternalServerError
        })?;

    let items = items_res;
    let total_row = count_res.unwrap_or(InventoryCountRow { total: 0 });
    let total = total_row.total.max(0) as u64;
    let summary_row = summary_res.unwrap_or_default();
    let summary = InventorySummaryJson {
        total_variants: summary_row.total_variants,
        total_stock: summary_row.total_stock,
        total_reserved: summary_row.total_reserved,
        total_available: summary_row.total_available,
        alert_count: summary_row.alert_count,
        out_of_stock_count: summary_row.out_of_stock_count,
    };

    let response = AdminPaginatedResponse::new(items, total, pagination);
    format::json(InventoryListResponse {
        items: response.items,
        total: response.total,
        page: response.page,
        page_size: response.page_size,
        summary,
    })
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateInventoryParams>,
) -> Result<Response> {
    let variant = product_variants::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let mut active_model: product_variants::ActiveModel = variant.into();
    if let Some(qty) = params.inventory_quantity {
        active_model.inventory_quantity = Set(qty);
    }
    if let Some(track) = params.track_inventory {
        active_model.track_inventory = Set(track);
    }
    if let Some(backorder) = params.allow_backorder {
        active_model.allow_backorder = Set(backorder);
    }
    if let Some(threshold) = params.low_stock_threshold {
        active_model.low_stock_threshold = Set(threshold);
    }
    active_model.updated_at = Set(chrono::Utc::now().into());
    let updated = active_model.update(&ctx.db).await?;
    invalidate_variants_cache();

    format::json(updated)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/inventory")
        .add("/", get(list))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}

#[derive(Debug, FromQueryResult)]
struct InventoryCountRow {
    total: i64,
}

fn inventory_base_sql() -> &'static str {
    r#"
    SELECT
        pv.id AS variant_id,
        pv.product_id,
        p.name AS product_name,
        pv.name AS variant_name,
        COALESCE(pv.sku, p.sku) AS sku,
        COALESCE(pv.active, p.active) AS active,
        pv.inventory_quantity,
        pv.reserved_quantity,
        (pv.inventory_quantity - pv.reserved_quantity) AS available_quantity,
        pv.track_inventory,
        pv.allow_backorder,
        pv.low_stock_threshold,
        CASE
            WHEN pv.track_inventory = false THEN 'not_tracked'
            WHEN (pv.inventory_quantity - pv.reserved_quantity) <= 0 AND pv.allow_backorder = false THEN 'out_of_stock'
            WHEN (pv.inventory_quantity - pv.reserved_quantity) <= 0 AND pv.allow_backorder = true THEN 'backorder'
            WHEN pv.track_inventory = true AND (pv.inventory_quantity - pv.reserved_quantity) > 0
                 AND (pv.inventory_quantity - pv.reserved_quantity) <= pv.low_stock_threshold THEN 'low'
            ELSE 'healthy'
        END AS stock_status
    FROM product_variants pv
    INNER JOIN products p ON p.id = pv.product_id
    "#
}

fn build_inventory_filters(
    params: &AdminInventoryListParams,
    _low_stock_threshold: i64,
    starting_placeholder: usize,
) -> (String, Vec<Value>) {
    let mut conditions = String::new();
    let mut values: Vec<Value> = Vec::new();
    let placeholder = starting_placeholder;

    if let Some(search) = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        conditions.push_str(&format!(
            " AND (
                LOWER(COALESCE(product_name, '')) LIKE LOWER(${placeholder})
                OR LOWER(COALESCE(variant_name, '')) LIKE LOWER(${placeholder})
                OR LOWER(COALESCE(sku, '')) LIKE LOWER(${placeholder})
            )"
        ));
        values.push(format!("%{search}%").into());
        let _ = placeholder;
    }

    if let Some(status) = params
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let condition = match status {
            "out" => "(stock_status = 'out_of_stock')".to_string(),
            "backorder" => "(stock_status = 'backorder')".to_string(),
            "reserved" => "(reserved_quantity > 0 AND available_quantity > 0)".to_string(),
            "low" => "(stock_status = 'low')".to_string(),
            "healthy" => "(stock_status = 'healthy')".to_string(),
            "not_tracked" => "(stock_status = 'not_tracked')".to_string(),
            _ => String::new(),
        };

        if !condition.is_empty() {
            conditions.push_str(" AND ");
            conditions.push_str(&condition);
        }
    }

    (conditions, values)
}
