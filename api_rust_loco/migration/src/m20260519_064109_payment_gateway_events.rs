use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_gateway_events (
                id SERIAL PRIMARY KEY,
                payment_gateway_id INTEGER NOT NULL REFERENCES payment_gateways(id) ON DELETE CASCADE ON UPDATE CASCADE,
                event_type VARCHAR NOT NULL,
                external_event_id VARCHAR NOT NULL,
                status SMALLINT NOT NULL,
                signature_valid BOOLEAN NOT NULL DEFAULT false,
                payload TEXT NOT NULL,
                processed_at TIMESTAMP,
                failure_message TEXT,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uidx_gateway_events_gateway_external ON payment_gateway_events (payment_gateway_id, external_event_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_events_gateway_type ON payment_gateway_events (payment_gateway_id, event_type)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_events_status_created ON payment_gateway_events (status, created_at)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_gateway_events")
            .await?;
        Ok(())
    }
}
