use chrono::Utc;
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};

use crate::models::_entities::payment_gateways;
use crate::models::_entities::payment_methods::{ActiveModel, Entity};
use crate::models::payment_gateway_status::{PaymentMethodDisplay, PaymentMethodType};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Payment methods already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();
    let manual_gateway_id = gateway_id(db, "manual").await?;
    let stripe_gateway_id = gateway_id(db, "stripe").await?;
    let paypal_gateway_id = gateway_id(db, "paypal").await?;

    let methods = vec![
        (
            "Credit Card",
            "credit_card",
            "Credit card through Stripe test gateway",
            stripe_gateway_id,
            PaymentMethodType::Card,
            true,
        ),
        (
            "Debit Card",
            "debit_card",
            "Debit card through Stripe test gateway",
            stripe_gateway_id,
            PaymentMethodType::Card,
            true,
        ),
        (
            "PayPal",
            "paypal",
            "PayPal sandbox wallet checkout",
            paypal_gateway_id,
            PaymentMethodType::Wallet,
            true,
        ),
        (
            "Bank Transfer",
            "bank_transfer",
            "Manual bank transfer confirmation",
            manual_gateway_id,
            PaymentMethodType::BankTransfer,
            false,
        ),
        (
            "Cash on Delivery",
            "cod",
            "Manual cash on delivery payment",
            manual_gateway_id,
            PaymentMethodType::Manual,
            false,
        ),
        (
            "Apple Pay",
            "apple_pay",
            "Apple Pay through Stripe test gateway",
            stripe_gateway_id,
            PaymentMethodType::Wallet,
            true,
        ),
        (
            "Google Pay",
            "google_pay",
            "Google Pay through Stripe test gateway",
            stripe_gateway_id,
            PaymentMethodType::Wallet,
            true,
        ),
    ];

    for (position, (name, code, description, gateway_id, method_type, auto_capture)) in
        methods.into_iter().enumerate()
    {
        let method = ActiveModel {
            name: Set(Some(name.to_string())),
            code: Set(Some(code.to_string())),
            active: Set(Some(true)),
            settings: Set(None),
            payment_gateway_id: Set(Some(gateway_id)),
            description: Set(Some(description.to_string())),
            method_type: Set(method_type.to_i16()),
            display_on: Set(PaymentMethodDisplay::Both.to_i16()),
            position: Set(position as i32),
            auto_capture: Set(auto_capture),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        method.insert(db).await?;
    }

    tracing::info!("7 payment methods generated");
    Ok(())
}

async fn gateway_id(db: &sea_orm::DatabaseConnection, code: &str) -> Result<i32> {
    payment_gateways::Entity::find()
        .filter(payment_gateways::Column::Code.eq(code))
        .one(db)
        .await?
        .map(|gateway| gateway.id)
        .ok_or_else(|| loco_rs::Error::Message(format!("payment gateway seed missing: {code}")))
}
