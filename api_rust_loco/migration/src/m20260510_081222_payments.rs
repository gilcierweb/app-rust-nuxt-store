use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "payments",
            &[
            
            ("id", ColType::PkAuto),
            
            ("amount", ColType::DecimalNull),
            ("currency", ColType::StringNull),
            ("status", ColType::IntegerNull),
            ("transaction_id", ColType::StringNull),
            ("gateway_response", ColType::TextNull),
            ("processed_at", ColType::DateTimeNull),
            ],
            &[
            ("order", ""),
            ("payment_method", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "payments").await
    }
}
