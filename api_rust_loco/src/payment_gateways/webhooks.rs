use loco_rs::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::models::_entities::payment_gateway_events;
use crate::models::_entities::payment_gateway_logs;
use crate::models::_entities::payment_gateways;
use crate::models::_entities::{orders, payment_methods, payments};
use crate::models::payment_gateway_status::PaymentGatewayEventStatus;
use crate::payment_gateways::registry::gateway_for_driver;
use crate::payment_gateways::session_response::order_payment_status;
use crate::payment_gateways::types::{WebhookAction, WebhookDecision, WebhookInput};

const LOG_DIRECTION_WEBHOOK: i16 = 3;
const LOG_LEVEL_INFO: i16 = 1;
const LOG_LEVEL_WARN: i16 = 2;
const LOG_LEVEL_ERROR: i16 = 3;

pub struct ReceiveWebhookInput {
    pub gateway_code: String,
    pub headers: Vec<(String, String)>,
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct ReceiveWebhookOutput {
    pub event_id: i32,
    pub gateway_code: String,
    pub external_event_id: String,
    pub event_type: String,
    pub status: i16,
    pub duplicate: bool,
    pub signature_valid: bool,
    pub processed: bool,
    pub ignored: bool,
    pub failure_message: Option<String>,
}

pub async fn receive_webhook<C>(db: &C, input: ReceiveWebhookInput) -> Result<ReceiveWebhookOutput>
where
    C: ConnectionTrait,
{
    let gateway = payment_gateways::Entity::find()
        .filter(payment_gateways::Column::Code.eq(input.gateway_code.clone()))
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let payload_info = PayloadInfo::from_payload(&input.payload);

    if let Some(existing) = payment_gateway_events::Entity::find()
        .filter(payment_gateway_events::Column::PaymentGatewayId.eq(gateway.id))
        .filter(
            payment_gateway_events::Column::ExternalEventId
                .eq(payload_info.external_event_id.clone()),
        )
        .one(db)
        .await?
    {
        return Ok(ReceiveWebhookOutput {
            event_id: existing.id,
            gateway_code: gateway.code,
            external_event_id: existing.external_event_id,
            event_type: existing.event_type,
            status: existing.status,
            duplicate: true,
            signature_valid: existing.signature_valid,
            processed: false,
            ignored: true,
            failure_message: existing.failure_message,
        });
    }

    let now = chrono::Utc::now();
    let event = payment_gateway_events::ActiveModel {
        payment_gateway_id: Set(gateway.id),
        event_type: Set(payload_info.event_type.clone()),
        external_event_id: Set(payload_info.external_event_id.clone()),
        status: Set(PaymentGatewayEventStatus::Received.to_i16()),
        signature_valid: Set(false),
        payload: Set(input.payload.clone()),
        processed_at: Set(None),
        failure_message: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    let mut gateway_result = match gateway_for_driver(&gateway.driver) {
        Ok(gateway_driver) => gateway_driver
            .handle_webhook(WebhookInput {
                gateway_code: gateway.code.clone(),
                headers: input.headers,
                payload: input.payload,
            })
            .await
            .map_err(|err| err.to_string()),
        Err(err) => Err(err.to_string()),
    };

    if let Ok(ref mut decision) = gateway_result {
        if decision.signature_valid {
            if let Some(action) = &decision.action {
                match apply_webhook_action(db, gateway.id, action).await {
                    Ok(_) => {
                        decision.processed = true;
                    }
                    Err(e) => {
                        decision.processed = false;
                        decision.failure_message = Some(format!("failed to apply webhook action: {}", e));
                    }
                }
            } else {
                // No action implies ignored
                decision.processed = false;
                decision.ignored = true;
            }
        }
    }

    let (status, signature_valid, processed, ignored, failure_message) = match gateway_result {
        Ok(decision) if !decision.signature_valid => (
            PaymentGatewayEventStatus::Failed,
            false,
            false,
            false,
            decision
                .failure_message
                .or_else(|| Some("webhook signature is invalid".to_string())),
        ),
        Ok(decision) if decision.ignored => (
            PaymentGatewayEventStatus::Ignored,
            decision.signature_valid,
            false,
            true,
            decision.failure_message,
        ),
        Ok(decision) if decision.processed => (
            PaymentGatewayEventStatus::Processed,
            decision.signature_valid,
            true,
            false,
            decision.failure_message,
        ),
        Ok(decision) => (
            PaymentGatewayEventStatus::Received,
            decision.signature_valid,
            false,
            false,
            decision.failure_message,
        ),
        Err(message) => (
            PaymentGatewayEventStatus::Failed,
            false,
            false,
            false,
            Some(message),
        ),
    };

    let terminal = matches!(
        status,
        PaymentGatewayEventStatus::Processed
            | PaymentGatewayEventStatus::Failed
            | PaymentGatewayEventStatus::Ignored
    );
    let processed_at = terminal.then(|| now.naive_utc());

    let mut event_update: payment_gateway_events::ActiveModel = event.clone().into();
    event_update.status = Set(status.to_i16());
    event_update.signature_valid = Set(signature_valid);
    event_update.processed_at = Set(processed_at);
    event_update.failure_message = Set(failure_message.clone());
    event_update.updated_at = Set(now.into());
    let event = event_update.update(db).await?;

    insert_webhook_log(db, event.id, status, failure_message.clone()).await?;

    Ok(ReceiveWebhookOutput {
        event_id: event.id,
        gateway_code: gateway.code,
        external_event_id: event.external_event_id,
        event_type: event.event_type,
        status: event.status,
        duplicate: false,
        signature_valid,
        processed,
        ignored,
        failure_message,
    })
}

async fn insert_webhook_log<C>(
    db: &C,
    event_id: i32,
    status: PaymentGatewayEventStatus,
    failure_message: Option<String>,
) -> Result<()>
where
    C: ConnectionTrait,
{
    let level = match status {
        PaymentGatewayEventStatus::Processed | PaymentGatewayEventStatus::Ignored => LOG_LEVEL_INFO,
        PaymentGatewayEventStatus::Failed => LOG_LEVEL_ERROR,
        _ => LOG_LEVEL_WARN,
    };

    payment_gateway_logs::ActiveModel {
        payment_id: Set(None),
        payment_session_id: Set(None),
        payment_gateway_event_id: Set(Some(event_id)),
        direction: Set(LOG_DIRECTION_WEBHOOK),
        level: Set(level),
        message: Set(Some(status.name().to_string())),
        payload: Set(failure_message),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}

pub async fn apply_webhook_action<C>(
    db: &C,
    gateway_id: i32,
    action: &WebhookAction,
) -> std::result::Result<(), String>
where
    C: ConnectionTrait,
{
    match action {
        WebhookAction::UpdatePaymentStatus { external_payment_id, status } => {
            let payment_opt = payments::Entity::find()
                .filter(payments::Column::ExternalPaymentId.eq(external_payment_id.clone()))
                .inner_join(payment_methods::Entity)
                .filter(payment_methods::Column::PaymentGatewayId.eq(gateway_id))
                .one(db)
                .await
                .map_err(|e| e.to_string())?;

            if let Some(payment) = payment_opt {
                let mut active_payment: payments::ActiveModel = payment.clone().into();
                active_payment.status = Set(Some(status.to_i16() as i32));
                let now = chrono::Utc::now().into();
                active_payment.updated_at = Set(now);
                active_payment.update(db).await.map_err(|e| e.to_string())?;

                if let Some(order) = orders::Entity::find_by_id(payment.order_id).one(db).await.map_err(|e| e.to_string())? {
                    let mut active_order: orders::ActiveModel = order.into();
                    active_order.payment_status = Set(Some(order_payment_status(Some(status.to_i16() as i32)).to_i32()));
                    active_order.updated_at = Set(now);
                    active_order.update(db).await.map_err(|e| e.to_string())?;
                }
            } else {
                return Err(format!("payment with external id {} not found", external_payment_id));
            }
            Ok(())
        }
        WebhookAction::UpdatePaymentStatusById { payment_id, status } => {
            let payment_opt = payments::Entity::find_by_id(*payment_id)
                .inner_join(payment_methods::Entity)
                .filter(payment_methods::Column::PaymentGatewayId.eq(gateway_id))
                .one(db)
                .await
                .map_err(|e| e.to_string())?;

            if let Some(payment) = payment_opt {
                let mut active_payment: payments::ActiveModel = payment.clone().into();
                active_payment.status = Set(Some(status.to_i16() as i32));
                let now = chrono::Utc::now().into();
                active_payment.updated_at = Set(now);
                active_payment.update(db).await.map_err(|e| e.to_string())?;

                if let Some(order) = orders::Entity::find_by_id(payment.order_id).one(db).await.map_err(|e| e.to_string())? {
                    let mut active_order: orders::ActiveModel = order.into();
                    active_order.payment_status = Set(Some(order_payment_status(Some(status.to_i16() as i32)).to_i32()));
                    active_order.updated_at = Set(now);
                    active_order.update(db).await.map_err(|e| e.to_string())?;
                }
            } else {
                return Err(format!("payment with id {} not found", payment_id));
            }
            Ok(())
        }
    }
}

struct PayloadInfo {
    event_type: String,
    external_event_id: String,
}

impl PayloadInfo {
    fn from_payload(payload: &str) -> Self {
        let parsed = serde_json::from_str::<Value>(payload).ok();
        let event_type = parsed
            .as_ref()
            .and_then(|value| first_string(value, &["type", "event_type", "action", "topic"]))
            .unwrap_or_else(|| "unknown".to_string());
        let external_event_id = parsed
            .as_ref()
            .and_then(external_event_id)
            .unwrap_or_else(|| format!("missing-event-id-{}", Uuid::new_v4()));

        Self {
            event_type,
            external_event_id,
        }
    }
}

fn external_event_id(value: &Value) -> Option<String> {
    first_string(value, &["id", "event_id"])
        .or_else(|| value.pointer("/data/id").and_then(value_to_string))
        .or_else(|| value.pointer("/resource/id").and_then(value_to_string))
}

fn first_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|key| value.get(*key).and_then(value_to_string))
        .next()
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
    use super::PayloadInfo;

    #[test]
    fn extracts_top_level_event_id_and_type() {
        let info = PayloadInfo::from_payload(r#"{"id":"evt_123","type":"payment.succeeded"}"#);

        assert_eq!(info.external_event_id, "evt_123");
        assert_eq!(info.event_type, "payment.succeeded");
    }

    #[test]
    fn extracts_nested_data_id_and_event_type() {
        let info = PayloadInfo::from_payload(
            r#"{"event_type":"charge.updated","data":{"id":"charge_123"}}"#,
        );

        assert_eq!(info.external_event_id, "charge_123");
        assert_eq!(info.event_type, "charge.updated");
    }

    #[test]
    fn generates_missing_event_id_for_invalid_payload() {
        let info = PayloadInfo::from_payload("not-json");

        assert!(info.external_event_id.starts_with("missing-event-id-"));
        assert_eq!(info.event_type, "unknown");
    }
}
