use async_trait::async_trait;
use loco_rs::prelude::*;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::GETNET_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput,
};

const GETNET_CLIENT_ID_ENV: &str = "GETNET_CLIENT_ID";
const GETNET_CLIENT_SECRET_ENV: &str = "GETNET_CLIENT_SECRET";
const GETNET_ENV_ENV: &str = "GETNET_ENV";
const GETNET_BASE_URL_ENV: &str = "GETNET_BASE_URL";
const GETNET_SELLER_ID_ENV: &str = "GETNET_SELLER_ID";
const GETNET_TRANSACTION_CHANNEL_ENTRY_ENV: &str = "GETNET_TRANSACTION_CHANNEL_ENTRY";

pub struct GetnetGateway;

#[async_trait]
impl PaymentGateway for GetnetGateway {
    fn driver(&self) -> &'static str {
        GETNET_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let mut body = input
            .gateway_payload
            .and_then(|value| value.as_object().cloned())
            .ok_or_else(|| Error::BadRequest("Getnet gateway payload is required".into()))?;

        body.insert(
            "idempotency_key".to_string(),
            Value::String(input.idempotency_key),
        );
        body.insert(
            "amount".to_string(),
            Value::Number(amount_to_minor_units(input.amount)?.into()),
        );
        body.insert("currency".to_string(), Value::String(input.currency));
        body.entry("order_id".to_string())
            .or_insert_with(|| Value::String(input.order_id.to_string()));

        if let Some(seller_id) = optional_env_value(GETNET_SELLER_ID_ENV) {
            body.entry("seller_id".to_string())
                .or_insert_with(|| Value::String(seller_id));
        }

        let value = getnet_post("/dpm/payments-gwproxy/v2/payments", Value::Object(body)).await?;

        Ok(PaymentSessionOutput {
            status: getnet_session_status(status_value(&value)),
            payment_status: getnet_attempt_status(status_value(&value)),
            external_session_id: string_field(&value, "payment_id"),
            external_payment_id: string_field(&value, "payment_id"),
            external_status: status_value(&value).map(ToString::to_string),
            external_client_secret: string_field(&value, "redirect_url")
                .or_else(|| string_field(&value, "checkout_url")),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let value = getnet_post(
            "/dpm/payments-gwproxy/v2/payments/capture",
            json!({
                "idempotency_key": input.idempotency_key,
                "payment_id": input.external_payment_id,
                "amount": amount_to_minor_units(input.amount)?,
            }),
        )
        .await?;

        Ok(PaymentOperationOutput {
            status: getnet_attempt_status(status_value(&value)),
            external_payment_id: string_field(&value, "payment_id"),
            external_status: status_value(&value).map(ToString::to_string),
            failure_code: string_field(&value, "reason_code"),
            failure_message: string_field(&value, "reason_message"),
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let value = getnet_post(
            "/dpm/payments-gwproxy/v2/payments/cancel",
            json!({
                "idempotency_key": input.idempotency_key,
                "payment_id": input.external_payment_id,
                "payment_method": "CREDIT",
            }),
        )
        .await?;

        Ok(PaymentOperationOutput {
            status: getnet_attempt_status(status_value(&value)),
            external_payment_id: string_field(&value, "payment_id"),
            external_status: status_value(&value).map(ToString::to_string),
            failure_code: string_field(&value, "reason_code"),
            failure_message: string_field(&value, "reason_message"),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let value = getnet_post(
            "/dpm/payments-gwproxy/v2/payments/cancel",
            json!({
                "idempotency_key": input.idempotency_key,
                "payment_id": input.external_payment_id,
                "payment_method": "CREDIT",
                "amount": amount_to_minor_units(input.amount)?,
            }),
        )
        .await?;

        Ok(RefundOutput {
            status: getnet_refund_status(status_value(&value)),
            external_refund_id: string_field(&value, "payment_id")
                .or_else(|| string_field(&value, "refund_id")),
            external_status: status_value(&value).map(ToString::to_string),
            failure_code: string_field(&value, "reason_code"),
            failure_message: string_field(&value, "reason_message"),
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "Getnet setup sessions require a tokenization/Web Checkout flow that is not wired to the current checkout DTO".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed.as_ref().and_then(|value| {
            string_field(value, "event_type").or_else(|| string_field(value, "status"))
        });
        let external_event_id = parsed.as_ref().and_then(|value| {
            string_field(value, "event_id")
                .or_else(|| string_field(value, "id"))
                .or_else(|| string_field(value, "payment_id"))
        });

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            signature_valid: false,
            processed: false,
            ignored: false,
            failure_message: Some(
                "Getnet webhook signature validation is not configured for this adapter"
                    .to_string(),
            ),
        })
    }
}

async fn getnet_post(path: &str, body: Value) -> Result<Value> {
    let token = getnet_access_token().await?;
    let mut request = reqwest::Client::new()
        .post(format!("{}{}", getnet_base_url(), path))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header(CONTENT_TYPE, "application/json")
        .json(&body);

    if let Some(seller_id) = optional_env_value(GETNET_SELLER_ID_ENV) {
        request = request.header("x-seller-id", seller_id);
    }
    if let Some(channel_entry) = optional_env_value(GETNET_TRANSACTION_CHANNEL_ENTRY_ENV) {
        request = request.header("x-transaction-channel-entry", channel_entry);
    }

    let response = request
        .send()
        .await
        .map_err(|err| Error::Message(format!("Getnet request failed: {err}")))?;
    response_json(response, "Getnet").await
}

async fn getnet_access_token() -> Result<String> {
    let client_id = env_value(GETNET_CLIENT_ID_ENV)?;
    let client_secret = env_value(GETNET_CLIENT_SECRET_ENV)?;
    let response = reqwest::Client::new()
        .post(format!(
            "{}/authentication/oauth2/access_token",
            getnet_base_url()
        ))
        .basic_auth(client_id, Some(client_secret))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await
        .map_err(|err| Error::Message(format!("Getnet token request failed: {err}")))?;
    let value = response_json(response, "Getnet token").await?;
    string_field(&value, "access_token")
        .ok_or_else(|| Error::Message("Getnet token response missing access_token".to_string()))
}

async fn response_json(response: reqwest::Response, provider: &str) -> Result<Value> {
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("{provider} response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(provider_error_message(provider, &value)));
    }

    Ok(value)
}

fn getnet_base_url() -> String {
    if let Some(base_url) = optional_env_value(GETNET_BASE_URL_ENV) {
        return base_url.trim_end_matches('/').to_string();
    }

    match std::env::var(GETNET_ENV_ENV)
        .unwrap_or_else(|_| "sandbox".to_string())
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "production" | "prod" | "live" => "https://api.globalgetnet.com".to_string(),
        _ => "https://api-sbx.globalgetnet.com".to_string(),
    }
}

fn getnet_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("AUTHORIZED") => PaymentAttemptStatus::Authorized,
        Some("APPROVED" | "CAPTURED") => PaymentAttemptStatus::Captured,
        Some("CANCELLED" | "CANCELED") => PaymentAttemptStatus::Cancelled,
        Some("DENIED" | "REJECTED" | "FAILED") => PaymentAttemptStatus::Failed,
        _ => PaymentAttemptStatus::Processing,
    }
}

fn getnet_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("CANCELLED" | "CANCELED" | "REFUNDED" | "APPROVED") => PaymentRefundStatus::Succeeded,
        Some("DENIED" | "REJECTED" | "FAILED") => PaymentRefundStatus::Failed,
        _ => PaymentRefundStatus::Processing,
    }
}

