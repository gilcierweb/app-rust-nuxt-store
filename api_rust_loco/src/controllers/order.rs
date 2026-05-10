#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryOrder;
use uuid::Uuid;

use crate::models::_entities::order_items;
use crate::models::_entities::orders::{ActiveModel, Entity};
use crate::models::orders::{CreateOrderParams, OrderWithItems, UpdateStatusParams};
use crate::models::order_status::OrderStatus;

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
    State(ctx): State<AppContext>,
    Json(params): Json<CreateOrderParams>,
) -> Result<Response> {
    if params.items.is_empty() {
        return Err(Error::BadRequest("Cart is empty".into()));
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
        payment_status: Set(Some(1)), // unpaid
        fulfillment_status: Set(Some(1)), // unfulfilled
        notes: Set(params.notes),
        user_id: Set(1), // default user until auth middleware
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

    format::json(serde_json::json!({
        "id": saved.id,
        "order_number": saved.order_number,
        "status": saved.status,
        "total_amount": saved.total_amount,
    }))
}

#[debug_handler]
pub async fn my_orders(State(ctx): State<AppContext>) -> Result<Response> {
    let orders: Vec<crate::models::_entities::orders::Model> = Entity::find()
        .filter(crate::models::_entities::orders::Column::UserId.eq(1))
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
pub async fn get_one(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let order = Entity::find_by_id(id).one(&ctx.db).await?;
    let order = order.ok_or(Error::NotFound)?;

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

    let current_status = OrderStatus::from_i32(order.status.unwrap_or(1)).unwrap_or(OrderStatus::Pending);
    let new_status = OrderStatus::from_i32(params.status).ok_or_else(|| {
        Error::BadRequest("Invalid status value".into())
    })?;

    if !current_status.can_transition_to(new_status) {
        return Err(Error::BadRequest(format!(
            "Cannot transition from {:?} to {:?}",
            current_status, new_status
        )));
    }

    let mut active: crate::models::_entities::orders::ActiveModel = order.into();
    active.status = Set(Some(new_status.to_i32()));
    active.updated_at = Set(chrono::Utc::now().into());
    active.update(&ctx.db).await?;

    format::json(serde_json::json!({ "status": new_status.to_i32() }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/orders/")
        .add("/", get(index))
        .add("checkout", post(checkout))
        .add("my_orders", get(my_orders))
        .add("list", get(list))
        .add("{id}", get(get_one))
        .add("{id}/status", put(update_status))
}
