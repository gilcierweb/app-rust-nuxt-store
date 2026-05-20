use async_trait::async_trait;
use hmac::{Hmac, Mac};
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::Value;
use sha2::Sha256;

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::STRIPE_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput, WebhookAction,
};

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";
const STRIPE_SECRET_KEY_ENV: &str = "STRIPE_SECRET_KEY";
const STRIPE_WEBHOOK_SECRET_ENV: &str = "STRIPE_WEBHOOK_SECRET";
const STRIPE_SIGNATURE_HEADER: &str = "stripe-signature";
const SIGNATURE_TOLERANCE_SECONDS: i64 = 300;

type HmacSha256 = Hmac<Sha256>;

pub struct StripeGateway;

#[async_trait]
impl PaymentGateway for StripeGateway {
    fn driver(&self) -> &'static str {
        STRIPE_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let capture_method = if input.auto_capture {
            "automatic"
        } else {
            "manual"
        };
        let body = form_body(&[
            ("amount", amount.to_string()),
            ("currency", input.currency.to_ascii_lowercase()),
            ("automatic_payment_methods[enabled]", "true".to_string()),
            ("capture_method", capture_method.to_string()),
            ("metadata[payment_id]", input.payment_id.to_string()),
            ("metadata[order_id]", input.order_id.to_string()),
        ]);

        let value = stripe_post("/payment_intents", &input.idempotency_key, body).await?;
        let status = string_field(&value, "status");
        let external_payment_id = string_field(&value, "id");

        Ok(PaymentSessionOutput {
            status: session_status(status.as_deref()),
            payment_status: attempt_status(status.as_deref()),
            external_session_id: external_payment_id.clone(),
            external_payment_id,
            external_status: status,
            external_client_secret: string_field(&value, "client_secret"),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let path = format!("/payment_intents/{}/capture", input.external_payment_id);
        let body = form_body(&[("amount_to_capture", amount.to_string())]);
        let value = stripe_post(&path, &input.idempotency_key, body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: attempt_status(status.as_deref()),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            failure_code: latest_charge_failure_code(&value),
            failure_message: latest_charge_failure_message(&value),
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let path = format!("/payment_intents/{}/cancel", input.external_payment_id);
        let value = stripe_post(&path, &input.idempotency_key, String::new()).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: attempt_status(status.as_deref()),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            failure_code: latest_charge_failure_code(&value),
            failure_message: latest_charge_failure_message(&value),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let body = form_body(&[
            ("payment_intent", input.external_payment_id),
            ("amount", amount.to_string()),
        ]);
        let value = stripe_post("/refunds", &input.idempotency_key, body).await?;
        let status = string_field(&value, "status");

        Ok(RefundOutput {
            status: refund_status(status.as_deref()),
            external_refund_id: string_field(&value, "id"),
            external_status: status,
            failure_code: string_field(&value, "failure_reason"),
            failure_message: string_field(&value, "failure_balance_transaction"),
        })
    }

    async fn create_setup_session(
        &self,
        input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        let body = form_body(&[
            ("automatic_payment_methods[enabled]", "true".to_string()),
            ("usage", "off_session".to_string()),
            ("metadata[user_id]", input.user_id.to_string()),
            (
                "metadata[payment_method_id]",
                input.payment_method_id.to_string(),
            ),
        ]);
        let value = stripe_post("/setup_intents", &input.idempotency_key, body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentSetupSessionOutput {
            status: session_status(status.as_deref()),
            external_setup_id: string_field(&value, "id"),
            external_client_secret: string_field(&value, "client_secret"),
        })
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let signature = header_value(&input.headers, STRIPE_SIGNATURE_HEADER);
        let webhook_secret = env_value(STRIPE_WEBHOOK_SECRET_ENV)?;
        let signature_valid = signature
            .as_deref()
            .map(|signature| verify_webhook_signature(&input.payload, signature, &webhook_secret))
            .transpose()?
            .unwrap_or(false);

        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "type"));
        let external_event_id = parsed.as_ref().and_then(|value| string_field(value, "id"));

        let action = match event_type.as_deref() {
            Some("payment_intent.succeeded") => {
                parsed.as_ref().and_then(|v| v.pointer("/data/object/id")).and_then(Value::as_str).map(|id| WebhookAction::UpdatePaymentStatus {
                    external_payment_id: id.to_string(),
                    status: PaymentAttemptStatus::Captured,
                })
            }
            Some("payment_intent.payment_failed") => {
                parsed.as_ref().and_then(|v| v.pointer("/data/object/id")).and_then(Value::as_str).map(|id| WebhookAction::UpdatePaymentStatus {
                    external_payment_id: id.to_string(),
                    status: PaymentAttemptStatus::Failed,
                })
            }
            Some("payment_intent.canceled") => {
                parsed.as_ref().and_then(|v| v.pointer("/data/object/id")).and_then(Value::as_str).map(|id| WebhookAction::UpdatePaymentStatus {
                    external_payment_id: id.to_string(),
                    status: PaymentAttemptStatus::Cancelled,
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
            failure_message: (!signature_valid).then(|| "invalid Stripe signature".to_string()),
        })
    }
}

async fn stripe_post(path: &str, idempotency_key: &str, body: String) -> Result<Value> {
    let secret_key = env_value(STRIPE_SECRET_KEY_ENV)?;
    let url = format!("{STRIPE_API_BASE}{path}");
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        "Idempotency-Key",
        HeaderValue::from_str(idempotency_key)
            .map_err(|err| Error::Message(format!("invalid Stripe idempotency key: {err}")))?,
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {secret_key}"))
            .map_err(|err| Error::Message(format!("invalid Stripe authorization header: {err}")))?,
    );

    let response = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("Stripe request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("Stripe response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(stripe_error_message(&value)));
    }

