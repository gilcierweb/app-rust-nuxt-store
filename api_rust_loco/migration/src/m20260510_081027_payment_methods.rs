use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "payment_methods",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::StringNull),
                ("code", ColType::StringNull),
                ("active", ColType::BooleanNull),
                ("settings", ColType::TextNull),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "payment_methods").await
    }
}
