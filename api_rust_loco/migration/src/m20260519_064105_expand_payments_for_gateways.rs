use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .add_column_if_not_exists(ColumnDef::new(Payments::Number).string().null())
                    .add_column_if_not_exists(
                        ColumnDef::new(Payments::PaymentSourceId).integer().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Payments::Intent)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Payments::ExternalPaymentId).string().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Payments::ExternalStatus).string().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Payments::IdempotencyKey).string().null(),
                    )
                    .add_column_if_not_exists(ColumnDef::new(Payments::FailureCode).string().null())
                    .add_column_if_not_exists(ColumnDef::new(Payments::FailureMessage).text().null())
                    .add_column_if_not_exists(ColumnDef::new(Payments::AuthorizedAt).timestamp().null())
                    .add_column_if_not_exists(ColumnDef::new(Payments::CapturedAt).timestamp().null())
                    .add_column_if_not_exists(ColumnDef::new(Payments::VoidedAt).timestamp().null())
                    .add_column_if_not_exists(ColumnDef::new(Payments::CancelledAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_payments_payment_source")
                    .from(Payments::Table, Payments::PaymentSourceId)
                    .to(PaymentSources::Table, PaymentSources::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        for index_statement in [
            Index::create()
                .if_not_exists()
                .unique()
                .name("uidx_payments_number")
                .table(Payments::Table)
                .col(Payments::Number)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .unique()
                .name("uidx_payments_idempotency_key")
                .table(Payments::Table)
                .col(Payments::IdempotencyKey)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_payments_order_status")
                .table(Payments::Table)
                .col(Payments::OrderId)
                .col(Payments::Status)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_payments_payment_source_id")
                .table(Payments::Table)
                .col(Payments::PaymentSourceId)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_payments_method_external")
                .table(Payments::Table)
                .col(Payments::PaymentMethodId)
                .col(Payments::ExternalPaymentId)
                .to_owned(),
        ] {
            manager.create_index(index_statement).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for index_name in [
            "idx_payments_method_external",
            "idx_payments_payment_source_id",
            "idx_payments_order_status",
            "uidx_payments_idempotency_key",
            "uidx_payments_number",
        ] {
            manager
                .drop_index(
                    Index::drop()
                        .if_exists()
                        .name(index_name)
                        .table(Payments::Table)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_payments_payment_source")
                    .table(Payments::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .drop_column(Payments::CancelledAt)
                    .drop_column(Payments::VoidedAt)
                    .drop_column(Payments::CapturedAt)
                    .drop_column(Payments::AuthorizedAt)
                    .drop_column(Payments::FailureMessage)
                    .drop_column(Payments::FailureCode)
                    .drop_column(Payments::IdempotencyKey)
                    .drop_column(Payments::ExternalStatus)
                    .drop_column(Payments::ExternalPaymentId)
                    .drop_column(Payments::Intent)
                    .drop_column(Payments::PaymentSourceId)
                    .drop_column(Payments::Number)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Payments {
    Table,
    OrderId,
    PaymentMethodId,
    Status,
    Number,
    PaymentSourceId,
    Intent,
    ExternalPaymentId,
    ExternalStatus,
    IdempotencyKey,
    FailureCode,
    FailureMessage,
    AuthorizedAt,
    CapturedAt,
    VoidedAt,
    CancelledAt,
}

#[derive(Iden)]
enum PaymentSources {
    Table,
    Id,
}
