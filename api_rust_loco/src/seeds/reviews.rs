use chrono::Utc;
use fakeit::{bool_rand, words};
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::reviews::{ActiveModel, Entity};
use crate::models::_entities::products::Entity as ProductEntity;
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Reviews already exist, skipping.");
        return Ok(());
    }

    let products = ProductEntity::find().all(db).await?;
    let users = UserEntity::find().all(db).await?;

    if products.is_empty() || users.is_empty() {
        tracing::warn!("No products or users found. Reviews cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for product in &products {
        let num_reviews = rand::rng().random_range(0..=5);
        
        for _ in 0..num_reviews {
            let user = &users[rand::rng().random_range(0..users.len())];
            let rating = rand::rng().random_range(1..=5);
            let title = words::sentence(rand::rng().random_range(3..6));
            let comment = words::paragraph(2, 3, 8, " ".to_string());

            let review = ActiveModel {
                rating: Set(Some(rating)),
                title: Set(Some(title)),
                comment: Set(Some(comment)),
                verified_purchase: Set(Some(bool_rand::bool())),
                active: Set(Some(true)),
                user_id: Set(user.id),
                product_id: Set(product.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            review.insert(db).await?;
        }
    }

    tracing::info!("Reviews generated for {} products", products.len());
    Ok(())
}
