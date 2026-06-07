#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use moka::sync::Cache;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, FromQueryResult, QueryFilter, Statement};
use serde::{Deserialize, Serialize};

use crate::models::_entities::product_variant_options::{
    Column as PivotColumn, Entity as PivotEntity, Model as PivotModel,
};
use crate::models::_entities::product_variants::{ActiveModel, Entity, Model};
use crate::models::_entities::variant_options::{
    Entity as OptionTypeEntity, Model as OptionTypeModel,
};
use crate::utils::pagination::PaginationParams;

static VARIANTS_CACHE: OnceLock<Cache<String, Arc<Vec<VariantWithOptions>>>> = OnceLock::new();

fn variants_cache() -> &'static Cache<String, Arc<Vec<VariantWithOptions>>> {
    VARIANTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(128)
            .build()
    })
}

pub(crate) fn invalidate_variants_cache() {
    variants_cache().invalidate_all();
}

#[derive(Debug, FromQueryResult)]
struct VariantJoinedRow {
    id: i32,
    name: Option<String>,
    sku: Option<String>,
    price: Option<Decimal>,
    cost_price: Option<Decimal>,
    compare_price: Option<Decimal>,
    inventory_quantity: Option<i32>,
    weight: Option<Decimal>,
    barcode: Option<String>,
    position: Option<i32>,
    active: Option<bool>,
    product_id: i32,
    option_id: Option<i32>,
    option_name: Option<String>,
    option_value: Option<String>,
}

fn variant_from_row(row: &VariantJoinedRow) -> VariantWithOptions {
    VariantWithOptions {
        id: row.id,
        name: row.name.clone(),
        sku: row.sku.clone(),
        price: row.price,
        cost_price: row.cost_price,
        compare_price: row.compare_price,
        inventory_quantity: row.inventory_quantity,
        weight: row.weight,
        barcode: row.barcode.clone(),
        position: row.position,
        active: row.active,
        product_id: row.product_id,
        options: Vec::new(),
    }
}

fn variants_from_rows(rows: Vec<VariantJoinedRow>) -> Vec<VariantWithOptions> {
    let mut variants = Vec::new();
    let mut positions = HashMap::new();

    for row in rows {
        let index = *positions.entry(row.id).or_insert_with(|| {
            variants.push(variant_from_row(&row));
            variants.len() - 1
        });

        if let Some(option_id) = row.option_id {
            variants[index].options.push(VariantOptionJson {
                id: option_id,
                option_name: row.option_name,
                value: row.option_value,
            });
        }
    }

    variants
}

fn variants_select_sql(from_clause: &str) -> String {
    format!(
        r#"
        SELECT
            pv.id,
            pv.name,
            pv.sku,
            pv.price,
            pv.cost_price,
            pv.compare_price,
            pv.inventory_quantity,
            pv.weight,
            pv.barcode,
            pv.position,
            pv.active,
            pv.product_id,
            pvo.id AS option_id,
            vo.name AS option_name,
            pvo.value AS option_value
        FROM {from_clause} pv
        LEFT JOIN product_variant_options pvo ON pvo.product_variant_id = pv.id
        LEFT JOIN variant_options vo ON vo.id = pvo.variant_option_id
        ORDER BY pv.position ASC, pv.id ASC, pvo.id ASC
        "#
    )
}

fn variants_index_select_sql() -> String {
    variants_select_sql(
        r#"
        (
            SELECT *
            FROM product_variants
            ORDER BY position ASC, id ASC
            LIMIT $1 OFFSET $2
        )
        "#,
    )
}

fn variants_by_product_select_sql() -> String {
    variants_select_sql(
        r#"
        (
            SELECT *
            FROM product_variants
            WHERE product_id = $1
            ORDER BY position ASC, id ASC
            LIMIT $2 OFFSET $3
        )
        "#,
    )
}

