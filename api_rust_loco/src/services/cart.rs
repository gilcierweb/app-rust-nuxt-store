use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult,
    QueryFilter, Statement,
};
use serde::Serialize;

use crate::models::_entities::cart_items;
use crate::models::_entities::carts;
use crate::models::_entities::product_variants;
use crate::models::_entities::stock_reservations;

#[derive(Debug, Serialize)]
pub struct CartItemWithProduct {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub product_variant_id: Option<i32>,
    pub quantity: i32,
    #[serde(with = "crate::utils::decimal")]
    pub price: Decimal,
    pub product_name: Option<String>,
    pub product_slug: Option<String>,
    pub product_image: Option<String>,
    pub in_stock: bool,
    pub available_quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct CartWithItems {
    pub id: i32,
    pub items: Vec<CartItemWithProduct>,
}

pub async fn get_or_create_cart<C>(db: &C, user_id: i32) -> Result<carts::Model>
where
    C: ConnectionTrait,
{
    let existing = carts::Entity::find()
        .filter(carts::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    if let Some(cart) = existing {
        return Ok(cart);
    }

    let now = chrono::Utc::now().into();
    let cart = carts::ActiveModel {
        user_id: Set(user_id),
        session_id: Set(None),
        expires_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    let saved = cart.insert(db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to create cart");
        Error::InternalServerError
    })?;
    Ok(saved)
}

pub async fn get_cart_with_items_for_user<C>(db: &C, user_id: i32) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    #[derive(Debug, FromQueryResult)]
    struct CartItemRow {
        id: i32,
        cart_id: i32,
        product_id: i32,
        product_variant_id: Option<i32>,
        quantity: Option<i32>,
        price: Option<Decimal>,
        product_name: Option<String>,
        product_slug: Option<String>,
        product_image: Option<String>,
        inventory_quantity: Option<i32>,
        reserved_quantity: Option<i32>,
        track_inventory: Option<bool>,
        allow_backorder: Option<bool>,
    }

    let backend = db.get_database_backend();

    let items_sql = r#"
        SELECT
            ci.id              AS id,
            ci.cart_id         AS cart_id,
            ci.product_id      AS product_id,
            ci.product_variant_id AS product_variant_id,
            ci.quantity        AS quantity,
            ci.price           AS price,
            p.name             AS product_name,
            p.slug             AS product_slug,
            cover.image        AS product_image,
            pv.inventory_quantity AS inventory_quantity,
            pv.reserved_quantity AS reserved_quantity,
            pv.track_inventory AS track_inventory,
            pv.allow_backorder AS allow_backorder
        FROM cart_items ci
        INNER JOIN (
            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
        ) c ON c.id = ci.cart_id
        INNER JOIN products p ON p.id = ci.product_id
        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
        LEFT JOIN LATERAL (
            SELECT image
            FROM product_images
            WHERE product_id = p.id AND cover = TRUE
            ORDER BY position
            LIMIT 1
        ) cover ON TRUE
        ORDER BY ci.id
    "#;

    let raw_items = CartItemRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        items_sql,
        vec![user_id.into()],
    ))
    .all(db)
    .await?;

    let cart_id = raw_items.first().map(|row| row.cart_id).unwrap_or(0);
    let items = raw_items
        .into_iter()
        .map(|row| {
            let inventory_qty = row.inventory_quantity.unwrap_or(0);
            let reserved_qty = row.reserved_quantity.unwrap_or(0);
            let track = row.track_inventory.unwrap_or(true);
            let allow_backorder = row.allow_backorder.unwrap_or(false);
            let available = inventory_qty - reserved_qty;
            let in_stock = !track || available > 0 || allow_backorder;

            CartItemWithProduct {
                id: row.id,
                cart_id: row.cart_id,
                product_id: row.product_id,
                product_variant_id: row.product_variant_id,
                quantity: row.quantity.unwrap_or(1),
                price: row.price.unwrap_or(Decimal::ZERO),
                product_name: row.product_name,
                product_slug: row.product_slug,
                product_image: row.product_image,
                in_stock,
                available_quantity: available,
            }
        })
        .collect();

    Ok(CartWithItems { id: cart_id, items })
}

