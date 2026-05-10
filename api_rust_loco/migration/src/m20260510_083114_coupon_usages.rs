use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "coupon_usages",
            &[
            
            ("id", ColType::PkAuto),
            
            ("used_at", ColType::DateTimeNull),
            ],
            &[
            ("coupon", ""),
            ("user", ""),
            ("order", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "coupon_usages").await
    }
}
