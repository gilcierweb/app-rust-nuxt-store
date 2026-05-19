use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentSetupSessions::Table)
                    .if_not_exists()
                    .col(pk(PaymentSetupSessions::Id))
                    .col(
                        ColumnDef::new(PaymentSetupSessions::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::PaymentMethodId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::PaymentSourceId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::Status)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::ExternalSetupId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::ExternalClientSecret)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::ExpiresAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSetupSessions::CompletedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(timestamptz(PaymentSetupSessions::CreatedAt))
                    .col(timestamptz(PaymentSetupSessions::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_setup_sessions_user")
                            .from(PaymentSetupSessions::Table, PaymentSetupSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_setup_sessions_payment_method")
                            .from(
                                PaymentSetupSessions::Table,
                                PaymentSetupSessions::PaymentMethodId,
                            )
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_setup_sessions_payment_source")
                            .from(
                                PaymentSetupSessions::Table,
                                PaymentSetupSessions::PaymentSourceId,
                            )
                            .to(PaymentSources::Table, PaymentSources::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_setup_sessions_user_status")
                    .table(PaymentSetupSessions::Table)
                    .col(PaymentSetupSessions::UserId)
                    .col(PaymentSetupSessions::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_setup_sessions_method_external")
                    .table(PaymentSetupSessions::Table)
                    .col(PaymentSetupSessions::PaymentMethodId)
                    .col(PaymentSetupSessions::ExternalSetupId)
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
                    .table(PaymentSetupSessions::Table)
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
    column_def.integer().not_null().auto_increment().primary_key();
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
enum PaymentSetupSessions {
    Table,
    Id,
    UserId,
    PaymentMethodId,
    PaymentSourceId,
    Status,
    ExternalSetupId,
    ExternalClientSecret,
    ExpiresAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentSources {
    Table,
    Id,
}
