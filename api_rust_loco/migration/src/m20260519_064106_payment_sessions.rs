use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_sessions (
                id SERIAL PRIMARY KEY,
                payment_id INTEGER NOT NULL REFERENCES payments(id) ON DELETE CASCADE ON UPDATE CASCADE,
                payment_method_id INTEGER NOT NULL REFERENCES payment_methods(id) ON DELETE RESTRICT ON UPDATE CASCADE,
                status SMALLINT NOT NULL,
                external_session_id VARCHAR,
                external_client_secret TEXT,
                expires_at TIMESTAMP,
                completed_at TIMESTAMP,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_sessions_payment_status ON payment_sessions (payment_id, status)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_sessions_expires_at ON payment_sessions (expires_at)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_sessions")
            .await?;
        Ok(())
    }
}
