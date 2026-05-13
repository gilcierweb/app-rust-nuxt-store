use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "orders",
            &[
                ("id", ColType::PkAuto),
                ("order_number", ColType::StringNull),
                ("status", ColType::IntegerNull),
                ("total_amount", ColType::DecimalNull),
                ("subtotal", ColType::DecimalNull),
                ("tax_amount", ColType::DecimalNull),
                ("shipping_amount", ColType::DecimalNull),
                ("discount_amount", ColType::DecimalNull),
                ("currency", ColType::StringNull),
                ("payment_status", ColType::IntegerNull),
                ("fulfillment_status", ColType::IntegerNull),
                ("notes", ColType::TextNull),
            ],
            &[("user", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "orders").await
    }
}
