use chrono::{Duration, Utc};
use fakeit::unique;
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::orders::Entity as OrderEntity;
use crate::models::_entities::shipments::{ActiveModel, Entity};
use crate::models::_entities::shipping_methods::Entity as ShippingMethodEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Shipments already exist, skipping.");
        return Ok(());
    }

    let orders = OrderEntity::find().all(db).await?;
    let shipping_methods = ShippingMethodEntity::find().all(db).await?;

    if orders.is_empty() || shipping_methods.is_empty() {
        tracing::warn!("No orders or shipping methods found. Shipments cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let carriers = vec!["UPS", "FedEx", "DHL", "USPS", "Amazon Logistics"];

    for order in &orders {
        let num_shipments = rand::rng().random_range(0..=2);

        for _ in 0..num_shipments {
            let shipping_method =
                &shipping_methods[rand::rng().random_range(0..shipping_methods.len())];
            let status = rand::rng().random_range(1..=5);

            let shipped_at = if status >= 2 {
                Some((now - Duration::days(rand::rng().random_range(1..10))).naive_utc())
            } else {
                None
            };

            let delivered_at = if status >= 4 {
                shipped_at.map(|dt| dt + Duration::days(rand::rng().random_range(1..7)))
            } else {
                None
            };

            let shipment = ActiveModel {
                tracking_number: Set(Some(
                    unique::uuid_v4()
                        .to_uppercase()
                        .replace("-", "")
                        .split_at(16)
                        .0
                        .to_string(),
                )),
                carrier: Set(Some(
                    carriers[rand::rng().random_range(0..carriers.len())].to_string(),
                )),
                status: Set(Some(status)),
                shipped_at: Set(shipped_at),
                delivered_at: Set(delivered_at),
                order_id: Set(order.id),
                shipping_method_id: Set(shipping_method.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            shipment.insert(db).await?;
        }
    }

    tracing::info!("Shipments generated for {} orders", orders.len());
    Ok(())
}
