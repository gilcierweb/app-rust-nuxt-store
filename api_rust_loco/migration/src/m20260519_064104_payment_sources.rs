use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentSources::Table)
                    .if_not_exists()
                    .col(pk(PaymentSources::Id))
                    .col(ColumnDef::new(PaymentSources::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(PaymentSources::PaymentMethodId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSources::GatewayCustomerId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSources::SourceType)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSources::ExternalSourceId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PaymentSources::Brand).string().null())
                    .col(ColumnDef::new(PaymentSources::Last4).string().null())
                    .col(
                        ColumnDef::new(PaymentSources::ExpMonth)
                            .small_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSources::ExpYear)
                            .small_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(PaymentSources::HolderName).string().null())
                    .col(
                        ColumnDef::new(PaymentSources::BillingAddressId)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentSources::Status)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(timestamptz(PaymentSources::CreatedAt))
                    .col(timestamptz(PaymentSources::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sources_user")
                            .from(PaymentSources::Table, PaymentSources::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sources_payment_method")
                            .from(PaymentSources::Table, PaymentSources::PaymentMethodId)
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sources_gateway_customer")
                            .from(PaymentSources::Table, PaymentSources::GatewayCustomerId)
                            .to(GatewayCustomers::Table, GatewayCustomers::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_sources_billing_address")
                            .from(PaymentSources::Table, PaymentSources::BillingAddressId)
                            .to(Addresses::Table, Addresses::Id)
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
                    .unique()
                    .name("uidx_payment_sources_method_external")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::PaymentMethodId)
                    .col(PaymentSources::ExternalSourceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sources_user_status")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::UserId)
                    .col(PaymentSources::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_sources_gateway_customer")
                    .table(PaymentSources::Table)
                    .col(PaymentSources::GatewayCustomerId)
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
                    .table(PaymentSources::Table)
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
enum PaymentSources {
    Table,
    Id,
    UserId,
    PaymentMethodId,
    GatewayCustomerId,
    SourceType,
    ExternalSourceId,
    Brand,
    Last4,
    ExpMonth,
    ExpYear,
    HolderName,
    BillingAddressId,
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
enum PaymentMethods {
    Table,
    Id,
}

#[derive(Iden)]
enum GatewayCustomers {
    Table,
    Id,
}

#[derive(Iden)]
enum Addresses {
    Table,
    Id,
}
