use chrono::Utc;
use fakeit::image;
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::product_images::{ActiveModel, Entity};
use crate::models::_entities::products::Entity as ProductEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Product images already exist, skipping.");
        return Ok(());
    }

    let products = ProductEntity::find().all(db).await?;
    if products.is_empty() {
        tracing::warn!("No products found. Product images cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for product in &products {
        let num_images = rand::rng().random_range(1..=5);
        
        for position in 0..num_images {
            let image_url = image::url(800, 600);
            let is_cover = position == 0;

            let product_image = ActiveModel {
                image: Set(Some(image_url)),
                alt_text: Set(Some(format!("Product {} image {}", product.id, position + 1))),
                active: Set(Some(true)),
                cover: Set(Some(is_cover)),
                position: Set(Some(position)),
                product_id: Set(product.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            product_image.insert(db).await?;
        }
    }

    tracing::info!("Product images generated for {} products", products.len());
    Ok(())
}
