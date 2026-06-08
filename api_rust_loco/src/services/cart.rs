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
            ci.id, ci.cart_id, ci.product_id, ci.product_variant_id,
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

    let rows = Row::find_by_statement(Statement::from_sql_and_values(
        backend,
        sql,
        vec![user_id.into()],
    ))
    .all(db)
    .await?;

    let cart_id = match rows.first() {
        Some(r) => r.cart_id,
        None => {
            let cart = carts::Entity::find()
                .filter(carts::Column::UserId.eq(user_id))
                .one(db)
                .await?;
            cart.map(|c| c.id).unwrap_or(0)
        }
    };

    if rows.is_empty() {
        return Ok(CartWithItems { id: cart_id, items: Vec::new() });
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
                cart_id: r.cart_id,
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
                let backend = txn.get_database_backend();

                let merged_sql = r#"
                    WITH
                    ensure_cart AS (
                        INSERT INTO carts (user_id, created_at, updated_at)
                        SELECT $1, NOW(), NOW()
                        WHERE NOT EXISTS (SELECT 1 FROM carts WHERE user_id = $1)
                        ON CONFLICT DO NOTHING
                        RETURNING id
                    ),
                    cart AS (
                        SELECT id FROM carts WHERE user_id = $1
                        UNION ALL
                        SELECT id FROM ensure_cart
                        LIMIT 1
                    ),
                    variant_info AS (
                        SELECT
                            COALESCE(pv.inventory_quantity, 0) - COALESCE(pv.reserved_quantity, 0) AS available,
                            COALESCE(pv.track_inventory, false) AS track_inventory,
                            COALESCE(pv.allow_backorder, false) AS allow_backorder
                        FROM (SELECT 1) dummy
                        LEFT JOIN product_variants pv ON pv.id = $3
                    ),
                    existing AS (
                        SELECT ci.id, COALESCE(ci.quantity, 0) AS quantity
                        FROM cart_items ci, cart c
                        WHERE ci.cart_id = c.id
                          AND ci.product_id = $2
                          AND ci.product_variant_id IS NOT DISTINCT FROM $3
                    ),
                    upserted AS (
                        UPDATE cart_items ci
                        SET quantity = (SELECT quantity FROM existing) + $4,
                            price = $5,
                            updated_at = NOW()
                        FROM existing e
                        WHERE ci.id = e.id
                        RETURNING ci.id
                    ),
                    inserted AS (
                        INSERT INTO cart_items (cart_id, product_id, product_variant_id, quantity, price, created_at, updated_at)
                        SELECT c.id, $2, $3, $4, $5, NOW(), NOW()
                        FROM cart c
                        WHERE NOT EXISTS (SELECT 1 FROM upserted)
                        RETURNING id
                    ),
                    cover AS (
                        SELECT DISTINCT ON (pi.product_id)
                            pi.product_id, pi.id AS image_id, pi.image, pi.alt_text, pi.active, pi.cover, pi.position
                        FROM product_images pi
                        WHERE pi.cover = TRUE
                          AND pi.product_id IN (
                              SELECT ci2.product_id FROM cart_items ci2
                              WHERE ci2.cart_id = (SELECT id FROM cart)
                          )
                        ORDER BY pi.product_id, pi.position
                    )
                    SELECT
                        ci.id,
                        ci.cart_id,
                        ci.product_id,
                        ci.product_variant_id,
                        ci.quantity,
                        ci.price,
                        p.name AS product_name,
                        p.slug AS product_slug,
                        cover.image AS product_image,
                        COALESCE(pv.inventory_quantity, 0) AS inventory_quantity,
                        COALESCE(pv.reserved_quantity, 0) AS reserved_quantity,
                        COALESCE(pv.track_inventory, false) AS pv_track_inventory,
                        COALESCE(pv.allow_backorder, false) AS pv_allow_backorder,
                        vi.available,
                        vi.track_inventory,
                        vi.allow_backorder,
                        COALESCE((SELECT quantity FROM existing), 0) + $4 AS new_quantity
                    FROM cart_items ci
                    INNER JOIN products p ON p.id = ci.product_id
                    LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
                    LEFT JOIN cover ON cover.product_id = ci.product_id
                    CROSS JOIN variant_info vi
                    WHERE ci.cart_id = (SELECT id FROM cart)
                    ORDER BY ci.id
                "#;

                #[derive(Debug, FromQueryResult)]
                struct MergedRow {
                    id: i32,
                    cart_id: i32,
                    product_id: i32,
                    product_variant_id: Option<i32>,
                    quantity: i32,
                    price: rust_decimal::Decimal,
                    product_name: String,
                    product_slug: String,
                    product_image: Option<String>,
                    inventory_quantity: i32,
                    reserved_quantity: i32,
                    pv_track_inventory: bool,
                    pv_allow_backorder: bool,
                    available: i32,
                    track_inventory: bool,
                    allow_backorder: bool,
                    new_quantity: i32,
                }

                let rows = MergedRow::find_by_statement(Statement::from_sql_and_values(
                    backend,
                    merged_sql,
                    vec![
                        user_id.into(),
                        product_id.into(),
                        product_variant_id.into(),
                        quantity.into(),
                        price.into(),
                    ],
                ))
                .all(txn)
                .await?;

                if rows.is_empty() {
                    return Err(Error::InternalServerError);
                }

                let first = &rows[0];
                if first.track_inventory && !first.allow_backorder {
                    if first.new_quantity > first.available {
                        return Err(Error::BadRequest(
                            t!("cart.insufficient_stock", available = first.available).into(),
                        ));
                    }
                }

                let cart_id = first.cart_id;
                let mut items = Vec::with_capacity(rows.len());
                for r in &rows {
                    let available = r.inventory_quantity - r.reserved_quantity;
                    let in_stock = !r.pv_track_inventory || available > 0 || r.pv_allow_backorder;
                    items.push(CartItemWithProduct {
                        id: r.id,
                        cart_id: r.cart_id,
                        product_id: r.product_id,
                        product_variant_id: r.product_variant_id,
                        quantity: r.quantity,
                        price: r.price,
                        product_name: Some(r.product_name.clone()),
                        product_slug: Some(r.product_slug.clone()),
                        product_image: r.product_image.clone(),
                        in_stock,
                        available_quantity: available,
                    });
                }

                Ok(CartWithItems { id: cart_id, items })
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
                    let del_merged_sql = r#"
                        WITH
                        cart AS (
                            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
                        ),
                        deleted AS (
                            DELETE FROM cart_items ci
                            USING cart c
                            WHERE ci.id = $2 AND ci.cart_id = c.id
                            RETURNING ci.cart_id
                        ),
                        cover AS (
                            SELECT DISTINCT ON (pi.product_id)
                                pi.product_id, pi.id AS image_id, pi.image, pi.alt_text, pi.active, pi.cover, pi.position
                            FROM product_images pi
                            WHERE pi.cover = TRUE
                              AND pi.product_id IN (
                                  SELECT ci2.product_id FROM cart_items ci2
                                  WHERE ci2.cart_id = (SELECT cart_id FROM deleted)
                              )
                            ORDER BY pi.product_id, pi.position
                        )
                        SELECT
                            ci.id,
                            ci.cart_id,
                            ci.product_id,
                            ci.product_variant_id,
                            ci.quantity,
                            ci.price,
                            p.name AS product_name,
                            p.slug AS product_slug,
                            cover.image AS product_image,
                            COALESCE(pv.inventory_quantity, 0) AS inventory_quantity,
                            COALESCE(pv.reserved_quantity, 0) AS reserved_quantity,
                            COALESCE(pv.track_inventory, false) AS track_inventory,
                            COALESCE(pv.allow_backorder, false) AS allow_backorder
                        FROM cart_items ci
                        INNER JOIN products p ON p.id = ci.product_id
                        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
                        LEFT JOIN cover ON cover.product_id = ci.product_id
                        WHERE ci.cart_id = (SELECT cart_id FROM deleted)
                        ORDER BY ci.id
                    "#;

                    #[derive(Debug, FromQueryResult)]
                    struct DelMergedRow {
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

                    let rows = DelMergedRow::find_by_statement(Statement::from_sql_and_values(
                        backend,
                        del_merged_sql,
                        vec![user_id.into(), item_id.into()],
                    ))
                    .all(txn)
                    .await?;

                    if rows.is_empty() {
                        return Err(Error::BadRequest(t!("cart.item_not_found").into()));
                    }

                    let cart_id = rows[0].cart_id;
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
                                cart_id: r.cart_id,
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

                    return Ok(CartWithItems { id: cart_id, items });
                }

                let merged_sql = r#"
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
                    ),
                    update_result AS (
                        SELECT
                            COALESCE((SELECT cart_id FROM do_update), 0) AS cart_id,
                            COALESCE((SELECT track_inventory FROM variant_info), false) AS track_inventory,
                            COALESCE((SELECT available FROM variant_info), 0) AS available,
                            COALESCE((SELECT allow_backorder FROM variant_info), false) AS allow_backorder
                    ),
                    cover AS (
                        SELECT DISTINCT ON (pi.product_id)
                            pi.product_id, pi.id AS image_id, pi.image, pi.alt_text, pi.active, pi.cover, pi.position
                        FROM product_images pi
                        WHERE pi.cover = TRUE
                          AND pi.product_id IN (
                              SELECT ci2.product_id FROM cart_items ci2
                              WHERE ci2.cart_id = (SELECT cart_id FROM update_result)
                          )
                        ORDER BY pi.product_id, pi.position
                    )
                    SELECT
                        ci.id,
                        ci.cart_id,
                        ci.product_id,
                        ci.product_variant_id,
                        ci.quantity,
                        ci.price,
                        p.name AS product_name,
                        p.slug AS product_slug,
                        cover.image AS product_image,
                        COALESCE(pv.inventory_quantity, 0) AS inventory_quantity,
                        COALESCE(pv.reserved_quantity, 0) AS reserved_quantity,
                        COALESCE(pv.track_inventory, false) AS pv_track_inventory,
                        COALESCE(pv.allow_backorder, false) AS pv_allow_backorder,
                        ur.cart_id AS update_cart_id,
                        ur.track_inventory,
                        ur.available,
                        ur.allow_backorder
                    FROM cart_items ci
                    INNER JOIN products p ON p.id = ci.product_id
                    LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
                    LEFT JOIN cover ON cover.product_id = ci.product_id
                    CROSS JOIN update_result ur
                    WHERE ci.cart_id = (SELECT cart_id FROM update_result)
                    ORDER BY ci.id
                "#;

                #[derive(Debug, FromQueryResult)]
                struct MergedRow {
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
                    pv_track_inventory: Option<bool>,
                    pv_allow_backorder: Option<bool>,
                    update_cart_id: i32,
                    track_inventory: bool,
                    available: i32,
                    allow_backorder: bool,
                }

                let rows = MergedRow::find_by_statement(Statement::from_sql_and_values(
                    backend,
                    merged_sql,
                    vec![user_id.into(), item_id.into(), quantity.into()],
                ))
                .all(txn)
                .await?;

                if rows.is_empty() {
                    return Err(Error::BadRequest(t!("cart.item_not_found").into()));
                }

                let first = &rows[0];
                if first.update_cart_id == 0 {
                    return Err(Error::BadRequest(t!("cart.item_not_found").into()));
                }

                if first.track_inventory && !first.allow_backorder && first.available < quantity {
                    return Err(Error::BadRequest(
                        t!("cart.insufficient_stock", available = first.available).into(),
                    ));
                }

                let cart_id = first.cart_id;
                let items: Vec<CartItemWithProduct> = rows
                    .into_iter()
                    .map(|r| {
                        let inv = r.inventory_quantity.unwrap_or(0);
                        let res = r.reserved_quantity.unwrap_or(0);
                        let track = r.pv_track_inventory.unwrap_or(true);
                        let allow = r.pv_allow_backorder.unwrap_or(false);
                        let available = inv - res;
                        CartItemWithProduct {
                            id: r.id,
                            cart_id: r.cart_id,
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

    let merged_sql = r#"
        WITH
        cart AS (
            SELECT id FROM carts WHERE user_id = $1 LIMIT 1
        ),
        deleted AS (
            DELETE FROM cart_items ci
            USING cart c
            WHERE ci.id = $2 AND ci.cart_id = c.id
            RETURNING ci.cart_id
        ),
        cover AS (
            SELECT DISTINCT ON (pi.product_id)
                pi.product_id, pi.id AS image_id, pi.image, pi.alt_text, pi.active, pi.cover, pi.position
            FROM product_images pi
            WHERE pi.cover = TRUE
              AND pi.product_id IN (
                  SELECT ci2.product_id FROM cart_items ci2
                  WHERE ci2.cart_id = (SELECT cart_id FROM deleted)
              )
            ORDER BY pi.product_id, pi.position
        )
        SELECT
            ci.id,
            ci.cart_id,
            ci.product_id,
            ci.product_variant_id,
            ci.quantity,
            ci.price,
            p.name AS product_name,
            p.slug AS product_slug,
            cover.image AS product_image,
            COALESCE(pv.inventory_quantity, 0) AS inventory_quantity,
            COALESCE(pv.reserved_quantity, 0) AS reserved_quantity,
            COALESCE(pv.track_inventory, false) AS track_inventory,
            COALESCE(pv.allow_backorder, false) AS allow_backorder
        FROM cart_items ci
        INNER JOIN products p ON p.id = ci.product_id
        LEFT JOIN product_variants pv ON pv.id = ci.product_variant_id
        LEFT JOIN cover ON cover.product_id = ci.product_id
        WHERE ci.cart_id = (SELECT cart_id FROM deleted)
        ORDER BY ci.id
    "#;

    #[derive(Debug, FromQueryResult)]
    struct MergedRow {
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

    let rows = MergedRow::find_by_statement(Statement::from_sql_and_values(
        backend,
        merged_sql,
        vec![user_id.into(), item_id.into()],
    ))
    .all(db)
    .await?;

    if rows.is_empty() {
        return Err(Error::BadRequest(t!("cart.item_not_found").into()));
    }

    let cart_id = rows[0].cart_id;
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
                cart_id: r.cart_id,
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
