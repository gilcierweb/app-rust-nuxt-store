use chrono::Utc;
use fakeit::{bool_rand,company, unique, words};
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait,  EntityTrait, Set};
use sea_orm::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::_entities::products::{ActiveModel as Product, Entity as ProductEntity};
use crate::models::_entities::categories::Entity as CategoryEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = <ProductEntity as sea_orm::EntityTrait>::find().count(db).await?;
    if count > 0 {
        tracing::info!("Products already exist, skipping.");
        return Ok(());
    }

    let category_ids: Vec<i32> = CategoryEntity::find()
        .all(db)
        .await?
        .into_iter()
        .map(|cat| cat.id)
        .collect();

    if category_ids.is_empty() {
        tracing::warn!("No categories found. Products cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for i in 0..30 {

        let number_one = rand::rng().random_range(1000..100_000);
        let number_two = rand::rng().random_range(100..5000);
        let number_three = rand::rng().random_range(1000..9999);

        let name_val = company::company();
        let slug_val = unique::uuid_v4();
        let sku_val = format!("SKU-{}", number_three);
        let short_desc = words::sentence(rand::rng().random_range(5..10));
        let desc = words::paragraph(3, 4, 10, "\n".to_string());

      


        let price = Decimal::new(number_one as i64, 2);
        let cost_price = price - Decimal::new(number_two as i64, 2);
        let compare_price = price + Decimal::new(number_two as i64, 2);

        // Aleatoriamente escolhe categoria
        let index = {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            (now.subsec_nanos() as usize + i) % category_ids.len()
        };
        let category_id = category_ids[index];

        let product = Product {
            name: Set(Some(name_val)),
            slug: Set(Some(slug_val)),
            sku: Set(Some(sku_val)),
            short_description: Set(Some(short_desc)),
            description: Set(Some(desc)),
            price: Set(Some(price)),
            cost_price: Set(Some(cost_price)),
            compare_price: Set(Some(compare_price)),
            featured: Set(Some(bool_rand::bool())),
            active: Set(Some(true)),
            status: Set(Some(1)),
            category_id: Set(category_id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        product.insert(db).await?;
    }

    tracing::info!("30 fake products created using `fakeit`.");
    Ok(())
}
