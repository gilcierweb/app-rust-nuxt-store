use chrono::Utc;
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::orders::{ActiveModel, Entity};
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Orders already exist, skipping.");
        return Ok(());
    }

    let users = UserEntity::find().all(db).await?;
    if users.is_empty() {
        tracing::warn!("No users found. Orders cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let currencies = vec!["USD", "EUR", "GBP", "BRL"];

    for user in &users {
        let num_orders = rand::rng().random_range(0..=5);
        
        for i in 0..num_orders {
            let order_number = format!("ORD-{}-{:06}", now.format("%Y%m%d"), i + 1);
            
            let subtotal = Decimal::new(rand::rng().random_range(1000..50000) as i64, 2);
            let tax_rate = Decimal::new(rand::rng().random_range(5..20) as i64, 2);
            let tax_amount = subtotal * tax_rate / Decimal::new(100, 0);
            let shipping_amount = Decimal::new(rand::rng().random_range(0..2000) as i64, 2);
            let discount_amount = Decimal::new(rand::rng().random_range(0..500) as i64, 2);
            let total_amount = subtotal + tax_amount + shipping_amount - discount_amount;

            let status_val = rand::rng().random_range(1..=6);
            let payment_status_val = rand::rng().random_range(1..=4);
            let fulfillment_status_val = rand::rng().random_range(1..=3);

            let order = ActiveModel {
                order_number: Set(Some(order_number)),
                status: Set(Some(status_val)),
                total_amount: Set(Some(total_amount)),
                subtotal: Set(Some(subtotal)),
                tax_amount: Set(Some(tax_amount)),
                shipping_amount: Set(Some(shipping_amount)),
                discount_amount: Set(Some(discount_amount)),
                currency: Set(Some(currencies[rand::rng().random_range(0..currencies.len())].to_string())),
                payment_status: Set(Some(payment_status_val)),
                fulfillment_status: Set(Some(fulfillment_status_val)),
                notes: Set(Some(format!("Order notes for customer {}", user.id))),
                user_id: Set(user.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            order.insert(db).await?;
        }
    }

    tracing::info!("Orders generated for {} users", users.len());
    Ok(())
}