fn variants_all_select_sql() -> String {
    variants_index_select_sql()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub sku: Option<String>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub price: Option<Decimal>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub cost_price: Option<Decimal>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub compare_price: Option<Decimal>,
    pub inventory_quantity: Option<i32>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub weight: Option<Decimal>,
    pub barcode: Option<String>,
    pub position: Option<i32>,
    pub active: Option<bool>,
    pub product_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub product_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.sku = Set(self.sku.clone());
        item.price = Set(self.price);
        item.cost_price = Set(self.cost_price);
        item.compare_price = Set(self.compare_price);
        item.inventory_quantity = Set(self.inventory_quantity);
        item.weight = Set(self.weight);
        item.barcode = Set(self.barcode.clone());
        item.position = Set(self.position);
        item.active = Set(self.active);
        item.product_id = Set(self.product_id);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct VariantWithOptions {
    pub id: i32,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub compare_price: Option<Decimal>,
    pub inventory_quantity: Option<i32>,
    pub weight: Option<Decimal>,
    pub barcode: Option<String>,
    pub position: Option<i32>,
    pub active: Option<bool>,
    pub product_id: i32,
    pub options: Vec<VariantOptionJson>,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariantOptionJson {
    pub id: i32,
    pub option_name: Option<String>,
    pub value: Option<String>,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

async fn load_variant_options(ctx: &AppContext, variant_id: i32) -> Result<Vec<VariantOptionJson>> {
    let rows: Vec<(PivotModel, Option<OptionTypeModel>)> = PivotEntity::find()
        .find_also_related(OptionTypeEntity)
        .filter(PivotColumn::ProductVariantId.eq(variant_id))
        .all(&ctx.db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|(pivot, opt)| VariantOptionJson {
            id: pivot.id,
            option_name: opt.as_ref().and_then(|o| o.name.clone()),
            value: pivot.value,
        })
        .collect())
}

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
    let default_pagination = PaginationParams::default();
    let cache_key = format!(
        "index:{}:{}",
        default_pagination.page_index(),
        default_pagination.page_size()
    );
    if let Some(value) = variants_cache().get(&cache_key) {
        return format::json(value);
    }

    let rows = VariantJoinedRow::find_by_statement(Statement::from_sql_and_values(
        ctx.db.get_database_backend(),
        &variants_index_select_sql(),
        [
            (default_pagination.page_size() as i64).into(),
            default_pagination.offset().into(),
        ],
    ))
    .all(&ctx.db)
    .await?;

    let variants = Arc::new(variants_from_rows(rows));
    variants_cache().insert(cache_key, Arc::clone(&variants));
    format::json(variants)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<ListQuery>,
) -> Result<Response> {
    let cache_key = format!(
        "list:{:?}:{}:{}",
        params.product_id,
        params.pagination.page_index(),
        params.pagination.page_size()
    );
    if let Some(value) = variants_cache().get(&cache_key) {
        return format::json(value);
    }

    let rows = if let Some(pid) = params.product_id {
        VariantJoinedRow::find_by_statement(Statement::from_sql_and_values(
            ctx.db.get_database_backend(),
            &variants_by_product_select_sql(),
            [
                pid.into(),
                (params.pagination.page_size() as i64).into(),
                params.pagination.offset().into(),
            ],
        ))
        .all(&ctx.db)
        .await?
    } else {
        VariantJoinedRow::find_by_statement(Statement::from_sql_and_values(
            ctx.db.get_database_backend(),
            &variants_all_select_sql(),
            [
                (params.pagination.page_size() as i64).into(),
                params.pagination.offset().into(),
            ],
        ))
        .all(&ctx.db)
        .await?
    };

    let variants = Arc::new(variants_from_rows(rows));
    variants_cache().insert(cache_key, Arc::clone(&variants));
    format::json(variants)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let v = load_item(&ctx, id).await?;
    let options = load_variant_options(&ctx, v.id).await?;
    format::json(VariantWithOptions {
        id: v.id,
        name: v.name,
        sku: v.sku,
        price: v.price,
        cost_price: v.cost_price,
        compare_price: v.compare_price,
        inventory_quantity: v.inventory_quantity,
        weight: v.weight,
        barcode: v.barcode,
        position: v.position,
        active: v.active,
        product_id: v.product_id,
        options,
    })
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let now = chrono::Utc::now().into();
    item.created_at = Set(now);
    item.updated_at = Set(now);
    let saved = item.insert(&ctx.db).await?;
    invalidate_variants_cache();
    format::json(saved)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    invalidate_variants_cache();
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    invalidate_variants_cache();
    format::empty()
}

pub fn routes() -> Routes {
    routes_with_prefix("api/variants/")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/variants/")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(index))
        .add("list", get(list))
        .add("{id}", get(get_one))
        .add("/", post(add))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
