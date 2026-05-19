use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_sources (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
                payment_method_id INTEGER NOT NULL REFERENCES payment_methods(id) ON DELETE RESTRICT ON UPDATE CASCADE,
                gateway_customer_id INTEGER REFERENCES gateway_customers(id) ON DELETE SET NULL ON UPDATE CASCADE,
                source_type SMALLINT NOT NULL,
                external_source_id VARCHAR NOT NULL,
                brand VARCHAR,
                last4 VARCHAR,
                exp_month SMALLINT,
                exp_year SMALLINT,
                holder_name VARCHAR,
                billing_address_id INTEGER REFERENCES addresses(id) ON DELETE SET NULL ON UPDATE CASCADE,
                status SMALLINT NOT NULL DEFAULT 1,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uidx_payment_sources_method_external ON payment_sources (payment_method_id, external_source_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_sources_user_status ON payment_sources (user_id, status)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_sources_gateway_customer ON payment_sources (gateway_customer_id)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_sources")
            .await?;
        Ok(())
    }
}
