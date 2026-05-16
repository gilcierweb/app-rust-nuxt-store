#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryOrder;
use uuid::Uuid;
use crate::middleware::auth::CookieJWT;

use crate::models::_entities::addresses;
use crate::models::_entities::coupon_usages;
use crate::models::_entities::coupons;
use crate::models::_entities::order_items;
use crate::models::_entities::orders::{ActiveModel, Entity};
use crate::models::_entities::payments;
use crate::models::_entities::shipments;
use crate::models::order_status::OrderStatus;
use crate::models::orders::{CreateOrderParams, OrderWithItems, UpdateStatusParams};
use crate::models::users;

fn generate_order_number() -> String {
    let ts = chrono::Utc::now().timestamp();
    let short = &Uuid::new_v4().to_string()[..8];
    format!("ORD-{}-{}", ts, short)
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn checkout(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateOrderParams>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if params.items.is_empty() {
        return Err(Error::BadRequest(t!("cart.empty").into()));
    }

    if let Some(ref code) = params.coupon_code {
        let coupon = coupons::Entity::find()
            .filter(coupons::Column::Code.eq(Some(code.clone())))
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::BadRequest(t!("coupon.not_found").into()))?;

        if !coupon.active.unwrap_or(false) {
            return Err(Error::BadRequest(t!("coupon.inactive").into()));
        }

        if let Some(expires_at) = coupon.expires_at {
            if expires_at < chrono::Utc::now().naive_utc() {
                return Err(Error::BadRequest(t!("coupon.expired").into()));
            }
        }

        if let Some(limit) = coupon.usage_limit {
            if limit > 0 && coupon.used_count.unwrap_or(0) >= limit {
                return Err(Error::BadRequest(t!("coupon.limit_reached").into()));
            }
        }

        if let Some(min) = coupon.minimum_amount {
            if params.total_amount < min {
                return Err(Error::BadRequest(t!("coupon.min_amount", min = min).into()));
            }
        }
    }

    let order_number = generate_order_number();
    let now = chrono::Utc::now().into();

    let order = ActiveModel {
        order_number: Set(Some(order_number)),
        status: Set(Some(OrderStatus::Pending.to_i32())),
        total_amount: Set(Some(params.total_amount)),
        subtotal: Set(Some(params.subtotal)),
        tax_amount: Set(Some(Decimal::ZERO)),
        shipping_amount: Set(params.shipping_amount),
        discount_amount: Set(params.discount_amount),
        currency: Set(Some("BRL".to_string())),
        payment_status: Set(Some(1)),     // unpaid
        fulfillment_status: Set(Some(1)), // unfulfilled
        notes: Set(params.notes),
        user_id: Set(current_user.id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let saved = order.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to create order");
        Error::InternalServerError
    })?;

    for item in &params.items {
        let item_total = item.price * Decimal::from(item.quantity);
        let order_item = order_items::ActiveModel {
            order_id: Set(saved.id),
            product_id: Set(item.product_id),
            product_variant_id: Set(None),
            quantity: Set(Some(item.quantity)),
            price: Set(Some(item.price)),
            total: Set(Some(item_total)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        order_item.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create order item");
            Error::InternalServerError
        })?;
    }

    if params.coupon_code.is_some() {
        let coupon = coupons::Entity::find()
            .filter(coupons::Column::Code.eq(params.coupon_code.clone()))
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::BadRequest(t!("coupon.not_found").into()))?;

        let usage = coupon_usages::ActiveModel {
            coupon_id: Set(coupon.id),
            user_id: Set(current_user.id),
            order_id: Set(saved.id),
            used_at: Set(Some(chrono::Utc::now().naive_utc())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        usage.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to record coupon usage");
            Error::InternalServerError
        })?;

        let current_count = coupon.used_count.unwrap_or(0);
        let mut coupon_active: coupons::ActiveModel = coupon.into();
        coupon_active.used_count = Set(Some(current_count + 1));
        coupon_active.updated_at = Set(now);
        coupon_active.update(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to update coupon count");
            Error::InternalServerError
        })?;
    }

    let mut payment_id = None;
    let mut payment_status = None;

    if let Some(pm_id) = params.payment_method_id {
        let transaction_id = format!("TXN-{}", uuid::Uuid::new_v4());
        let payment = payments::ActiveModel {
            order_id: Set(saved.id),
            payment_method_id: Set(pm_id),
            amount: Set(Some(params.total_amount)),
            currency: Set(Some("BRL".to_string())),
            status: Set(Some(1)),
            transaction_id: Set(Some(transaction_id)),
            processed_at: Set(Some(chrono::Utc::now().naive_utc())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved_payment = payment.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create payment");
            Error::InternalServerError
        })?;
        payment_id = Some(saved_payment.id);
        payment_status = Some(1);

        let mut order_active: crate::models::_entities::orders::ActiveModel = saved.clone().into();
        order_active.payment_status = Set(Some(2));
        order_active.updated_at = Set(chrono::Utc::now().into());
        order_active.update(&ctx.db).await?;
    }

    let mut address_id = None;

    if params.address1.is_some() {
        let addr = addresses::ActiveModel {
            first_name: Set(params.address_first_name.clone()),
            last_name: Set(params.address_last_name.clone()),
            company: Set(params.address_company.clone()),
            address1: Set(params.address1.clone()),
            address2: Set(params.address2.clone()),
            city: Set(params.address_city.clone()),
            state: Set(params.address_state.clone()),
            zip_code: Set(params.address_zip_code.clone()),
            country: Set(params.address_country.clone()),
            phone: Set(params.address_phone.clone()),
            user_id: Set(current_user.id),
            r#type: Set(Some("shipping".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved_addr = addr.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create address");
            Error::InternalServerError
        })?;
        address_id = Some(saved_addr.id);
    }

    let mut shipment_id = None;

    if let Some(sm_id) = params.shipping_method_id {
        let ship = shipments::ActiveModel {
            order_id: Set(saved.id),
            shipping_method_id: Set(sm_id),
            status: Set(Some(1)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved_ship = ship.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create shipment");
            Error::InternalServerError
        })?;
        shipment_id = Some(saved_ship.id);
    }

    format::json(serde_json::json!({
        "id": saved.id,
        "order_number": saved.order_number,
        "status": saved.status,
        "total_amount": saved.total_amount,
        "payment_id": payment_id,
        "payment_status": payment_status,
        "address_id": address_id,
        "shipment_id": shipment_id,
    }))
}

