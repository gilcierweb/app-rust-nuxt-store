use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_gateway_logs (
                id SERIAL PRIMARY KEY,
                payment_id INTEGER REFERENCES payments(id) ON DELETE SET NULL ON UPDATE CASCADE,
                payment_session_id INTEGER REFERENCES payment_sessions(id) ON DELETE SET NULL ON UPDATE CASCADE,
                payment_gateway_event_id INTEGER REFERENCES payment_gateway_events(id) ON DELETE SET NULL ON UPDATE CASCADE,
                direction SMALLINT NOT NULL,
                level SMALLINT NOT NULL,
                message TEXT,
                payload TEXT,
                created_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_logs_payment_id ON payment_gateway_logs (payment_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_logs_session_id ON payment_gateway_logs (payment_session_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_logs_event_id ON payment_gateway_logs (payment_gateway_event_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_gateway_logs_level_created ON payment_gateway_logs (level, created_at)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_gateway_logs")
            .await?;
        Ok(())
    }
}
