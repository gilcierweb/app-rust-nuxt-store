use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add reserved_quantity column to product_variants
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .add_column(
                        ColumnDef::new(ProductVariants::ReservedQuantity)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // Add track_inventory column to product_variants
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .add_column(
                        ColumnDef::new(ProductVariants::TrackInventory)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // Add allow_backorder column to product_variants
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .add_column(
                        ColumnDef::new(ProductVariants::AllowBackorder)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Add low_stock_threshold column to product_variants
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .add_column(
                        ColumnDef::new(ProductVariants::LowStockThreshold)
                            .integer()
                            .not_null()
                            .default(5),
                    )
                    .to_owned(),
            )
            .await?;

        // Add inventory_quantity NOT NULL default 0
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .modify_column(
                        ColumnDef::new(ProductVariants::InventoryQuantity)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .drop_column(ProductVariants::ReservedQuantity)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .drop_column(ProductVariants::TrackInventory)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .drop_column(ProductVariants::AllowBackorder)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ProductVariants::Table)
                    .drop_column(ProductVariants::LowStockThreshold)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProductVariants {
    Table,
    InventoryQuantity,
    ReservedQuantity,
    TrackInventory,
    AllowBackorder,
    LowStockThreshold,
}
