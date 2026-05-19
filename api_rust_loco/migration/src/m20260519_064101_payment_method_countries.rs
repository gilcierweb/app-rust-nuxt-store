use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentMethodCountries::Table)
                    .if_not_exists()
                    .col(pk(PaymentMethodCountries::Id))
                    .col(
                        ColumnDef::new(PaymentMethodCountries::PaymentMethodId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodCountries::CountryCode)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_method_countries_payment_method")
                            .from(
                                PaymentMethodCountries::Table,
                                PaymentMethodCountries::PaymentMethodId,
                            )
                            .to(PaymentMethods::Table, PaymentMethods::Id)
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
                    .name("uidx_payment_method_countries_method_country")
                    .table(PaymentMethodCountries::Table)
                    .col(PaymentMethodCountries::PaymentMethodId)
                    .col(PaymentMethodCountries::CountryCode)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payment_method_countries_country")
                    .table(PaymentMethodCountries::Table)
                    .col(PaymentMethodCountries::CountryCode)
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
                    .table(PaymentMethodCountries::Table)
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

#[derive(Iden)]
enum PaymentMethodCountries {
    Table,
    Id,
    PaymentMethodId,
    CountryCode,
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Id,
}
