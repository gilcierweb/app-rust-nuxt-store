use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "addresses",
            &[
                ("id", ColType::PkAuto),
                ("type", ColType::StringNull),
                ("first_name", ColType::StringNull),
                ("last_name", ColType::StringNull),
                ("company", ColType::StringNull),
                ("address1", ColType::StringNull),
                ("address2", ColType::StringNull),
                ("city", ColType::StringNull),
                ("state", ColType::StringNull),
                ("zip_code", ColType::StringNull),
                ("country", ColType::StringNull),
                ("phone", ColType::StringNull),
                ("default", ColType::BooleanNull),
            ],
            &[("user", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "addresses").await
    }
}
