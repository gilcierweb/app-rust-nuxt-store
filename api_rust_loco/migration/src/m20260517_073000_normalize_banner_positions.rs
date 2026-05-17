use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared(
            r#"
            UPDATE banner_positions
            SET code = CONCAT('position_', id)
            WHERE code IS NULL
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            UPDATE banner_positions
            SET name = code
            WHERE name IS NULL
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            UPDATE banner_positions
            SET is_active = TRUE
            WHERE is_active IS NULL
            "#,
        )
        .await?;

        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN id TYPE BIGINT")
            .await?;
        db.execute_unprepared("ALTER SEQUENCE IF EXISTS banner_positions_id_seq AS BIGINT")
            .await?;
        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN code TYPE VARCHAR(80)")
            .await?;
        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN code SET NOT NULL")
            .await?;
        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN name TYPE VARCHAR(150)")
            .await?;
        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN name SET NOT NULL")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE banner_positions ALTER COLUMN is_active SET DEFAULT TRUE",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE banner_positions ALTER COLUMN is_active SET NOT NULL")
            .await?;
        db.execute_unprepared("ALTER TABLE banner_positions DROP COLUMN IF EXISTS updated_at")
            .await?;

        db.execute_unprepared(
            r#"
            DO $$
            BEGIN
                IF NOT EXISTS (
                    SELECT 1
                    FROM pg_constraint
                    WHERE conname = 'banner_positions_code_key'
                ) THEN
                    ALTER TABLE banner_positions
                    ADD CONSTRAINT banner_positions_code_key UNIQUE (code);
                END IF;
            END;
            $$ LANGUAGE plpgsql
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            INSERT INTO banner_positions (code, name) VALUES
            ('home_top', 'Topo da home'),
            ('home_middle', 'Meio da home'),
            ('category_top', 'Topo da categoria'),
            ('product_page', 'Página de produto')
            ON CONFLICT (code) DO NOTHING
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_banner_positions_code ON banner_positions (code)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, _m: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
