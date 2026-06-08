use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Index for variants list endpoint: ORDER BY position ASC, id ASC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variants_position_id")
                    .table(ProductVariants::Table)
                    .col(ProductVariants::Position)
                    .col(ProductVariants::Id)
                    .to_owned(),
            )
            .await?;

        // Covering index for reviews by product: filter by product_id, sort by created_at DESC
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reviews_product_id_created_at_desc")
                    .table(Reviews::Table)
                    .col(Reviews::ProductId)
                    .col(Reviews::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Covering index for product_images LATERAL subquery:
        // WHERE product_id = $1 ORDER BY cover DESC, position ASC, id ASC LIMIT 1
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX IF NOT EXISTS idx_product_images_cover_lateral \
                 ON product_images (product_id, cover DESC, position ASC NULLS LAST, id ASC)",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_variants_position_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_reviews_product_id_created_at_desc")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_images_cover_lateral")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProductVariants {
    Table,
    Position,
    Id,
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    ProductId,
    CreatedAt,
}
