use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentRefunds::Table)
                    .if_not_exists()
                    .col(pk(PaymentRefunds::Id))
                    .col(ColumnDef::new(PaymentRefunds::PaymentId).integer().not_null())
                    .col(ColumnDef::new(PaymentRefunds::Amount).decimal().not_null())
                    .col(ColumnDef::new(PaymentRefunds::Currency).string().not_null())
                    .col(
                        ColumnDef::new(PaymentRefunds::Status)
                            .small_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PaymentRefunds::Reason).small_integer().null())
                    .col(
                        ColumnDef::new(PaymentRefunds::ExternalRefundId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentRefunds::IdempotencyKey)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(PaymentRefunds::FailureCode).string().null())
                    .col(ColumnDef::new(PaymentRefunds::FailureMessage).text().null())
                    .col(ColumnDef::new(PaymentRefunds::ProcessedAt).timestamp().null())
                    .col(timestamptz(PaymentRefunds::CreatedAt))
                    .col(timestamptz(PaymentRefunds::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_refunds_payment")
                            .from(PaymentRefunds::Table, PaymentRefunds::PaymentId)
                            .to(Payments::Table, Payments::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_refunds_payment_status")
                    .table(PaymentRefunds::Table)
                    .col(PaymentRefunds::PaymentId)
                    .col(PaymentRefunds::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_refunds_external_id")
                    .table(PaymentRefunds::Table)
                    .col(PaymentRefunds::ExternalRefundId)
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
                    .table(PaymentRefunds::Table)
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
enum PaymentRefunds {
    Table,
    Id,
    PaymentId,
    Amount,
    Currency,
    Status,
    Reason,
    ExternalRefundId,
    IdempotencyKey,
    FailureCode,
    FailureMessage,
    ProcessedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Payments {
    Table,
    Id,
}
