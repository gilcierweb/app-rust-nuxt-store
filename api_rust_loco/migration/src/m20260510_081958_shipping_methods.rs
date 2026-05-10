use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "shipping_methods",
            &[
            
            ("id", ColType::PkAuto),
            
            ("name", ColType::StringNull),
            ("code", ColType::StringNull),
            ("base_price", ColType::DecimalNull),
            ("free_shipping_threshold", ColType::DecimalNull),
            ("active", ColType::BooleanNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "shipping_methods").await
    }
}
