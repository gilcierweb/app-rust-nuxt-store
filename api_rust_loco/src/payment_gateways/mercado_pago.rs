use async_trait::async_trait;
use hmac::{Hmac, Mac};
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::Sha256;

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::MERCADO_PAGO_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput,
};

const MERCADO_PAGO_BASE: &str = "https://api.mercadopago.com";
const MERCADO_PAGO_ACCESS_TOKEN_ENV: &str = "MERCADO_PAGO_ACCESS_TOKEN";
const MERCADO_PAGO_WEBHOOK_SECRET_ENV: &str = "MERCADO_PAGO_WEBHOOK_SECRET";

type HmacSha256 = Hmac<Sha256>;

pub struct MercadoPagoGateway;

#[async_trait]
impl PaymentGateway for MercadoPagoGateway {
    fn driver(&self) -> &'static str {
        MERCADO_PAGO_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = decimal_amount(input.amount);
        let body = json!({
            "external_reference": input.payment_id.to_string(),
            "items": [{
                "id": input.payment_id.to_string(),
                "title": format!("Order {}", input.order_id),
                "quantity": 1,
                "unit_price": amount,
                "currency_id": input.currency.to_ascii_uppercase(),
            }],
            "metadata": {
                "payment_id": input.payment_id,
                "order_id": input.order_id,
                "payment_method_id": input.payment_method_id,
            }
        });
        let value =
            mercado_pago_post_json("/checkout/preferences", Some(&input.idempotency_key), body)
                .await?;

        Ok(PaymentSessionOutput {
            status: PaymentSessionStatus::RequiresAction,
            payment_status: PaymentAttemptStatus::Pending,
            external_session_id: string_field(&value, "id"),
            external_payment_id: string_field(&value, "id"),
            external_status: Some("preference_created".to_string()),
            external_client_secret: string_field(&value, "init_point")
                .or_else(|| string_field(&value, "sandbox_init_point")),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let body = json!({"capture": true});
        let path = format!("/v1/payments/{}", input.external_payment_id);
        let value = mercado_pago_put_json(&path, Some(&input.idempotency_key), body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: mercado_pago_attempt_status(status.as_deref()),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            failure_code: status_detail(&value),
            failure_message: string_field(&value, "status_detail"),
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let body = json!({"status": "cancelled"});
        let path = format!("/v1/payments/{}", input.external_payment_id);
        let value = mercado_pago_put_json(&path, Some(&input.idempotency_key), body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: mercado_pago_attempt_status(status.as_deref()),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            failure_code: status_detail(&value),
            failure_message: string_field(&value, "status_detail"),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let body = json!({"amount": decimal_amount(input.amount)});
        let path = format!("/v1/payments/{}/refunds", input.external_payment_id);
        let value = mercado_pago_post_json(&path, Some(&input.idempotency_key), body).await?;
        let status = string_field(&value, "status");

        Ok(RefundOutput {
            status: mercado_pago_refund_status(status.as_deref()),
            external_refund_id: string_field(&value, "id"),
            external_status: status,
            failure_code: string_field(&value, "status_detail"),
            failure_message: string_field(&value, "status_detail"),
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "Mercado Pago saved-source setup is not implemented".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "type").or_else(|| string_field(value, "topic")));
        let external_event_id = parsed
            .as_ref()
            .and_then(|value| string_field(value, "id").or_else(|| data_id(value)));
        let signature_valid = verify_signature(&input.headers, &input.payload)?;

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            signature_valid,
            processed: false,
            ignored: false,
            failure_message: (!signature_valid)
                .then(|| "invalid Mercado Pago signature".to_string()),
        })
    }
}

async fn mercado_pago_post_json(
    path: &str,
    idempotency_key: Option<&str>,
    body: Value,
) -> Result<Value> {
    mercado_pago_request(reqwest::Method::POST, path, idempotency_key, body).await
}

async fn mercado_pago_put_json(
    path: &str,
    idempotency_key: Option<&str>,
    body: Value,
) -> Result<Value> {
    mercado_pago_request(reqwest::Method::PUT, path, idempotency_key, body).await
}

async fn mercado_pago_request(
    method: reqwest::Method,
    path: &str,
    idempotency_key: Option<&str>,
    body: Value,
) -> Result<Value> {
    let token = env_value(MERCADO_PAGO_ACCESS_TOKEN_ENV)?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).map_err(|err| {
            Error::Message(format!("invalid Mercado Pago authorization header: {err}"))
        })?,
    );
    if let Some(idempotency_key) = idempotency_key {
        headers.insert(
            "X-Idempotency-Key",
            HeaderValue::from_str(idempotency_key).map_err(|err| {
                Error::Message(format!("invalid Mercado Pago idempotency key: {err}"))
            })?,
        );
    }

    let response = reqwest::Client::new()
        .request(method, format!("{MERCADO_PAGO_BASE}{path}"))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("Mercado Pago request failed: {err}")))?;
    let status = response.status();
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("Mercado Pago response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(mercado_pago_error_message(&value)));
    }

    Ok(value)
}

