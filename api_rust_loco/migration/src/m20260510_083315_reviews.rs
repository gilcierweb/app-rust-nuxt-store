use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "reviews",
            &[
            
            ("id", ColType::PkAuto),
            
            ("rating", ColType::IntegerNull),
            ("title", ColType::StringNull),
            ("comment", ColType::TextNull),
            ("verified_purchase", ColType::BooleanNull),
            ("active", ColType::BooleanNull),
            ],
            &[
            ("user", ""),
            ("product", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "reviews").await
    }
}
