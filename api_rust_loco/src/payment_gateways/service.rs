use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait};
use serde_json::Value;
use uuid::Uuid;

use crate::models::_entities::payment_gateways;
use crate::models::_entities::payment_methods;
use crate::models::_entities::payment_sessions;
use crate::models::_entities::payment_setup_sessions;
use crate::models::_entities::payments;
use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentIntent, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::MANUAL_DRIVER;
use crate::payment_gateways::registry::gateway_for_driver;
use crate::payment_gateways::types::{CreatePaymentSessionInput, CreateSetupSessionInput};

pub struct CreatePaymentAttemptInput {
    pub order_id: i32,
    pub payment_method: payment_methods::Model,
    pub amount: Decimal,
    pub currency: String,
    pub gateway_payload: Option<Value>,
}

pub struct CreatePaymentSetupSessionInput {
    pub user_id: i32,
    pub payment_method: payment_methods::Model,
}

pub async fn create_payment_attempt<C>(
    db: &C,
    input: CreatePaymentAttemptInput,
) -> Result<payments::Model>
where
    C: ConnectionTrait,
{
    let now = chrono::Utc::now();
    let idempotency_key = input
        .gateway_payload
        .as_ref()
        .and_then(|payload| payload.get("idempotency_key"))
        .and_then(|val| val.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("pay-{}", Uuid::new_v4()));
    let number = generate_payment_number();

    let payment = payments::ActiveModel {
        order_id: Set(input.order_id),
        payment_method_id: Set(input.payment_method.id),
        amount: Set(Some(input.amount)),
        currency: Set(Some(input.currency.clone())),
        status: Set(Some(PaymentAttemptStatus::Processing.to_i16() as i32)),
        transaction_id: Set(None),
        gateway_response: Set(None),
        processed_at: Set(None),
        number: Set(Some(number)),
        payment_source_id: Set(None),
        intent: Set(PaymentIntent::Purchase.to_i16()),
        external_payment_id: Set(None),
        external_status: Set(None),
        idempotency_key: Set(Some(idempotency_key.clone())),
        failure_code: Set(None),
        failure_message: Set(None),
        authorized_at: Set(None),
        captured_at: Set(None),
        voided_at: Set(None),
        cancelled_at: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let driver = driver_for_payment_method(db, &input.payment_method).await?;
    let gateway = gateway_for_driver(&driver)?;
    let output = gateway
        .create_payment_session(CreatePaymentSessionInput {
            payment_id: payment.id,
            order_id: input.order_id,
            payment_method_id: input.payment_method.id,
            amount: input.amount,
            currency: input.currency,
            idempotency_key,
            auto_capture: input.payment_method.auto_capture,
            gateway_payload: input.gateway_payload,
        })
        .await?;

    payment_sessions::ActiveModel {
        payment_id: Set(payment.id),
        payment_method_id: Set(input.payment_method.id),
        status: Set(output.status.to_i16()),
        external_session_id: Set(output.external_session_id.clone()),
        external_client_secret: Set(output.external_client_secret),
        expires_at: Set(None),
        completed_at: Set(if output.status == PaymentSessionStatus::Completed {
            Some(now.naive_utc())
        } else {
            None
        }),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let mut payment_update: payments::ActiveModel = payment.into();
    payment_update.status = Set(Some(output.payment_status.to_i16() as i32));
    payment_update.transaction_id = Set(output.external_payment_id.clone());
    payment_update.external_payment_id = Set(output.external_payment_id);
    payment_update.external_status = Set(output.external_status);
    payment_update.processed_at = Set(Some(now.naive_utc()));
    payment_update.authorized_at = Set(
        if output.payment_status == PaymentAttemptStatus::Authorized {
            Some(now.naive_utc())
        } else {
            None
        },
    );
    payment_update.captured_at = Set(if output.payment_status == PaymentAttemptStatus::Captured {
        Some(now.naive_utc())
    } else {
        None
    });
    payment_update.updated_at = Set(now.into());

    payment_update.update(db).await.map_err(Into::into)
}

pub async fn create_payment_setup_session<C>(
    db: &C,
    input: CreatePaymentSetupSessionInput,
) -> Result<payment_setup_sessions::Model>
where
    C: ConnectionTrait,
{
    let now = chrono::Utc::now();
    let idempotency_key = format!("setup-{}", Uuid::new_v4());
    let driver = driver_for_payment_method(db, &input.payment_method).await?;
    let gateway = gateway_for_driver(&driver)?;
    let output = gateway
        .create_setup_session(CreateSetupSessionInput {
            user_id: input.user_id,
            payment_method_id: input.payment_method.id,
            idempotency_key,
        })
        .await?;

    payment_setup_sessions::ActiveModel {
        user_id: Set(input.user_id),
        payment_method_id: Set(input.payment_method.id),
        payment_source_id: Set(None),
        status: Set(output.status.to_i16()),
        external_setup_id: Set(output.external_setup_id),
        external_client_secret: Set(output.external_client_secret),
        expires_at: Set(None),
        completed_at: Set(if output.status == PaymentSessionStatus::Completed {
            Some(now.naive_utc())
        } else {
            None
        }),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(Into::into)
}

async fn driver_for_payment_method<C>(
    db: &C,
    payment_method: &payment_methods::Model,
) -> Result<String>
where
    C: ConnectionTrait,
{
    let Some(payment_gateway_id) = payment_method.payment_gateway_id else {
        return Ok(MANUAL_DRIVER.to_string());
    };

    let gateway = payment_gateways::Entity::find_by_id(payment_gateway_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    Ok(gateway.driver)
}

fn generate_payment_number() -> String {
    let ts = chrono::Utc::now().timestamp();
    let short = &Uuid::new_v4().to_string()[..8];
    format!("PAY-{ts}-{short}")
}
