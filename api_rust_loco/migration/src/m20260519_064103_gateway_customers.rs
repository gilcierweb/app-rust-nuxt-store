use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GatewayCustomers::Table)
                    .if_not_exists()
                    .col(pk(GatewayCustomers::Id))
                    .col(
                        ColumnDef::new(GatewayCustomers::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GatewayCustomers::PaymentGatewayId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GatewayCustomers::ExternalCustomerId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GatewayCustomers::Status)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(timestamptz(GatewayCustomers::CreatedAt))
                    .col(timestamptz(GatewayCustomers::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gateway_customers_user")
                            .from(GatewayCustomers::Table, GatewayCustomers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gateway_customers_payment_gateway")
                            .from(GatewayCustomers::Table, GatewayCustomers::PaymentGatewayId)
                            .to(PaymentGateways::Table, PaymentGateways::Id)
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
                    .unique()
                    .name("uidx_gateway_customers_user_gateway")
                    .table(GatewayCustomers::Table)
                    .col(GatewayCustomers::UserId)
                    .col(GatewayCustomers::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("uidx_gateway_customers_gateway_external")
                    .table(GatewayCustomers::Table)
                    .col(GatewayCustomers::PaymentGatewayId)
                    .col(GatewayCustomers::ExternalCustomerId)
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
                    .table(GatewayCustomers::Table)
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
enum GatewayCustomers {
    Table,
    Id,
    UserId,
    PaymentGatewayId,
    ExternalCustomerId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentGateways {
    Table,
    Id,
}
