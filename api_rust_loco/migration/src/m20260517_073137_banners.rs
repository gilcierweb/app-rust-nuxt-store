use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared(
            r#"
            CREATE TABLE banners (
                id BIGSERIAL PRIMARY KEY,
                title VARCHAR(150) NOT NULL,
                description TEXT,
                image_desktop_url TEXT NOT NULL,
                image_mobile_url TEXT,
                alt_text VARCHAR(150),
                link_url TEXT,
                link_target SMALLINT NOT NULL DEFAULT 1,
                position_id BIGINT NOT NULL REFERENCES banner_positions(id),
                device SMALLINT NOT NULL DEFAULT 1,
                start_at TIMESTAMPTZ NOT NULL,
                end_at TIMESTAMPTZ,
                priority INTEGER NOT NULL DEFAULT 0,
                status SMALLINT NOT NULL DEFAULT 1,
                campaign_name VARCHAR(150),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                CONSTRAINT chk_banners_link_target CHECK (link_target IN (1, 2)),
                CONSTRAINT chk_banners_device CHECK (device IN (1, 2, 3)),
                CONSTRAINT chk_banners_status CHECK (status IN (1, 2, 3, 4)),
                CONSTRAINT chk_banners_dates CHECK (end_at IS NULL OR end_at >= start_at)
            )
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE INDEX idx_banners_active_lookup
            ON banners (position_id, device, priority DESC, start_at, end_at)
            WHERE status = 2
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX idx_banners_status_dates ON banners (status, start_at, end_at)",
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ LANGUAGE plpgsql
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER trg_banners_updated_at
            BEFORE UPDATE ON banners
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at()
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP TRIGGER IF EXISTS trg_banners_updated_at ON banners")
            .await?;
        db.execute_unprepared("DROP TABLE IF EXISTS banners")
            .await?;
        Ok(())
    }
}
