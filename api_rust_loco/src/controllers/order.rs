#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::auth::CookieJWT;
use axum::{debug_handler, extract::Query};
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::{PaginatorTrait, QueryOrder, TransactionTrait};
use uuid::Uuid;

use crate::cache::{json_cache, invalidate_json_cache_with_prefix};
use crate::models::_entities::addresses;
use crate::models::_entities::coupon_usages;
use crate::models::_entities::coupons;
use crate::models::_entities::order_items;
use crate::models::_entities::orders::{ActiveModel, Entity};
use crate::models::_entities::payment_methods;
use crate::models::_entities::product_variants;
use crate::models::_entities::shipments;
use crate::models::_entities::stock_movements;
use crate::models::_entities::stock_reservations;
use crate::models::order_status::{FulfillmentStatus, OrderStatus, PaymentStatus};
use crate::models::orders::{CreateOrderParams, OrderWithItems, ShipmentJson, UpdateStatusParams};
use crate::models::users;
use crate::payment_gateways::{
    create_payment_attempt, latest_payment_session_json, order_payment_status,
    CreatePaymentAttemptInput,
};
use crate::services::cart;
use crate::services::invoice;
use crate::services::nfe;
use crate::services::quotation;
use crate::utils::auth::current_user_id;
use crate::utils::pagination::PaginationParams;
use crate::mailers::order::OrderMailer;
use crate::mailers::email_service::EmailService;

fn generate_order_number() -> String {
    let ts = chrono::Utc::now().timestamp();
    let short = &Uuid::new_v4().to_string()[..8];
    format!("ORD-{}-{}", ts, short)
}

