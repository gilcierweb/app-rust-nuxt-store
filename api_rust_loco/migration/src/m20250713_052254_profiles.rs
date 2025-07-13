use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "profiles",
            &[
            
            ("id", ColType::PkAuto),
            
            ("first_name", ColType::StringNull),
            ("last_name", ColType::StringNull),
            ("full_name", ColType::StringNull),
            ("username", ColType::StringNull),
            ("nickname", ColType::StringNull),
            ("phone", ColType::BigIntegerNull),
            ("birth_date", ColType::DateNull),
            ("avatar", ColType::StringNull),
            ("bio", ColType::TextNull),
            ("whatsapp", ColType::BigIntegerNull),
            ],
            &[
            ("user", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "profiles").await
    }
}
