use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "products",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::StringNull),
                ("slug", ColType::StringNull),
                ("sku", ColType::StringNull),
                ("short_description", ColType::TextNull),
                ("description", ColType::TextNull),
                ("price", ColType::DecimalNull),
                ("cost_price", ColType::DecimalNull),
                ("compare_price", ColType::DecimalNull),
                ("featured", ColType::BooleanNull),
                ("active", ColType::BooleanNull),
                ("status", ColType::IntegerNull),
            ],
            &[("category", "")],
        )
        .await?;

        m.create_index(
            sea_query::Index::create()
                .name("idx_products_slug")
                .table("products")
                .col("slug")
                .index_type(sea_query::IndexType::BTree)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_index(
            sea_query::Index::drop()
                .name("idx_products_slug")
                .table("products")
                .to_owned(),
        )
        .await?;
        drop_table(m, "products").await
    }
}
