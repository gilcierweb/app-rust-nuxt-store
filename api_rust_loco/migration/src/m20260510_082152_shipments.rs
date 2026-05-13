use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "shipments",
            &[
                ("id", ColType::PkAuto),
                ("tracking_number", ColType::StringNull),
                ("carrier", ColType::StringNull),
                ("status", ColType::IntegerNull),
                ("shipped_at", ColType::DateTimeNull),
                ("delivered_at", ColType::DateTimeNull),
            ],
            &[("order", ""), ("shipping_method", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "shipments").await
    }
}
