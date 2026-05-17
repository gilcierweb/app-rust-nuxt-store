use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reviews_product_id")
                    .table(Reviews::Table)
                    .col(Reviews::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_wishlists_user_id_created_at")
                    .table(Wishlists::Table)
                    .col(Wishlists::UserId)
                    .col(Wishlists::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_wishlists_user_id_product_id")
                    .table(Wishlists::Table)
                    .col(Wishlists::UserId)
                    .col(Wishlists::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variants_product_id_position")
                    .table(ProductVariants::Table)
                    .col(ProductVariants::ProductId)
                    .col(ProductVariants::Position)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variant_options_product_variant_id")
                    .table(ProductVariantOptions::Table)
                    .col(ProductVariantOptions::ProductVariantId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_variant_options_variant_option_id")
                    .table(ProductVariantOptions::Table)
                    .col(ProductVariantOptions::VariantOptionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_variant_options_variant_option_id")
                    .table(ProductVariantOptions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_variant_options_product_variant_id")
                    .table(ProductVariantOptions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_product_variants_product_id_position")
                    .table(ProductVariants::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_wishlists_user_id_product_id")
                    .table(Wishlists::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_wishlists_user_id_created_at")
                    .table(Wishlists::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_reviews_product_id")
                    .table(Reviews::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Reviews {
    Table,
    ProductId,
}

#[derive(Iden)]
enum Wishlists {
    Table,
    UserId,
    ProductId,
    CreatedAt,
}

#[derive(Iden)]
enum ProductVariants {
    Table,
    ProductId,
    Position,
}

#[derive(Iden)]
enum ProductVariantOptions {
    Table,
    ProductVariantId,
    VariantOptionId,
}
