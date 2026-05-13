use chrono::Utc;
use fakeit::{bool_rand, unique};
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::product_variants::{ActiveModel, Entity};
use crate::models::_entities::products::Entity as ProductEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Product variants already exist, skipping.");
        return Ok(());
    }

    let products = ProductEntity::find().all(db).await?;
    if products.is_empty() {
        tracing::warn!("No products found. Product variants cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let variant_names = vec!["Standard", "Premium", "Limited Edition", "Basic", "Deluxe"];

    for product in &products {
        let num_variants = rand::rng().random_range(1..=4);

        for (i, variant_name) in variant_names.iter().take(num_variants).enumerate() {
            let base_price = product.price.unwrap_or(Decimal::new(1000, 2));
            let price_adjustment = Decimal::new(rand::rng().random_range(-500..500) as i64, 2);
            let price = base_price + price_adjustment;
            let cost_price = price - Decimal::new(rand::rng().random_range(100..300) as i64, 2);
            let compare_price = price + Decimal::new(rand::rng().random_range(100..500) as i64, 2);

            let sku = format!(
                "{}-{}",
                product.sku.clone().unwrap_or_else(|| "SKU".to_string()),
                unique::uuid_v4().split('-').next().unwrap_or("VAR")
            );

            let variant = ActiveModel {
                name: Set(Some(variant_name.to_string())),
                sku: Set(Some(sku)),
                price: Set(Some(price)),
                cost_price: Set(Some(cost_price)),
                compare_price: Set(Some(compare_price)),
                inventory_quantity: Set(Some(rand::rng().random_range(10..500))),
                weight: Set(Some(Decimal::new(
                    rand::rng().random_range(100..2000) as i64,
                    2,
                ))),
                barcode: Set(Some(unique::uuid_v4())),
                position: Set(Some(i as i32)),
                active: Set(Some(bool_rand::bool())),
                product_id: Set(product.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            variant.insert(db).await?;
        }
    }

    tracing::info!("Product variants generated for {} products", products.len());
    Ok(())
}
