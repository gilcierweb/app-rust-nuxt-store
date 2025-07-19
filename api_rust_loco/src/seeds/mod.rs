pub mod posts;
pub mod profiles; 
pub mod categories; 
pub mod products; 

use sea_orm::DatabaseConnection;
use loco_rs::Result;


pub async fn seed_all(db: &DatabaseConnection) -> Result<()> {
    posts::seed(db).await?;
    profiles::seed(db).await?;
    categories::seed(db).await?;
    products::seed(db).await?;
    Ok(())
}
