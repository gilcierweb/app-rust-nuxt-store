use chrono::Utc;
use loco_rs::Result;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::shipping_methods::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Shipping methods already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    let methods = vec![
        ("Standard Shipping", "standard", 599, 5000),
        ("Express Shipping", "express", 1299, 7500),
        ("Overnight Delivery", "overnight", 2499, 10000),
        ("Free Shipping", "free", 0, 0),
        ("Local Pickup", "pickup", 0, 0),
    ];

    for (name, code, base_price, threshold) in methods {
        let method = ActiveModel {
            name: Set(Some(name.to_string())),
            code: Set(Some(code.to_string())),
            base_price: Set(Some(Decimal::new(base_price, 2))),
            free_shipping_threshold: Set(if threshold > 0 {
                Some(Decimal::new(threshold, 2))
            } else {
                None
            }),
            active: Set(Some(true)),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        method.insert(db).await?;
    }

    tracing::info!("5 shipping methods generated");
    Ok(())
}
