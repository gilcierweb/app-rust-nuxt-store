use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_payment_gateways(manager).await?;
        expand_payment_methods(manager).await?;
        create_payment_method_eligibility(manager).await?;
        create_gateway_customers(manager).await?;
        create_payment_sources(manager).await?;
        expand_payments(manager).await?;
        create_payment_sessions(manager).await?;
        create_payment_setup_sessions(manager).await?;
        create_payment_refunds(manager).await?;
        create_payment_gateway_events(manager).await?;
        create_payment_gateway_logs(manager).await?;
        create_indexes(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_indexes(manager).await?;

        manager
            .drop_table(Table::drop().table(PaymentGatewayLogs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaymentGatewayEvents::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaymentRefunds::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaymentSetupSessions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaymentSessions::Table).to_owned())
            .await?;

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
                    .drop_column(Payments::Number)
                    .drop_column(Payments::PaymentSourceId)
                    .drop_column(Payments::Intent)
                    .drop_column(Payments::ExternalPaymentId)
                    .drop_column(Payments::ExternalStatus)
                    .drop_column(Payments::IdempotencyKey)
                    .drop_column(Payments::FailureCode)
                    .drop_column(Payments::FailureMessage)
                    .drop_column(Payments::AuthorizedAt)
                    .drop_column(Payments::CapturedAt)
                    .drop_column(Payments::VoidedAt)
                    .drop_column(Payments::CancelledAt)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(PaymentSources::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GatewayCustomers::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(PaymentMethodCurrencies::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(PaymentMethodCountries::Table).to_owned())
            .await?;

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
                    .drop_column(PaymentMethods::PaymentGatewayId)
                    .drop_column(PaymentMethods::Description)
                    .drop_column(PaymentMethods::MethodType)
                    .drop_column(PaymentMethods::DisplayOn)
                    .drop_column(PaymentMethods::Position)
                    .drop_column(PaymentMethods::AutoCapture)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(PaymentGateways::Table).to_owned())
            .await?;

        Ok(())
    }
}

async fn create_payment_gateways(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
        .await
}

