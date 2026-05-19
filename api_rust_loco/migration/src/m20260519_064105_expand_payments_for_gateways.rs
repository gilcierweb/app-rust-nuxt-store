use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            ALTER TABLE payments
                ADD COLUMN IF NOT EXISTS number VARCHAR,
                ADD COLUMN IF NOT EXISTS payment_source_id INTEGER REFERENCES payment_sources(id) ON DELETE SET NULL ON UPDATE CASCADE,
                ADD COLUMN IF NOT EXISTS intent SMALLINT NOT NULL DEFAULT 1,
                ADD COLUMN IF NOT EXISTS external_payment_id VARCHAR,
                ADD COLUMN IF NOT EXISTS external_status VARCHAR,
                ADD COLUMN IF NOT EXISTS idempotency_key VARCHAR,
                ADD COLUMN IF NOT EXISTS failure_code VARCHAR,
                ADD COLUMN IF NOT EXISTS failure_message TEXT,
                ADD COLUMN IF NOT EXISTS authorized_at TIMESTAMP,
                ADD COLUMN IF NOT EXISTS captured_at TIMESTAMP,
                ADD COLUMN IF NOT EXISTS voided_at TIMESTAMP,
                ADD COLUMN IF NOT EXISTS cancelled_at TIMESTAMP
            "#,
        )
        .await?;
        db.execute_unprepared("CREATE UNIQUE INDEX IF NOT EXISTS uidx_payments_number ON payments (number)")
            .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uidx_payments_idempotency_key ON payments (idempotency_key)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payments_order_status ON payments (order_id, status)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payments_payment_source_id ON payments (payment_source_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payments_method_external ON payments (payment_method_id, external_payment_id)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_payments_method_external")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_payments_payment_source_id")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_payments_order_status")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS uidx_payments_idempotency_key")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS uidx_payments_number")
            .await?;
        db.execute_unprepared(
            r#"
            ALTER TABLE payments
                DROP COLUMN IF EXISTS cancelled_at,
                DROP COLUMN IF EXISTS voided_at,
                DROP COLUMN IF EXISTS captured_at,
                DROP COLUMN IF EXISTS authorized_at,
                DROP COLUMN IF EXISTS failure_message,
                DROP COLUMN IF EXISTS failure_code,
                DROP COLUMN IF EXISTS idempotency_key,
                DROP COLUMN IF EXISTS external_status,
                DROP COLUMN IF EXISTS external_payment_id,
                DROP COLUMN IF EXISTS intent,
                DROP COLUMN IF EXISTS payment_source_id,
                DROP COLUMN IF EXISTS number
            "#,
        )
        .await?;
        Ok(())
    }
}
