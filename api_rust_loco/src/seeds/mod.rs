pub mod addresses;
pub mod cart_items;
pub mod carts;
pub mod categories;
pub mod coupon_usages;
pub mod coupons;
pub mod order_items;
pub mod orders;
pub mod payment_methods;
pub mod payments;
pub mod posts;
pub mod product_images;
pub mod product_variant_options;
pub mod product_variants;
pub mod products;
pub mod profiles;
pub mod reviews;
pub mod shipments;
pub mod shipping_methods;
pub mod users;
pub mod variant_options;
pub mod wishlists;

use loco_rs::Result;
use sea_orm::DatabaseConnection;

/// Seeds all database tables in the correct dependency order.
/// This function is called during application boot when running seeds.
pub async fn seed_all(db: &DatabaseConnection) -> Result<()> {
    // Level 1: Independent entities (no foreign keys)
    users::seed(db).await?;
    categories::seed(db).await?;
    posts::seed(db).await?;
    variant_options::seed(db).await?;
    shipping_methods::seed(db).await?;
    payment_methods::seed(db).await?;
    coupons::seed(db).await?;

    // Level 2: Depends on Level 1
    profiles::seed(db).await?;
    products::seed(db).await?;
    addresses::seed(db).await?;
    carts::seed(db).await?;

    // Level 3: Depends on Level 2
    product_images::seed(db).await?;
    product_variants::seed(db).await?;
    cart_items::seed(db).await?;
    orders::seed(db).await?;
    reviews::seed(db).await?;
    wishlists::seed(db).await?;

    // Level 4: Depends on Level 3
    product_variant_options::seed(db).await?;
    order_items::seed(db).await?;

    // Level 5: Depends on Level 4
    payments::seed(db).await?;
    shipments::seed(db).await?;
    coupon_usages::seed(db).await?;

    Ok(())
}
