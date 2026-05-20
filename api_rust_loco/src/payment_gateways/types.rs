use async_trait::async_trait;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentSessionInput {
    pub payment_id: i32,
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub idempotency_key: String,
    pub auto_capture: bool,
    pub gateway_payload: Option<Value>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookAction {
    UpdatePaymentStatus {
        external_payment_id: String,
        status: PaymentAttemptStatus,
    },
    UpdatePaymentStatusById {
        payment_id: i32,
        status: PaymentAttemptStatus,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookDecision {
    pub event_type: Option<String>,
    pub external_event_id: Option<String>,
    pub action: Option<WebhookAction>,
    pub signature_valid: bool,
    pub processed: bool,
    pub ignored: bool,
    pub failure_message: Option<String>,
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