#[debug_handler]
pub async fn checkout(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    body: axum::body::Bytes,
) -> Result<Response> {
    let params: CreateOrderParams = serde_json::from_slice(&body).map_err(|e| {
        let body_str = String::from_utf8_lossy(&body);
        tracing::error!(
            body = %body_str,
            error = %e,
            "checkout body deserialization failed"
        );
        Error::BadRequest(format!("invalid request body: {e}"))
    })?;

    let current_user_id = current_user_id(&ctx, &auth).await?;

    if params.items.is_empty() {
        return Err(Error::BadRequest(t!("cart.empty").into()));
    }

    let coupon = if let Some(ref code) = params.coupon_code {
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

        Some(coupon)
    } else {
        None
    };

    let txn = ctx.db.begin().await?;

    for item in &params.items {
        if let Some(variant_id) = item.product_variant_id {
            let variant = product_variants::Entity::find_by_id(variant_id)
                .one(&txn)
                .await?
                .ok_or_else(|| {
                    Error::BadRequest(
                        t!("order.variant_not_found", id = variant_id).into(),
                    )
                })?;

            if variant.track_inventory {
                let available = variant.inventory_quantity - variant.reserved_quantity;
                if available < item.quantity && !variant.allow_backorder {
                    return Err(Error::BadRequest(
                        t!(
                            "order.insufficient_stock",
                            sku = variant.sku.unwrap_or_default(),
                            available = available
                        )
                        .into(),
                    ));
                }
            }
        }
    }

    let order_number = generate_order_number();
    let order_number_clone = order_number.clone();
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
        payment_status: Set(Some(PaymentStatus::Unpaid.to_i32())),
        fulfillment_status: Set(Some(FulfillmentStatus::Unfulfilled.to_i32())),
        notes: Set(params.notes),
        user_id: Set(current_user_id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let saved = order.insert(&txn).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to create order");
        Error::InternalServerError
    })?;

    for item in &params.items {
        let item_total = item.price * Decimal::from(item.quantity);
        let order_item = order_items::ActiveModel {
            order_id: Set(saved.id),
            product_id: Set(item.product_id),
            product_variant_id: Set(item.product_variant_id),
            quantity: Set(Some(item.quantity)),
            price: Set(Some(item.price)),
            total: Set(Some(item_total)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        order_item.insert(&txn).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create order item");
            Error::InternalServerError
        })?;

        if let Some(variant_id) = item.product_variant_id {
            let variant = product_variants::Entity::find_by_id(variant_id)
                .one(&txn)
                .await?
                .ok_or_else(|| Error::InternalServerError)?;

            let quantity_before = variant.inventory_quantity;
            let new_inventory = quantity_before - item.quantity;

            let mut pv_active: product_variants::ActiveModel = variant.into();
            pv_active.inventory_quantity = Set(new_inventory);
            pv_active.updated_at = Set(now);
            pv_active.update(&txn).await?;

            let movement = stock_movements::ActiveModel {
                product_variant_id: Set(variant_id),
                order_id: Set(Some(saved.id)),
                user_id: Set(Some(current_user_id)),
                r#type: Set("checkout".to_string()),
                quantity: Set(-item.quantity),
                quantity_before: Set(quantity_before),
                quantity_after: Set(new_inventory),
                reference: Set(Some(format!("order:{}", saved.id))),
                notes: Set(Some(format!("Order {} checkout", order_number_clone))),
                created_at: Set(now),
                ..Default::default()
            };
            movement.insert(&txn).await?;
        }
    }

    let cart = cart::get_or_create_cart(&txn, current_user_id).await?;
    cart::release_cart_reservations(&txn, cart.id).await?;
    cart::clear_cart(&txn, cart.id).await?;

    if let Some(coupon) = coupon {
        let usage = coupon_usages::ActiveModel {
            coupon_id: Set(coupon.id),
            user_id: Set(current_user_id),
            order_id: Set(saved.id),
            used_at: Set(Some(chrono::Utc::now().naive_utc())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        usage.insert(&txn).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to record coupon usage");
            Error::InternalServerError
        })?;

        let current_count = coupon.used_count.unwrap_or(0);
        let mut coupon_active: coupons::ActiveModel = coupon.into();
        coupon_active.used_count = Set(Some(current_count + 1));
        coupon_active.updated_at = Set(now);
        coupon_active.update(&txn).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to update coupon count");
            Error::InternalServerError
        })?;
    }

    let mut payment_id = None;
    let mut payment_status = None;
    let mut payment_session = None;

    if let Some(pm_id) = params.payment_method_id {
        let payment_method = payment_methods::Entity::find_by_id(pm_id)
            .one(&txn)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let saved_payment = create_payment_attempt(
            &txn,
            CreatePaymentAttemptInput {
                order_id: saved.id,
                payment_method,
                amount: params.total_amount,
                currency: "BRL".to_string(),
                gateway_payload: params.payment_gateway_payload.clone(),
            },
        )
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to create payment");
            e
        })?;
        payment_id = Some(saved_payment.id);
        payment_status = saved_payment.status;
        payment_session = latest_payment_session_json(&txn, saved_payment.id).await?;

        let mut order_active: crate::models::_entities::orders::ActiveModel = saved.clone().into();
        order_active.payment_status =
            Set(Some(order_payment_status(saved_payment.status).to_i32()));
        order_active.updated_at = Set(chrono::Utc::now().into());
        order_active.update(&txn).await?;
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
            user_id: Set(current_user_id),
            r#type: Set(Some("shipping".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved_addr = addr.insert(&txn).await.map_err(|e| {
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
        let saved_ship = ship.insert(&txn).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to create shipment");
            Error::InternalServerError
        })?;
        shipment_id = Some(saved_ship.id);
    }

    stock_reservations::Entity::delete_many()
        .filter(stock_reservations::Column::CartId.eq(cart.id))
        .filter(stock_reservations::Column::Status.eq("active"))
        .exec(&txn)
        .await?;

    txn.commit().await?;

    // Invalidate admin orders cache so the new order appears immediately.
    invalidate_json_cache_with_prefix("orders:");

    // Send order confirmation email (non-blocking, logged)
    if let Ok(user) = users::Entity::find_by_id(current_user_id)
        .one(&ctx.db)
        .await
    {
        if let Some(u) = user {
            let order_number = saved.order_number.clone();
            let _ = EmailService::send_logged(
                &ctx,
                u.email.clone(),
                "order_confirmation".to_string(),
                format!("Order Confirmation - {}", order_number.as_deref().unwrap_or("")),
                serde_json::json!({
                    "user_id": u.id,
                    "order_id": saved.id,
                    "order_number": saved.order_number,
                }),
                || async {
                    let order_clone = saved.clone();
                    OrderMailer::send_order_confirmation(&ctx, &u, &order_clone).await
                },
            )
            .await;
        }
    }

    format::json(serde_json::json!({
        "id": saved.id,
        "order_number": saved.order_number,
        "status": saved.status,
        "total_amount": saved.total_amount,
        "payment_id": payment_id,
        "payment_status": payment_status,
        "payment_session": payment_session,
        "address_id": address_id,
        "shipment_id": shipment_id,
    }))
}

