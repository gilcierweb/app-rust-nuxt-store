#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cache::{invalidate_posts_cache, post_detail_cache, posts_cache};
use crate::models::{
    _entities::posts::{ActiveModel, Entity, Model},
    post_status::PostStatus,
};
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<PostStatus>,
    pub user_id: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
        item.status = Set(self.status.map(|s| s.into()));

        if let Some(user_id) = self.user_id {
            item.user_id = Set(user_id);
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
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let cache_key = format!(
        "list:{}:{}",
        pagination.page_index(),
        pagination.page_size()
    );
    if let Some(value) = posts_cache().get(&cache_key) {
        return format::json(value);
    }

    let items = Entity::find()
        .order_by_desc(crate::models::_entities::posts::Column::CreatedAt)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    let items = Arc::new(items);
    posts_cache().insert(cache_key, Arc::clone(&items));
    format::json(items)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    invalidate_posts_cache();
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
    invalidate_posts_cache();
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    invalidate_posts_cache();
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let cache_key = format!("detail:{id}");
    if let Some(value) = post_detail_cache().get(&cache_key) {
        return format::json(value);
    }

    let item = Arc::new(load_item(&ctx, id).await?);
    post_detail_cache().insert(cache_key, Arc::clone(&item));
    format::json(item)
}

pub fn routes() -> Routes {
    routes_with_prefix("api/posts/")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/posts/")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
