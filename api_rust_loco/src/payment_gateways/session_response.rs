use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};
use serde_json::Value;

use crate::models::_entities::payment_sessions;
use crate::models::order_status::PaymentStatus;
use crate::models::payment_gateway_status::PaymentAttemptStatus;

pub async fn latest_payment_session_json<C>(db: &C, payment_id: i32) -> Result<Option<Value>>
where
    C: ConnectionTrait,
{
    let session = payment_sessions::Entity::find()
        .filter(payment_sessions::Column::PaymentId.eq(payment_id))
        .order_by_desc(payment_sessions::Column::CreatedAt)
        .one(db)
        .await?;

    Ok(session.map(payment_session_json))
}

pub fn order_payment_status(status: Option<i32>) -> PaymentStatus {
    match status
        .and_then(|value| i16::try_from(value).ok())
        .and_then(PaymentAttemptStatus::from_i16)
    {
        Some(PaymentAttemptStatus::Captured) => PaymentStatus::Paid,
        Some(PaymentAttemptStatus::Refunded) => PaymentStatus::Refunded,
        Some(PaymentAttemptStatus::PartiallyRefunded) => PaymentStatus::PartiallyRefunded,
        _ => PaymentStatus::Unpaid,
    }
}

fn payment_session_json(session: payment_sessions::Model) -> Value {
    let action_url = session
        .external_client_secret
        .as_deref()
        .filter(|value| value.starts_with("http://") || value.starts_with("https://"))
        .map(ToString::to_string);
    let requires_action = action_url.is_some()
        || session.status
            == crate::models::payment_gateway_status::PaymentSessionStatus::RequiresAction.to_i16();

    serde_json::json!({
        "id": session.id,
        "payment_id": session.payment_id,
        "payment_method_id": session.payment_method_id,
        "status": session.status,
        "external_session_id": session.external_session_id,
        "external_client_secret": session.external_client_secret,
        "action_url": action_url,
        "requires_action": requires_action,
    })
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::models::payment_gateway_status::PaymentSessionStatus;

    #[test]
    fn maps_only_captured_and_refunded_attempts_to_paid_order_states() {
        assert_eq!(
            order_payment_status(Some(PaymentAttemptStatus::Captured.to_i16() as i32)),
            PaymentStatus::Paid
        );
        assert_eq!(
            order_payment_status(Some(PaymentAttemptStatus::Refunded.to_i16() as i32)),
            PaymentStatus::Refunded
        );
        assert_eq!(
            order_payment_status(Some(PaymentAttemptStatus::PartiallyRefunded.to_i16() as i32)),
            PaymentStatus::PartiallyRefunded
        );
        assert_eq!(
            order_payment_status(Some(PaymentAttemptStatus::Authorized.to_i16() as i32)),
            PaymentStatus::Unpaid
        );
        assert_eq!(
            order_payment_status(Some(PaymentAttemptStatus::Processing.to_i16() as i32)),
            PaymentStatus::Unpaid
        );
    }

    #[test]
    fn exposes_hosted_checkout_url_as_action_url() {
        let value = payment_session_json(payment_sessions::Model {
            id: 1,
            payment_id: 2,
            payment_method_id: 3,
            status: PaymentSessionStatus::RequiresAction.to_i16(),
            external_session_id: Some("session-1".to_string()),
            external_client_secret: Some("https://checkout.example/pay/session-1".to_string()),
            expires_at: None,
            completed_at: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        });

        assert_eq!(
            value["action_url"],
            "https://checkout.example/pay/session-1"
        );
        assert_eq!(value["requires_action"], true);
    }

    #[test]
    fn keeps_sdk_client_secret_out_of_action_url() {
        let value = payment_session_json(payment_sessions::Model {
            id: 1,
            payment_id: 2,
            payment_method_id: 3,
            status: PaymentSessionStatus::RequiresAction.to_i16(),
            external_session_id: Some("pi_123".to_string()),
            external_client_secret: Some("pi_123_secret_456".to_string()),
            expires_at: None,
            completed_at: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        });

        assert!(value["action_url"].is_null());
        assert_eq!(value["external_client_secret"], "pi_123_secret_456");
        assert_eq!(value["requires_action"], true);
    }
}
