use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // -- cart_items ------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cart_items_cart_id")
                    .table(CartItems::Table)
                    .col(CartItems::CartId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cart_items_cart_product_variant")
                    .table(CartItems::Table)
                    .col(CartItems::CartId)
                    .col(CartItems::ProductId)
                    .col(CartItems::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cart_items_product_variant_id")
                    .table(CartItems::Table)
                    .col(CartItems::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        // -- carts -----------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_carts_user_id")
                    .table(Carts::Table)
                    .col(Carts::UserId)
                    .to_owned(),
            )
            .await?;

        // -- product_images --------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_images_product_id_cover")
                    .table(ProductImages::Table)
                    .col(ProductImages::ProductId)
                    .col(ProductImages::Cover)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_images_product_id_position")
                    .table(ProductImages::Table)
                    .col(ProductImages::ProductId)
                    .col(ProductImages::Position)
                    .to_owned(),
            )
            .await?;

        // -- product_variant_images -----------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variant_images_variant_id")
                    .table(ProductVariantImages::Table)
                    .col(ProductVariantImages::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variant_images_variant_cover")
                    .table(ProductVariantImages::Table)
                    .col(ProductVariantImages::ProductVariantId)
                    .col(ProductVariantImages::Cover)
                    .to_owned(),
            )
            .await?;

        // -- products --------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_active")
                    .table(Products::Table)
                    .col(Products::Active)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_status_active")
                    .table(Products::Table)
                    .col(Products::Status)
                    .col(Products::Active)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_slug")
                    .table(Products::Table)
                    .col(Products::Slug)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_category_id_active")
                    .table(Products::Table)
                    .col(Products::CategoryId)
                    .col(Products::Active)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_featured_active")
                    .table(Products::Table)
                    .col(Products::Featured)
                    .col(Products::Active)
                    .to_owned(),
            )
            .await?;

        // -- categories ------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_categories_parent_id")
                    .table(Categories::Table)
                    .col(Categories::ParentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_categories_active_position")
                    .table(Categories::Table)
                    .col(Categories::Active)
                    .col(Categories::Position)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_categories_slug")
                    .table(Categories::Table)
                    .col(Categories::Slug)
                    .to_owned(),
            )
            .await?;

        // -- payments --------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_status_created_at")
                    .table(Payments::Table)
                    .col(Payments::Status)
                    .col(Payments::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_payment_method_id")
                    .table(Payments::Table)
                    .col(Payments::PaymentMethodId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_currency")
                    .table(Payments::Table)
                    .col(Payments::Currency)
                    .to_owned(),
            )
            .await?;

        // -- payment_sessions ------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sessions_payment_id_created_at")
                    .table(PaymentSessions::Table)
                    .col(PaymentSessions::PaymentId)
                    .col(PaymentSessions::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- payment_refunds -------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_refunds_payment_id")
                    .table(PaymentRefunds::Table)
                    .col(PaymentRefunds::PaymentId)
                    .to_owned(),
            )
            .await?;

        // -- payment_gateway_logs --------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_gateway_logs_payment_id_created_at")
                    .table(PaymentGatewayLogs::Table)
                    .col(PaymentGatewayLogs::PaymentId)
                    .col(PaymentGatewayLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_gateway_logs_payment_session_id")
                    .table(PaymentGatewayLogs::Table)
                    .col(PaymentGatewayLogs::PaymentSessionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_gateway_logs_created_at")
                    .table(PaymentGatewayLogs::Table)
                    .col(PaymentGatewayLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- payment_gateway_events ------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_gateway_events_status_created_at")
                    .table(PaymentGatewayEvents::Table)
                    .col(PaymentGatewayEvents::Status)
                    .col(PaymentGatewayEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- payment_methods -------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_methods_payment_gateway_id")
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_methods_active")
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::Active)
                    .to_owned(),
            )
            .await?;

        // -- payment_sources -------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sources_user_id")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sources_payment_method_id")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::PaymentMethodId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sources_gateway_customer_id")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::GatewayCustomerId)
                    .to_owned(),
            )
            .await?;

        // -- payment_setup_sessions ------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_setup_sessions_user_id_created_at")
                    .table(PaymentSetupSessions::Table)
                    .col(PaymentSetupSessions::UserId)
                    .col(PaymentSetupSessions::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- gateway_customers -----------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_gateway_customers_user_id_gateway_id")
                    .table(GatewayCustomers::Table)
                    .col(GatewayCustomers::UserId)
                    .col(GatewayCustomers::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_gateway_customers_payment_gateway_id")
                    .table(GatewayCustomers::Table)
                    .col(GatewayCustomers::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        // -- order_items -----------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_order_items_product_variant_id")
                    .table(OrderItems::Table)
                    .col(OrderItems::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        // -- orders ----------------------------------------------------------------
        // Composite for `my_orders`: WHERE user_id = ? ORDER BY created_at DESC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_orders_user_id_created_at")
                    .table(Orders::Table)
                    .col(Orders::UserId)
                    .col(Orders::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Composite for admin list filtered by status with date sort
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_orders_status_created_at")
                    .table(Orders::Table)
                    .col(Orders::Status)
                    .col(Orders::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Single-column lookup for order_number (admin order details)
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_orders_order_number")
                    .table(Orders::Table)
                    .col(Orders::OrderNumber)
                    .to_owned(),
            )
            .await?;

        // -- coupons ---------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_coupons_active_expires_at")
                    .table(Coupons::Table)
                    .col(Coupons::Active)
                    .col(Coupons::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        // -- coupon_usages ---------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_coupon_usages_coupon_id")
                    .table(CouponUsages::Table)
                    .col(CouponUsages::CouponId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_coupon_usages_user_id")
                    .table(CouponUsages::Table)
                    .col(CouponUsages::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_coupon_usages_order_id")
                    .table(CouponUsages::Table)
                    .col(CouponUsages::OrderId)
                    .to_owned(),
            )
            .await?;

        // -- reviews ---------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reviews_product_id_active_created_at")
                    .table(Reviews::Table)
                    .col(Reviews::ProductId)
                    .col(Reviews::Active)
                    .col(Reviews::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- admin_audit_logs ------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_action_created_at")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::Action)
                    .col(AdminAuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_resource_type_created_at")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ResourceType)
                    .col(AdminAuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_actor_user_id_created_at")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ActorUserId)
                    .col(AdminAuditLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- admin_settings --------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_settings_namespace_key")
                    .table(AdminSettings::Table)
                    .col(AdminSettings::Namespace)
                    .col(AdminSettings::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // -- users (idempotency / search) -----------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_email_verified_at_created_at")
                    .table(Users::Table)
                    .col(Users::EmailVerifiedAt)
                    .col(Users::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- email_logs ------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_email_logs_status_sent_at")
                    .table(EmailLogs::Table)
                    .col(EmailLogs::Status)
                    .col(EmailLogs::SentAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_email_logs_template_name_created_at")
                    .table(EmailLogs::Table)
                    .col(EmailLogs::TemplateName)
                    .col(EmailLogs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // -- roles -----------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_roles_name_resource_type_resource_id")
                    .table(Roles::Table)
                    .col(Roles::Name)
                    .col(Roles::ResourceType)
                    .col(Roles::ResourceId)
                    .to_owned(),
            )
            .await?;

        // -- users_roles: extra role_id index (composite PK already covers user_id)
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_roles_role_id")
                    .table(UsersRoles::Table)
                    .col(UsersRoles::RoleId)
                    .to_owned(),
            )
            .await?;

        // -- addresses -------------------------------------------------------------
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_addresses_user_id_default")
                    .table(Addresses::Table)
                    .col(Addresses::UserId)
                    .col(Addresses::Default)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drops: &[(&str, &str)] = &[
            ("idx_addresses_user_id_default", "addresses"),
            ("idx_users_roles_role_id", "users_roles"),
            ("idx_roles_name_resource_type_resource_id", "roles"),
            ("idx_email_logs_template_name_created_at", "email_logs"),
            ("idx_email_logs_status_sent_at", "email_logs"),
            ("idx_users_email_verified_at_created_at", "users"),
            ("idx_admin_settings_namespace_key", "admin_settings"),
            (
                "idx_admin_audit_logs_actor_user_id_created_at",
                "admin_audit_logs",
            ),
            (
                "idx_admin_audit_logs_resource_type_created_at",
                "admin_audit_logs",
            ),
            (
                "idx_admin_audit_logs_action_created_at",
                "admin_audit_logs",
            ),
            (
                "idx_reviews_product_id_active_created_at",
                "reviews",
            ),
            ("idx_coupon_usages_order_id", "coupon_usages"),
            ("idx_coupon_usages_user_id", "coupon_usages"),
            ("idx_coupon_usages_coupon_id", "coupon_usages"),
            ("idx_coupons_active_expires_at", "coupons"),
            ("idx_order_items_product_variant_id", "order_items"),
            ("idx_orders_user_id_created_at", "orders"),
            ("idx_orders_status_created_at", "orders"),
            ("idx_orders_order_number", "orders"),
            (
                "idx_gateway_customers_payment_gateway_id",
                "gateway_customers",
            ),
            (
                "idx_gateway_customers_user_id_gateway_id",
                "gateway_customers",
            ),
            (
                "idx_payment_setup_sessions_user_id_created_at",
                "payment_setup_sessions",
            ),
            (
                "idx_payment_sources_gateway_customer_id",
                "payment_sources",
            ),
            (
                "idx_payment_sources_payment_method_id",
                "payment_sources",
            ),
            ("idx_payment_sources_user_id", "payment_sources"),
            ("idx_payment_methods_active", "payment_methods"),
            (
                "idx_payment_methods_payment_gateway_id",
                "payment_methods",
            ),
            (
                "idx_payment_gateway_events_status_created_at",
                "payment_gateway_events",
            ),
            ("idx_payment_gateway_logs_created_at", "payment_gateway_logs"),
            (
                "idx_payment_gateway_logs_payment_session_id",
                "payment_gateway_logs",
            ),
            (
                "idx_payment_gateway_logs_payment_id_created_at",
                "payment_gateway_logs",
            ),
            ("idx_payment_refunds_payment_id", "payment_refunds"),
            (
                "idx_payment_sessions_payment_id_created_at",
                "payment_sessions",
            ),
            ("idx_payments_currency", "payments"),
            ("idx_payments_payment_method_id", "payments"),
            ("idx_payments_status_created_at", "payments"),
            ("idx_categories_slug", "categories"),
            ("idx_categories_active_position", "categories"),
            ("idx_categories_parent_id", "categories"),
            ("idx_products_featured_active", "products"),
            ("idx_products_category_id_active", "products"),
            ("idx_products_slug", "products"),
            ("idx_products_status_active", "products"),
            ("idx_products_active", "products"),
            (
                "idx_product_variant_images_variant_cover",
                "product_variant_images",
            ),
            (
                "idx_product_variant_images_variant_id",
                "product_variant_images",
            ),
            (
                "idx_product_images_product_id_position",
                "product_images",
            ),
            (
                "idx_product_images_product_id_cover",
                "product_images",
            ),
            ("idx_carts_user_id", "carts"),
            ("idx_cart_items_product_variant_id", "cart_items"),
            ("idx_cart_items_cart_product_variant", "cart_items"),
            ("idx_cart_items_cart_id", "cart_items"),
        ];

        for (name, table) in drops {
            manager
                .drop_index(
                    Index::drop()
                        .name(*name)
                        .table(Alias::new(*table))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum CartItems {
    Table,
    CartId,
    ProductId,
    ProductVariantId,
}

#[derive(Iden)]
enum Carts {
    Table,
    UserId,
}

#[derive(Iden)]
enum ProductImages {
    Table,
    ProductId,
    Cover,
    Position,
}

#[derive(Iden)]
enum ProductVariantImages {
    Table,
    ProductVariantId,
    Cover,
}

#[derive(Iden)]
enum Products {
    Table,
    Active,
    Status,
    Slug,
    CategoryId,
    Featured,
}

#[derive(Iden)]
enum Categories {
    Table,
    ParentId,
    Active,
    Position,
    Slug,
}

#[derive(Iden)]
enum Payments {
    Table,
    Status,
    CreatedAt,
    PaymentMethodId,
    Currency,
}

#[derive(Iden)]
enum PaymentSessions {
    Table,
    PaymentId,
    CreatedAt,
}

#[derive(Iden)]
enum PaymentRefunds {
    Table,
    PaymentId,
}

#[derive(Iden)]
enum PaymentGatewayLogs {
    Table,
    PaymentId,
    PaymentSessionId,
    CreatedAt,
}

#[derive(Iden)]
enum PaymentGatewayEvents {
    Table,
    Status,
    CreatedAt,
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    PaymentGatewayId,
    Active,
}

#[derive(Iden)]
enum PaymentSources {
    Table,
    UserId,
    PaymentMethodId,
    GatewayCustomerId,
}

#[derive(Iden)]
enum PaymentSetupSessions {
    Table,
    UserId,
    CreatedAt,
}

#[derive(Iden)]
enum GatewayCustomers {
    Table,
    UserId,
    PaymentGatewayId,
}

#[derive(Iden)]
enum OrderItems {
    Table,
    ProductVariantId,
}

#[derive(Iden)]
enum Orders {
    Table,
    UserId,
    Status,
    CreatedAt,
    OrderNumber,
}

#[derive(Iden)]
enum Coupons {
    Table,
    Active,
    ExpiresAt,
}

#[derive(Iden)]
enum CouponUsages {
    Table,
    CouponId,
    UserId,
    OrderId,
}

#[derive(Iden)]
enum Reviews {
    Table,
    ProductId,
    Active,
    CreatedAt,
}

#[derive(Iden)]
enum AdminAuditLogs {
    Table,
    Action,
    ResourceType,
    ActorUserId,
    CreatedAt,
}

#[derive(Iden)]
enum AdminSettings {
    Table,
    Namespace,
    Key,
}

#[derive(Iden)]
enum Users {
    Table,
    EmailVerifiedAt,
    CreatedAt,
}

#[derive(Iden)]
enum EmailLogs {
    Table,
    Status,
    SentAt,
    TemplateName,
    CreatedAt,
}

#[derive(Iden)]
enum Roles {
    Table,
    Name,
    ResourceType,
    ResourceId,
}

#[derive(Iden)]
enum UsersRoles {
    Table,
    RoleId,
}

#[derive(Iden)]
enum Addresses {
    Table,
    UserId,
    Default,
}
