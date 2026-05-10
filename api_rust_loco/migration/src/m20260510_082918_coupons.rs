use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "coupons",
            &[
            
            ("id", ColType::PkAuto),
            
            ("code", ColType::StringNull),
            ("discount_type", ColType::IntegerNull),
            ("discount_value", ColType::DecimalNull),
            ("minimum_amount", ColType::DecimalNull),
            ("maximum_discount", ColType::DecimalNull),
            ("usage_limit", ColType::IntegerNull),
            ("used_count", ColType::IntegerNull),
            ("expires_at", ColType::DateTimeNull),
            ("active", ColType::BooleanNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "coupons").await
    }
}
