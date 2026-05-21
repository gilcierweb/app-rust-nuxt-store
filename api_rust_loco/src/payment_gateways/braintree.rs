use async_trait::async_trait;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use serde_json::{json, Value};

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::drivers::BRAINTREE_DRIVER;
use crate::payment_gateways::types::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookAction, WebhookDecision, WebhookInput,
};

const BRAINTREE_VERSION: &str = "2019-01-01";
const BRAINTREE_PUBLIC_KEY_ENV: &str = "BRAINTREE_PUBLIC_KEY";
const BRAINTREE_PRIVATE_KEY_ENV: &str = "BRAINTREE_PRIVATE_KEY";
const BRAINTREE_ENV_ENV: &str = "BRAINTREE_ENV";
const BRAINTREE_MERCHANT_ACCOUNT_ID_ENV: &str = "BRAINTREE_MERCHANT_ACCOUNT_ID";

pub struct BraintreeGateway;

#[async_trait]
impl PaymentGateway for BraintreeGateway {
    fn driver(&self) -> &'static str {
        BRAINTREE_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let payment_method_id = payload_string(
            input.gateway_payload.as_ref(),
            &["payment_method_id", "paymentMethodId"],
        )?;
        let value = braintree_graphql(
            r#"
            mutation ChargePaymentMethod($input: ChargePaymentMethodInput!) {
              chargePaymentMethod(input: $input) {
                transaction {
                  id
                  status
                }
              }
            }
            "#,
            json!({
                "input": {
                    "paymentMethodId": payment_method_id,
                    "transaction": {
                        "amount": amount_string(input.amount),
                        "orderId": input.order_id.to_string(),
                        "options": {
                            "submitForSettlement": input.auto_capture,
                        },
                    },
                    "clientMutationId": input.idempotency_key,
                }
            }),
        )
        .await?;
        let transaction = value
            .pointer("/data/chargePaymentMethod/transaction")
            .ok_or_else(|| {
                Error::Message("Braintree charge response missing transaction".to_string())
            })?;
        let payment_status = braintree_attempt_status(status_value(transaction));

