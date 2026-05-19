use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS payment_method_countries (
                id SERIAL PRIMARY KEY,
                payment_method_id INTEGER NOT NULL REFERENCES payment_methods(id) ON DELETE CASCADE ON UPDATE CASCADE,
                country_code VARCHAR NOT NULL
            )
            "#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uidx_payment_method_countries_method_country ON payment_method_countries (payment_method_id, country_code)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_payment_method_countries_country ON payment_method_countries (country_code)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_method_countries")
            .await?;
        Ok(())
    }
}
