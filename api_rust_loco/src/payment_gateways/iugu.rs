use async_trait::async_trait;
use hmac::{Hmac, Mac};
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::Sha256;

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::IUGU_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput, WebhookAction,
};

const IUGU_BASE: &str = "https://api.iugu.com/v1";
const IUGU_API_TOKEN_ENV: &str = "IUGU_API_TOKEN";
const IUGU_CUSTOMER_EMAIL_ENV: &str = "IUGU_CUSTOMER_EMAIL";
const IUGU_WEBHOOK_SECRET_ENV: &str = "IUGU_WEBHOOK_SECRET";
const IUGU_SIGNATURE_HEADER: &str = "x-iugu-signature";

type HmacSha256 = Hmac<Sha256>;

pub struct IuguGateway;

#[async_trait]
impl PaymentGateway for IuguGateway {
    fn driver(&self) -> &'static str {
        IUGU_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let body = json!({
            "email": env_value(IUGU_CUSTOMER_EMAIL_ENV)?,
            "due_date": chrono::Utc::now().date_naive().to_string(),
            "ensure_workday_due_date": false,
            "payable_with": ["pix", "bank_slip", "credit_card"],
            "items": [{
                "description": format!("Order {}", input.order_id),
                "quantity": 1,
                "price_cents": amount,
            }],
            "custom_variables": [
                {"name": "payment_id", "value": input.payment_id.to_string()},
                {"name": "order_id", "value": input.order_id.to_string()},
                {"name": "idempotency_key", "value": input.idempotency_key}
            ]
        });
        let value = iugu_post_json("/invoices", body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentSessionOutput {
            status: iugu_session_status(status.as_deref()),
            payment_status: iugu_attempt_status(status.as_deref()),
            external_session_id: string_field(&value, "id"),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            external_client_secret: string_field(&value, "secure_url")
                .or_else(|| string_field(&value, "bank_slip_url")),
        })
    }

    async fn capture_payment(&self, _input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        Err(Error::BadRequest(
            "Iugu invoice flow does not support manual capture in this adapter".into(),
        ))
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let path = format!("/invoices/{}/cancel", input.external_payment_id);
        let value = iugu_post_json(&path, json!({})).await?;
        let status = string_field(&value, "status").or_else(|| Some("canceled".to_string()));

        Ok(PaymentOperationOutput {
            status: iugu_attempt_status(status.as_deref()),
            external_payment_id: Some(input.external_payment_id),
            external_status: status,
            failure_code: None,
            failure_message: string_field(&value, "message"),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let path = format!("/invoices/{}/refund", input.external_payment_id);
        let value = iugu_post_json(&path, json!({})).await?;
        let status = string_field(&value, "status").or_else(|| Some("refunded".to_string()));

        Ok(RefundOutput {
            status: iugu_refund_status(status.as_deref()),
            external_refund_id: string_field(&value, "id")
                .or_else(|| Some(input.external_payment_id)),
            external_status: status,
            failure_code: None,
            failure_message: string_field(&value, "message"),
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "Iugu saved-source setup requires iugu.js tokenization and is not implemented".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "event"));
        let external_event_id = parsed
            .as_ref()
            .and_then(|value| value.pointer("/data/id").and_then(value_to_string))
            .or_else(|| parsed.as_ref().and_then(|value| string_field(value, "id")));
        let signature_valid = verify_signature(&input.headers, &input.payload)?;

        let action = match event_type.as_deref() {
            Some("invoice.status_changed" | "invoice.refund") => {
                let status_str = parsed.as_ref().and_then(|v| {
                    string_field(v, "status")
                        .or_else(|| v.pointer("/data/status").and_then(Value::as_str).map(ToString::to_string))
                });
                external_event_id.as_ref().map(|id| WebhookAction::UpdatePaymentStatus {
                    external_payment_id: id.to_string(),
                    status: iugu_attempt_status(status_str.as_deref()),
                })
            }
            _ => None,
        };

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            action,
            signature_valid,
            processed: false,
            ignored: false,
            failure_message: (!signature_valid).then(|| "invalid Iugu signature".to_string()),
        })
    }
}

