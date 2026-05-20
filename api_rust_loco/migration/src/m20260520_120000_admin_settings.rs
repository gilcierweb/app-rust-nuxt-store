use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminSettings::Table)
                    .if_not_exists()
                    .col(pk(AdminSettings::Id))
                    .col(ColumnDef::new(AdminSettings::Namespace).string().not_null())
                    .col(ColumnDef::new(AdminSettings::Key).string().not_null())
                    .col(ColumnDef::new(AdminSettings::Value).text().null())
                    .col(
                        ColumnDef::new(AdminSettings::ValueType)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(timestamptz(AdminSettings::CreatedAt))
                    .col(timestamptz(AdminSettings::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("uidx_admin_settings_namespace_key")
                    .table(AdminSettings::Table)
                    .col(AdminSettings::Namespace)
                    .col(AdminSettings::Key)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_admin_settings_namespace")
                    .table(AdminSettings::Table)
                    .col(AdminSettings::Namespace)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(AdminSettings::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

fn pk<T>(column: T) -> ColumnDef
where
    T: Iden + 'static,
{
    let mut column_def = ColumnDef::new(column);
    column_def
        .integer()
        .not_null()
        .auto_increment()
        .primary_key();
    column_def
}

fn timestamptz<T>(column: T) -> ColumnDef
where
    T: Iden + 'static,
{
    let mut column_def = ColumnDef::new(column);
    column_def.timestamp_with_time_zone().not_null();
    column_def
}

#[derive(Iden)]
enum AdminSettings {
    Table,
    Id,
    Namespace,
    Key,
    Value,
    ValueType,
    CreatedAt,
    UpdatedAt,
}
