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
    pub inventory_quantity: Option<i32>,
    pub reserved_quantity: i64,
}

#[derive(Debug, Default, FromQueryResult, Serialize)]
pub struct InventorySummaryRow {
    pub total_variants: i64,
    pub total_stock: i64,
    pub total_reserved: i64,
    pub alert_count: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct InventorySummaryJson {
    pub total_variants: i64,
    pub total_stock: i64,
    pub total_reserved: i64,
    pub alert_count: i64,
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

    // Filter values start at placeholder $1. We append the page_size / offset
    // after the filter values so they always land at the end of the SQL.
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
    let items = InventoryItemJson::find_by_statement(Statement::from_sql_and_values(
        backend,
        &items_sql,
        filter_values.clone(),
    ))
    .all(&ctx.db)
    .await?;

    let count_sql = format!(
        "WITH inventory_rows AS ({base})
        SELECT COUNT(*) AS total FROM inventory_rows
        WHERE 1=1{filters}",
        base = inventory_base_sql(),
        filters = filters_sql,
    );
    let total_row = InventoryCountRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        &count_sql,
        filter_values,
    ))
    .one(&ctx.db)
    .await?
    .unwrap_or(InventoryCountRow { total: 0 });
    let total = total_row.total.max(0) as u64;

    let summary_sql = format!(
        "WITH inventory_rows AS ({base}),
        filtered AS (
            SELECT * FROM inventory_rows WHERE 1=1{filters}
        )
        SELECT
            COUNT(*) AS total_variants,
            COALESCE(SUM(inventory_quantity), 0)::BIGINT AS total_stock,
            COALESCE(SUM(reserved_quantity), 0)::BIGINT AS total_reserved,
            COALESCE(SUM(
                CASE WHEN
                    COALESCE(inventory_quantity, 0) <= 0
                    OR (COALESCE(inventory_quantity, 0) - COALESCE(reserved_quantity, 0)) <= 0
                    OR (
                        COALESCE(reserved_quantity, 0) = 0
                        AND (COALESCE(inventory_quantity, 0) - COALESCE(reserved_quantity, 0)) > 0
                        AND (COALESCE(inventory_quantity, 0) - COALESCE(reserved_quantity, 0)) <= {low_stock_threshold}
                    )
                THEN 1 ELSE 0 END
            ), 0)::BIGINT AS alert_count
        FROM filtered",
        base = inventory_base_sql(),
        filters = filters_sql,
    );
    let summary_row = InventorySummaryRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        &summary_sql,
        vec![low_stock_threshold.into()],
    ))
    .one(&ctx.db)
    .await?
    .unwrap_or_default();
    let summary = InventorySummaryJson {
        total_variants: summary_row.total_variants,
        total_stock: summary_row.total_stock,
        total_reserved: summary_row.total_reserved,
        alert_count: summary_row.alert_count,
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
    active_model.inventory_quantity = Set(params.inventory_quantity);
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
        COALESCE(SUM(ci.quantity), 0) AS reserved_quantity
    FROM product_variants pv
    INNER JOIN products p ON p.id = pv.product_id
    LEFT JOIN cart_items ci ON ci.product_variant_id = pv.id
    GROUP BY
        pv.id,
        pv.product_id,
        p.name,
        pv.name,
        pv.sku,
        p.sku,
        pv.active,
        p.active,
        pv.inventory_quantity
    "#
}

fn build_inventory_filters(
    params: &AdminInventoryListParams,
    low_stock_threshold: i64,
    starting_placeholder: usize,
) -> (String, Vec<Value>) {
    let mut conditions = String::new();
    let mut values: Vec<Value> = Vec::new();
    let mut placeholder = starting_placeholder;

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
        placeholder += 1;
    }

    if let Some(status) = params
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let available_expr = "COALESCE(inventory_quantity, 0) - COALESCE(reserved_quantity, 0)";
        let condition = match status {
            "out" => format!(
                "({inventory} <= 0 OR {available} <= 0)",
                inventory = "COALESCE(inventory_quantity, 0)",
                available = available_expr,
            ),
            "reserved" => format!(
                "(COALESCE(reserved_quantity, 0) > 0 AND {available} > 0)",
                available = available_expr,
            ),
            "low" => format!(
                "(COALESCE(reserved_quantity, 0) = 0 AND {available} > 0 AND {available} <= ${placeholder})",
                available = available_expr,
            ),
            "healthy" => format!(
                "(COALESCE(reserved_quantity, 0) = 0 AND {available} > ${placeholder})",
                available = available_expr,
            ),
            _ => String::new(),
        };

        if !condition.is_empty() {
            conditions.push_str(" AND ");
            conditions.push_str(&condition);
            if matches!(status, "low" | "healthy") {
                values.push(low_stock_threshold.into());
                let _ = &mut placeholder;
            }
        }
    }

    (conditions, values)
}