    Ok(value)
}

fn env_value(name: &str) -> Result<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn amount_to_minor_units(amount: Decimal) -> Result<i64> {
    (amount * Decimal::new(100, 0))
        .round()
        .to_i64()
        .filter(|amount| *amount > 0)
        .ok_or_else(|| Error::BadRequest("payment amount must be positive".into()))
}

fn form_body(params: &[(&str, String)]) -> String {
    params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("succeeded") => PaymentAttemptStatus::Captured,
        Some("requires_capture") => PaymentAttemptStatus::Authorized,
        Some("processing") => PaymentAttemptStatus::Processing,
        Some("canceled") => PaymentAttemptStatus::Cancelled,
        Some("requires_action" | "requires_confirmation" | "requires_payment_method") => {
            PaymentAttemptStatus::Pending
        }
        _ => PaymentAttemptStatus::Processing,
    }
}

fn session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("succeeded" | "requires_capture") => PaymentSessionStatus::Completed,
        Some("requires_action") => PaymentSessionStatus::RequiresAction,
        Some("processing") => PaymentSessionStatus::Processing,
        Some("canceled") => PaymentSessionStatus::Cancelled,
        Some("requires_confirmation" | "requires_payment_method") => PaymentSessionStatus::Pending,
        _ => PaymentSessionStatus::Pending,
    }
}

fn refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("succeeded") => PaymentRefundStatus::Succeeded,
        Some("failed") => PaymentRefundStatus::Failed,
        Some("canceled") => PaymentRefundStatus::Cancelled,
        Some("requires_action") => PaymentRefundStatus::Pending,
        _ => PaymentRefundStatus::Processing,
    }
}

fn stripe_error_message(value: &Value) -> String {
    value
        .pointer("/error/message")
        .and_then(Value::as_str)
        .map(|message| format!("Stripe error: {message}"))
        .unwrap_or_else(|| "Stripe request failed".to_string())
}

fn latest_charge_failure_code(value: &Value) -> Option<String> {
    value
        .pointer("/last_payment_error/code")
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn latest_charge_failure_message(value: &Value) -> Option<String> {
    value
        .pointer("/last_payment_error/message")
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn header_value(headers: &[(String, String)], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.to_string())
}

fn verify_webhook_signature(payload: &str, header: &str, secret: &str) -> Result<bool> {
    let timestamp = signature_part(header, "t")
        .ok_or_else(|| Error::BadRequest("Stripe-Signature header missing timestamp".into()))?;
    let timestamp = timestamp
        .parse::<i64>()
        .map_err(|_| Error::BadRequest("Stripe-Signature timestamp is invalid".into()))?;
    let now = chrono::Utc::now().timestamp();
    if (now - timestamp).abs() > SIGNATURE_TOLERANCE_SECONDS {
        return Ok(false);
    }

    let signed_payload = format!("{timestamp}.{payload}");
    let expected = hmac_sha256_hex(secret.as_bytes(), signed_payload.as_bytes())?;

    Ok(signature_parts(header, "v1").any(|actual| constant_time_eq(actual, &expected)))
}

fn signature_part<'a>(header: &'a str, key: &'a str) -> Option<&'a str> {
    signature_parts(header, key).next()
}

fn signature_parts<'a>(header: &'a str, key: &'a str) -> impl Iterator<Item = &'a str> {
    header.split(',').filter_map(move |part| {
        let (part_key, part_value) = part.split_once('=')?;
        (part_key == key).then_some(part_value)
    })
}

fn hmac_sha256_hex(secret: &[u8], payload: &[u8]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|err| Error::Message(format!("invalid Stripe webhook secret: {err}")))?;
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
    fn maps_stripe_statuses_to_local_statuses() {
        assert_eq!(
            attempt_status(Some("succeeded")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            attempt_status(Some("requires_capture")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            session_status(Some("requires_action")),
            PaymentSessionStatus::RequiresAction
        );
        assert_eq!(refund_status(Some("failed")), PaymentRefundStatus::Failed);
    }

    #[test]
    fn converts_amount_to_minor_units() {
        assert_eq!(amount_to_minor_units(Decimal::new(1234, 2)).unwrap(), 1234);
        assert!(amount_to_minor_units(Decimal::ZERO).is_err());
    }

    #[test]
    fn verifies_valid_stripe_signature() {
        let timestamp = chrono::Utc::now().timestamp();
        let payload = r#"{"id":"evt_123","type":"payment_intent.succeeded"}"#;
        let secret = "whsec_test";
        let signed_payload = format!("{timestamp}.{payload}");
        let signature = hmac_sha256_hex(secret.as_bytes(), signed_payload.as_bytes()).unwrap();
        let header = format!("t={timestamp},v1={signature}");

        assert!(verify_webhook_signature(payload, &header, secret).unwrap());
    }

    #[test]
    fn rejects_invalid_stripe_signature() {
        let timestamp = chrono::Utc::now().timestamp();
        let payload = r#"{"id":"evt_123"}"#;
        let header = format!("t={timestamp},v1=bad");

        assert!(!verify_webhook_signature(payload, &header, "whsec_test").unwrap());
    }
}
