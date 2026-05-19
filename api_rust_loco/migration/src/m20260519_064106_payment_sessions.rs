use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentSessions::Table)
                    .if_not_exists()
                    .col(pk(PaymentSessions::Id))
                    .col(ColumnDef::new(PaymentSessions::PaymentId).integer().not_null())
                    .col(
                        ColumnDef::new(PaymentSessions::PaymentMethodId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSessions::Status)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSessions::ExternalSessionId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSessions::ExternalClientSecret)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(PaymentSessions::ExpiresAt).timestamp().null())
                    .col(ColumnDef::new(PaymentSessions::CompletedAt).timestamp().null())
                    .col(timestamptz(PaymentSessions::CreatedAt))
                    .col(timestamptz(PaymentSessions::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sessions_payment")
                            .from(PaymentSessions::Table, PaymentSessions::PaymentId)
                            .to(Payments::Table, Payments::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sessions_payment_method")
                            .from(PaymentSessions::Table, PaymentSessions::PaymentMethodId)
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sessions_payment_status")
                    .table(PaymentSessions::Table)
                    .col(PaymentSessions::PaymentId)
                    .col(PaymentSessions::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sessions_expires_at")
                    .table(PaymentSessions::Table)
                    .col(PaymentSessions::ExpiresAt)
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
                    .table(PaymentSessions::Table)
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
enum PaymentSessions {
    Table,
    Id,
    PaymentId,
    PaymentMethodId,
    Status,
    ExternalSessionId,
    ExternalClientSecret,
    ExpiresAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Payments {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Id,
}
