use api_rust_loco::app::App;
use api_rust_loco::models::_entities::{payment_gateways, payment_gateway_events};
use api_rust_loco::models::payment_gateway_status::PaymentGatewayEventStatus;
use api_rust_loco::payment_gateways::drivers::MANUAL_DRIVER;
use loco_rs::app::AppContext;
use loco_rs::testing::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};
use serial_test::serial;
use serde_json::Value;

async fn setup_webhook_gateway(ctx: &AppContext, driver: &str) -> payment_gateways::Model {
    let now = chrono::Utc::now();
    payment_gateways::ActiveModel {
        code: Set(format!("{}_test", driver)),
        name: Set(format!("{} Test", driver)),
        driver: Set(driver.to_string()),
        environment: Set(1),
        status: Set(1),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap()
}

#[tokio::test]
#[serial]
async fn can_get_payment_webhooks() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/webhooks/payments/").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn unknown_gateway_webhook_returns_not_found() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/api/webhooks/payments/unknown")
            .json(&serde_json::json!({
                "id": "evt_unknown",
                "type": "payment.updated"
            }))
            .await;
        assert_eq!(res.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn webhook_deduplication_works() {
    request::<App, _, _>(|request, ctx| async move {
        let gateway = setup_webhook_gateway(&ctx, MANUAL_DRIVER).await;

        let payload = serde_json::json!({
            "id": "evt_dedup_123",
            "type": "payment.succeeded"
        });

        // First attempt
        let res1 = request
            .post(&format!("/api/webhooks/payments/{}", gateway.code))
            .json(&payload)
            .await;
        assert_eq!(res1.status_code(), 200);
        let json1: Value = serde_json::from_str(&res1.text()).unwrap();
        assert_eq!(json1["duplicate"], false);
        assert_eq!(json1["processed"], false); // Manual driver ignores by default
        assert_eq!(json1["ignored"], true);

        // Second attempt
        let res2 = request
            .post(&format!("/api/webhooks/payments/{}", gateway.code))
            .json(&payload)
            .await;
        assert_eq!(res2.status_code(), 200);
        let json2: Value = serde_json::from_str(&res2.text()).unwrap();
        assert_eq!(json2["duplicate"], true);

        // Verify only one event was created in the DB
        let events = payment_gateway_events::Entity::find()
            .filter(payment_gateway_events::Column::PaymentGatewayId.eq(gateway.id))
            .all(&ctx.db)
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn webhook_invalid_signature_fails() {
    request::<App, _, _>(|request, ctx| async move {
        // We use stripe since the stripe driver actually validates signatures
        // The manual driver always returns valid=true.
        let gateway = setup_webhook_gateway(&ctx, "stripe").await;

        let payload = serde_json::json!({
            "id": "evt_fail_123",
            "type": "payment.succeeded"
        });

        let res = request
            .post(&format!("/api/webhooks/payments/{}", gateway.code))
            // Missing stripe signature header
            .json(&payload)
            .await;
        
        assert_eq!(res.status_code(), 200);
        let json: Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(json["signature_valid"], false);
        assert_eq!(json["status"], PaymentGatewayEventStatus::Failed.to_i16() as i32);

        let events = payment_gateway_events::Entity::find()
            .filter(payment_gateway_events::Column::PaymentGatewayId.eq(gateway.id))
            .all(&ctx.db)
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].status, PaymentGatewayEventStatus::Failed.to_i16());
        assert_eq!(events[0].signature_valid, false);
    })
    .await;
}
