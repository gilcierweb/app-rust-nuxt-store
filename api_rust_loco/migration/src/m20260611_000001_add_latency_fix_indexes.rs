use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // -- orders: composite index for admin list ORDER BY created_at DESC, id DESC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_orders_created_at_id")
                    .table(Orders::Table)
                    .col(Orders::CreatedAt)
                    .col(Orders::Id)
                    .to_owned(),
            )
            .await?;

        // -- products: covering index for list pagination
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_products_id_cover")
                    .table(Products::Table)
                    .col(Products::Id)
                    .to_owned(),
            )
            .await?;

        // -- product_images: partial covering index for cover image lookups
        conn.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS idx_product_images_cover_lookup
               ON product_images (product_id, position)
               WHERE cover = TRUE"#,
        )
        .await?;

        // -- posts: composite index for admin list ORDER BY created_at DESC, id DESC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_posts_created_at_id")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .col(Posts::Id)
                    .to_owned(),
            )
            .await?;

        // -- payments: composite index for admin list ORDER BY created_at DESC, id DESC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_created_at_id")
                    .table(Payments::Table)
                    .col(Payments::CreatedAt)
                    .col(Payments::Id)
                    .to_owned(),
            )
            .await?;

        // -- categories: index for position ordering
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_categories_position")
                    .table(Categories::Table)
                    .col(Categories::Position)
                    .to_owned(),
            )
            .await?;

        // -- admin_audit_logs: index for search on actor_name
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_actor_name")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ActorName)
                    .to_owned(),
            )
            .await?;

        // -- admin_audit_logs: index for search on actor_email
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_actor_email")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ActorEmail)
                    .to_owned(),
            )
            .await?;

        // -- admin_audit_logs: index for search on resource_label
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_audit_logs_resource_label")
                    .table(AdminAuditLogs::Table)
                    .col(AdminAuditLogs::ResourceLabel)
                    .to_owned(),
            )
            .await?;

        // -- payments: index for number search
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_number")
                    .table(Payments::Table)
                    .col(Payments::Number)
                    .to_owned(),
            )
            .await?;

        // -- payments: index for transaction_id search
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_transaction_id")
                    .table(Payments::Table)
                    .col(Payments::TransactionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        let drops: &[(&str, &str)] = &[
            ("idx_payments_transaction_id", "payments"),
            ("idx_payments_number", "payments"),
            ("idx_admin_audit_logs_resource_label", "admin_audit_logs"),
            ("idx_admin_audit_logs_actor_email", "admin_audit_logs"),
            ("idx_admin_audit_logs_actor_name", "admin_audit_logs"),
            ("idx_categories_position", "categories"),
            ("idx_payments_created_at_id", "payments"),
            ("idx_posts_created_at_id", "posts"),
            ("idx_products_id_cover", "products"),
            ("idx_orders_created_at_id", "orders"),
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

        conn.execute_unprepared(r#"DROP INDEX IF EXISTS idx_product_images_cover_lookup"#)
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Orders {
    Table,
    CreatedAt,
    Id,
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
}

#[derive(Iden)]
enum Posts {
    Table,
    CreatedAt,
    Id,
}

#[derive(Iden)]
enum Payments {
    Table,
    CreatedAt,
    Id,
    Number,
    TransactionId,
}

#[derive(Iden)]
enum Categories {
    Table,
    Position,
}

#[derive(Iden)]
enum AdminAuditLogs {
    Table,
    ActorName,
    ActorEmail,
    ResourceLabel,
}
