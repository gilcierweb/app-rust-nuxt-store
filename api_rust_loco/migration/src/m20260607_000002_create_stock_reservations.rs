use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockReservations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockReservations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::ProductVariantId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::CartId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::OrderId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::Status)
                            .string_len(20)
                            .not_null()
                            .default("active"),
                    )
                    .col(
                        ColumnDef::new(StockReservations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockReservations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_reservations_product_variant")
                            .from(StockReservations::Table, StockReservations::ProductVariantId)
                            .to(ProductVariants::Table, ProductVariants::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_reservations_cart")
                            .from(StockReservations::Table, StockReservations::CartId)
                            .to(Carts::Table, Carts::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_reservations_order")
                            .from(StockReservations::Table, StockReservations::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_variant")
                    .table(StockReservations::Table)
                    .col(StockReservations::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_cart")
                    .table(StockReservations::Table)
                    .col(StockReservations::CartId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_order")
                    .table(StockReservations::Table)
                    .col(StockReservations::OrderId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_status")
                    .table(StockReservations::Table)
                    .col(StockReservations::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_expires")
                    .table(StockReservations::Table)
                    .col(StockReservations::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockReservations::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum StockReservations {
    Table,
    Id,
    ProductVariantId,
    CartId,
    OrderId,
    Quantity,
    ExpiresAt,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ProductVariants {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Carts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}
