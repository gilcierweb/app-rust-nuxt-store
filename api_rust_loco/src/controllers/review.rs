#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::debug_handler;
use axum::extract::Query;

use crate::models::_entities::reviews::{ActiveModel, Entity, Model};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub product_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub rating: Option<i32>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub verified_purchase: Option<bool>,
    pub active: Option<bool>,
    pub user_id: Option<i32>,
    pub product_id: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.rating = Set(self.rating);
        item.title = Set(self.title.clone());
        item.comment = Set(self.comment.clone());
        item.verified_purchase = Set(self.verified_purchase);
        item.active = Set(self.active);
        if let Some(user_id) = self.user_id {
            item.user_id = Set(user_id);
        }
        if let Some(product_id) = self.product_id {
            item.product_id = Set(product_id);
        }
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(query): Query<ListQuery>,
) -> Result<Response> {
    let mut query_builder = Entity::find();
    if let Some(product_id) = query.product_id {
        query_builder = query_builder.filter(crate::models::_entities::reviews::Column::ProductId.eq(product_id));
    }
    format::json(query_builder.all(&ctx.db).await?)
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
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/reviews/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
