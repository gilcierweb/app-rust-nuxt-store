#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::CookieJWT;
use crate::models::_entities::addresses::{ActiveModel, Column, Entity, Model};
use crate::models::users;
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub r#type: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub default: Option<bool>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.r#type = Set(self.r#type.clone());
        item.first_name = Set(self.first_name.clone());
        item.last_name = Set(self.last_name.clone());
        item.company = Set(self.company.clone());
        item.address1 = Set(self.address1.clone());
        item.address2 = Set(self.address2.clone());
        item.city = Set(self.city.clone());
        item.state = Set(self.state.clone());
        item.zip_code = Set(self.zip_code.clone());
        item.country = Set(self.country.clone());
        item.phone = Set(self.phone.clone());
        item.default = Set(self.default);
    }
}

async fn current_user_id(ctx: &AppContext, auth: &CookieJWT) -> Result<i32> {
    if let Some(user_id) = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
    {
        return Ok(user_id);
    }

    Ok(users::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await?
        .id)
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
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let items = Entity::find()
        .order_by_desc(Column::CreatedAt)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    format::json(items)
}

#[debug_handler]
pub async fn add(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let mut item = ActiveModel {
        user_id: Set(user_id),
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let user_id = current_user_id(&ctx, &auth).await?;
    let item = load_item_for_user(&ctx, id, user_id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
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
        .prefix("api/addresses/")
        .add("/", get(list))
        .add("/{id}", get(get_one))
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/addresses/")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/addresses/")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(account_get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
}
