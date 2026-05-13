use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "UPDATE order_items SET product_variant_id = NULL WHERE product_variant_id = 0",
        )
        .await?;
        m.alter_table(
            sea_query::Table::alter()
                .table(OrderItems::Table)
                .modify_column(
                    sea_query::ColumnDef::new(OrderItems::ProductVariantId)
                        .integer()
                        .null(),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            sea_query::Table::alter()
                .table(OrderItems::Table)
                .modify_column(
                    sea_query::ColumnDef::new(OrderItems::ProductVariantId)
                        .integer()
                        .not_null(),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum OrderItems {
    Table,
    ProductVariantId,
}