async fn expand_payment_methods(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table(PaymentMethods::Table)
                .add_column(
                    ColumnDef::new(PaymentMethods::PaymentGatewayId)
                        .integer()
                        .null(),
                )
                .add_column(ColumnDef::new(PaymentMethods::Description).text().null())
                .add_column(
                    ColumnDef::new(PaymentMethods::MethodType)
                        .small_integer()
                        .not_null()
                        .default(1),
                )
                .add_column(
                    ColumnDef::new(PaymentMethods::DisplayOn)
                        .small_integer()
                        .not_null()
                        .default(3),
                )
                .add_column(
                    ColumnDef::new(PaymentMethods::Position)
                        .integer()
                        .not_null()
                        .default(0),
                )
                .add_column(
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
        .await
}

async fn create_payment_method_eligibility(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
        .create_table(
            Table::create()
                .table(PaymentMethodCurrencies::Table)
                .if_not_exists()
                .col(pk(PaymentMethodCurrencies::Id))
                .col(
                    ColumnDef::new(PaymentMethodCurrencies::PaymentMethodId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PaymentMethodCurrencies::Currency)
                        .string()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_payment_method_currencies_payment_method")
                        .from(
                            PaymentMethodCurrencies::Table,
                            PaymentMethodCurrencies::PaymentMethodId,
                        )
                        .to(PaymentMethods::Table, PaymentMethods::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
}

async fn create_gateway_customers(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(GatewayCustomers::Table)
                .if_not_exists()
                .col(pk(GatewayCustomers::Id))
                .col(ColumnDef::new(GatewayCustomers::UserId).integer().not_null())
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
        .await
}

async fn create_payment_sources(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                .col(ColumnDef::new(PaymentSources::ExpMonth).small_integer().null())
                .col(ColumnDef::new(PaymentSources::ExpYear).small_integer().null())
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
        .await
}

async fn expand_payments(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table(Payments::Table)
                .add_column(ColumnDef::new(Payments::Number).string().null())
                .add_column(ColumnDef::new(Payments::PaymentSourceId).integer().null())
                .add_column(
                    ColumnDef::new(Payments::Intent)
                        .small_integer()
                        .not_null()
                        .default(1),
                )
                .add_column(ColumnDef::new(Payments::ExternalPaymentId).string().null())
                .add_column(ColumnDef::new(Payments::ExternalStatus).string().null())
                .add_column(ColumnDef::new(Payments::IdempotencyKey).string().null())
                .add_column(ColumnDef::new(Payments::FailureCode).string().null())
                .add_column(ColumnDef::new(Payments::FailureMessage).text().null())
                .add_column(ColumnDef::new(Payments::AuthorizedAt).timestamp().null())
                .add_column(ColumnDef::new(Payments::CapturedAt).timestamp().null())
                .add_column(ColumnDef::new(Payments::VoidedAt).timestamp().null())
                .add_column(ColumnDef::new(Payments::CancelledAt).timestamp().null())
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
        .await
}

async fn create_payment_sessions(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
        .await
}

async fn create_payment_setup_sessions(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(PaymentSetupSessions::Table)
                .if_not_exists()
                .col(pk(PaymentSetupSessions::Id))
                .col(
                    ColumnDef::new(PaymentSetupSessions::UserId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::PaymentMethodId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::PaymentSourceId)
                        .integer()
                        .null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::Status)
                        .small_integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::ExternalSetupId)
                        .string()
                        .null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::ExternalClientSecret)
                        .text()
                        .null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::ExpiresAt)
                        .timestamp()
                        .null(),
                )
                .col(
                    ColumnDef::new(PaymentSetupSessions::CompletedAt)
                        .timestamp()
                        .null(),
                )
                .col(timestamptz(PaymentSetupSessions::CreatedAt))
                .col(timestamptz(PaymentSetupSessions::UpdatedAt))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_payment_setup_sessions_user")
                        .from(PaymentSetupSessions::Table, PaymentSetupSessions::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_payment_setup_sessions_payment_method")
                        .from(
                            PaymentSetupSessions::Table,
                            PaymentSetupSessions::PaymentMethodId,
                        )
                        .to(PaymentMethods::Table, PaymentMethods::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_payment_setup_sessions_payment_source")
                        .from(
                            PaymentSetupSessions::Table,
                            PaymentSetupSessions::PaymentSourceId,
                        )
                        .to(PaymentSources::Table, PaymentSources::Id)
                        .on_delete(ForeignKeyAction::SetNull)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
}

async fn create_payment_refunds(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                        .not_null(),
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
        .await
}

async fn create_payment_gateway_events(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                .col(ColumnDef::new(PaymentGatewayEvents::Payload).text().not_null())
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
        .await
}

async fn create_payment_gateway_logs(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
        .await
}

async fn create_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let indexes = [
        unique_index("uidx_payment_gateways_code", PaymentGateways::Table, &[PaymentGateways::Code]),
        index(
            "idx_payment_gateways_status_environment",
            PaymentGateways::Table,
            &[PaymentGateways::Status, PaymentGateways::Environment],
        ),
        unique_index("uidx_payment_methods_code", PaymentMethods::Table, &[PaymentMethods::Code]),
        index(
            "idx_payment_methods_active_display_position",
            PaymentMethods::Table,
            &[PaymentMethods::Active, PaymentMethods::DisplayOn, PaymentMethods::Position],
        ),
        index(
            "idx_payment_methods_gateway_id",
            PaymentMethods::Table,
            &[PaymentMethods::PaymentGatewayId],
        ),
        unique_index(
            "uidx_payment_method_countries_method_country",
            PaymentMethodCountries::Table,
            &[
                PaymentMethodCountries::PaymentMethodId,
                PaymentMethodCountries::CountryCode,
            ],
        ),
        index(
            "idx_payment_method_countries_country",
            PaymentMethodCountries::Table,
            &[PaymentMethodCountries::CountryCode],
        ),
        unique_index(
            "uidx_payment_method_currencies_method_currency",
            PaymentMethodCurrencies::Table,
            &[
                PaymentMethodCurrencies::PaymentMethodId,
                PaymentMethodCurrencies::Currency,
            ],
        ),
        index(
            "idx_payment_method_currencies_currency",
            PaymentMethodCurrencies::Table,
            &[PaymentMethodCurrencies::Currency],
        ),
        unique_index(
            "uidx_gateway_customers_user_gateway",
            GatewayCustomers::Table,
            &[GatewayCustomers::UserId, GatewayCustomers::PaymentGatewayId],
        ),
        unique_index(
            "uidx_gateway_customers_gateway_external",
            GatewayCustomers::Table,
            &[
                GatewayCustomers::PaymentGatewayId,
                GatewayCustomers::ExternalCustomerId,
            ],
        ),
        unique_index(
            "uidx_payment_sources_method_external",
            PaymentSources::Table,
            &[
                PaymentSources::PaymentMethodId,
                PaymentSources::ExternalSourceId,
            ],
        ),
        index(
            "idx_payment_sources_user_status",
            PaymentSources::Table,
            &[PaymentSources::UserId, PaymentSources::Status],
        ),
        index(
            "idx_payment_sources_gateway_customer",
            PaymentSources::Table,
            &[PaymentSources::GatewayCustomerId],
        ),
        unique_index("uidx_payments_number", Payments::Table, &[Payments::Number]),
        unique_index(
            "uidx_payments_idempotency_key",
            Payments::Table,
            &[Payments::IdempotencyKey],
        ),
        index(
            "idx_payments_order_status",
            Payments::Table,
            &[Payments::OrderId, Payments::Status],
        ),
        index(
            "idx_payments_payment_source_id",
            Payments::Table,
            &[Payments::PaymentSourceId],
        ),
        index(
            "idx_payments_method_external",
            Payments::Table,
            &[Payments::PaymentMethodId, Payments::ExternalPaymentId],
        ),
        index(
            "idx_payment_sessions_payment_status",
            PaymentSessions::Table,
            &[PaymentSessions::PaymentId, PaymentSessions::Status],
        ),
        index(
            "idx_payment_sessions_expires_at",
            PaymentSessions::Table,
            &[PaymentSessions::ExpiresAt],
        ),
        index(
            "idx_payment_setup_sessions_user_status",
            PaymentSetupSessions::Table,
            &[
                PaymentSetupSessions::UserId,
                PaymentSetupSessions::Status,
            ],
        ),
        index(
            "idx_payment_setup_sessions_method_external",
            PaymentSetupSessions::Table,
            &[
                PaymentSetupSessions::PaymentMethodId,
                PaymentSetupSessions::ExternalSetupId,
            ],
        ),
        unique_index(
            "uidx_payment_refunds_idempotency_key",
            PaymentRefunds::Table,
            &[PaymentRefunds::IdempotencyKey],
        ),
        index(
            "idx_payment_refunds_payment_status",
            PaymentRefunds::Table,
            &[PaymentRefunds::PaymentId, PaymentRefunds::Status],
        ),
        index(
            "idx_payment_refunds_external_id",
            PaymentRefunds::Table,
            &[PaymentRefunds::ExternalRefundId],
        ),
        unique_index(
            "uidx_gateway_events_gateway_external",
            PaymentGatewayEvents::Table,
            &[
                PaymentGatewayEvents::PaymentGatewayId,
                PaymentGatewayEvents::ExternalEventId,
            ],
        ),
        index(
            "idx_gateway_events_gateway_type",
            PaymentGatewayEvents::Table,
            &[
                PaymentGatewayEvents::PaymentGatewayId,
                PaymentGatewayEvents::EventType,
            ],
        ),
        index(
            "idx_gateway_events_status_created",
            PaymentGatewayEvents::Table,
            &[PaymentGatewayEvents::Status, PaymentGatewayEvents::CreatedAt],
        ),
        index(
            "idx_gateway_logs_payment_id",
            PaymentGatewayLogs::Table,
            &[PaymentGatewayLogs::PaymentId],
        ),
        index(
            "idx_gateway_logs_session_id",
            PaymentGatewayLogs::Table,
            &[PaymentGatewayLogs::PaymentSessionId],
        ),
        index(
            "idx_gateway_logs_event_id",
            PaymentGatewayLogs::Table,
            &[PaymentGatewayLogs::PaymentGatewayEventId],
        ),
        index(
            "idx_gateway_logs_level_created",
            PaymentGatewayLogs::Table,
            &[PaymentGatewayLogs::Level, PaymentGatewayLogs::CreatedAt],
        ),
    ];

    for index_statement in indexes {
        manager.create_index(index_statement).await?;
    }

    Ok(())
}

async fn drop_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let indexes = [
        drop_index_statement("idx_gateway_logs_level_created", PaymentGatewayLogs::Table),
        drop_index_statement("idx_gateway_logs_event_id", PaymentGatewayLogs::Table),
        drop_index_statement("idx_gateway_logs_session_id", PaymentGatewayLogs::Table),
        drop_index_statement("idx_gateway_logs_payment_id", PaymentGatewayLogs::Table),
        drop_index_statement(
            "idx_gateway_events_status_created",
            PaymentGatewayEvents::Table,
        ),
        drop_index_statement(
            "idx_gateway_events_gateway_type",
            PaymentGatewayEvents::Table,
        ),
        drop_index_statement(
            "uidx_gateway_events_gateway_external",
            PaymentGatewayEvents::Table,
        ),
        drop_index_statement("idx_payment_refunds_external_id", PaymentRefunds::Table),
        drop_index_statement("idx_payment_refunds_payment_status", PaymentRefunds::Table),
        drop_index_statement(
            "uidx_payment_refunds_idempotency_key",
            PaymentRefunds::Table,
        ),
        drop_index_statement(
            "idx_payment_setup_sessions_method_external",
            PaymentSetupSessions::Table,
        ),
        drop_index_statement(
            "idx_payment_setup_sessions_user_status",
            PaymentSetupSessions::Table,
        ),
        drop_index_statement("idx_payment_sessions_expires_at", PaymentSessions::Table),
        drop_index_statement("idx_payment_sessions_payment_status", PaymentSessions::Table),
        drop_index_statement("idx_payments_method_external", Payments::Table),
        drop_index_statement("idx_payments_payment_source_id", Payments::Table),
        drop_index_statement("idx_payments_order_status", Payments::Table),
        drop_index_statement("uidx_payments_idempotency_key", Payments::Table),
        drop_index_statement("uidx_payments_number", Payments::Table),
        drop_index_statement("idx_payment_sources_gateway_customer", PaymentSources::Table),
        drop_index_statement("idx_payment_sources_user_status", PaymentSources::Table),
        drop_index_statement("uidx_payment_sources_method_external", PaymentSources::Table),
        drop_index_statement(
            "uidx_gateway_customers_gateway_external",
            GatewayCustomers::Table,
        ),
        drop_index_statement(
            "uidx_gateway_customers_user_gateway",
            GatewayCustomers::Table,
        ),
        drop_index_statement(
            "idx_payment_method_currencies_currency",
            PaymentMethodCurrencies::Table,
        ),
        drop_index_statement(
            "uidx_payment_method_currencies_method_currency",
            PaymentMethodCurrencies::Table,
        ),
        drop_index_statement(
            "idx_payment_method_countries_country",
            PaymentMethodCountries::Table,
        ),
        drop_index_statement(
            "uidx_payment_method_countries_method_country",
            PaymentMethodCountries::Table,
        ),
        drop_index_statement("idx_payment_methods_gateway_id", PaymentMethods::Table),
        drop_index_statement(
            "idx_payment_methods_active_display_position",
            PaymentMethods::Table,
        ),
        drop_index_statement("uidx_payment_methods_code", PaymentMethods::Table),
        drop_index_statement(
            "idx_payment_gateways_status_environment",
            PaymentGateways::Table,
        ),
        drop_index_statement("uidx_payment_gateways_code", PaymentGateways::Table),
    ];

    for index_statement in indexes {
        manager.drop_index(index_statement).await?;
    }

    Ok(())
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

fn index<T, C>(name: &str, table: T, columns: &[C]) -> IndexCreateStatement
where
    T: Iden + 'static,
    C: Iden + Clone + 'static,
{
    let mut index_statement = Index::create();
    index_statement.if_not_exists().name(name).table(table);
    for column in columns {
        index_statement.col(column.clone());
    }
    index_statement.to_owned()
}

fn unique_index<T, C>(name: &str, table: T, columns: &[C]) -> IndexCreateStatement
where
    T: Iden + 'static,
    C: Iden + Clone + 'static,
{
    let mut index_statement = Index::create();
    index_statement
        .if_not_exists()
        .name(name)
        .table(table)
        .unique();
    for column in columns {
        index_statement.col(column.clone());
    }
    index_statement.to_owned()
}

fn drop_index_statement<T>(name: &str, table: T) -> IndexDropStatement
where
    T: Iden + 'static,
{
    Index::drop().name(name).table(table).to_owned()
}

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
enum PaymentMethods {
    Table,
    Id,
    Code,
    Active,
    PaymentGatewayId,
    Description,
    MethodType,
    DisplayOn,
    Position,
    AutoCapture,
}

#[derive(Clone, Copy, Iden)]
enum PaymentMethodCountries {
    Table,
    Id,
    PaymentMethodId,
    CountryCode,
}

#[derive(Clone, Copy, Iden)]
enum PaymentMethodCurrencies {
    Table,
    Id,
    PaymentMethodId,
    Currency,
}

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
enum Payments {
    Table,
    Id,
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

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
enum PaymentSetupSessions {
    Table,
    Id,
    UserId,
    PaymentMethodId,
    PaymentSourceId,
    Status,
    ExternalSetupId,
    ExternalClientSecret,
    ExpiresAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
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

#[derive(Clone, Copy, Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Clone, Copy, Iden)]
enum Addresses {
    Table,
    Id,
}
