#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::models::_entities::product_variant_options::{
    Column as PivotColumn, Entity as PivotEntity,
};
use crate::models::_entities::product_variants::{
    ActiveModel, Column as VariantColumn, Entity, Model,
};
use crate::models::_entities::variant_options::{
    Column as OptionColumn, Entity as OptionTypeEntity,
};
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
    let pivots = PivotEntity::find()
        .filter(PivotColumn::ProductVariantId.eq(variant_id))
        .all(&ctx.db)
        .await?;

    let mut result = Vec::new();
    for p in pivots {
        let opt = OptionTypeEntity::find_by_id(p.variant_option_id)
            .one(&ctx.db)
            .await?;
        result.push(VariantOptionJson {
            id: p.id,
            option_name: opt.as_ref().and_then(|o| o.name.clone()),
            value: p.value.clone(),
        });
    }
    Ok(result)
}

async fn load_options_by_variant(
    ctx: &AppContext,
    variant_ids: &[i32],
) -> Result<HashMap<i32, Vec<VariantOptionJson>>> {
    if variant_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let pivots = PivotEntity::find()
        .filter(PivotColumn::ProductVariantId.is_in(variant_ids.to_vec()))
        .all(&ctx.db)
        .await?;

    if pivots.is_empty() {
        return Ok(HashMap::new());
    }

    let option_ids: Vec<i32> = pivots.iter().map(|p| p.variant_option_id).collect();
    let options = OptionTypeEntity::find()
        .filter(OptionColumn::Id.is_in(option_ids))
        .all(&ctx.db)
        .await?;

    let option_names: HashMap<i32, Option<String>> =
        options.into_iter().map(|o| (o.id, o.name)).collect();
    let mut by_variant: HashMap<i32, Vec<VariantOptionJson>> = HashMap::new();

    for pivot in pivots {
        by_variant
            .entry(pivot.product_variant_id)
            .or_default()
            .push(VariantOptionJson {
                id: pivot.id,
                option_name: option_names
                    .get(&pivot.variant_option_id)
                    .cloned()
                    .flatten(),
                value: pivot.value,
            });
    }

    Ok(by_variant)
}

async fn variants_with_options(
    ctx: &AppContext,
    variants: Vec<Model>,
) -> Result<Vec<VariantWithOptions>> {
    let variant_ids: Vec<i32> = variants.iter().map(|v| v.id).collect();
    let mut options_by_variant = load_options_by_variant(ctx, &variant_ids).await?;

    Ok(variants
        .into_iter()
        .map(|v| VariantWithOptions {
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
            options: options_by_variant.remove(&v.id).unwrap_or_default(),
        })
        .collect())
}

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
    let variants: Vec<Model> = Entity::find()
        .order_by_asc(VariantColumn::Position)
        .paginate(&ctx.db, PaginationParams::default().page_size())
        .fetch_page(PaginationParams::default().page_index())
        .await?;

    format::json(variants_with_options(&ctx, variants).await?)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<ListQuery>,
) -> Result<Response> {
    let mut query = Entity::find().order_by_asc(VariantColumn::Position);
    if let Some(pid) = params.product_id {
        query = query.filter(VariantColumn::ProductId.eq(pid));
    }
    let variants: Vec<Model> = query
        .paginate(&ctx.db, params.pagination.page_size())
        .fetch_page(params.pagination.page_index())
        .await?;

    format::json(variants_with_options(&ctx, variants).await?)
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
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
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
