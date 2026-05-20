use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{payment_gateway_events, payment_gateways};
use crate::models::payment_gateway_status::PaymentGatewayEventStatus;
use crate::payment_gateways::registry::gateway_for_driver;
use crate::payment_gateways::types::WebhookInput;
use crate::payment_gateways::webhooks::apply_webhook_action;

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, _args: WorkerArgs) -> Result<()> {
        println!("=================PaymentWebhookRetry=======================");
        let db = &self.ctx.db;

        // Find failed events that have valid signatures and haven't been processed
        let failed_events = payment_gateway_events::Entity::find()
            .filter(payment_gateway_events::Column::Status.eq(PaymentGatewayEventStatus::Failed.to_i16()))
            .filter(payment_gateway_events::Column::SignatureValid.eq(true))
            .filter(payment_gateway_events::Column::ProcessedAt.is_null())
            .all(db)
            .await?;

        for event in failed_events {
            println!("Retrying webhook event {}...", event.id);
            let event_id = event.id;
            if let Ok(Some(gateway)) = payment_gateways::Entity::find_by_id(event.payment_gateway_id).one(db).await {
                if let Ok(gateway_driver) = gateway_for_driver(&gateway.driver) {
                    // Re-parse the payload by calling handle_webhook with empty headers.
                    // This will fail signature validation, but will still extract the action.
                    let input = WebhookInput {
                        gateway_code: gateway.code.clone(),
                        headers: vec![], // Empty headers since we skip validation on retry
                        payload: event.payload.clone(),
                    };

                    if let Ok(decision) = gateway_driver.handle_webhook(input).await {
                        if let Some(action) = &decision.action {
                            match apply_webhook_action(db, gateway.id, action).await {
                                Ok(_) => {
                                    let mut active_event: payment_gateway_events::ActiveModel = event.into();
                                    active_event.status = Set(PaymentGatewayEventStatus::Processed.to_i16());
                                    active_event.processed_at = Set(Some(chrono::Utc::now().naive_utc()));
                                    active_event.failure_message = Set(None);
                                    active_event.updated_at = Set(chrono::Utc::now().into());
                                    if let Err(e) = active_event.update(db).await {
                                        tracing::error!("Failed to update event {} status after retry: {}", event_id, e);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Retry for event {} failed to apply action: {}", event_id, e);
                                    let mut active_event: payment_gateway_events::ActiveModel = event.into();
                                    active_event.failure_message = Set(Some(format!("Retry failed: {}", e)));
                                    active_event.updated_at = Set(chrono::Utc::now().into());
                                    let _ = active_event.update(db).await;
                                }
                            }
                        } else {
                            // No action, mark ignored
                            let mut active_event: payment_gateway_events::ActiveModel = event.into();
                            active_event.status = Set(PaymentGatewayEventStatus::Ignored.to_i16());
                            active_event.processed_at = Set(Some(chrono::Utc::now().naive_utc()));
                            active_event.updated_at = Set(chrono::Utc::now().into());
                            let _ = active_event.update(db).await;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
