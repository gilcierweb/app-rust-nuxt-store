use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            ALTER TABLE payment_methods
                ADD COLUMN IF NOT EXISTS payment_gateway_id INTEGER REFERENCES payment_gateways(id) ON DELETE SET NULL ON UPDATE CASCADE,
                ADD COLUMN IF NOT EXISTS description TEXT,
                ADD COLUMN IF NOT EXISTS method_type SMALLINT NOT NULL DEFAULT 1,
                ADD COLUMN IF NOT EXISTS display_on SMALLINT NOT NULL DEFAULT 3,
                ADD COLUMN IF NOT EXISTS position INTEGER NOT NULL DEFAULT 0,
                ADD COLUMN IF NOT EXISTS auto_capture BOOLEAN NOT NULL DEFAULT true
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uidx_payment_methods_code ON payment_methods (code)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_methods_active_display_position ON payment_methods (active, display_on, position)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_methods_gateway_id ON payment_methods (payment_gateway_id)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_payment_methods_gateway_id")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_payment_methods_active_display_position")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS uidx_payment_methods_code")
            .await?;
        db.execute_unprepared(
            r#"
            ALTER TABLE payment_methods
                DROP COLUMN IF EXISTS auto_capture,
                DROP COLUMN IF EXISTS position,
                DROP COLUMN IF EXISTS display_on,
                DROP COLUMN IF EXISTS method_type,
                DROP COLUMN IF EXISTS description,
                DROP COLUMN IF EXISTS payment_gateway_id
            "#,
        )
        .await?;
        Ok(())
    }
}
