#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::coupons::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub code: Option<String>,
    pub discount_type: Option<i32>,
    pub discount_value: Option<Decimal>,
    pub minimum_amount: Option<Decimal>,
    pub maximum_discount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub used_count: Option<i32>,
    pub expires_at: Option<DateTime>,
    pub active: Option<bool>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.code = Set(self.code.clone());
        item.discount_type = Set(self.discount_type);
        item.discount_value = Set(self.discount_value);
        item.minimum_amount = Set(self.minimum_amount);
        item.maximum_discount = Set(self.maximum_discount);
        item.usage_limit = Set(self.usage_limit);
        item.used_count = Set(self.used_count);
        item.expires_at = Set(self.expires_at);
        item.active = Set(self.active);
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidateParams {
    pub code: String,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
pub struct ValidateResult {
    pub valid: bool,
    pub coupon: Option<Model>,
    pub discount: Option<Decimal>,
    pub message: String,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

fn calculate_discount(coupon: &Model, total: Decimal) -> Option<Decimal> {
    match coupon.discount_type.unwrap_or(1) {
        1 => {
            let pct = coupon.discount_value.unwrap_or(Decimal::ZERO) / Decimal::from(100);
            let calculated = total * pct;
            if let Some(max) = coupon.maximum_discount {
                Some(calculated.min(max))
            } else {
                Some(calculated)
            }
        }
        2 => Some(coupon.discount_value.unwrap_or(Decimal::ZERO)),
        _ => None,
    }
}

#[debug_handler]
pub async fn validate(
    State(ctx): State<AppContext>,
    Json(params): Json<ValidateParams>,
) -> Result<Response> {
    let coupon = Entity::find()
        .filter(crate::models::_entities::coupons::Column::Code.eq(Some(params.code.clone())))
        .one(&ctx.db)
        .await?;

    let coupon = match coupon {
        Some(c) => c,
        None => {
            return format::json(ValidateResult {
                valid: false,
                coupon: None,
                discount: None,
                message: "Coupon not found".to_string(),
            })
        }
    };

    if !coupon.active.unwrap_or(false) {
        return format::json(ValidateResult {
            valid: false,
            coupon: None,
            discount: None,
            message: "Coupon is inactive".to_string(),
        });
    }

    if let Some(expires_at) = coupon.expires_at {
        if expires_at < chrono::Utc::now().naive_utc() {
            return format::json(ValidateResult {
                valid: false,
                coupon: None,
                discount: None,
                message: "Coupon has expired".to_string(),
            });
        }
    }

    if let Some(limit) = coupon.usage_limit {
        if limit > 0 && coupon.used_count.unwrap_or(0) >= limit {
            return format::json(ValidateResult {
                valid: false,
                coupon: None,
                discount: None,
                message: "Coupon usage limit reached".to_string(),
            });
        }
    }

    if let Some(min) = coupon.minimum_amount {
        if params.total_amount < min {
            return format::json(ValidateResult {
                valid: false,
                coupon: None,
                discount: None,
                message: format!("Minimum order amount is {}", min),
            });
        }
    }

    let discount = calculate_discount(&coupon, params.total_amount);

    format::json(ValidateResult {
        valid: true,
        coupon: Some(coupon),
        discount,
        message: "Coupon applied successfully".to_string(),
    })
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
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/coupons/")
        .add("/", get(list))
        .add("/", post(add))
        .add("validate", post(validate))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}