async fn iugu_post_json(path: &str, body: Value) -> Result<Value> {
    let api_token = env_value(IUGU_API_TOKEN_ENV)?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let response = reqwest::Client::new()
        .post(format!("{IUGU_BASE}{path}?api_token={api_token}"))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("Iugu request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("Iugu response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(iugu_error_message(&value)));
    }

    Ok(value)
}

fn verify_signature(headers: &[(String, String)], payload: &str) -> Result<bool> {
    let secret = match std::env::var(IUGU_WEBHOOK_SECRET_ENV)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
    {
        Some(secret) => secret,
        None => return Ok(false),
    };
    let signature = match header_value(headers, IUGU_SIGNATURE_HEADER) {
        Some(signature) => signature,
        None => return Ok(false),
    };
    let expected = hmac_sha256_hex(secret.as_bytes(), payload.as_bytes())?;

    Ok(constant_time_eq(&signature, &expected))
}

fn iugu_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("paid" | "externally_paid") => PaymentAttemptStatus::Captured,
        Some("authorized") => PaymentAttemptStatus::Authorized,
        Some("canceled" | "expired") => PaymentAttemptStatus::Cancelled,
        Some("refunded") => PaymentAttemptStatus::Refunded,
        Some("chargeback") => PaymentAttemptStatus::Failed,
        _ => PaymentAttemptStatus::Pending,
    }
}

fn iugu_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("paid" | "externally_paid" | "authorized") => PaymentSessionStatus::Completed,
        Some("canceled") => PaymentSessionStatus::Cancelled,
        Some("expired") => PaymentSessionStatus::Expired,
        _ => PaymentSessionStatus::RequiresAction,
    }
}

fn iugu_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("refunded") => PaymentRefundStatus::Succeeded,
        Some("canceled" | "expired") => PaymentRefundStatus::Cancelled,
        _ => PaymentRefundStatus::Processing,
    }
}

fn amount_to_minor_units(amount: Decimal) -> Result<i64> {
    (amount * Decimal::new(100, 0))
        .round()
        .to_i64()
        .filter(|amount| *amount > 0)
        .ok_or_else(|| Error::BadRequest("payment amount must be positive".into()))
}

fn env_value(name: &str) -> Result<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn iugu_error_message(value: &Value) -> String {
    value
        .get("message")
        .and_then(Value::as_str)
        .map(|message| format!("Iugu error: {message}"))
        .unwrap_or_else(|| "Iugu request failed".to_string())
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn value_to_string(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(ToString::to_string)
        .or_else(|| value.as_i64().map(|number| number.to_string()))
        .or_else(|| value.as_u64().map(|number| number.to_string()))
}

fn header_value(headers: &[(String, String)], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.to_string())
}

fn hmac_sha256_hex(secret: &[u8], payload: &[u8]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|err| Error::Message(format!("invalid Iugu webhook secret: {err}")))?;
    mac.update(payload);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn constant_time_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let max_len = left.len().max(right.len());
    let mut diff = left.len() ^ right.len();

    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);
        diff |= (left_byte ^ right_byte) as usize;
    }

    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_iugu_statuses_to_local_statuses() {
        assert_eq!(
            iugu_attempt_status(Some("paid")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            iugu_session_status(Some("expired")),
            PaymentSessionStatus::Expired
        );
        assert_eq!(
            iugu_refund_status(Some("refunded")),
            PaymentRefundStatus::Succeeded
        );
    }

    #[test]
    fn verifies_iugu_signature_when_configured() {
        std::env::set_var(IUGU_WEBHOOK_SECRET_ENV, "iugu-secret");
        let payload = r#"{"event":"invoice.status_changed","data":{"id":"inv_123"}}"#;
        let signature = hmac_sha256_hex(b"iugu-secret", payload.as_bytes()).unwrap();
        let headers = vec![(IUGU_SIGNATURE_HEADER.to_string(), signature)];

        assert!(verify_signature(&headers, payload).unwrap());
    }
}
