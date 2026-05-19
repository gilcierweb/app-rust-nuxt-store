use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PaymentMethods::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::PaymentGatewayId)
                            .integer()
                            .null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::Description).text().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::MethodType)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::DisplayOn)
                            .small_integer()
                            .not_null()
                            .default(3),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(PaymentMethods::AutoCapture)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_payment_methods_payment_gateway")
                    .from(PaymentMethods::Table, PaymentMethods::PaymentGatewayId)
                    .to(PaymentGateways::Table, PaymentGateways::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("uidx_payment_methods_code")
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::Code)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_methods_active_display_position")
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::Active)
                    .col(PaymentMethods::DisplayOn)
                    .col(PaymentMethods::Position)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_methods_gateway_id")
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for index_name in [
            "idx_payment_methods_gateway_id",
            "idx_payment_methods_active_display_position",
            "uidx_payment_methods_code",
        ] {
            manager
                .drop_index(
                    Index::drop()
                        .if_exists()
                        .name(index_name)
                        .table(PaymentMethods::Table)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_payment_methods_payment_gateway")
                    .table(PaymentMethods::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(PaymentMethods::Table)
                    .drop_column(PaymentMethods::AutoCapture)
                    .drop_column(PaymentMethods::Position)
                    .drop_column(PaymentMethods::DisplayOn)
                    .drop_column(PaymentMethods::MethodType)
                    .drop_column(PaymentMethods::Description)
                    .drop_column(PaymentMethods::PaymentGatewayId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Code,
    Active,
    PaymentGatewayId,
    Description,
    MethodType,
    DisplayOn,
    Position,
    AutoCapture,
}

#[derive(Iden)]
enum PaymentGateways {
    Table,
    Id,
}
