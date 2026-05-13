use chrono::{Duration, Utc};
use fakeit::bool_rand;
use loco_rs::Result;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::coupons::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Coupons already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    let coupon_data = vec![
        ("WELCOME10", 1, 1000, Some(5000), Some(1000), 100), // 10% off, max $10 discount
        ("SAVE20", 1, 2000, Some(10000), Some(2000), 50),    // 20% off, max $20 discount
        ("FLAT50", 2, 5000, Some(10000), None, 30),          // $50 flat off
        ("FREESHIP", 2, 0, Some(5000), None, 200),           // Free shipping
        ("HOLIDAY25", 1, 2500, Some(7500), Some(2500), 25),
        ("FLASH15", 1, 1500, Some(3000), Some(1500), 10),
        ("BULK10", 2, 1000, Some(2000), None, 1000),
    ];

    for (code, discount_type, discount_value, min_amount, max_discount, usage_limit) in coupon_data
    {
        let expires_at = if bool_rand::bool() {
            Some((now + Duration::days(30)).naive_utc())
        } else {
            None
        };

        let coupon = ActiveModel {
            code: Set(Some(code.to_string())),
            discount_type: Set(Some(discount_type)),
            discount_value: Set(Some(Decimal::new(discount_value, 2))),
            minimum_amount: Set(min_amount.map(|v| Decimal::new(v, 2))),
            maximum_discount: Set(max_discount.map(|v| Decimal::new(v, 2))),
            usage_limit: Set(Some(usage_limit)),
            used_count: Set(Some(0)),
            expires_at: Set(expires_at),
            active: Set(Some(true)),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        coupon.insert(db).await?;
    }

    tracing::info!("7 coupons generated");
    Ok(())
}
