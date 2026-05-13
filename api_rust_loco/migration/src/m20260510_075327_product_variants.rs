use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "product_variants",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::StringNull),
                ("sku", ColType::StringNull),
                ("price", ColType::DecimalNull),
                ("cost_price", ColType::DecimalNull),
                ("compare_price", ColType::DecimalNull),
                ("inventory_quantity", ColType::IntegerNull),
                ("weight", ColType::DecimalNull),
                ("barcode", ColType::StringNull),
                ("position", ColType::IntegerNull),
                ("active", ColType::BooleanNull),
            ],
            &[("product", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "product_variants").await
    }
}
