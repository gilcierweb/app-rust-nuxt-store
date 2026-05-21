use async_trait::async_trait;
use loco_rs::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::CIELO_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookAction, WebhookDecision, WebhookInput,
};

const CIELO_CHECKOUT_BASE: &str = "https://cieloecommerce.cielo.com.br/api/public";
const CIELO_MERCHANT_ID_ENV: &str = "CIELO_MERCHANT_ID";
const CIELO_CLIENT_ID_ENV: &str = "CIELO_CLIENT_ID";
const CIELO_CLIENT_SECRET_ENV: &str = "CIELO_CLIENT_SECRET";

pub struct CieloGateway;

#[async_trait]
impl PaymentGateway for CieloGateway {
    fn driver(&self) -> &'static str {
        CIELO_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let amount = amount_to_minor_units(input.amount)?;
        let body = json!({
            "OrderNumber": format!("{}{}", input.order_id, input.payment_id),
            "SoftDescriptor": "STORE",
            "Cart": {
                "Items": [{
                    "Name": format!("Order {}", input.order_id),
                    "Description": format!("Payment {}", input.payment_id),
                    "UnitPrice": amount,
                    "Quantity": 1,
                    "Type": "Asset",
                }]
            },
            "Payment": {
                "Capture": input.auto_capture,
            },
        });
        let value = cielo_checkout_post("/v1/orders/", body).await?;

        Ok(PaymentSessionOutput {
            status: PaymentSessionStatus::RequiresAction,
            payment_status: PaymentAttemptStatus::Pending,
            external_session_id: string_field(&value, "OrderNumber")
                .or_else(|| string_field(&value, "CheckoutCieloOrderNumber")),
            external_payment_id: string_field(&value, "OrderNumber")
                .or_else(|| string_field(&value, "CheckoutCieloOrderNumber")),
            external_status: Some("checkout_created".to_string()),
            external_client_secret: string_field(&value, "CheckoutUrl"),
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let token = cielo_access_token().await?;
        let path = format!("/v2/orders/{}/capture", input.external_payment_id);
        let value = cielo_control_put(&path, &token).await?;

        Ok(PaymentOperationOutput {
            status: cielo_attempt_status(status_value(&value)),
            external_payment_id: Some(input.external_payment_id),
            external_status: status_value(&value).map(ToString::to_string),
            failure_code: string_field(&value, "ReturnCode"),
            failure_message: string_field(&value, "ReturnMessage"),
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let token = cielo_access_token().await?;
        let path = format!("/v2/orders/{}/void", input.external_payment_id);
        let value = cielo_control_put(&path, &token).await?;

        Ok(PaymentOperationOutput {
            status: PaymentAttemptStatus::Voided,
            external_payment_id: Some(input.external_payment_id),
            external_status: status_value(&value)
                .map(ToString::to_string)
                .or_else(|| Some("voided".to_string())),
            failure_code: string_field(&value, "ReturnCode"),
            failure_message: string_field(&value, "ReturnMessage"),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let token = cielo_access_token().await?;
        let path = format!("/v2/orders/{}/void", input.external_payment_id);
        let value = cielo_control_put(&path, &token).await?;

        Ok(RefundOutput {
            status: PaymentRefundStatus::Succeeded,
            external_refund_id: Some(input.external_payment_id),
            external_status: status_value(&value)
                .map(ToString::to_string)
                .or_else(|| Some("voided".to_string())),
            failure_code: string_field(&value, "ReturnCode"),
            failure_message: string_field(&value, "ReturnMessage"),
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Err(Error::BadRequest(
            "Cielo setup sessions require tokenization/SOP flow and are not implemented".into(),
        ))
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed.as_ref().and_then(|value| {
            string_field(value, "Status").or_else(|| string_field(value, "PaymentStatus"))
        });
        let external_event_id = parsed.as_ref().and_then(|value| {
            string_field(value, "CheckoutCieloOrderNumber")
                .or_else(|| string_field(value, "OrderNumber"))
        });

        let action = external_event_id
            .as_ref()
            .map(|id| WebhookAction::UpdatePaymentStatus {
                external_payment_id: id.to_string(),
                status: cielo_attempt_status(event_type.as_deref()),
            });

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            action,
            signature_valid: false,
            processed: false,
            ignored: false,
            failure_message: Some(
                "Cielo Checkout notifications do not provide a configured signature header"
                    .to_string(),
            ),
        })
    }
}

async fn cielo_checkout_post(path: &str, body: Value) -> Result<Value> {
    let merchant_id = env_value(CIELO_MERCHANT_ID_ENV)?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "MerchantId",
        HeaderValue::from_str(&merchant_id)
            .map_err(|err| Error::Message(format!("invalid Cielo MerchantId header: {err}")))?,
    );

    let response = reqwest::Client::new()
        .post(format!("{CIELO_CHECKOUT_BASE}{path}"))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::Message(format!("Cielo request failed: {err}")))?;
    response_json(response, "Cielo").await
}

async fn cielo_control_put(path: &str, token: &str) -> Result<Value> {
    let response = reqwest::Client::new()
        .put(format!("{CIELO_CHECKOUT_BASE}{path}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await
        .map_err(|err| Error::Message(format!("Cielo control request failed: {err}")))?;
    response_json(response, "Cielo").await
}

async fn cielo_access_token() -> Result<String> {
    let client_id = env_value(CIELO_CLIENT_ID_ENV)?;
    let client_secret = env_value(CIELO_CLIENT_SECRET_ENV)?;
    let response = reqwest::Client::new()
        .post(format!("{CIELO_CHECKOUT_BASE}/v2/token"))
        .basic_auth(client_id, Some(client_secret))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
        .map_err(|err| Error::Message(format!("Cielo token request failed: {err}")))?;
    let value = response_json(response, "Cielo token").await?;
    string_field(&value, "access_token")
        .or_else(|| string_field(&value, "AccessToken"))
        .ok_or_else(|| Error::Message("Cielo token response missing access_token".to_string()))
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

fn cielo_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("Paid" | "2" | "Captured") => PaymentAttemptStatus::Captured,
        Some("Authorized" | "1") => PaymentAttemptStatus::Authorized,
        Some("Cancelled" | "Canceled" | "10" | "11") => PaymentAttemptStatus::Cancelled,
        Some("Denied" | "NotFinished" | "Aborted" | "12" | "13") => PaymentAttemptStatus::Failed,
        _ => PaymentAttemptStatus::Processing,
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

fn provider_error_message(provider: &str, value: &Value) -> String {
    string_field(value, "Message")
        .or_else(|| string_field(value, "ReturnMessage"))
        .map(|message| format!("{provider} error: {message}"))
        .unwrap_or_else(|| format!("{provider} request failed"))
}

fn status_value(value: &Value) -> Option<&str> {
    value
        .get("Status")
        .and_then(Value::as_str)
        .or_else(|| value.get("PaymentStatus").and_then(Value::as_str))
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
    fn maps_cielo_statuses_to_local_statuses() {
        assert_eq!(
            cielo_attempt_status(Some("Paid")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            cielo_attempt_status(Some("1")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            cielo_attempt_status(Some("Denied")),
            PaymentAttemptStatus::Failed
        );
    }
}
