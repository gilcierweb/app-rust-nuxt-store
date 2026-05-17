use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared(
            r#"
            CREATE TABLE banner_events (
                id BIGSERIAL PRIMARY KEY,
                banner_id BIGINT NOT NULL REFERENCES banners(id) ON DELETE CASCADE,
                event_type SMALLINT NOT NULL,
                user_id BIGINT,
                session_id VARCHAR(100),
                ip_address INET,
                user_agent TEXT,
                page_url TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT chk_banner_events_type CHECK (event_type IN (1, 2))
            )
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX idx_banner_events_banner_date ON banner_events (banner_id, created_at DESC)",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX idx_banner_events_type_date ON banner_events (event_type, created_at DESC)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS banner_events")
            .await?;
        Ok(())
    }
}
