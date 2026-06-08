use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult,
    QueryFilter, Statement, TransactionError, TransactionTrait,
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

#[derive(Debug, FromQueryResult)]
struct CartItemRowCombined {
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

async fn fetch_cart_with_items_combined<C>(db: &C, cart_id: i32) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    let backend = db.get_database_backend();

    let sql = r#"
        WITH cover AS (
            SELECT DISTINCT ON (pi.product_id) pi.product_id, pi.image
            FROM product_images pi
            WHERE pi.cover = TRUE
              AND pi.product_id IN (SELECT ci.product_id FROM cart_items ci WHERE ci.cart_id = $1)
            ORDER BY pi.product_id, pi.position
        )
        SELECT
            ci.id              AS id,
            ci.cart_id         AS cart_id,
            ci.product_id      AS product_id,
            ci.product_variant_id AS product_variant_id,
            ci.quantity        AS quantity,
            ci.price           AS price,
            p.name             AS product_name,
            p.slug             AS product_slug,
            c.image            AS product_image,
            pv.inventory_quantity AS inventory_quantity,
            pv.reserved_quantity AS reserved_quantity,
            pv.track_inventory AS track_inventory,
            pv.allow_backorder AS allow_backorder
        FROM cart_items ci
        INNER JOIN products p ON p.id = ci.product_id
        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
        LEFT JOIN cover c ON c.product_id = ci.product_id
        WHERE ci.cart_id = $1
        ORDER BY ci.id
    "#;

    let rows = CartItemRowCombined::find_by_statement(Statement::from_sql_and_values(
        backend,
        sql,
        vec![cart_id.into()],
    ))
    .all(db)
    .await?;

    let items = rows
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
    let backend = db.get_database_backend();

    let sql = r#"
        WITH
        cart AS (
            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
        ),
        cover AS (
            SELECT DISTINCT ON (pi.product_id) pi.product_id, pi.image
            FROM product_images pi
            WHERE pi.cover = TRUE
              AND pi.product_id IN (SELECT ci.product_id FROM cart_items ci WHERE ci.cart_id = (SELECT id FROM cart))
            ORDER BY pi.product_id, pi.position
        )
        SELECT
            COALESCE((SELECT id FROM cart), 0) AS cart_id,
            ci.id, ci.cart_id AS ci_cart_id, ci.product_id, ci.product_variant_id,
            ci.quantity, ci.price,
            p.name AS product_name, p.slug AS product_slug,
            cover.image AS product_image,
            COALESCE(pv.inventory_quantity, 0) AS inventory_quantity,
            COALESCE(pv.reserved_quantity, 0) AS reserved_quantity,
            COALESCE(pv.track_inventory, false) AS track_inventory,
            COALESCE(pv.allow_backorder, false) AS allow_backorder
        FROM cart_items ci
        INNER JOIN products p ON p.id = ci.product_id
        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
        LEFT JOIN cover ON cover.product_id = ci.product_id
        WHERE ci.cart_id = (SELECT id FROM cart)
        ORDER BY ci.id
    "#;

    #[derive(Debug, FromQueryResult)]
    struct Row {
        cart_id: i32,
        id: i32,
        ci_cart_id: i32,
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

    let rows = Row::find_by_statement(Statement::from_sql_and_values(
        backend,
        sql,
        vec![user_id.into()],
    ))
    .all(db)
    .await?;

    let cart_id = rows.first().map(|r| r.cart_id).unwrap_or(0);

    if rows.is_empty() {
        if cart_id == 0 {
            let cart = carts::Entity::find()
                .filter(carts::Column::UserId.eq(user_id))
                .one(db)
                .await?;
            let real_cart_id = cart.map(|c| c.id).unwrap_or(0);
            return Ok(CartWithItems {
                id: real_cart_id,
                items: Vec::new(),
            });
        }
        return Ok(CartWithItems {
            id: cart_id,
            items: Vec::new(),
        });
    }

