use chrono::Utc;
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::wishlists::{ActiveModel, Entity};
use crate::models::_entities::products::Entity as ProductEntity;
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Wishlists already exist, skipping.");
        return Ok(());
    }

    let products = ProductEntity::find().all(db).await?;
    let users = UserEntity::find().all(db).await?;

    if products.is_empty() || users.is_empty() {
        tracing::warn!("No products or users found. Wishlists cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for user in &users {
        let num_items = rand::rng().random_range(0..=8);
        
        for _ in 0..num_items {
            let product = &products[rand::rng().random_range(0..products.len())];

            let wishlist_item = ActiveModel {
                user_id: Set(user.id),
                product_id: Set(product.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            wishlist_item.insert(db).await?;
        }
    }

    tracing::info!("Wishlist items generated for {} users", users.len());
    Ok(())
}
