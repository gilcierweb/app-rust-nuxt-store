use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentGateways::Table)
                    .if_not_exists()
                    .col(pk(PaymentGateways::Id))
                    .col(ColumnDef::new(PaymentGateways::Code).string().not_null())
                    .col(ColumnDef::new(PaymentGateways::Name).string().not_null())
                    .col(ColumnDef::new(PaymentGateways::Driver).string().not_null())
                    .col(
                        ColumnDef::new(PaymentGateways::Environment)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::Status)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::SupportsAuthorize)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::SupportsCapture)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::SupportsRefund)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::SupportsVoid)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PaymentGateways::SupportsSavedSources)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(PaymentGateways::PublicKeyEnv).string().null())
                    .col(ColumnDef::new(PaymentGateways::SecretKeyEnv).string().null())
                    .col(
                        ColumnDef::new(PaymentGateways::WebhookSecretEnv)
                            .string()
                            .null(),
                    )
                    .col(timestamptz(PaymentGateways::CreatedAt))
                    .col(timestamptz(PaymentGateways::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("uidx_payment_gateways_code")
                    .table(PaymentGateways::Table)
                    .col(PaymentGateways::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_gateways_status_environment")
                    .table(PaymentGateways::Table)
                    .col(PaymentGateways::Status)
                    .col(PaymentGateways::Environment)
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
                    .table(PaymentGateways::Table)
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
enum PaymentGateways {
    Table,
    Id,
    Code,
    Name,
    Driver,
    Environment,
    Status,
    SupportsAuthorize,
    SupportsCapture,
    SupportsRefund,
    SupportsVoid,
    SupportsSavedSources,
    PublicKeyEnv,
    SecretKeyEnv,
    WebhookSecretEnv,
    CreatedAt,
    UpdatedAt,
}
