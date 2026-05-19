use async_trait::async_trait;
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::PAYPAL_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput,
};

const PAYPAL_CLIENT_ID_ENV: &str = "PAYPAL_CLIENT_ID";
const PAYPAL_CLIENT_SECRET_ENV: &str = "PAYPAL_CLIENT_SECRET";
const PAYPAL_ENV_ENV: &str = "PAYPAL_ENV";
const PAYPAL_WEBHOOK_ID_ENV: &str = "PAYPAL_WEBHOOK_ID";
const PAYPAL_SANDBOX_BASE: &str = "https://api-m.sandbox.paypal.com";
const PAYPAL_LIVE_BASE: &str = "https://api-m.paypal.com";

pub struct PayPalGateway;

#[async_trait]
impl PaymentGateway for PayPalGateway {
    fn driver(&self) -> &'static str {
        PAYPAL_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let token = paypal_access_token().await?;
        let intent = if input.auto_capture {
            "CAPTURE"
        } else {
            "AUTHORIZE"
        };
        let body = json!({
            "intent": intent,
            "purchase_units": [{
                "reference_id": input.payment_id.to_string(),
                "custom_id": input.order_id.to_string(),
                "amount": {
                    "currency_code": input.currency.to_ascii_uppercase(),
                    "value": decimal_amount(input.amount),
                }
            }]
        });

        let value = paypal_post_json(
            "/v2/checkout/orders",
            &token,
            Some(&input.idempotency_key),
            body,
        )
        .await?;
        let status = string_field(&value, "status");
        let order_id = string_field(&value, "id");

        Ok(PaymentSessionOutput {
            status: paypal_session_status(status.as_deref()),
            payment_status: paypal_attempt_status(status.as_deref()),
            external_session_id: order_id.clone(),
            external_payment_id: order_id,
            external_status: status,
            external_client_secret: approve_link(&value),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let token = paypal_access_token().await?;
        let path = format!("/v2/checkout/orders/{}/capture", input.external_payment_id);
        let value =
            paypal_post_json(&path, &token, Some(&input.idempotency_key), json!({})).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: paypal_attempt_status(status.as_deref()),
            external_payment_id: capture_id(&value).or_else(|| string_field(&value, "id")),
            external_status: status,
            failure_code: None,
            failure_message: None,
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let token = paypal_access_token().await?;
        let path = format!(
            "/v2/payments/authorizations/{}/void",
            input.external_payment_id
        );
        paypal_post_json(&path, &token, Some(&input.idempotency_key), json!({})).await?;

        Ok(PaymentOperationOutput {
            status: PaymentAttemptStatus::Voided,
            external_payment_id: Some(input.external_payment_id),
            external_status: Some("VOIDED".to_string()),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let token = paypal_access_token().await?;
        let path = format!("/v2/payments/captures/{}/refund", input.external_payment_id);
        let body = json!({
            "amount": {
                "currency_code": input.currency.to_ascii_uppercase(),
                "value": decimal_amount(input.amount),
            }
        });
        let value = paypal_post_json(&path, &token, Some(&input.idempotency_key), body).await?;
        let status = string_field(&value, "status");

        Ok(RefundOutput {
            status: paypal_refund_status(status.as_deref()),
            external_refund_id: string_field(&value, "id"),
            external_status: status,
            failure_code: None,
            failure_message: None,
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "PayPal setup sessions require provider vault flow and are not implemented".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let token = paypal_access_token().await?;
        let webhook_id = env_value(PAYPAL_WEBHOOK_ID_ENV)?;
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "event_type"));
        let external_event_id = parsed.as_ref().and_then(|value| string_field(value, "id"));

        let body = json!({
            "auth_algo": header_value(&input.headers, "paypal-auth-algo").unwrap_or_default(),
            "cert_url": header_value(&input.headers, "paypal-cert-url").unwrap_or_default(),
            "transmission_id": header_value(&input.headers, "paypal-transmission-id").unwrap_or_default(),
            "transmission_sig": header_value(&input.headers, "paypal-transmission-sig").unwrap_or_default(),
            "transmission_time": header_value(&input.headers, "paypal-transmission-time").unwrap_or_default(),
            "webhook_id": webhook_id,
            "webhook_event": parsed.clone().unwrap_or_else(|| json!({})),
        });
        let value = paypal_post_json(
            "/v1/notifications/verify-webhook-signature",
            &token,
            None,
            body,
        )
        .await?;
        let signature_valid = string_field(&value, "verification_status")
            .map(|status| status == "SUCCESS")
            .unwrap_or(false);

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            signature_valid,
            processed: false,
            ignored: false,
            failure_message: (!signature_valid).then(|| "invalid PayPal signature".to_string()),
        })
    }
}

async fn paypal_access_token() -> Result<String> {
    let client_id = env_value(PAYPAL_CLIENT_ID_ENV)?;
    let client_secret = env_value(PAYPAL_CLIENT_SECRET_ENV)?;
    let response = reqwest::Client::new()
        .post(format!("{}/v1/oauth2/token", paypal_base_url()))
        .basic_auth(client_id, Some(client_secret))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await
        .map_err(|err| Error::Message(format!("PayPal token request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("PayPal token response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(paypal_error_message(&value)));
    }

    string_field(&value, "access_token")
        .ok_or_else(|| Error::Message("PayPal token response missing access_token".to_string()))
}

async fn paypal_post_json(
    path: &str,
    token: &str,
    idempotency_key: Option<&str>,
    body: Value,
) -> Result<Value> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|err| Error::Message(format!("invalid PayPal authorization header: {err}")))?,
    );
    if let Some(idempotency_key) = idempotency_key {
        headers.insert(
            "PayPal-Request-Id",
            HeaderValue::from_str(idempotency_key)
                .map_err(|err| Error::Message(format!("invalid PayPal request id: {err}")))?,
        );
    }

    let response = reqwest::Client::new()
        .post(format!("{}{}", paypal_base_url(), path))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("PayPal request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("PayPal response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(paypal_error_message(&value)));
    }

    Ok(value)
}

