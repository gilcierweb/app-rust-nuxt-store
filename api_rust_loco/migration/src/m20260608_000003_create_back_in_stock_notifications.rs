use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BackInStockNotifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BackInStockNotifications::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::ProductVariantId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::NotifiedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BackInStockNotifications::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_back_in_stock_user")
                            .from(BackInStockNotifications::Table, BackInStockNotifications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_back_in_stock_variant")
                            .from(
                                BackInStockNotifications::Table,
                                BackInStockNotifications::ProductVariantId,
                            )
                            .to(ProductVariants::Table, ProductVariants::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_back_in_stock_user")
                    .table(BackInStockNotifications::Table)
                    .col(BackInStockNotifications::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_back_in_stock_variant")
                    .table(BackInStockNotifications::Table)
                    .col(BackInStockNotifications::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_back_in_stock_status")
                    .table(BackInStockNotifications::Table)
                    .col(BackInStockNotifications::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(BackInStockNotifications::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum BackInStockNotifications {
    Table,
    Id,
    UserId,
    ProductVariantId,
    Status,
    NotifiedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ProductVariants {
    Table,
    Id,
}
