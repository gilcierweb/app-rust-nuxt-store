use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    QueryOrder,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::models::_entities::cart_items;
use crate::models::_entities::carts;
use crate::models::_entities::product_images;
use crate::models::_entities::product_variants;
use crate::models::_entities::products;

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
    let raw_items = cart_items::Entity::find()
        .filter(cart_items::Column::CartId.eq(cart_id))
        .order_by_asc(cart_items::Column::Id)
        .find_also_related(products::Entity)
        .all(db)
        .await?;

    let product_ids: Vec<i32> = raw_items
        .iter()
        .filter_map(|(_, product)| product.as_ref().map(|p| p.id))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    let mut cover_images: HashMap<i32, Option<String>> = HashMap::new();
    if !product_ids.is_empty() {
        let cover_rows = product_images::Entity::find()
            .filter(product_images::Column::ProductId.is_in(product_ids.clone()))
            .filter(product_images::Column::Cover.eq(Some(true)))
            .all(db)
            .await?;

        for img in cover_rows {
            if let Some(path) = img.image.clone() {
                cover_images.entry(img.product_id).or_insert(Some(path));
            }
        }
    }

    let mut items = Vec::with_capacity(raw_items.len());

    for (item, product) in &raw_items {
        let product_image = product
            .as_ref()
            .and_then(|p| cover_images.get(&p.id).and_then(|v| v.clone()));

        items.push(CartItemWithProduct {
            id: item.id,
            cart_id: item.cart_id,
            product_id: item.product_id,
            product_variant_id: item.product_variant_id,
            quantity: item.quantity.unwrap_or(1),
            price: item.price.unwrap_or(Decimal::ZERO),
            product_name: product.as_ref().and_then(|p| p.name.clone()),
            product_slug: product.as_ref().and_then(|p| p.slug.clone()),
            product_image,
        });
    }

    let cart = carts::Entity::find_by_id(cart_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    Ok(CartWithItems {
        id: cart.id,
        items,
    })
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
