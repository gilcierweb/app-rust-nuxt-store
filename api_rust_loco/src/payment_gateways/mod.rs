use async_trait::async_trait;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::payment_gateways;
use crate::models::_entities::payment_methods;
use crate::models::_entities::payment_sessions;
use crate::models::_entities::payments;
use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentIntent, PaymentRefundStatus, PaymentSessionStatus,
};

pub mod manual;

pub const MANUAL_DRIVER: &str = "manual";
pub const STRIPE_DRIVER: &str = "stripe";
pub const PAYPAL_DRIVER: &str = "paypal";
pub const BRAINTREE_DRIVER: &str = "braintree";
pub const MERCADO_PAGO_DRIVER: &str = "mercado_pago";
pub const PAGARME_DRIVER: &str = "pagarme";
pub const IUGU_DRIVER: &str = "iugu";
pub const ABACATEPAY_DRIVER: &str = "abacatepay";
pub const CIELO_DRIVER: &str = "cielo";
pub const GETNET_DRIVER: &str = "getnet";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentSessionInput {
    pub payment_id: i32,
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub idempotency_key: String,
    pub auto_capture: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSessionOutput {
    pub status: PaymentSessionStatus,
    pub payment_status: PaymentAttemptStatus,
    pub external_session_id: Option<String>,
    pub external_payment_id: Option<String>,
    pub external_status: Option<String>,
    pub external_client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentInput {
    pub payment_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub external_payment_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidPaymentInput {
    pub payment_id: i32,
    pub external_payment_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPaymentInput {
    pub payment_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub external_payment_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentOperationOutput {
    pub status: PaymentAttemptStatus,
    pub external_payment_id: Option<String>,
    pub external_status: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefundOutput {
    pub status: PaymentRefundStatus,
    pub external_refund_id: Option<String>,
    pub external_status: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSetupSessionInput {
    pub user_id: i32,
    pub payment_method_id: i32,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSetupSessionOutput {
    pub status: PaymentSessionStatus,
    pub external_setup_id: Option<String>,
    pub external_client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInput {
    pub gateway_code: String,
    pub headers: Vec<(String, String)>,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookDecision {
    pub event_type: Option<String>,
    pub external_event_id: Option<String>,
    pub signature_valid: bool,
    pub processed: bool,
    pub ignored: bool,
    pub failure_message: Option<String>,
}

pub struct CreatePaymentAttemptInput {
    pub order_id: i32,
    pub payment_method: payment_methods::Model,
    pub amount: Decimal,
    pub currency: String,
}

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    fn driver(&self) -> &'static str;

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput>;

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput>;

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput>;

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput>;

    async fn create_setup_session(
        &self,
        input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput>;

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision>;
}

pub fn gateway_for_driver(driver: &str) -> Result<Box<dyn PaymentGateway>> {
    match driver {
        MANUAL_DRIVER => Ok(Box::new(manual::ManualGateway)),
        STRIPE_DRIVER | PAYPAL_DRIVER | BRAINTREE_DRIVER | MERCADO_PAGO_DRIVER | PAGARME_DRIVER
        | IUGU_DRIVER | ABACATEPAY_DRIVER | CIELO_DRIVER | GETNET_DRIVER => {
            Err(not_implemented(driver))
        }
        _ => Err(Error::BadRequest(
            format!("unsupported payment gateway driver: {driver}").into(),
        )),
    }
}

pub async fn create_payment_attempt<C>(
    db: &C,
    input: CreatePaymentAttemptInput,
) -> Result<payments::Model>
where
    C: ConnectionTrait,
{
    let now = chrono::Utc::now();
    let idempotency_key = format!("pay-{}", Uuid::new_v4());
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

fn not_implemented(driver: &str) -> Error {
    Error::BadRequest(format!("payment gateway driver is not implemented yet: {driver}").into())
}