    let items: Vec<CartItemWithProduct> = rows
        .into_iter()
        .map(|r| {
            let inv = r.inventory_quantity.unwrap_or(0);
            let res = r.reserved_quantity.unwrap_or(0);
            let track = r.track_inventory.unwrap_or(true);
            let allow = r.allow_backorder.unwrap_or(false);
            let available = inv - res;
            CartItemWithProduct {
                id: r.id,
                cart_id: r.ci_cart_id,
                product_id: r.product_id,
                product_variant_id: r.product_variant_id,
                quantity: r.quantity.unwrap_or(1),
                price: r.price.unwrap_or(Decimal::ZERO),
                product_name: r.product_name,
                product_slug: r.product_slug,
                product_image: r.product_image,
                in_stock: !track || available > 0 || allow,
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
    fetch_cart_with_items_combined(db, cart_id).await
}

#[derive(Debug, FromQueryResult)]
struct AddItemLookup {
    cart_id: Option<i32>,
    item_id: Option<i32>,
    item_quantity: Option<i32>,
    track_inventory: Option<bool>,
    allow_backorder: Option<bool>,
    available: Option<i32>,
}

pub async fn add_item<C>(
    db: &C,
    user_id: i32,
    product_id: i32,
    product_variant_id: Option<i32>,
    quantity: i32,
    price: Decimal,
) -> Result<CartWithItems>
where
    C: ConnectionTrait + TransactionTrait,
{
    let result = db
        .transaction::<_, _, Error>(|txn| {
            Box::pin(async move {
                let now = chrono::Utc::now().into();
                let backend = txn.get_database_backend();

                let lookup_sql = r#"
                    SELECT
                        c.id AS cart_id,
                        ci.id AS item_id,
                        ci.quantity AS item_quantity,
                        pv.track_inventory AS track_inventory,
                        pv.allow_backorder AS allow_backorder,
                        (COALESCE(pv.inventory_quantity, 0) - COALESCE(pv.reserved_quantity, 0)) AS available
                    FROM carts c
                    LEFT JOIN cart_items ci
                        ON ci.cart_id = c.id
                        AND ci.product_id = $2
                        AND ci.product_variant_id IS NOT DISTINCT FROM $3
                    LEFT JOIN product_variants pv
                        ON pv.id = $3
                    WHERE c.user_id = $1
                    LIMIT 1
                "#;

                let lookup = AddItemLookup::find_by_statement(Statement::from_sql_and_values(
                    backend,
                    lookup_sql,
                    vec![user_id.into(), product_id.into(), product_variant_id.into()],
                ))
                .one(txn)
                .await?;

                let cart_id = match &lookup {
                    Some(l) => l.cart_id,
                    None => None,
                };

                let cart_id = match cart_id {
                    Some(id) => id,
                    None => {
                        let new_cart = carts::ActiveModel {
                            user_id: Set(user_id),
                            session_id: Set(None),
                            expires_at: Set(None),
                            created_at: Set(now),
                            updated_at: Set(now),
                            ..Default::default()
                        };
                        new_cart.insert(txn).await?.id
                    }
                };

                if let Some(ref l) = lookup {
                    let track = l.track_inventory.unwrap_or(false);
                    let allow = l.allow_backorder.unwrap_or(false);
                    if track && !allow {
                        let available = l.available.unwrap_or(0);
                        let current_qty = l.item_quantity.unwrap_or(0);
                        let new_qty = current_qty + quantity;
                        if new_qty > available {
                            return Err(Error::BadRequest(
                                t!("cart.insufficient_stock", available = available).into(),
                            ));
                        }
                    }
                }

                match lookup.and_then(|l| l.item_id) {
                    Some(item_id) => {
                        let existing = cart_items::Entity::find_by_id(item_id)
                            .one(txn)
                            .await?;
                        if let Some(item) = existing {
                            let current_qty = item.quantity.unwrap_or(0);
                            let mut active: cart_items::ActiveModel = item.into();
                            active.quantity = Set(Some(current_qty + quantity));
                            active.price = Set(Some(price));
                            active.updated_at = Set(now);
                            active.update(txn).await?;
                        }
                    }
                    None => {
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
                        new_item.insert(txn).await?;
                    }
                }

                fetch_cart_with_items_combined(txn, cart_id).await
            })
        })
        .await;

    match result {
        Ok(cart) => Ok(cart),
        Err(TransactionError::Connection(e)) => Err(Error::DB(e)),
        Err(TransactionError::Transaction(e)) => Err(e),
    }
}

pub async fn update_item_quantity<C>(
    db: &C,
    user_id: i32,
    item_id: i32,
    quantity: i32,
) -> Result<CartWithItems>
where
    C: ConnectionTrait + TransactionTrait,
{
    let result = db
        .transaction::<_, _, Error>(|txn| {
            Box::pin(async move {
                let backend = txn.get_database_backend();

                if quantity <= 0 {
                    let del_sql = r#"
                        WITH
                        cart AS (
                            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
                        ),
                        deleted AS (
                            DELETE FROM cart_items ci
                            USING cart c
                            WHERE ci.id = $2 AND ci.cart_id = c.id
                            RETURNING ci.cart_id
                        )
                        SELECT cart_id FROM deleted
                    "#;

                    #[derive(Debug, FromQueryResult)]
                    struct DelResult {
                        cart_id: i32,
                    }

                    let del = DelResult::find_by_statement(Statement::from_sql_and_values(
                        backend,
                        del_sql,
                        vec![user_id.into(), item_id.into()],
                    ))
                    .one(txn)
                    .await?;

                    return match del {
                        None => Err(Error::BadRequest(t!("cart.item_not_found").into())),
                        Some(d) => fetch_cart_with_items_combined(txn, d.cart_id).await,
                    };
                }

                let setup_sql = r#"
                    WITH
                    cart AS (
                        SELECT id FROM carts WHERE user_id = $1 LIMIT 1
                    ),
                    item_check AS (
                        SELECT ci.id, ci.cart_id, ci.product_variant_id
                        FROM cart_items ci, cart c
                        WHERE ci.id = $2 AND ci.cart_id = c.id
                    ),
                    variant_info AS (
                        SELECT
                            COALESCE(pv.inventory_quantity, 0) - COALESCE(pv.reserved_quantity, 0) AS available,
                            COALESCE(pv.track_inventory, false) AS track_inventory,
                            COALESCE(pv.allow_backorder, false) AS allow_backorder
                        FROM item_check ic
                        LEFT JOIN product_variants pv ON pv.id = ic.product_variant_id
                    ),
                    do_update AS (
                        UPDATE cart_items ci
                        SET quantity = $3, updated_at = NOW()
                        FROM item_check ic
                        WHERE ci.id = ic.id
                          AND (
                            NOT (SELECT track_inventory FROM variant_info)
                            OR (SELECT allow_backorder FROM variant_info)
                            OR (SELECT available FROM variant_info) >= $3
                          )
                        RETURNING ci.id, ci.cart_id
                    )
                    SELECT
                        COALESCE((SELECT cart_id FROM do_update), 0) AS cart_id,
                        COALESCE((SELECT track_inventory FROM variant_info), false) AS track_inventory,
                        COALESCE((SELECT available FROM variant_info), 0) AS available,
                        COALESCE((SELECT allow_backorder FROM variant_info), false) AS allow_backorder
                "#;

                #[derive(Debug, FromQueryResult)]
                struct UpdateResult {
                    cart_id: i32,
                    track_inventory: bool,
                    available: i32,
                    allow_backorder: bool,
                }

                let ur = UpdateResult::find_by_statement(Statement::from_sql_and_values(
                    backend,
                    setup_sql,
                    vec![user_id.into(), item_id.into(), quantity.into()],
                ))
                .one(txn)
                .await?
                .ok_or_else(|| Error::InternalServerError)?;

                if ur.cart_id == 0 {
                    return Err(Error::BadRequest(t!("cart.item_not_found").into()));
                }

                if ur.track_inventory && !ur.allow_backorder && ur.available < quantity {
                    return Err(Error::BadRequest(
                        t!("cart.insufficient_stock", available = ur.available).into(),
                    ));
                }

                fetch_cart_with_items_combined(txn, ur.cart_id).await
            })
        })
        .await;

    match result {
        Ok(cart) => Ok(cart),
        Err(TransactionError::Connection(e)) => Err(Error::DB(e)),
        Err(TransactionError::Transaction(e)) => Err(e),
    }
}

pub async fn remove_item<C>(db: &C, user_id: i32, item_id: i32) -> Result<CartWithItems>
where
    C: ConnectionTrait,
{
    let backend = db.get_database_backend();

    let del_sql = r#"
        WITH
        cart AS (
            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
        ),
        deleted AS (
            DELETE FROM cart_items ci
            USING cart c
            WHERE ci.id = $2 AND ci.cart_id = c.id
            RETURNING ci.cart_id
        )
        SELECT cart_id FROM deleted
    "#;

    #[derive(Debug, FromQueryResult)]
    struct DelResult {
        cart_id: i32,
    }

    let del = DelResult::find_by_statement(Statement::from_sql_and_values(
        backend,
        del_sql,
        vec![user_id.into(), item_id.into()],
    ))
    .one(db)
    .await?;

    match del {
        None => Err(Error::BadRequest(t!("cart.item_not_found").into())),
        Some(d) => {
            let remaining = fetch_cart_with_items_combined(db, d.cart_id).await?;
            Ok(remaining)
        }
    }
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
