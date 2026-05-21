use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "product_variant_images",
            &[
                ("id", ColType::PkAuto),
                ("image", ColType::StringNull),
                ("alt_text", ColType::StringNull),
                ("active", ColType::BooleanNull),
                ("cover", ColType::BooleanNull),
                ("position", ColType::IntegerNull),
            ],
            &[("product_variant", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "product_variant_images").await
    }
}