#[debug_handler]
pub async fn my_orders(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let orders: Vec<crate::models::_entities::orders::Model> = Entity::find()
        .filter(crate::models::_entities::orders::Column::UserId.eq(current_user_id))
        .order_by_desc(crate::models::_entities::orders::Column::CreatedAt)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    format::json(orders)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let cache_key = format!("orders:p{}:s{}", pagination.page_index(), pagination.page_size());

    if let Some(cached) = json_cache().get(&cache_key) {
        return format::json(cached);
    }

    let orders: Vec<crate::models::_entities::orders::Model> = Entity::find()
        .order_by_desc(crate::models::_entities::orders::Column::CreatedAt)
        .order_by_desc(crate::models::_entities::orders::Column::Id)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    let value = std::sync::Arc::new(serde_json::to_value(&orders).unwrap_or_default());
    json_cache().insert(cache_key, value.clone());

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
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let order = Entity::find_by_id(id).one(&ctx.db).await?;
    let order = order.ok_or(Error::NotFound)?;
    if order.user_id != current_user_id {
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

    let shipments = crate::models::_entities::shipments::Entity::find()
        .filter(crate::models::_entities::shipments::Column::OrderId.eq(id))
        .all(&ctx.db)
        .await?;

    let shipments_json = shipments
        .into_iter()
        .map(|s| ShipmentJson {
            id: s.id,
            tracking_number: s.tracking_number,
            carrier: s.carrier,
            status: s.status,
            shipped_at: s.shipped_at.map(|dt| dt.and_utc().to_rfc3339()),
            delivered_at: s.delivered_at.map(|dt| dt.and_utc().to_rfc3339()),
            shipping_method_id: s.shipping_method_id,
            created_at: s.created_at,
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
        shipments: shipments_json,
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

    if new_status == OrderStatus::Cancelled {
        let order_items = order_items::Entity::find()
            .filter(crate::models::_entities::order_items::Column::OrderId.eq(order.id))
            .all(&ctx.db)
            .await?;

        for item in &order_items {
            if let Some(variant_id) = item.product_variant_id {
                let variant = product_variants::Entity::find_by_id(variant_id)
                    .one(&ctx.db)
                    .await?;
                if let Some(v) = variant {
                    let quantity_before = v.inventory_quantity;
                    let restored_qty = item.quantity.unwrap_or(0);
                    let new_inventory = quantity_before + restored_qty;

                    let mut pv_active: product_variants::ActiveModel = v.into();
                    pv_active.inventory_quantity = Set(new_inventory);
                    pv_active.updated_at = Set(chrono::Utc::now().into());
                    pv_active.update(&ctx.db).await?;

                    let movement = stock_movements::ActiveModel {
                        product_variant_id: Set(variant_id),
                        order_id: Set(Some(order.id)),
                        user_id: Set(None),
                        r#type: Set("cancellation".to_string()),
                        quantity: Set(restored_qty),
                        quantity_before: Set(quantity_before),
                        quantity_after: Set(new_inventory),
                        reference: Set(Some(format!("order:{}", order.id))),
                        notes: Set(Some(format!("Order {} cancelled", order.order_number.as_deref().unwrap_or("UNKNOWN")))),
                        created_at: Set(chrono::Utc::now().into()),
                        ..Default::default()
                    };
                    movement.insert(&ctx.db).await?;
                }
            }
        }
    }

    let mut active: crate::models::_entities::orders::ActiveModel = order.into();
    active.status = Set(Some(new_status.to_i32()));
    active.updated_at = Set(chrono::Utc::now().into());
    let updated_order = active.update(&ctx.db).await?;

    // Invalidate admin orders list cache after status change.
    invalidate_json_cache_with_prefix("orders:");

    // Send notification emails based on status transition
    if let Ok(user) = users::Entity::find_by_id(updated_order.user_id)
        .one(&ctx.db)
        .await
    {
        if let Some(u) = user {
            match new_status {
                OrderStatus::Shipped => {
                    let shipment = shipments::Entity::find()
                        .filter(crate::models::_entities::shipments::Column::OrderId.eq(updated_order.id))
                        .one(&ctx.db)
                        .await
                        .ok()
                        .flatten();
                    let order_number = updated_order.order_number.clone();
                    let _ = EmailService::send_logged(
                        &ctx,
                        u.email.clone(),
                        "shipping_update".to_string(),
                        format!("Your order {} has shipped!", order_number.as_deref().unwrap_or("")),
                        serde_json::json!({
                            "user_id": u.id,
                            "order_id": updated_order.id,
                            "order_number": updated_order.order_number,
                        }),
                        || async {
                            let order_clone = updated_order.clone();
                            OrderMailer::send_shipping_update(
                                &ctx,
                                &u,
                                &order_clone,
                                shipment.as_ref().and_then(|s| s.tracking_number.as_deref()),
                                shipment.as_ref().and_then(|s| s.carrier.as_deref()),
                                shipment.as_ref().and_then(|s| s.shipped_at.map(|d| d.format("%B %d, %Y").to_string())),
                            )
                            .await
                        },
                    )
                    .await;
                }
                OrderStatus::Delivered => {
                    let shipment = shipments::Entity::find()
                        .filter(crate::models::_entities::shipments::Column::OrderId.eq(updated_order.id))
                        .one(&ctx.db)
                        .await
                        .ok()
                        .flatten();
                    let order_number = updated_order.order_number.clone();
                    let _ = EmailService::send_logged(
                        &ctx,
                        u.email.clone(),
                        "delivery_confirmation".to_string(),
                        format!("Your order {} has been delivered!", order_number.as_deref().unwrap_or("")),
                        serde_json::json!({
                            "user_id": u.id,
                            "order_id": updated_order.id,
                            "order_number": updated_order.order_number,
                        }),
                        || async {
                            let order_clone = updated_order.clone();
                            OrderMailer::send_delivery_confirmation(
                                &ctx,
                                &u,
                                &order_clone,
                                shipment.as_ref().and_then(|s| s.delivered_at.map(|d| d.format("%B %d, %Y").to_string())),
                                shipment.as_ref().and_then(|s| s.carrier.as_deref()),
                                shipment.as_ref().and_then(|s| s.tracking_number.as_deref()),
                            )
                            .await
                        },
                    )
                    .await;
                }
                _ => {}
            }
        }
    }

    format::json(serde_json::json!({ "status": new_status.to_i32() }))
}

#[debug_handler]
pub async fn account_invoice(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let data = invoice::load_invoice_data(&ctx.db, id, Some(current_user_id)).await?;
    let pdf_bytes = invoice::generate_invoice_pdf(&data)?;

    let filename = format!(
        "invoice-{}.pdf",
        data.order
            .order_number
            .as_deref()
            .unwrap_or("unknown")
    );

    let headers = [
        (
            axum::http::header::CONTENT_TYPE,
            "application/pdf".to_string(),
        ),
        (
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];

    Ok((headers, pdf_bytes).into_response())
}

#[debug_handler]
pub async fn admin_invoice(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let data = invoice::load_invoice_data(&ctx.db, id, None).await?;
    let pdf_bytes = invoice::generate_invoice_pdf(&data)?;

    let filename = format!(
        "invoice-{}.pdf",
        data.order
            .order_number
            .as_deref()
            .unwrap_or("unknown")
    );

    let headers = [
        (
            axum::http::header::CONTENT_TYPE,
            "application/pdf".to_string(),
        ),
        (
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];

    Ok((headers, pdf_bytes).into_response())
}

#[debug_handler]
pub async fn account_quotation(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let data = quotation::load_quotation_data(&ctx.db, id, Some(current_user_id)).await?;
    let pdf_bytes = quotation::generate_quotation_pdf(&data)?;

    let filename = format!(
        "quotation-{}.pdf",
        data.order
            .order_number
            .as_deref()
            .unwrap_or("unknown")
    );

    let headers = [
        (
            axum::http::header::CONTENT_TYPE,
            "application/pdf".to_string(),
        ),
        (
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];

    Ok((headers, pdf_bytes).into_response())
}

#[debug_handler]
pub async fn admin_quotation(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let data = quotation::load_quotation_data(&ctx.db, id, None).await?;
    let pdf_bytes = quotation::generate_quotation_pdf(&data)?;

    let filename = format!(
        "quotation-{}.pdf",
        data.order
            .order_number
            .as_deref()
            .unwrap_or("unknown")
    );

    let headers = [
        (
            axum::http::header::CONTENT_TYPE,
            "application/pdf".to_string(),
        ),
        (
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];

    Ok((headers, pdf_bytes).into_response())
}

// NF-e/DANFE endpoint: admin-only by design (legal compliance).
// The DANFE is a Brazilian fiscal document containing sensitive tax data
// (CNPJ, IE, ICMS/PIS/COFINS rates, SEFAZ authorization). Per Brazilian
// tax regulations, DANFEs are issued by the seller and provided to the
// buyer only upon request or as required by the fiscal operation.
// Customers do not need direct access to this document.
#[debug_handler]
pub async fn admin_nfe(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let data = nfe::load_nfe_data(&ctx.db, id).await?;
    let pdf_bytes = nfe::generate_nfe_pdf(&data)?;

    let filename = format!("danfe-{}.pdf", data.numero);

    let headers = [
        (
            axum::http::header::CONTENT_TYPE,
            "application/pdf".to_string(),
        ),
        (
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];

    Ok((headers, pdf_bytes).into_response())
}

#[debug_handler]
pub async fn bulk_export(
    State(ctx): State<AppContext>,
    Json(params): Json<crate::utils::bulk_export::BulkExportParams>,
) -> Result<Response> {
    if params.ids.is_empty() {
        return Err(Error::BadRequest(t!("order.no_ids_provided").into()));
    }

    let (zip_bytes, filename) = crate::services::bulk_pdf::build_orders_zip(&ctx.db, &params.ids)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to build orders ZIP");
            loco_rs::Error::string(&format!("Bulk export error: {e}"))
        })?;

    Ok(axum::response::Response::builder()
        .header("content-type", "application/zip")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(zip_bytes))
        .map_err(|e| loco_rs::Error::string(&format!("Response build error: {e}")))?)
}

pub fn routes() -> Routes {
    routes_with_prefix("api/orders")
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/orders")
        .add("/", get(list))
        .add("/bulk-export", post(bulk_export))
        .add("/{id}", get(get_one))
        .add("/{id}/status", put(update_status))
        .add("/{id}/invoice", get(admin_invoice))
        .add("/{id}/quotation", get(admin_quotation))
        .add("/{id}/nfe", get(admin_nfe))
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/orders")
        .add("/", get(my_orders))
        .add("/checkout", post(checkout))
        .add("/{id}", get(account_get_one))
        .add("/{id}/invoice", get(account_invoice))
        .add("/{id}/quotation", get(account_quotation))
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(list))
        .add("/checkout", post(checkout))
        .add("/my_orders", get(my_orders))
        .add("/{id}", get(get_one))
        .add("/{id}/status", put(update_status))
}
