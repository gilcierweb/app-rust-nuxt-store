use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JwtBlacklist::Table)
                    .if_not_exists()
                    .col(pk(JwtBlacklist::Id))
                    .col(
                        ColumnDef::new(JwtBlacklist::TokenHash)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(JwtBlacklist::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(JwtBlacklist::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(timestamptz(JwtBlacklist::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_jwt_blacklist_user")
                            .from(JwtBlacklist::Table, JwtBlacklist::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_jwt_blacklist_token_hash")
                    .table(JwtBlacklist::Table)
                    .col(JwtBlacklist::TokenHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_jwt_blacklist_expires_at")
                    .table(JwtBlacklist::Table)
                    .col(JwtBlacklist::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_jwt_blacklist_user_id")
                    .table(JwtBlacklist::Table)
                    .col(JwtBlacklist::UserId)
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
                    .table(JwtBlacklist::Table)
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
enum JwtBlacklist {
    Table,
    Id,
    TokenHash,
    UserId,
    ExpiresAt,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
