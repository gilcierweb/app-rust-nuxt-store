use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "email_logs",
            &[
                ("id", ColType::PkAuto),
                ("recipient", ColType::String),
                ("template_name", ColType::String),
                ("subject", ColType::String),
                ("locals_json", ColType::Text),
                ("status", ColType::SmallInteger),
                ("error_message", ColType::TextNull),
                ("sent_at", ColType::DateTimeNull),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "email_logs").await
    }
}

