use async_trait::async_trait;
use hmac::{Hmac, Mac};
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::Sha256;

use crate::models::payment_gateway_status::{PaymentAttemptStatus, PaymentSessionStatus};
use crate::payment_gateways::drivers::ABACATEPAY_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookAction, WebhookDecision, WebhookInput,
};

const ABACATEPAY_BASE: &str = "https://api.abacatepay.com/v1";
const ABACATEPAY_API_KEY_ENV: &str = "ABACATEPAY_API_KEY";
const ABACATEPAY_WEBHOOK_SECRET_ENV: &str = "ABACATEPAY_WEBHOOK_SECRET";
const ABACATEPAY_RETURN_URL_ENV: &str = "ABACATEPAY_RETURN_URL";
const ABACATEPAY_COMPLETION_URL_ENV: &str = "ABACATEPAY_COMPLETION_URL";

type HmacSha256 = Hmac<Sha256>;

pub struct AbacatePayGateway;

#[async_trait]
impl PaymentGateway for AbacatePayGateway {
    fn driver(&self) -> &'static str {
        ABACATEPAY_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let body = json!({
            "frequency": "ONE_TIME",
            "methods": ["PIX", "CARD"],
            "products": [{
                "externalId": input.payment_id.to_string(),
                "name": format!("Order {}", input.order_id),
                "description": format!("Payment {}", input.payment_id),
                "quantity": 1,
                "price": amount,
            }],
            "returnUrl": env_value(ABACATEPAY_RETURN_URL_ENV)?,
            "completionUrl": env_value(ABACATEPAY_COMPLETION_URL_ENV)?,
            "allowCoupons": false,
            "externalId": input.payment_id.to_string(),
            "metadata": {
                "payment_id": input.payment_id,
                "order_id": input.order_id,
                "idempotency_key": input.idempotency_key,
            }
        });
        let value = abacatepay_post_json("/billing/create", body).await?;
        let data = value.get("data").unwrap_or(&value);
        let status = string_field(data, "status");

        Ok(PaymentSessionOutput {
            status: abacatepay_session_status(status.as_deref()),
            payment_status: abacatepay_attempt_status(status.as_deref()),
            external_session_id: string_field(data, "id"),
            external_payment_id: string_field(data, "id"),
            external_status: status,
            external_client_secret: string_field(data, "url"),
        })
    }

    async fn capture_payment(&self, _input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        Err(Error::BadRequest(
            "AbacatePay hosted checkout does not support manual capture in this adapter".into(),
        ))
    }

    async fn void_payment(&self, _input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        Err(Error::BadRequest(
            "AbacatePay hosted checkout cancellation is not implemented".into(),
        ))
    }

    async fn refund_payment(&self, _input: RefundPaymentInput) -> Result<RefundOutput> {
        Err(Error::BadRequest(
            "AbacatePay refund API is not implemented in this adapter".into(),
        ))
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "AbacatePay saved-source setup is not supported".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "event").or_else(|| string_field(value, "type")));
        let external_event_id = parsed.as_ref().and_then(|value| {
            string_field(value, "id")
                .or_else(|| value.pointer("/data/id").and_then(value_to_string))
        });
        let signature_valid = verify_signature(&input.headers, &input.payload)?;

        let action = match event_type.as_deref() {
            Some("billing.paid" | "billing.cancelled" | "billing.expired" | "billing.refunded") => {
                let status_str = parsed.as_ref().and_then(|v| {
                    string_field(v, "status").or_else(|| {
                        v.pointer("/data/status")
                            .and_then(Value::as_str)
                            .map(ToString::to_string)
                    })
                });
                external_event_id
                    .as_ref()
                    .map(|id| WebhookAction::UpdatePaymentStatus {
                        external_payment_id: id.to_string(),
                        status: abacatepay_attempt_status(status_str.as_deref()),
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
            failure_message: (!signature_valid).then(|| "invalid AbacatePay signature".to_string()),
        })
    }
}

async fn abacatepay_post_json(path: &str, body: Value) -> Result<Value> {
    let api_key = env_value(ABACATEPAY_API_KEY_ENV)?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|err| Error::Message(format!("invalid AbacatePay auth header: {err}")))?,
    );

    let response = reqwest::Client::new()
        .post(format!("{ABACATEPAY_BASE}{path}"))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("AbacatePay request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("AbacatePay response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(abacatepay_error_message(&value)));
    }

    Ok(value)
}

fn verify_signature(headers: &[(String, String)], payload: &str) -> Result<bool> {
    let secret = match std::env::var(ABACATEPAY_WEBHOOK_SECRET_ENV)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
    {
        Some(secret) => secret,
        None => return Ok(false),
    };
    let signature = match header_value(headers, "x-webhook-signature") {
        Some(signature) => signature,
        None => return Ok(false),
    };
    let expected = hmac_sha256_hex(secret.as_bytes(), payload.as_bytes())?;

    Ok(constant_time_eq(&signature, &expected))
}

fn abacatepay_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("COMPLETE" | "PAID") => PaymentSessionStatus::Completed,
        Some("CANCELLED") => PaymentSessionStatus::Cancelled,
        Some("EXPIRED") => PaymentSessionStatus::Expired,
        _ => PaymentSessionStatus::RequiresAction,
    }
}

fn abacatepay_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("COMPLETE" | "PAID") => PaymentAttemptStatus::Captured,
        Some("CANCELLED") => PaymentAttemptStatus::Cancelled,
        Some("EXPIRED") => PaymentAttemptStatus::Failed,
        Some("REFUNDED") => PaymentAttemptStatus::Refunded,
        _ => PaymentAttemptStatus::Pending,
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

fn abacatepay_error_message(value: &Value) -> String {
    value
        .get("error")
        .and_then(Value::as_str)
        .map(|message| format!("AbacatePay error: {message}"))
        .unwrap_or_else(|| "AbacatePay request failed".to_string())
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
        .map_err(|err| Error::Message(format!("invalid AbacatePay webhook secret: {err}")))?;
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
    fn maps_abacatepay_statuses_to_local_statuses() {
        assert_eq!(
            abacatepay_attempt_status(Some("COMPLETE")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            abacatepay_session_status(Some("EXPIRED")),
            PaymentSessionStatus::Expired
        );
    }

    #[test]
    fn verifies_abacatepay_signature() {
        std::env::set_var(ABACATEPAY_WEBHOOK_SECRET_ENV, "abacate-secret");
        let payload = r#"{"id":"evt_123","event":"checkout.completed"}"#;
        let signature = hmac_sha256_hex(b"abacate-secret", payload.as_bytes()).unwrap();
        let headers = vec![("x-webhook-signature".to_string(), signature)];

        assert!(verify_signature(&headers, payload).unwrap());
    }
}
