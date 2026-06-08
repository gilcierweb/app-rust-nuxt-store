use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockMovements::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockMovements::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::ProductVariantId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::OrderId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::UserId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Type)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::QuantityBefore)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::QuantityAfter)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Reference)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Notes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_movements_product_variant")
                            .from(StockMovements::Table, StockMovements::ProductVariantId)
                            .to(ProductVariants::Table, ProductVariants::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_movements_order")
                            .from(StockMovements::Table, StockMovements::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_movements_user")
                            .from(StockMovements::Table, StockMovements::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .index(
                        Index::create()
                            .name("idx_stock_movements_variant")
                            .col(StockMovements::ProductVariantId),
                    )
                    .index(
                        Index::create()
                            .name("idx_stock_movements_order")
                            .col(StockMovements::OrderId),
                    )
                    .index(
                        Index::create()
                            .name("idx_stock_movements_user")
                            .col(StockMovements::UserId),
                    )
                    .index(
                        Index::create()
                            .name("idx_stock_movements_type")
                            .col(StockMovements::Type),
                    )
                    .index(
                        Index::create()
                            .name("idx_stock_movements_created")
                            .col(StockMovements::CreatedAt),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockMovements::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum StockMovements {
    Table,
    Id,
    ProductVariantId,
    OrderId,
    UserId,
    Type,
    Quantity,
    QuantityBefore,
    QuantityAfter,
    Reference,
    Notes,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ProductVariants {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
