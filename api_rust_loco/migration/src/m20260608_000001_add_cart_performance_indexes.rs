use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Covering index for LATERAL JOIN in get_cart_with_items
        // Eliminates sort on position inside the lateral subquery
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_images_cover_lookup")
                    .table(ProductImages::Table)
                    .col(ProductImages::ProductId)
                    .col(ProductImages::Cover)
                    .col(ProductImages::Position)
                    .to_owned(),
            )
            .await?;

        // Composite index for expire_stale_reservations
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_stock_reservations_status_expires")
                    .table(StockReservations::Table)
                    .col(StockReservations::Status)
                    .col(StockReservations::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_images_cover_lookup")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_stock_reservations_status_expires")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProductImages {
    Table,
    ProductId,
    Cover,
    Position,
}

#[derive(DeriveIden)]
enum StockReservations {
    Table,
    Status,
    ExpiresAt,
}
