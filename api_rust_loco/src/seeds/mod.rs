pub mod posts;

use sea_orm::DatabaseConnection;
use loco_rs::Result;

pub async fn seed_all(db: &DatabaseConnection) -> Result<()> {
    posts::seed(db).await?;
    Ok(())
}
