use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_refunds (
                id SERIAL PRIMARY KEY,
                payment_id INTEGER NOT NULL REFERENCES payments(id) ON DELETE CASCADE ON UPDATE CASCADE,
                amount DECIMAL NOT NULL,
                currency VARCHAR NOT NULL,
                status SMALLINT NOT NULL,
                reason SMALLINT,
                external_refund_id VARCHAR,
                idempotency_key VARCHAR NOT NULL UNIQUE,
                failure_code VARCHAR,
                failure_message TEXT,
                processed_at TIMESTAMP,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_refunds_payment_status ON payment_refunds (payment_id, status)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_refunds_external_id ON payment_refunds (external_refund_id)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_refunds")
            .await?;
        Ok(())
    }
}
