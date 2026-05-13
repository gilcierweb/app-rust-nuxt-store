use chrono::{Duration, Utc};
use fakeit::unique;
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::carts::{ActiveModel, Entity};
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Carts already exist, skipping.");
        return Ok(());
    }

    let users = UserEntity::find().all(db).await?;
    if users.is_empty() {
        tracing::warn!("No users found. Carts cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for user in &users {
        let num_carts = rand::rng().random_range(0..=2);

        for _ in 0..num_carts {
            let has_session = rand::rng().random_bool(0.3);
            let session_id = if has_session {
                Some(unique::uuid_v4())
            } else {
                None
            };

            let expires_at = if rand::rng().random_bool(0.7) {
                Some((now + Duration::hours(24)).naive_utc())
            } else {
                None
            };

            let cart = ActiveModel {
                session_id: Set(session_id),
                expires_at: Set(expires_at),
                user_id: Set(user.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            cart.insert(db).await?;
        }
    }

    tracing::info!("Carts generated for {} users", users.len());
    Ok(())
}
