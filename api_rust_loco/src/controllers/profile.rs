#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, Condition, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cache::{invalidate_profiles_cache, profile_detail_cache, profiles_cache};
use crate::models::_entities::profiles::{ActiveModel, Entity, Model};
use crate::utils::pagination::{AdminPaginatedResponse, AdminPaginationParams, PaginationParams};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub phone: Option<i64>,
    pub birth_date: Option<Date>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub whatsapp: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AdminProfileListParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.first_name = Set(self.first_name.clone());
        item.last_name = Set(self.last_name.clone());
        item.full_name = Set(self.full_name.clone());
        item.username = Set(self.username.clone());
        item.nickname = Set(self.nickname.clone());
        item.phone = Set(self.phone.clone());
        item.birth_date = Set(self.birth_date.clone());
        item.avatar = Set(self.avatar.clone());
        item.bio = Set(self.bio.clone());
        item.whatsapp = Set(self.whatsapp.clone());
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
    if let Some(value) = profiles_cache().get(&cache_key) {
        return format::json(value);
    }

    let items = Entity::find()
        .order_by_asc(crate::models::_entities::profiles::Column::Id)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    let items = Arc::new(items);
    profiles_cache().insert(cache_key, Arc::clone(&items));
    format::json(items)
}

#[debug_handler]
pub async fn admin_list(
    State(ctx): State<AppContext>,
    Query(params): Query<AdminProfileListParams>,
) -> Result<Response> {
    let pagination = AdminPaginationParams::new(params.page, params.page_size);
    let mut query = Entity::find()
        .order_by_desc(crate::models::_entities::profiles::Column::CreatedAt)
        .order_by_desc(crate::models::_entities::profiles::Column::Id);

    if let Some(search) = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let mut condition = Condition::any()
            .add(crate::models::_entities::profiles::Column::FirstName.contains(search))
            .add(crate::models::_entities::profiles::Column::LastName.contains(search))
            .add(crate::models::_entities::profiles::Column::FullName.contains(search))
            .add(crate::models::_entities::profiles::Column::Username.contains(search));

        if let Ok(identifier) = search.parse::<i32>() {
            condition = condition
                .add(crate::models::_entities::profiles::Column::Id.eq(identifier))
                .add(crate::models::_entities::profiles::Column::UserId.eq(identifier));
        }

        query = query.filter(condition);
    }

    let total_fut = query.clone().count(&ctx.db);
    let paginator = query.paginate(&ctx.db, pagination.page_size());
    let items_fut = paginator.fetch_page(pagination.page_index());

    let (total, items) = tokio::try_join!(total_fut, items_fut).map_err(|e| {
        tracing::error!(error = ?e, "failed to load admin profiles list in parallel");
        Error::InternalServerError
    })?;

    format::json(AdminPaginatedResponse::new(items, total, pagination))
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    invalidate_profiles_cache();
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
    invalidate_profiles_cache();
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    invalidate_profiles_cache();
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let cache_key = format!("detail:{id}");
    if let Some(value) = profile_detail_cache().get(&cache_key) {
        return format::json(value);
    }

    let item = Arc::new(load_item(&ctx, id).await?);
    profile_detail_cache().insert(cache_key, Arc::clone(&item));
    format::json(item)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/profiles")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/profiles")
        .add("/", get(admin_list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}
