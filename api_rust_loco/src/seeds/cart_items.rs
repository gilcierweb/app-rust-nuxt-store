use chrono::Utc;
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::cart_items::{ActiveModel, Entity};
use crate::models::_entities::carts::Entity as CartEntity;
use crate::models::_entities::product_variants::Entity as VariantEntity;
use crate::models::_entities::products::Entity as ProductEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Cart items already exist, skipping.");
        return Ok(());
    }

    let carts = CartEntity::find().all(db).await?;
    let products = ProductEntity::find().all(db).await?;
    let variants = VariantEntity::find().all(db).await?;

    if carts.is_empty() || products.is_empty() {
        tracing::warn!("No carts or products found. Cart items cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for cart in &carts {
        let num_items = rand::rng().random_range(0..=5);

        for _ in 0..num_items {
            let product = &products[rand::rng().random_range(0..products.len())];
            let product_variants: Vec<_> = variants
                .iter()
                .filter(|v| v.product_id == product.id)
                .collect();

            let variant_id = if !product_variants.is_empty() && rand::rng().random_bool(0.7) {
                product_variants[rand::rng().random_range(0..product_variants.len())].id
            } else if !variants.is_empty() {
                variants[rand::rng().random_range(0..variants.len())].id
            } else {
                continue;
            };

            let quantity = rand::rng().random_range(1..=5);
            let price = product.price.unwrap_or(Decimal::new(1000, 2));

            let cart_item = ActiveModel {
                quantity: Set(Some(quantity)),
                price: Set(Some(price)),
                cart_id: Set(cart.id),
                product_id: Set(product.id),
                product_variant_id: Set(variant_id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            cart_item.insert(db).await?;
        }
    }

    tracing::info!("Cart items generated for {} carts", carts.len());
    Ok(())
}
