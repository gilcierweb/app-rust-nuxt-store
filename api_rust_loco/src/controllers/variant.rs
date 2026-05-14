#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::models::_entities::product_variant_options::Entity as PivotEntity;
use crate::models::_entities::product_variants::{ActiveModel, Entity, Model};
use crate::models::_entities::variant_options::Entity as OptionTypeEntity;

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
        .filter(
            crate::models::_entities::product_variant_options::Column::ProductVariantId
                .eq(variant_id),
        )
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

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
    let variants: Vec<Model> = Entity::find()
        .order_by_asc(crate::models::_entities::product_variants::Column::Position)
        .all(&ctx.db)
        .await?;

    let mut result = Vec::new();
    for v in variants {
        let options = load_variant_options(&ctx, v.id).await?;
        result.push(VariantWithOptions {
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
        });
    }
    format::json(result)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<Vec<(String, String)>>,
) -> Result<Response> {
    let product_id: Option<i32> = params
        .iter()
        .find(|(k, _)| k == "product_id")
        .and_then(|(_, v)| v.parse().ok());

    let mut query =
        Entity::find().order_by_asc(crate::models::_entities::product_variants::Column::Position);
    if let Some(pid) = product_id {
        query = query.filter(crate::models::_entities::product_variants::Column::ProductId.eq(pid));
    }
    let variants: Vec<Model> = query.all(&ctx.db).await?;

    let mut result = Vec::new();
    for v in variants {
        let options = load_variant_options(&ctx, v.id).await?;
        result.push(VariantWithOptions {
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
        });
    }
    format::json(result)
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
