use chrono::{Duration, Utc};
use loco_rs::Result;
use rand::Rng;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::coupon_usages::{ActiveModel, Entity};
use crate::models::_entities::coupons::Entity as CouponEntity;
use crate::models::_entities::orders::Entity as OrderEntity;
use crate::models::_entities::users::Entity as UserEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Coupon usages already exist, skipping.");
        return Ok(());
    }

    let coupons = CouponEntity::find().all(db).await?;
    let orders = OrderEntity::find().all(db).await?;
    let users = UserEntity::find().all(db).await?;

    if coupons.is_empty() || orders.is_empty() || users.is_empty() {
        tracing::warn!("No coupons, orders, or users found. Coupon usages cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for order in &orders {
        if rand::rng().random_bool(0.3) {
            let coupon = &coupons[rand::rng().random_range(0..coupons.len())];
            let user = &users[rand::rng().random_range(0..users.len())];

            let used_at = (now - Duration::days(rand::rng().random_range(1..30))).naive_utc();

            let coupon_usage = ActiveModel {
                used_at: Set(Some(used_at)),
                coupon_id: Set(coupon.id),
                user_id: Set(user.id),
                order_id: Set(order.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            coupon_usage.insert(db).await?;
        }
    }

    tracing::info!("Coupon usages generated for {} orders", orders.len());
    Ok(())
}