fn getnet_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("APPROVED" | "AUTHORIZED" | "CAPTURED") => PaymentSessionStatus::Completed,
        Some("DENIED" | "REJECTED" | "FAILED") => PaymentSessionStatus::Failed,
        Some("CANCELLED" | "CANCELED") => PaymentSessionStatus::Cancelled,
        _ => PaymentSessionStatus::Processing,
    }
}

fn amount_to_minor_units(amount: Decimal) -> Result<i64> {
    (amount * Decimal::new(100, 0))
        .round()
        .to_i64()
        .filter(|amount| *amount > 0)
        .ok_or_else(|| Error::BadRequest("payment amount must be positive".into()))
}

fn provider_error_message(provider: &str, value: &Value) -> String {
    string_field(value, "reason_message")
        .or_else(|| string_field(value, "message"))
        .or_else(|| string_field(value, "error_description"))
        .map(|message| format!("{provider} error: {message}"))
        .unwrap_or_else(|| format!("{provider} request failed"))
}

fn env_value(name: &str) -> Result<String> {
    optional_env_value(name)
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn optional_env_value(name: &str) -> Option<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
}

fn status_value(value: &Value) -> Option<&str> {
    value.get("status").and_then(Value::as_str)
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_getnet_payment_statuses_to_local_statuses() {
        assert_eq!(
            getnet_attempt_status(Some("AUTHORIZED")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            getnet_attempt_status(Some("CAPTURED")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            getnet_attempt_status(Some("CANCELLED")),
            PaymentAttemptStatus::Cancelled
        );
        assert_eq!(
            getnet_attempt_status(Some("DENIED")),
            PaymentAttemptStatus::Failed
        );
    }

    #[test]
    fn uses_sandbox_base_url_by_default() {
        std::env::remove_var(GETNET_ENV_ENV);
        std::env::remove_var(GETNET_BASE_URL_ENV);
        assert_eq!(getnet_base_url(), "https://api-sbx.globalgetnet.com");
    }
}
