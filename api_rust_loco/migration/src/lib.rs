#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;



mod m20250713_001217_posts;
mod m20250713_052254_profiles;

mod m20250713_094714_categories;
mod m20250713_094959_add_parent_to_categories;
mod m20250719_064242_products;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
         
        
            Box::new(m20250713_001217_posts::Migration),
            Box::new(m20250713_052254_profiles::Migration),           
 
            Box::new(m20250713_094714_categories::Migration),
            Box::new(m20250713_094959_add_parent_to_categories::Migration),
            Box::new(m20250719_064242_products::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}