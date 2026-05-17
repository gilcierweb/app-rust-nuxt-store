use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared(
            r#"
            CREATE TABLE banner_positions (
                id BIGSERIAL PRIMARY KEY,
                code VARCHAR(80) NOT NULL UNIQUE,
                name VARCHAR(150) NOT NULL,
                description TEXT,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
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
            "#,
        )
        .await?;

        db.execute_unprepared("CREATE INDEX idx_banner_positions_code ON banner_positions (code)")
            .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS banner_positions")
            .await?;
        Ok(())
    }
}
