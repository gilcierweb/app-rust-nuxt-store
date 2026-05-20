#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

use crate::controllers::variant::invalidate_variants_cache;
use crate::models::_entities::product_variants;

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

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryParams {
    pub inventory_quantity: Option<i32>,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = InventoryItemJson::find_by_statement(Statement::from_sql_and_values(
        ctx.db.get_database_backend(),
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
        ORDER BY p.name ASC, pv.position ASC, pv.id ASC
        "#,
        [],
    ))
    .all(&ctx.db)
    .await?;

    format::json(rows)
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
        .prefix("api/admin/inventory/")
        .add("/", get(list))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
