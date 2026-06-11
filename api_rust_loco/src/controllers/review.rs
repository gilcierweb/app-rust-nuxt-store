#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cache::{invalidate_json_cache_with_prefix, json_cache};
use crate::middleware::auth::CookieJWT;
use crate::models::_entities::reviews::{ActiveModel, Column, Entity, Model};
use crate::utils::auth::current_user_id;
use crate::utils::pagination::PaginationParams;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub product_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub rating: Option<i32>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub verified_purchase: Option<bool>,
    pub active: Option<bool>,
    pub product_id: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.rating = Set(self.rating);
        item.title = Set(self.title.clone());
        item.comment = Set(self.comment.clone());
        item.verified_purchase = Set(self.verified_purchase);
        item.active = Set(self.active);
        if let Some(product_id) = self.product_id {
            item.product_id = Set(product_id);
        }
    }
}

const REVIEW_RATING_MIN: i32 = 1;
const REVIEW_RATING_MAX: i32 = 5;

fn validate_rating(rating: Option<i32>) -> Result<()> {
    match rating {
        Some(r) if r >= REVIEW_RATING_MIN && r <= REVIEW_RATING_MAX => Ok(()),
        Some(r) => Err(Error::BadRequest(
            format!(
                "rating must be between {} and {}, got {}",
                REVIEW_RATING_MIN, REVIEW_RATING_MAX, r
            )
            .into(),
        )),
        None => Err(Error::BadRequest("rating is required".into())),
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

async fn load_item_for_user(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find()
        .filter(Column::Id.eq(id))
        .filter(Column::UserId.eq(user_id))
        .one(&ctx.db)
        .await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(query): Query<ListQuery>,
) -> Result<Response> {
    let cache_key = format!(
        "reviews:list:{:?}:{}:{}",
        query.product_id,
        query.pagination.page_index(),
        query.pagination.page_size()
    );
    if let Some(value) = json_cache().get(&cache_key) {
        return format::json(value);
    }
    let mut query_builder = Entity::find();
    if let Some(product_id) = query.product_id {
        query_builder = query_builder.filter(Column::ProductId.eq(product_id));
    }
    let items = query_builder
        .order_by_desc(Column::CreatedAt)
        .paginate(&ctx.db, query.pagination.page_size())
        .fetch_page(query.pagination.page_index())
        .await?;
    let value = Arc::new(serde_json::to_value(&items).map_err(|e| {
        tracing::error!(error = ?e, "failed to serialize reviews");
        Error::InternalServerError
    })?);
    json_cache().insert(cache_key, Arc::clone(&value));
    format::json(value)
}

#[debug_handler]
pub async fn add(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    validate_rating(params.rating)?;

    let user_id = current_user_id(&ctx, &auth).await?;
    let mut item = ActiveModel {
        user_id: Set(user_id),
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    invalidate_json_cache_with_prefix("reviews:list:");
    format::json(item)
}

#[debug_handler]
pub async fn update(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    validate_rating(params.rating)?;

    let user_id = current_user_id(&ctx, &auth).await?;
    let item = load_item_for_user(&ctx, id, user_id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    invalidate_json_cache_with_prefix("reviews:list:");
    format::json(item)
}

#[debug_handler]
pub async fn remove(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let item = load_item_for_user(&ctx, id, user_id).await?;
    item.delete(&ctx.db).await?;
    invalidate_json_cache_with_prefix("reviews:list:");
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

#[debug_handler]
pub async fn account_get_one(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    format::json(load_item_for_user(&ctx, id, user_id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/reviews/")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/reviews/")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/reviews/")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(account_get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}
