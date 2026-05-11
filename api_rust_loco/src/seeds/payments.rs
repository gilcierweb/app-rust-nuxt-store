use chrono::{Utc, Duration};
use fakeit::unique;
use loco_rs::Result;
use rand::Rng;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::payments::{ActiveModel, Entity};
use crate::models::_entities::orders::Entity as OrderEntity;
use crate::models::_entities::payment_methods::Entity as PaymentMethodEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Payments already exist, skipping.");
        return Ok(());
    }

    let orders = OrderEntity::find().all(db).await?;
    let payment_methods = PaymentMethodEntity::find().all(db).await?;

    if orders.is_empty() || payment_methods.is_empty() {
        tracing::warn!("No orders or payment methods found. Payments cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let currencies = vec!["USD", "EUR", "GBP", "BRL"];

    for order in &orders {
        let num_payments = rand::rng().random_range(0..=2);
        
        for _ in 0..num_payments {
            let payment_method = &payment_methods[rand::rng().random_range(0..payment_methods.len())];
            let amount = order.total_amount.unwrap_or(Decimal::new(1000, 2));
            let status = rand::rng().random_range(1..=4);

            let processed_at = if status > 1 {
                Some((now - Duration::hours(rand::rng().random_range(1..168))).naive_utc())
            } else {
                None
            };

            let payment = ActiveModel {
                amount: Set(Some(amount)),
                currency: Set(Some(currencies[rand::rng().random_range(0..currencies.len())].to_string())),
                status: Set(Some(status)),
                transaction_id: Set(Some(unique::uuid_v4())),
                gateway_response: Set(Some(r#"{"status": "success", "code": "200"}"#.to_string())),
                processed_at: Set(processed_at),
                order_id: Set(order.id),
                payment_method_id: Set(payment_method.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            payment.insert(db).await?;
        }
    }

    tracing::info!("Payments generated for {} orders", orders.len());
    Ok(())
}
