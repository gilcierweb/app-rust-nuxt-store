use chrono::Utc;
use fakeit::{address, contact, name, words};
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::addresses::{ActiveModel, Entity};
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Addresses already exist, skipping.");
        return Ok(());
    }

    let users = UserEntity::find().all(db).await?;
    if users.is_empty() {
        tracing::warn!("No users found. Addresses cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let address_types = vec!["home", "work", "billing", "shipping"];

    for user in &users {
        let num_addresses = rand::rng().random_range(1..=3);
        
        for i in 0..num_addresses {
            let address_type = address_types[i % address_types.len()];
            let is_default = i == 0;

            let address = ActiveModel {
                r#type: Set(Some(address_type.to_string())),
                first_name: Set(Some(name::first())),
                last_name: Set(Some(name::last())),
                company: Set(Some(words::sentence(2))),
                address1: Set(Some(address::street())),
                address2: Set(Some(format!("Apt {}", rand::rng().random_range(1..999)))),
                city: Set(Some(address::city())),
                state: Set(Some(address::state())),
                zip_code: Set(Some(address::zip())),
                country: Set(Some(address::country())),
                phone: Set(Some(contact::phone())),
                default: Set(Some(is_default)),
                user_id: Set(user.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            address.insert(db).await?;
        }
    }

    tracing::info!("Addresses generated for {} users", users.len());
    Ok(())
}
