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

#[tokio::test]
#[serial]
async fn webhook_state_transition_works() {
    request::<App, _, _>(|request, ctx| async move {
        let now = chrono::Utc::now();
        // Setup user, order, payment method, payment
        let user = api_rust_loco::models::_entities::users::ActiveModel {
            name: Set("Test User".to_string()),
            email: Set(format!("test{}@example.com", uuid::Uuid::new_v4())),
            password: Set("password123".to_string()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let gateway = setup_webhook_gateway(&ctx, "pagarme").await;

        let method = api_rust_loco::models::_entities::payment_methods::ActiveModel {
            payment_gateway_id: Set(Some(gateway.id)),
            code: Set(Some("pagarme_cc".to_string())),
            name: Set(Some("Pagarme CC".to_string())),
            active: Set(Some(true)),
            method_type: Set(1),
            auto_capture: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let order = api_rust_loco::models::_entities::orders::ActiveModel {
            user_id: Set(user.id),
            status: Set(Some(api_rust_loco::models::order_status::OrderStatus::Pending.to_i32())),
            payment_status: Set(Some(api_rust_loco::models::order_status::PaymentStatus::Unpaid.to_i32())),
            total_amount: Set(Some(rust_decimal::Decimal::new(1000, 2))),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let payment = api_rust_loco::models::_entities::payments::ActiveModel {
            order_id: Set(order.id),
            payment_method_id: Set(method.id),
            amount: Set(Some(rust_decimal::Decimal::new(1000, 2))),
            status: Set(Some(api_rust_loco::models::payment_gateway_status::PaymentAttemptStatus::Pending.to_i16() as i32)),
            intent: Set(api_rust_loco::models::payment_gateway_status::PaymentIntent::Purchase.to_i16()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let payload = serde_json::json!({
            "type": "order.paid",
            "data": {
                "id": "or_pagarme_123",
                "status": "paid",
                "metadata": {
                    "payment_id": payment.id.to_string()
                }
            }
        });

        let res = request
            .post(&format!("/api/webhooks/payments/{}", gateway.code))
            .json(&payload)
            .await;
        
        assert_eq!(res.status_code(), 200);
        let json: Value = serde_json::from_str(&res.text()).unwrap();
        assert_eq!(json["signature_valid"], true);
        assert_eq!(json["processed"], true);
        assert_eq!(json["status"], PaymentGatewayEventStatus::Processed.to_i16() as i32);

        // Verify the payment status was updated to Captured
        let updated_payment = api_rust_loco::models::_entities::payments::Entity::find_by_id(payment.id)
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated_payment.status, Some(api_rust_loco::models::payment_gateway_status::PaymentAttemptStatus::Captured.to_i16() as i32));

        // Verify the order payment status was updated to Paid
        let updated_order = api_rust_loco::models::_entities::orders::Entity::find_by_id(order.id)
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated_order.payment_status, Some(api_rust_loco::models::order_status::PaymentStatus::Paid.to_i32()));
    })
    .await;
}
