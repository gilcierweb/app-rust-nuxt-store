use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_gateways (
                id SERIAL PRIMARY KEY,
                code VARCHAR NOT NULL UNIQUE,
                name VARCHAR NOT NULL,
                driver VARCHAR NOT NULL,
                environment SMALLINT NOT NULL DEFAULT 1,
                status SMALLINT NOT NULL DEFAULT 1,
                supports_authorize BOOLEAN NOT NULL DEFAULT false,
                supports_capture BOOLEAN NOT NULL DEFAULT false,
                supports_refund BOOLEAN NOT NULL DEFAULT false,
                supports_void BOOLEAN NOT NULL DEFAULT false,
                supports_saved_sources BOOLEAN NOT NULL DEFAULT false,
                public_key_env VARCHAR,
                secret_key_env VARCHAR,
                webhook_secret_env VARCHAR,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_gateways_status_environment ON payment_gateways (status, environment)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_gateways")
            .await?;
        Ok(())
    }
}