        Ok(PaymentSessionOutput {
            status: braintree_session_status(status_value(transaction)),
            payment_status,
            external_session_id: string_field(transaction, "id"),
            external_payment_id: string_field(transaction, "id"),
            external_status: status_value(transaction).map(ToString::to_string),
            external_client_secret: None,
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        let value = braintree_graphql(
            r#"
            mutation CaptureTransaction($input: CaptureTransactionInput!) {
              captureTransaction(input: $input) {
                transaction {
                  id
                  status
                }
              }
            }
            "#,
            json!({
                "input": {
                    "transactionId": input.external_payment_id,
                }
            }),
        )
        .await?;
        let transaction = value
            .pointer("/data/captureTransaction/transaction")
            .ok_or_else(|| {
                Error::Message("Braintree capture response missing transaction".to_string())
            })?;

        Ok(PaymentOperationOutput {
            status: braintree_attempt_status(status_value(transaction)),
            external_payment_id: string_field(transaction, "id"),
            external_status: status_value(transaction).map(ToString::to_string),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        let value = braintree_graphql(
            r#"
            mutation VoidTransaction($input: VoidTransactionInput!) {
              voidTransaction(input: $input) {
                transaction {
                  id
                  status
                }
                errors {
                  field
                  message
                  code
                }
              }
            }
            "#,
            json!({
                "input": {
                    "transactionId": input.external_payment_id,
                    "clientMutationId": input.idempotency_key,
                }
            }),
        )
        .await?;
        let transaction = value.pointer("/data/voidTransaction/transaction");
        let errors = value.pointer("/data/voidTransaction/errors");

        Ok(PaymentOperationOutput {
            status: transaction
                .and_then(status_value)
                .map(|status| braintree_attempt_status(Some(status)))
                .unwrap_or(PaymentAttemptStatus::Failed),
            external_payment_id: transaction.and_then(|value| string_field(value, "id")),
            external_status: transaction.and_then(status_value).map(ToString::to_string),
            failure_code: first_error_code(errors),
            failure_message: first_error_message(errors),
        })
    }

    async fn refund_payment(&self, input: RefundPaymentInput) -> Result<RefundOutput> {
        let value = braintree_graphql(
            r#"
            mutation RefundTransaction($input: RefundTransactionInput!) {
              refundTransaction(input: $input) {
                refund {
                  id
                  status
                }
              }
            }
            "#,
            json!({
                "input": {
                    "transactionId": input.external_payment_id,
                    "refund": {
                        "amount": amount_string(input.amount),
                        "orderId": format!("refund-{}", input.payment_id),
                    }
                }
            }),
        )
        .await?;
        let refund = value
            .pointer("/data/refundTransaction/refund")
            .ok_or_else(|| {
                Error::Message("Braintree refund response missing refund".to_string())
            })?;

        Ok(RefundOutput {
            status: braintree_refund_status(status_value(refund)),
            external_refund_id: string_field(refund, "id"),
            external_status: status_value(refund).map(ToString::to_string),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        let merchant_account_id = std::env::var(BRAINTREE_MERCHANT_ACCOUNT_ID_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let variables = merchant_account_id
            .map(|merchant_account_id| {
                json!({
                    "input": {
                        "clientToken": {
                            "merchantAccountId": merchant_account_id,
                        }
                    }
                })
            })
            .unwrap_or_else(|| json!({ "input": null }));
        let value = braintree_graphql(
            r#"
            mutation CreateClientToken($input: CreateClientTokenInput) {
              createClientToken(input: $input) {
                clientToken
              }
            }
            "#,
            variables,
        )
        .await?;
        let token = value
            .pointer("/data/createClientToken/clientToken")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Message("Braintree client token response missing clientToken".to_string())
            })?;

        Ok(PaymentSetupSessionOutput {
            status: PaymentSessionStatus::RequiresAction,
            external_setup_id: None,
            external_client_secret: Some(token.to_string()),
        })
    }

    async fn handle_webhook(&self, input: WebhookInput) -> Result<WebhookDecision> {
        let parsed = serde_json::from_str::<Value>(&input.payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| string_field(value, "kind").or_else(|| string_field(value, "type")));
        let external_event_id = parsed.as_ref().and_then(|value| {
            string_field(value, "id")
                .or_else(|| string_field(value, "notification_id"))
                .or_else(|| {
                    value
                        .pointer("/transaction/id")
                        .and_then(Value::as_str)
                        .map(ToString::to_string)
                })
        });

        let action = match event_type.as_deref() {
            Some("transaction_authorized") => {
                external_event_id
                    .as_ref()
                    .map(|id| WebhookAction::UpdatePaymentStatus {
                        external_payment_id: id.to_string(),
                        status: PaymentAttemptStatus::Authorized,
                    })
            }
            Some("transaction_settled") => {
                external_event_id
                    .as_ref()
                    .map(|id| WebhookAction::UpdatePaymentStatus {
                        external_payment_id: id.to_string(),
                        status: PaymentAttemptStatus::Captured,
                    })
            }
            Some("transaction_declined") => {
                external_event_id
                    .as_ref()
                    .map(|id| WebhookAction::UpdatePaymentStatus {
                        external_payment_id: id.to_string(),
                        status: PaymentAttemptStatus::Failed,
                    })
            }
            _ => None,
        };

        Ok(WebhookDecision {
            event_type,
            external_event_id,
            action,
            signature_valid: false,
            processed: false,
            ignored: false,
            failure_message: Some(
                "Braintree webhook validation requires provider signature parsing before processing"
                    .to_string(),
            ),
        })
    }
}

async fn braintree_graphql(query: &str, variables: Value) -> Result<Value> {
    let public_key = env_value(BRAINTREE_PUBLIC_KEY_ENV)?;
    let private_key = env_value(BRAINTREE_PRIVATE_KEY_ENV)?;
    let response = reqwest::Client::new()
        .post(braintree_endpoint())
        .basic_auth(public_key, Some(private_key))
        .header("Braintree-Version", BRAINTREE_VERSION)
        .json(&json!({
            "query": query,
            "variables": variables,
        }))
        .send()
        .await
        .map_err(|err| Error::Message(format!("Braintree GraphQL request failed: {err}")))?;
    let status = response.status();
    let value = response.json::<Value>().await.map_err(|err| {
        Error::Message(format!("Braintree GraphQL response JSON is invalid: {err}"))
    })?;

    if !status.is_success() {
        return Err(Error::Message(provider_error_message(&value)));
    }
    if let Some(message) = first_graphql_error_message(&value) {
        return Err(Error::Message(format!(
            "Braintree GraphQL error: {message}"
        )));
    }

    Ok(value)
}

fn braintree_endpoint() -> &'static str {
    match std::env::var(BRAINTREE_ENV_ENV)
        .unwrap_or_else(|_| "sandbox".to_string())
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "production" | "prod" | "live" => "https://payments.braintree-api.com/graphql",
        _ => "https://payments.sandbox.braintree-api.com/graphql",
    }
}

fn braintree_attempt_status(status: Option<&str>) -> PaymentAttemptStatus {
    match status {
        Some("AUTHORIZED" | "AUTHORIZING") => PaymentAttemptStatus::Authorized,
        Some("SUBMITTED_FOR_SETTLEMENT" | "SETTLING" | "SETTLED") => PaymentAttemptStatus::Captured,
        Some("VOIDED") => PaymentAttemptStatus::Voided,
        Some("PROCESSOR_DECLINED" | "GATEWAY_REJECTED" | "FAILED" | "SETTLEMENT_DECLINED") => {
            PaymentAttemptStatus::Failed
        }
        _ => PaymentAttemptStatus::Processing,
    }
}

fn braintree_refund_status(status: Option<&str>) -> PaymentRefundStatus {
    match status {
        Some("SUBMITTED_FOR_SETTLEMENT" | "SETTLING" | "SETTLED") => PaymentRefundStatus::Succeeded,
        Some("FAILED" | "SETTLEMENT_DECLINED") => PaymentRefundStatus::Failed,
        Some("VOIDED") => PaymentRefundStatus::Cancelled,
        _ => PaymentRefundStatus::Processing,
    }
}

fn braintree_session_status(status: Option<&str>) -> PaymentSessionStatus {
    match status {
        Some("AUTHORIZED" | "SUBMITTED_FOR_SETTLEMENT" | "SETTLING" | "SETTLED") => {
            PaymentSessionStatus::Completed
        }
        Some("PROCESSOR_DECLINED" | "GATEWAY_REJECTED" | "FAILED" | "SETTLEMENT_DECLINED") => {
            PaymentSessionStatus::Failed
        }
        Some("VOIDED") => PaymentSessionStatus::Cancelled,
        _ => PaymentSessionStatus::Processing,
    }
}

fn amount_string(amount: Decimal) -> String {
    amount.round_dp(2).to_string()
}

fn env_value(name: &str) -> Result<String> {
    std::env::var(name)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Message(format!("missing required environment variable: {name}")))
}

fn payload_string(payload: Option<&Value>, keys: &[&str]) -> Result<String> {
    let payload = payload
        .and_then(Value::as_object)
        .ok_or_else(|| Error::BadRequest("Braintree gateway payload is required".into()))?;

    keys.iter()
        .find_map(|key| {
            payload
                .get(*key)
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
        })
        .ok_or_else(|| {
            Error::BadRequest("Braintree gateway payload must include payment_method_id".into())
        })
}

fn first_graphql_error_message(value: &Value) -> Option<String> {
    value
        .get("errors")
        .and_then(Value::as_array)
        .and_then(|errors| errors.first())
        .and_then(|error| string_field(error, "message"))
}

fn provider_error_message(value: &Value) -> String {
    first_graphql_error_message(value).unwrap_or_else(|| "Braintree request failed".to_string())
}

fn first_error_message(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_array)
        .and_then(|errors| errors.first())
        .and_then(|error| string_field(error, "message"))
}

