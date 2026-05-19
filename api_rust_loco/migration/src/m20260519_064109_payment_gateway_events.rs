use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentGatewayEvents::Table)
                    .if_not_exists()
                    .col(pk(PaymentGatewayEvents::Id))
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::PaymentGatewayId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::EventType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::ExternalEventId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::Status)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::SignatureValid)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::Payload)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::ProcessedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayEvents::FailureMessage)
                            .text()
                            .null(),
                    )
                    .col(timestamptz(PaymentGatewayEvents::CreatedAt))
                    .col(timestamptz(PaymentGatewayEvents::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_gateway_events_payment_gateway")
                            .from(
                                PaymentGatewayEvents::Table,
                                PaymentGatewayEvents::PaymentGatewayId,
                            )
                            .to(PaymentGateways::Table, PaymentGateways::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        for index_statement in [
            Index::create()
                .if_not_exists()
                .unique()
                .name("uidx_gateway_events_gateway_external")
                .table(PaymentGatewayEvents::Table)
                .col(PaymentGatewayEvents::PaymentGatewayId)
                .col(PaymentGatewayEvents::ExternalEventId)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_gateway_events_gateway_type")
                .table(PaymentGatewayEvents::Table)
                .col(PaymentGatewayEvents::PaymentGatewayId)
                .col(PaymentGatewayEvents::EventType)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_gateway_events_status_created")
                .table(PaymentGatewayEvents::Table)
                .col(PaymentGatewayEvents::Status)
                .col(PaymentGatewayEvents::CreatedAt)
                .to_owned(),
        ] {
            manager.create_index(index_statement).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PaymentGatewayEvents::Table)
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
enum PaymentGatewayEvents {
    Table,
    Id,
    PaymentGatewayId,
    EventType,
    ExternalEventId,
    Status,
    SignatureValid,
    Payload,
    ProcessedAt,
    FailureMessage,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PaymentGateways {
    Table,
    Id,
}
