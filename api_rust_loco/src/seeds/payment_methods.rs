use chrono::Utc;
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::payment_methods::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Payment methods already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    let methods = vec![
        ("Credit Card", "credit_card", r#"{"gateway": "stripe"}"#),
        ("Debit Card", "debit_card", r#"{"gateway": "stripe"}"#),
        ("PayPal", "paypal", r#"{"gateway": "paypal"}"#),
        ("Bank Transfer", "bank_transfer", r#"{"gateway": "manual"}"#),
        ("Cash on Delivery", "cod", r#"{"gateway": "manual"}"#),
        ("Apple Pay", "apple_pay", r#"{"gateway": "stripe"}"#),
        ("Google Pay", "google_pay", r#"{"gateway": "stripe"}"#),
    ];

    for (name, code, settings) in methods {
        let method = ActiveModel {
            name: Set(Some(name.to_string())),
            code: Set(Some(code.to_string())),
            active: Set(Some(true)),
            settings: Set(Some(settings.to_string())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        method.insert(db).await?;
    }

    tracing::info!("7 payment methods generated");
    Ok(())
}