fn first_error_code(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_array)
        .and_then(|errors| errors.first())
        .and_then(|error| string_field(error, "code"))
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
    use serde_json::json;

    use super::*;

    #[test]
    fn maps_braintree_transaction_statuses_to_local_statuses() {
        assert_eq!(
            braintree_attempt_status(Some("AUTHORIZED")),
            PaymentAttemptStatus::Authorized
        );
        assert_eq!(
            braintree_attempt_status(Some("SUBMITTED_FOR_SETTLEMENT")),
            PaymentAttemptStatus::Captured
        );
        assert_eq!(
            braintree_attempt_status(Some("VOIDED")),
            PaymentAttemptStatus::Voided
        );
        assert_eq!(
            braintree_attempt_status(Some("PROCESSOR_DECLINED")),
            PaymentAttemptStatus::Failed
        );
    }

    #[test]
    fn uses_sandbox_endpoint_by_default() {
        std::env::remove_var(BRAINTREE_ENV_ENV);
        assert_eq!(
            braintree_endpoint(),
            "https://payments.sandbox.braintree-api.com/graphql"
        );
    }

    #[test]
    fn maps_braintree_session_statuses_to_local_statuses() {
        assert_eq!(
            braintree_session_status(Some("AUTHORIZED")),
            PaymentSessionStatus::Completed
        );
        assert_eq!(
            braintree_session_status(Some("SETTLED")),
            PaymentSessionStatus::Completed
        );
        assert_eq!(
            braintree_session_status(Some("PROCESSOR_DECLINED")),
            PaymentSessionStatus::Failed
        );
        assert_eq!(
            braintree_session_status(Some("VOIDED")),
            PaymentSessionStatus::Cancelled
        );
    }

    #[test]
    fn reads_payment_method_id_from_gateway_payload() {
        let payload = json!({
            "payment_method_id": "  pm_braintree_123  ",
        });

        assert_eq!(
            payload_string(Some(&payload), &["payment_method_id"]).unwrap(),
            "pm_braintree_123"
        );
    }

    #[test]
    fn accepts_camel_case_payment_method_id_alias() {
        let payload = json!({
            "paymentMethodId": "pm_braintree_456",
        });

        assert_eq!(
            payload_string(Some(&payload), &["payment_method_id", "paymentMethodId"]).unwrap(),
            "pm_braintree_456"
        );
    }

    #[test]
    fn rejects_missing_payment_method_payload() {
        assert!(payload_string(None, &["payment_method_id"]).is_err());
        assert!(payload_string(
            Some(&json!({"payment_method_id": ""})),
            &["payment_method_id"]
        )
        .is_err());
    }
}
