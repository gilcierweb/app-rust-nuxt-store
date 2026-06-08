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
mod m20260513_223617_create_roles_and_users_roles;
mod m20260516_000001_add_performance_indexes;
mod m20260517_072928_banner_positions;
mod m20260517_073000_normalize_banner_positions;
mod m20260517_073137_banners;
mod m20260517_073909_banner_events;
mod m20260517_203700_add_catalog_lookup_indexes;
mod m20260517_211000_add_controller_query_indexes;
mod m20260519_064033_payment_gateways;
mod m20260519_064100_expand_payment_methods_for_gateways;
mod m20260519_064101_payment_method_countries;
mod m20260519_064102_payment_method_currencies;
mod m20260519_064103_gateway_customers;
mod m20260519_064104_payment_sources;
mod m20260519_064105_expand_payments_for_gateways;
mod m20260519_064106_payment_sessions;
mod m20260519_064107_payment_setup_sessions;
mod m20260519_064108_payment_refunds;
mod m20260519_064109_payment_gateway_events;
mod m20260519_064110_payment_gateway_logs;
mod m20260520_035158_product_variant_images;
mod m20260520_065417_create_email_logs;
mod m20260520_120000_admin_settings;
mod m20260520_234941_admin_audit_logs;
mod m20260604_000001_alter_cart_items_product_variant_nullable;
mod m20260606_000001_add_query_optimization_indexes;
mod m20260607_000001_add_inventory_hardening_columns;
mod m20260607_000002_create_stock_reservations;
mod m20260607_000003_create_stock_movements;
mod m20260608_000001_add_cart_performance_indexes;
mod m20260608_000002_add_remaining_performance_indexes;
mod m20260608_000003_create_back_in_stock_notifications;
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
            Box::new(m20260513_223617_create_roles_and_users_roles::Migration),
            Box::new(m20260516_000001_add_performance_indexes::Migration),
            Box::new(m20260517_072928_banner_positions::Migration),
            Box::new(m20260517_073000_normalize_banner_positions::Migration),
            Box::new(m20260517_073137_banners::Migration),
            Box::new(m20260517_073909_banner_events::Migration),
            Box::new(m20260517_203700_add_catalog_lookup_indexes::Migration),
            Box::new(m20260517_211000_add_controller_query_indexes::Migration),
            Box::new(m20260519_064033_payment_gateways::Migration),
            Box::new(m20260519_064100_expand_payment_methods_for_gateways::Migration),
            Box::new(m20260519_064101_payment_method_countries::Migration),
            Box::new(m20260519_064102_payment_method_currencies::Migration),
            Box::new(m20260519_064103_gateway_customers::Migration),
            Box::new(m20260519_064104_payment_sources::Migration),
            Box::new(m20260519_064105_expand_payments_for_gateways::Migration),
            Box::new(m20260519_064106_payment_sessions::Migration),
            Box::new(m20260519_064107_payment_setup_sessions::Migration),
            Box::new(m20260519_064108_payment_refunds::Migration),
            Box::new(m20260519_064109_payment_gateway_events::Migration),
            Box::new(m20260519_064110_payment_gateway_logs::Migration),
            Box::new(m20260520_035158_product_variant_images::Migration),
            Box::new(m20260520_120000_admin_settings::Migration),
            Box::new(m20260520_065417_create_email_logs::Migration),
            Box::new(m20260520_234941_admin_audit_logs::Migration),
            Box::new(m20260604_000001_alter_cart_items_product_variant_nullable::Migration),
            Box::new(m20260606_000001_add_query_optimization_indexes::Migration),
            Box::new(m20260607_000001_add_inventory_hardening_columns::Migration),
            Box::new(m20260607_000002_create_stock_reservations::Migration),
            Box::new(m20260607_000003_create_stock_movements::Migration),
            Box::new(m20260608_000001_add_cart_performance_indexes::Migration),
            Box::new(m20260608_000002_add_remaining_performance_indexes::Migration),
            Box::new(m20260608_000003_create_back_in_stock_notifications::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