pub async fn get_cart_with_items<C>(db: &C, cart_id: i32) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    #[derive(Debug, FromQueryResult)]
    struct CartItemRow {
        id: i32,
        cart_id: i32,
        product_id: i32,
        product_variant_id: Option<i32>,
        quantity: Option<i32>,
        price: Option<Decimal>,
        product_name: Option<String>,
        product_slug: Option<String>,
        product_image: Option<String>,
        inventory_quantity: Option<i32>,
        reserved_quantity: Option<i32>,
        track_inventory: Option<bool>,
        allow_backorder: Option<bool>,
    }

    let backend = db.get_database_backend();
    let sql = r#"
        SELECT
            ci.id              AS id,
            ci.cart_id         AS cart_id,
            ci.product_id      AS product_id,
            ci.product_variant_id AS product_variant_id,
            ci.quantity        AS quantity,
            ci.price           AS price,
            p.name             AS product_name,
            p.slug             AS product_slug,
            cover.image        AS product_image,
            pv.inventory_quantity AS inventory_quantity,
            pv.reserved_quantity AS reserved_quantity,
            pv.track_inventory AS track_inventory,
            pv.allow_backorder AS allow_backorder
        FROM cart_items ci
        INNER JOIN products p ON p.id = ci.product_id
        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
        LEFT JOIN LATERAL (
            SELECT image
            FROM product_images
            WHERE product_id = p.id AND cover = TRUE
            ORDER BY position
            LIMIT 1
        ) cover ON TRUE
        WHERE ci.cart_id = $1
        ORDER BY ci.id
    "#;

    let raw_items = CartItemRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        sql,
        vec![cart_id.into()],
    ))
    .all(db)
    .await?;

    let items = raw_items
        .into_iter()
        .map(|row| {
            let inventory_qty = row.inventory_quantity.unwrap_or(0);
            let reserved_qty = row.reserved_quantity.unwrap_or(0);
            let track = row.track_inventory.unwrap_or(true);
            let allow_backorder = row.allow_backorder.unwrap_or(false);
            let available = inventory_qty - reserved_qty;
            let in_stock = !track || available > 0 || allow_backorder;

            CartItemWithProduct {
                id: row.id,
                cart_id: row.cart_id,
                product_id: row.product_id,
                product_variant_id: row.product_variant_id,
                quantity: row.quantity.unwrap_or(1),
                price: row.price.unwrap_or(Decimal::ZERO),
                product_name: row.product_name,
                product_slug: row.product_slug,
                product_image: row.product_image,
                in_stock,
                available_quantity: available,
            }
        })
        .collect();

    Ok(CartWithItems { id: cart_id, items })
}

pub async fn add_item<C>(
    db: &C,
    cart_id: i32,
    product_id: i32,
    product_variant_id: Option<i32>,
    quantity: i32,
    price: Decimal,
) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    if let Some(variant_id) = product_variant_id {
        let variant = product_variants::Entity::find_by_id(variant_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest(t!("cart.variant_not_found").into()))?;

        if variant.track_inventory {
            let available = variant.inventory_quantity - variant.reserved_quantity;
            if available < quantity && !variant.allow_backorder {
                return Err(Error::BadRequest(
                    t!("cart.insufficient_stock", available = available).into(),
                ));
            }
        }
    }

    let existing = cart_items::Entity::find()
        .filter(cart_items::Column::CartId.eq(cart_id))
        .filter(cart_items::Column::ProductId.eq(product_id))
        .filter(cart_items::Column::ProductVariantId.eq(product_variant_id))
        .one(db)
        .await?;

    let now = chrono::Utc::now().into();

    if let Some(existing_item) = existing {
        let new_qty = existing_item.quantity.unwrap_or(0) + quantity;

        if let Some(variant_id) = product_variant_id {
            let variant = product_variants::Entity::find_by_id(variant_id)
                .one(db)
                .await?;
            if let Some(v) = variant {
                if v.track_inventory {
                    let available = v.inventory_quantity - v.reserved_quantity;
                    if available < new_qty && !v.allow_backorder {
                        return Err(Error::BadRequest(
                            t!("cart.insufficient_stock", available = available).into(),
                        ));
                    }
                }
            }
        }

        let mut active: cart_items::ActiveModel = existing_item.into();
        active.quantity = Set(Some(new_qty));
        active.price = Set(Some(price));
        active.updated_at = Set(now);
        active.update(db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to update cart item quantity");
            Error::InternalServerError
        })?;
    } else {
        let new_item = cart_items::ActiveModel {
            cart_id: Set(cart_id),
            product_id: Set(product_id),
            product_variant_id: Set(product_variant_id),
            quantity: Set(Some(quantity)),
            price: Set(Some(price)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        new_item.insert(db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to add cart item");
            Error::InternalServerError
        })?;
    }

    get_cart_with_items(db, cart_id).await
}

