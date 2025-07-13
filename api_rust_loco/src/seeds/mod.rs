pub mod posts;
pub mod profiles; 

use sea_orm::DatabaseConnection;
use loco_rs::Result;


pub async fn seed_all(db: &DatabaseConnection) -> Result<()> {
    posts::seed(db).await?;
    profiles::seed(db).await?;
    Ok(())
}
