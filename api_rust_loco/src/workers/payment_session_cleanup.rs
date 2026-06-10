use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::models::_entities::{
    payment_gateway_events, payment_gateway_logs, payment_sessions, payment_setup_sessions,
    payments,
};
use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentGatewayEventStatus, PaymentSessionStatus,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct WorkerArgs {}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, _args: WorkerArgs) -> Result<()> {
        tracing::info!("running payment session cleanup worker");
        let db = &self.ctx.db;
        let now = chrono::Utc::now();
        let mut expired_sessions: u64 = 0;
        let mut expired_setup_sessions: u64 = 0;
        let mut timed_out_payments: u64 = 0;
        let mut purged_events: u64 = 0;
        let mut purged_logs: u64 = 0;

        // 1. Expire stale payment_sessions (Pending/Processing/RequiresAction past expires_at)
        let stale_sessions = payment_sessions::Entity::find()
            .filter(
                payment_sessions::Column::Status
                    .is_in([
                        PaymentSessionStatus::Pending.to_i16(),
                        PaymentSessionStatus::Processing.to_i16(),
                        PaymentSessionStatus::RequiresAction.to_i16(),
                    ]),
            )
            .filter(payment_sessions::Column::ExpiresAt.lt(now.naive_utc()))
            .filter(payment_sessions::Column::ExpiresAt.is_not_null())
            .all(db)
            .await?;

        for session in stale_sessions {
            let mut active: payment_sessions::ActiveModel = session.into();
            active.status = Set(PaymentSessionStatus::Expired.to_i16());
            active.updated_at = Set(now.into());
            if let Err(e) = active.update(db).await {
                tracing::error!("failed to expire payment session: {}", e);
            } else {
                expired_sessions += 1;
            }
        }

        // 2. Expire stale payment_setup_sessions
        let stale_setup_sessions = payment_setup_sessions::Entity::find()
            .filter(
                payment_setup_sessions::Column::Status
                    .is_in([
                        PaymentSessionStatus::Pending.to_i16(),
                        PaymentSessionStatus::Processing.to_i16(),
                        PaymentSessionStatus::RequiresAction.to_i16(),
                    ]),
            )
            .filter(payment_setup_sessions::Column::ExpiresAt.lt(now.naive_utc()))
            .filter(payment_setup_sessions::Column::ExpiresAt.is_not_null())
            .all(db)
            .await?;

        for session in stale_setup_sessions {
            let mut active: payment_setup_sessions::ActiveModel = session.into();
            active.status = Set(PaymentSessionStatus::Expired.to_i16());
            active.updated_at = Set(now.into());
            if let Err(e) = active.update(db).await {
                tracing::error!("failed to expire payment setup session: {}", e);
            } else {
                expired_setup_sessions += 1;
            }
        }

        // 3. Mark payments stuck in Pending/Processing/Checkout for > 24h as Failed
        let stale_cutoff = now - chrono::Duration::hours(24);
        let stale_payments = payments::Entity::find()
            .filter(
                payments::Column::Status
                    .is_in([
                        PaymentAttemptStatus::Pending.to_i16() as i32,
                        PaymentAttemptStatus::Processing.to_i16() as i32,
                        PaymentAttemptStatus::Checkout.to_i16() as i32,
                    ]),
            )
            .filter(payments::Column::CreatedAt.lt(stale_cutoff.naive_utc()))
            .all(db)
            .await?;

        for payment in stale_payments {
            let payment_id = payment.id;
            let mut active: payments::ActiveModel = payment.into();
            active.status = Set(Some(PaymentAttemptStatus::Failed.to_i16() as i32));
            active.failure_code = Set(Some("SESSION_TIMEOUT".to_string()));
            active.failure_message = Set(Some(
                "Payment timed out after 24 hours without completion".to_string(),
            ));
            active.updated_at = Set(now.into());
            if let Err(e) = active.update(db).await {
                tracing::error!("failed to mark payment {} as timed out: {}", payment_id, e);
            } else {
                timed_out_payments += 1;
            }
        }

        // 4. Purge old processed/failed/ignored gateway events (older than 30 days)
        let events_cutoff = now - chrono::Duration::days(30);
        let old_events = payment_gateway_events::Entity::find()
            .filter(
                payment_gateway_events::Column::Status
                    .is_in([
                        PaymentGatewayEventStatus::Processed.to_i16(),
                        PaymentGatewayEventStatus::Failed.to_i16(),
                        PaymentGatewayEventStatus::Ignored.to_i16(),
                    ]),
            )
            .filter(payment_gateway_events::Column::CreatedAt.lt(events_cutoff.naive_utc()))
            .all(db)
            .await?;

        let event_ids: Vec<i32> = old_events.iter().map(|e| e.id).collect();
        if !event_ids.is_empty() {
            let delete_result = payment_gateway_events::Entity::delete_many()
                .filter(payment_gateway_events::Column::Id.is_in(event_ids.clone()))
                .exec(db)
                .await;
            match delete_result {
                Ok(res) => {
                    purged_events = res.rows_affected;
                }
                Err(e) => {
                    tracing::error!("failed to purge old gateway events: {}", e);
                }
            }
        }

        // 5. Purge old gateway logs (older than 30 days)
        let old_logs = payment_gateway_logs::Entity::find()
            .filter(payment_gateway_logs::Column::CreatedAt.lt(events_cutoff.naive_utc()))
            .all(db)
            .await?;

        let log_ids: Vec<i32> = old_logs.iter().map(|l| l.id).collect();
        if !log_ids.is_empty() {
            let delete_result = payment_gateway_logs::Entity::delete_many()
                .filter(payment_gateway_logs::Column::Id.is_in(log_ids.clone()))
                .exec(db)
                .await;
            match delete_result {
                Ok(res) => {
                    purged_logs = res.rows_affected;
                }
                Err(e) => {
                    tracing::error!("failed to purge old gateway logs: {}", e);
                }
            }
        }

        tracing::info!(
            expired_sessions,
            expired_setup_sessions,
            timed_out_payments,
            purged_events,
            purged_logs,
            "payment session cleanup completed"
        );

        Ok(())
    }
}