pub async fn update_item_quantity<C>(
    db: &C,
    item_id: i32,
    cart_id: i32,
    quantity: i32,
) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    let item = cart_items::Entity::find_by_id(item_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::BadRequest(t!("cart.item_not_found").into()))?;

    if item.cart_id != cart_id {
        return Err(Error::NotFound);
    }

    if quantity <= 0 {
        item.delete(db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to remove cart item");
            Error::InternalServerError
        })?;
    } else {
        if let Some(variant_id) = item.product_variant_id {
            let variant = product_variants::Entity::find_by_id(variant_id)
                .one(db)
                .await?;
            if let Some(v) = variant {
                if v.track_inventory {
                    let available = v.inventory_quantity - v.reserved_quantity;
                    if available < quantity && !v.allow_backorder {
                        return Err(Error::BadRequest(
                            t!("cart.insufficient_stock", available = available).into(),
                        ));
                    }
                }
            }
        }

        let mut active: cart_items::ActiveModel = item.into();
        active.quantity = Set(Some(quantity));
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(db).await.map_err(|e| {
            tracing::error!(error = ?e, "failed to update cart item quantity");
            Error::InternalServerError
        })?;
    }

    get_cart_with_items(db, cart_id).await
}

pub async fn remove_item<C>(db: &C, item_id: i32, cart_id: i32) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    let item = cart_items::Entity::find_by_id(item_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::BadRequest(t!("cart.item_not_found").into()))?;

    if item.cart_id != cart_id {
        return Err(Error::NotFound);
    }

    item.delete(db).await.map_err(|e| {
        tracing::error!(error = ?e, "failed to remove cart item");
        Error::InternalServerError
    })?;

    get_cart_with_items(db, cart_id).await
}

pub async fn clear_cart<C>(db: &C, cart_id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    cart_items::Entity::delete_many()
        .filter(cart_items::Column::CartId.eq(cart_id))
        .exec(db)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to clear cart");
            Error::InternalServerError
        })?;
    Ok(())
}

pub async fn expire_stale_reservations<C>(db: &C) -> Result<u64>
where
    C: ConnectionTrait,
{
    let now = chrono::Utc::now();
    let expired = stock_reservations::Entity::find()
        .filter(stock_reservations::Column::Status.eq("active"))
        .filter(stock_reservations::Column::ExpiresAt.lt(now))
        .all(db)
        .await?;

    let mut released_count: u64 = 0;
    for reservation in &expired {
        let variant = product_variants::Entity::find_by_id(reservation.product_variant_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::InternalServerError)?;

        let current_reserved = variant.reserved_quantity;
        let new_reserved = (current_reserved - reservation.quantity).max(0);

        let mut pv_active: product_variants::ActiveModel = variant.into();
        pv_active.reserved_quantity = Set(new_reserved);
        pv_active.updated_at = Set(now.into());
        pv_active.update(db).await?;

        let mut res_active: stock_reservations::ActiveModel = reservation.clone().into();
        res_active.status = Set("expired".to_string());
        res_active.updated_at = Set(now.into());
        res_active.update(db).await?;
        released_count += 1;
    }

    Ok(released_count)
}

pub async fn release_cart_reservations<C>(db: &C, cart_id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    let reservations = stock_reservations::Entity::find()
        .filter(stock_reservations::Column::CartId.eq(cart_id))
        .filter(stock_reservations::Column::Status.eq("active"))
        .all(db)
        .await?;

    for reservation in &reservations {
        let variant = product_variants::Entity::find_by_id(reservation.product_variant_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::InternalServerError)?;

        let current_reserved = variant.reserved_quantity;
        let new_reserved = (current_reserved - reservation.quantity).max(0);

        let mut pv_active: product_variants::ActiveModel = variant.into();
        pv_active.reserved_quantity = Set(new_reserved);
        pv_active.updated_at = Set(chrono::Utc::now().into());
        pv_active.update(db).await?;

        let mut res_active: stock_reservations::ActiveModel = reservation.clone().into();
        res_active.status = Set("released".to_string());
        res_active.updated_at = Set(chrono::Utc::now().into());
        res_active.update(db).await?;
    }

    Ok(())
}