fn verify_signature(headers: &[(String, String)], payload: &str) -> Result<bool> {
    let secret = match std::env::var(MERCADO_PAGO_WEBHOOK_SECRET_ENV)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
    {
        Some(secret) => secret,
        None => return Ok(false),
    };
    let signature = match header_value(headers, "x-signature") {
        Some(signature) => signature,
        None => return Ok(false),
    };
    let request_id = header_value(headers, "x-request-id").unwrap_or_default();
    let parsed = serde_json::from_str::<Value>(payload).ok();
    let data_id = parsed
        .as_ref()
        .and_then(data_id)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let ts = signature_part(&signature, "ts").unwrap_or_default();
    let expected_payload = format!("id:{data_id};request-id:{request_id};ts:{ts};");
    let expected = hmac_sha256_hex(secret.as_bytes(), expected_payload.as_bytes())?;

    let valid = signature_parts(&signature, "v1").any(|actual| constant_time_eq(actual, &expected));
    Ok(valid)
}

fn mercado_pago_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("approved") => PaymentAttemptStatus::Captured,
        Some("authorized") => PaymentAttemptStatus::Authorized,
        Some("cancelled") => PaymentAttemptStatus::Cancelled,
        Some("rejected") => PaymentAttemptStatus::Failed,
        Some("refunded") => PaymentAttemptStatus::Refunded,
        Some("in_process" | "pending") => PaymentAttemptStatus::Processing,
        _ => PaymentAttemptStatus::Pending,
    }
}

fn mercado_pago_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("approved") => PaymentRefundStatus::Succeeded,
        Some("rejected" | "cancelled") => PaymentRefundStatus::Failed,
        _ => PaymentRefundStatus::Processing,
    }
}

fn decimal_amount(amount: Decimal) -> f64 {
    amount.round_dp(2).to_f64().unwrap_or(0.0)
}

fn env_value(name: &str) -> Result<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn mercado_pago_error_message(value: &Value) -> String {
    value
        .get("message")
        .and_then(Value::as_str)
        .map(|message| format!("Mercado Pago error: {message}"))
        .unwrap_or_else(|| "Mercado Pago request failed".to_string())
}

fn status_detail(value: &Value) -> Option<String> {
    string_field(value, "status_detail")
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn data_id(value: &Value) -> Option<String> {
    value.pointer("/data/id").and_then(value_to_string)
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

fn signature_part(header: &str, key: &str) -> Option<String> {
    signature_parts(header, key).next().map(ToString::to_string)
}

fn signature_parts<'a>(header: &'a str, key: &'a str) -> impl Iterator<Item = &'a str> {
    header.split(',').filter_map(move |part| {
        let (part_key, part_value) = part.trim().split_once('=')?;
        (part_key == key).then_some(part_value)
    })
}

fn hmac_sha256_hex(secret: &[u8], payload: &[u8]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|err| Error::Message(format!("invalid Mercado Pago webhook secret: {err}")))?;
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
    fn maps_mercado_pago_statuses_to_local_statuses() {
        assert_eq!(
            mercado_pago_attempt_status(Some("approved")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            mercado_pago_attempt_status(Some("authorized")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            mercado_pago_refund_status(Some("approved")),
            PaymentRefundStatus::Succeeded
        );
    }

    #[test]
    fn verifies_mercado_pago_signature() {
        std::env::set_var(MERCADO_PAGO_WEBHOOK_SECRET_ENV, "mp-secret");
        let payload = r#"{"data":{"id":"12345"},"type":"payment"}"#;
        let request_id = "request-123";
        let ts = "1700000000";
        let signed = format!("id:12345;request-id:{request_id};ts:{ts};");
        let signature = hmac_sha256_hex(b"mp-secret", signed.as_bytes()).unwrap();
        let headers = vec![
            ("x-request-id".to_string(), request_id.to_string()),
            ("x-signature".to_string(), format!("ts={ts},v1={signature}")),
        ];

        assert!(verify_signature(&headers, payload).unwrap());
    }
}
