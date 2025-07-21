#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::categories::Entity as Categories;
use crate::models::_entities::products::{ActiveModel, Entity, Model};
use crate::models::products::{ProductWithCategory, Products};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub sku: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub compare_price: Option<Decimal>,
    pub featured: Option<bool>,
    pub active: Option<bool>,
    pub status: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.slug = Set(self.slug.clone());
        item.sku = Set(self.sku.clone());
        item.short_description = Set(self.short_description.clone());
        item.description = Set(self.description.clone());
        item.price = Set(self.price.clone());
        item.cost_price = Set(self.cost_price.clone());
        item.compare_price = Set(self.compare_price.clone());
        item.featured = Set(self.featured.clone());
        item.active = Set(self.active.clone());
        item.status = Set(self.status.clone());
    }
}

pub async fn get_products_with_categories(
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let products = Products::find()
        .find_also_related(Categories)
        .all(&ctx.db)
        .await?;

    let data: Vec<ProductWithCategory> = products.into_iter().map(Into::into).collect();

    format::json(data)
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
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

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let result = Products::find_by_id(id)
        .find_also_related(Categories)
        .one(&ctx.db)
        .await?;

    match result {
        Some((product, category)) => format::json(ProductWithCategory::from((product, category))),
        None => Err(Error::NotFound),
    }

    // format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/")
        // .add("/", get(list))
        .add("/", get(get_products_with_categories))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