fn paypal_base_url() -> &'static str {
    match std::env::var(PAYPAL_ENV_ENV)
        .unwrap_or_else(|_| "sandbox".to_string())
        .trim()
    {
        "live" | "production" => PAYPAL_LIVE_BASE,
        _ => PAYPAL_SANDBOX_BASE,
    }
}

fn paypal_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("COMPLETED") => PaymentAttemptStatus::Captured,
        Some("APPROVED") => PaymentAttemptStatus::Authorized,
        Some("VOIDED") => PaymentAttemptStatus::Voided,
        Some("CANCELLED" | "CANCELED") => PaymentAttemptStatus::Cancelled,
        Some("CREATED" | "PAYER_ACTION_REQUIRED") => PaymentAttemptStatus::Pending,
        _ => PaymentAttemptStatus::Processing,
    }
}

fn paypal_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("COMPLETED" | "APPROVED") => PaymentSessionStatus::Completed,
        Some("CREATED" | "PAYER_ACTION_REQUIRED") => PaymentSessionStatus::RequiresAction,
        Some("VOIDED" | "CANCELLED" | "CANCELED") => PaymentSessionStatus::Cancelled,
        _ => PaymentSessionStatus::Pending,
    }
}

fn paypal_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("COMPLETED") => PaymentRefundStatus::Succeeded,
        Some("FAILED") => PaymentRefundStatus::Failed,
        Some("CANCELLED" | "CANCELED") => PaymentRefundStatus::Cancelled,
        _ => PaymentRefundStatus::Processing,
    }
}

fn approve_link(value: &Value) -> Option<String> {
    value.get("links")?.as_array()?.iter().find_map(|link| {
        let rel = link.get("rel")?.as_str()?;
        (rel == "approve" || rel == "payer-action")
            .then(|| {
                link.get("href")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })
            .flatten()
    })
}

fn capture_id(value: &Value) -> Option<String> {
    value
        .pointer("/purchase_units/0/payments/captures/0/id")
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn decimal_amount(amount: Decimal) -> String {
    format!("{amount:.2}")
}

fn env_value(name: &str) -> Result<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn paypal_error_message(value: &Value) -> String {
    value
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| value.get("error_description").and_then(Value::as_str))
        .map(|message| format!("PayPal error: {message}"))
        .unwrap_or_else(|| "PayPal request failed".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_paypal_statuses_to_local_statuses() {
        assert_eq!(
            paypal_attempt_status(Some("COMPLETED")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            paypal_session_status(Some("CREATED")),
            PaymentSessionStatus::RequiresAction
        );
        assert_eq!(
            paypal_refund_status(Some("FAILED")),
            PaymentRefundStatus::Failed
        );
    }

    #[test]
    fn extracts_approval_link() {
        let value = json!({
            "links": [
                {"rel": "self", "href": "https://example.com/self"},
                {"rel": "approve", "href": "https://example.com/approve"}
            ]
        });

        assert_eq!(
            approve_link(&value),
            Some("https://example.com/approve".to_string())
        );
    }
}
