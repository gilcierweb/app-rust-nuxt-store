use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentGatewayLogs::Table)
                    .if_not_exists()
                    .col(pk(PaymentGatewayLogs::Id))
                    .col(ColumnDef::new(PaymentGatewayLogs::PaymentId).integer().null())
                    .col(
                        ColumnDef::new(PaymentGatewayLogs::PaymentSessionId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayLogs::PaymentGatewayEventId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayLogs::Direction)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentGatewayLogs::Level)
                            .small_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PaymentGatewayLogs::Message).text().null())
                    .col(ColumnDef::new(PaymentGatewayLogs::Payload).text().null())
                    .col(timestamptz(PaymentGatewayLogs::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_gateway_logs_payment")
                            .from(PaymentGatewayLogs::Table, PaymentGatewayLogs::PaymentId)
                            .to(Payments::Table, Payments::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_gateway_logs_payment_session")
                            .from(
                                PaymentGatewayLogs::Table,
                                PaymentGatewayLogs::PaymentSessionId,
                            )
                            .to(PaymentSessions::Table, PaymentSessions::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_gateway_logs_gateway_event")
                            .from(
                                PaymentGatewayLogs::Table,
                                PaymentGatewayLogs::PaymentGatewayEventId,
                            )
                            .to(PaymentGatewayEvents::Table, PaymentGatewayEvents::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        for index_statement in [
            Index::create()
                .if_not_exists()
                .name("idx_gateway_logs_payment_id")
                .table(PaymentGatewayLogs::Table)
                .col(PaymentGatewayLogs::PaymentId)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_gateway_logs_session_id")
                .table(PaymentGatewayLogs::Table)
                .col(PaymentGatewayLogs::PaymentSessionId)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_gateway_logs_event_id")
                .table(PaymentGatewayLogs::Table)
                .col(PaymentGatewayLogs::PaymentGatewayEventId)
                .to_owned(),
            Index::create()
                .if_not_exists()
                .name("idx_gateway_logs_level_created")
                .table(PaymentGatewayLogs::Table)
                .col(PaymentGatewayLogs::Level)
                .col(PaymentGatewayLogs::CreatedAt)
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
                    .table(PaymentGatewayLogs::Table)
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
enum PaymentGatewayLogs {
    Table,
    Id,
    PaymentId,
    PaymentSessionId,
    PaymentGatewayEventId,
    Direction,
    Level,
    Message,
    Payload,
    CreatedAt,
}

#[derive(Iden)]
enum Payments {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentSessions {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentGatewayEvents {
    Table,
    Id,
}
