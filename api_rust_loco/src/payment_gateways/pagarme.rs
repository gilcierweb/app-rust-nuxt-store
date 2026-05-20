use async_trait::async_trait;
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::PAGARME_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput, WebhookAction,
};

const PAGARME_SECRET_KEY_ENV: &str = "PAGARME_SECRET_KEY";
const PAGARME_BASE_URL_ENV: &str = "PAGARME_BASE_URL";
const PAGARME_DEFAULT_BASE: &str = "https://sdx-api.pagar.me/core/v5";
const PAGARME_USER_AGENT: &str = "app-rust-nuxt-store/1.0";

pub struct PagarmeGateway;

#[async_trait]
impl PaymentGateway for PagarmeGateway {
    fn driver(&self) -> &'static str {
        PAGARME_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let body = json!({
            "type": "order",
            "name": format!("Order {}", input.order_id),
            "order_code": input.order_id.to_string(),
            "max_paid_sessions": 1,
            "payment_settings": {
                "accepted_payment_methods": ["credit_card", "pix", "boleto"],
                "credit_card_settings": {
                    "operation_type": if input.auto_capture { "auth_and_capture" } else { "auth_only" }
                }
            },
            "cart_settings": {
                "items": [{
                    "amount": amount,
                    "description": format!("Payment {}", input.payment_id),
                    "quantity": 1,
                }]
            },
            "metadata": {
                "payment_id": input.payment_id.to_string(),
                "order_id": input.order_id.to_string(),
                "idempotency_key": input.idempotency_key,
            }
        });
        let value = pagarme_request(reqwest::Method::POST, "/paymentlinks", body).await?;
        let status = string_field(&value, "status");

        Ok(PaymentSessionOutput {
            status: pagarme_session_status(status.as_deref()),
            payment_status: pagarme_attempt_status(status.as_deref()),
            external_session_id: string_field(&value, "id"),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            external_client_secret: string_field(&value, "url"),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let path = format!("/charges/{}/capture", input.external_payment_id);
        let value =
            pagarme_request(reqwest::Method::POST, &path, json!({ "amount": amount })).await?;
        let status = string_field(&value, "status");

        Ok(PaymentOperationOutput {
            status: pagarme_attempt_status(status.as_deref()),
            external_payment_id: string_field(&value, "id"),
            external_status: status,
            failure_code: last_transaction_field(&value, "acquirer_return_code"),
            failure_message: last_transaction_field(&value, "acquirer_message"),
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let path = format!("/paymentlinks/{}/cancel", input.external_payment_id);
        pagarme_request(reqwest::Method::PATCH, &path, json!({})).await?;

        Ok(PaymentOperationOutput {
            status: PaymentAttemptStatus::Cancelled,
            external_payment_id: Some(input.external_payment_id),
            external_status: Some("cancelled".to_string()),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let path = format!("/charges/{}/cancel", input.external_payment_id);
        let value =
            pagarme_request(reqwest::Method::DELETE, &path, json!({ "amount": amount })).await?;
        let status = string_field(&value, "status");

        Ok(RefundOutput {
            status: pagarme_refund_status(status.as_deref()),
            external_refund_id: string_field(&value, "id"),
            external_status: status,
            failure_code: last_transaction_field(&value, "acquirer_return_code"),
            failure_message: last_transaction_field(&value, "acquirer_message"),
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "Pagar.me setup sessions require frontend tokenization and are not implemented".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "type").or_else(|| string_field(value, "event")));
        let external_event_id = parsed.as_ref().and_then(|value| {
            string_field(value, "id")
                .or_else(|| value.pointer("/data/id").and_then(value_to_string))
        });

        let action = match event_type.as_deref() {
            Some("order.paid" | "order.payment_failed" | "order.canceled") | Some("charge.paid" | "charge.payment_failed" | "charge.refunded") => {
                let status_str = parsed.as_ref().and_then(|v| {
                    v.pointer("/data/status").and_then(Value::as_str).map(ToString::to_string)
                });
                parsed.as_ref().and_then(|v| {
                    v.pointer("/data/metadata/payment_id")
                        .and_then(Value::as_str)
                        .and_then(|id| id.parse::<i32>().ok())
                }).map(|payment_id| WebhookAction::UpdatePaymentStatusById {
                    payment_id,
                    status: pagarme_attempt_status(status_str.as_deref()),
                })
            }
            _ => None,
        };

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            action,
            signature_valid: true,
            processed: false,
            ignored: false,
            failure_message: None,
        })
    }
}

async fn pagarme_request(method: reqwest::Method, path: &str, body: Value) -> Result<Value> {
    let secret_key = env_value(PAGARME_SECRET_KEY_ENV)?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static(PAGARME_USER_AGENT));

    let response = reqwest::Client::new()
        .request(method, format!("{}{}", pagarme_base_url(), path))
        .basic_auth(secret_key, Some(""))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("Pagar.me request failed: {err}")))?;
    let status = response.status();
    if status.as_u16() == 204 {
        return Ok(json!({"status": "cancelled"}));
    }
    let value = response
        .json::<Value>()
        .await
        .map_err(|err| Error::Message(format!("Pagar.me response JSON is invalid: {err}")))?;