#[debug_handler]
pub async fn my_orders(auth: CookieJWT, State(ctx): State<AppContext>) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let orders: Vec<crate::models::_entities::orders::Model> = Entity::find()
        .filter(crate::models::_entities::orders::Column::UserId.eq(current_user.id))
        .order_by_desc(crate::models::_entities::orders::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(orders)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let orders: Vec<crate::models::_entities::orders::Model> = Entity::find()
        .order_by_desc(crate::models::_entities::orders::Column::CreatedAt)
        .all(&ctx.db)
        .await?;

    format::json(orders)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let order = Entity::find_by_id(id).one(&ctx.db).await?;
    let order = order.ok_or(Error::NotFound)?;
    respond_with_order_items(&ctx, order).await
}

#[debug_handler]
pub async fn account_get_one(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let order = Entity::find_by_id(id).one(&ctx.db).await?;
    let order = order.ok_or(Error::NotFound)?;
    if order.user_id != current_user.id {
        return unauthorized(t!("auth.unauthorized"));
    }

    respond_with_order_items(&ctx, order).await
}

async fn respond_with_order_items(
    ctx: &AppContext,
    order: crate::models::_entities::orders::Model,
) -> Result<Response> {
    let id = order.id;
    let items = order_items::Entity::find()
        .filter(crate::models::_entities::order_items::Column::OrderId.eq(id))
        .find_also_related(crate::models::_entities::products::Entity)
        .all(&ctx.db)
        .await?;

    let items_json = items
        .into_iter()
        .map(|(item, product)| crate::models::orders::OrderItemJson {
            id: item.id,
            product_id: item.product_id,
            quantity: item.quantity,
            price: item.price,
            total: item.total,
            product_name: product.as_ref().and_then(|p| p.name.clone()),
        })
        .collect::<Vec<_>>();

    let response = OrderWithItems {
        id: order.id,
        order_number: order.order_number,
        status: order.status,
        total_amount: order.total_amount,
        subtotal: order.subtotal,
        tax_amount: order.tax_amount,
        shipping_amount: order.shipping_amount,
        discount_amount: order.discount_amount,
        currency: order.currency,
        payment_status: order.payment_status,
        fulfillment_status: order.fulfillment_status,
        notes: order.notes,
        user_id: order.user_id,
        created_at: order.created_at,
        updated_at: order.updated_at,
        items: items_json,
    };

    format::json(response)
}

#[debug_handler]
pub async fn update_status(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateStatusParams>,
) -> Result<Response> {
    let order = Entity::find_by_id(id).one(&ctx.db).await?;
    let order = order.ok_or(Error::NotFound)?;

    let current_status =
        OrderStatus::from_i32(order.status.unwrap_or(1)).unwrap_or(OrderStatus::Pending);
    let new_status = OrderStatus::from_i32(params.status)
        .ok_or_else(|| Error::BadRequest(t!("order.invalid_status").into()))?;

    if !current_status.can_transition_to(new_status) {
        return Err(Error::BadRequest(
            t!(
                "order.invalid_transition",
                from = format!("{:?}", current_status),
                to = format!("{:?}", new_status)
            )
            .into(),
        ));
    }

    let mut active: crate::models::_entities::orders::ActiveModel = order.into();
    active.status = Set(Some(new_status.to_i32()));
    active.updated_at = Set(chrono::Utc::now().into());
    active.update(&ctx.db).await?;

    format::json(serde_json::json!({ "status": new_status.to_i32() }))
}

pub fn routes() -> Routes {
    routes_with_prefix("api/orders")
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/orders")
        .add("/", get(index))
        .add("/list", get(list))
        .add("/{id}", get(get_one))
        .add("/{id}/status", put(update_status))
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/orders")
        .add("/", get(my_orders))
        .add("/checkout", post(checkout))
        .add("/{id}", get(account_get_one))
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(index))
        .add("/checkout", post(checkout))
        .add("/my_orders", get(my_orders))
        .add("/list", get(list))
        .add("/{id}", get(get_one))
        .add("/{id}/status", put(update_status))
}
