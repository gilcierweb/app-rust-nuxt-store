#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;



mod m20250713_001217_posts;
mod m20250713_052254_profiles;

mod m20250713_094714_categories;
mod m20250713_094959_add_parent_to_categories;
mod m20250719_064242_products;
mod m20250816_043653_product_images;
mod m20260510_075327_product_variants;
mod m20260510_075525_variant_options;
mod m20260510_075752_product_variant_options;
mod m20260510_080449_carts;
mod m20260510_080646_cart_items;
mod m20260510_080646_orders;
mod m20260510_080836_order_items;
mod m20260510_081027_payment_methods;
mod m20260510_081222_payments;
mod m20260510_081746_addresses;
mod m20260510_081958_shipping_methods;
mod m20260510_082152_shipments;
mod m20260510_082918_coupons;
mod m20260510_083114_coupon_usages;
mod m20260510_083315_reviews;
mod m20260510_083524_wishlists;
mod m20260510_101510_alter_order_items_product_variant_nullable;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
         
        
            Box::new(m20250713_001217_posts::Migration),
            Box::new(m20250713_052254_profiles::Migration),           
 
            Box::new(m20250713_094714_categories::Migration),
            Box::new(m20250713_094959_add_parent_to_categories::Migration),
            Box::new(m20250719_064242_products::Migration),
            Box::new(m20250816_043653_product_images::Migration),
            Box::new(m20260510_075327_product_variants::Migration),
            Box::new(m20260510_075525_variant_options::Migration),
            Box::new(m20260510_075752_product_variant_options::Migration),
            Box::new(m20260510_080449_carts::Migration),
            Box::new(m20260510_080646_cart_items::Migration),
            Box::new(m20260510_080646_orders::Migration),
            Box::new(m20260510_080836_order_items::Migration),
            Box::new(m20260510_081027_payment_methods::Migration),
            Box::new(m20260510_081222_payments::Migration),
            Box::new(m20260510_081746_addresses::Migration),
            Box::new(m20260510_081958_shipping_methods::Migration),
            Box::new(m20260510_082152_shipments::Migration),
            Box::new(m20260510_082918_coupons::Migration),
            Box::new(m20260510_083114_coupon_usages::Migration),
            Box::new(m20260510_083315_reviews::Migration),
            Box::new(m20260510_083524_wishlists::Migration),
            Box::new(m20260510_101510_alter_order_items_product_variant_nullable::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}