    if !status.is_success() {
        return Err(Error::Message(pagarme_error_message(&value)));
    }

    Ok(value)
}

fn pagarme_base_url() -> String {
    std::env::var(PAGARME_BASE_URL_ENV)
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| PAGARME_DEFAULT_BASE.to_string())
}

fn pagarme_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("paid" | "captured") => PaymentAttemptStatus::Captured,
        Some("authorized_pending_capture") => PaymentAttemptStatus::Authorized,
        Some("active" | "pending" | "waiting_payment") => PaymentAttemptStatus::Pending,
        Some("cancelled" | "canceled" | "voided") => PaymentAttemptStatus::Cancelled,
        Some("refunded") => PaymentAttemptStatus::Refunded,
        Some("failed" | "not_authorized" | "with_error") => PaymentAttemptStatus::Failed,
        _ => PaymentAttemptStatus::Processing,
    }
}

fn pagarme_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("active") => PaymentSessionStatus::RequiresAction,
        Some("paid" | "captured") => PaymentSessionStatus::Completed,
        Some("cancelled" | "canceled") => PaymentSessionStatus::Cancelled,
        Some("expired") => PaymentSessionStatus::Expired,
        _ => PaymentSessionStatus::Pending,
    }
}

fn pagarme_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("refunded" | "voided" | "cancelled" | "canceled") => PaymentRefundStatus::Succeeded,
        Some("failed" | "with_error") => PaymentRefundStatus::Failed,
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

fn pagarme_error_message(value: &Value) -> String {
    value
        .pointer("/errors/0/message")
        .and_then(Value::as_str)
        .or_else(|| value.get("message").and_then(Value::as_str))
        .map(|message| format!("Pagar.me error: {message}"))
        .unwrap_or_else(|| "Pagar.me request failed".to_string())
}

fn last_transaction_field(value: &Value, key: &str) -> Option<String> {
    value
        .get("last_transaction")
        .and_then(|transaction| string_field(transaction, key))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_pagarme_statuses_to_local_statuses() {
        assert_eq!(
            pagarme_attempt_status(Some("authorized_pending_capture")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            pagarme_session_status(Some("active")),
            PaymentSessionStatus::RequiresAction
        );
        assert_eq!(
            pagarme_refund_status(Some("refunded")),
            PaymentRefundStatus::Succeeded
        );
    }

    #[test]
    fn default_base_url_is_sandbox() {
        std::env::remove_var(PAGARME_BASE_URL_ENV);
        assert_eq!(pagarme_base_url(), PAGARME_DEFAULT_BASE);
    }
}
