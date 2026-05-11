use chrono::Utc;
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::order_items::{ActiveModel, Entity};
use crate::models::_entities::orders::Entity as OrderEntity;
use crate::models::_entities::products::Entity as ProductEntity;
use crate::models::_entities::product_variants::Entity as VariantEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Order items already exist, skipping.");
        return Ok(());
    }

    let orders = OrderEntity::find().all(db).await?;
    let products = ProductEntity::find().all(db).await?;
    let variants = VariantEntity::find().all(db).await?;

    if orders.is_empty() || products.is_empty() {
        tracing::warn!("No orders or products found. Order items cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for order in &orders {
        let num_items = rand::rng().random_range(1..=4);
        
        for _ in 0..num_items {
            let product = &products[rand::rng().random_range(0..products.len())];
            let product_variants: Vec<_> = variants.iter().filter(|v| v.product_id == product.id).collect();
            
            let variant_id = if !product_variants.is_empty() && rand::rng().random_bool(0.6) {
                Some(product_variants[rand::rng().random_range(0..product_variants.len())].id)
            } else {
                None
            };

            let quantity = rand::rng().random_range(1..=3);
            let unit_price = product.price.unwrap_or(Decimal::new(1000, 2));
            let total = unit_price * Decimal::new(quantity as i64, 0);

            let order_item = ActiveModel {
                quantity: Set(Some(quantity)),
                price: Set(Some(unit_price)),
                total: Set(Some(total)),
                order_id: Set(order.id),
                product_id: Set(product.id),
                product_variant_id: Set(variant_id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            order_item.insert(db).await?;
        }
    }

    tracing::info!("Order items generated for {} orders", orders.len());
    Ok(())
}
