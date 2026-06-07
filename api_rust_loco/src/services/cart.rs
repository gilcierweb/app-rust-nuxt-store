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
            cover.image        AS product_image
        FROM cart_items ci
        INNER JOIN products p ON p.id = ci.product_id
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
        .map(|row| CartItemWithProduct {
            id: row.id,
            cart_id: row.cart_id,
            product_id: row.product_id,
            product_variant_id: row.product_variant_id,
            quantity: row.quantity.unwrap_or(1),
            price: row.price.unwrap_or(Decimal::ZERO),
            product_name: row.product_name,
            product_slug: row.product_slug,
            product_image: row.product_image,
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
            .await?;
        if variant.is_none() {
            return Err(Error::BadRequest(t!("cart.variant_not_found").into()));
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
