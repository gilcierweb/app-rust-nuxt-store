use api_rust_loco::models::_entities::{payment_gateway_events, payment_gateways};
use api_rust_loco::models::payment_gateway_status::PaymentGatewayEventStatus;
use api_rust_loco::{
    app::App,
    workers::payment_webhook_retry::{Worker, WorkerArgs},
};
use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_payment_webhook_retry_worker() {
    let boot = boot_test::<App>().await.unwrap();
    let ctx = &boot.app_context;
    let now = chrono::Utc::now();

    // 1. Setup User, Gateway, Method, Order, Payment
    let user = api_rust_loco::models::_entities::users::ActiveModel {
        name: Set("Test User".to_string()),
        email: Set(format!("test{}@example.com", uuid::Uuid::new_v4())),
        password: Set("password123".to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    let gateway = payment_gateways::ActiveModel {
        code: Set("pagarme_test_worker".to_string()),
        name: Set("Pagarme Test".to_string()),
        driver: Set("pagarme".to_string()),
        environment: Set(1),
        status: Set(1),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

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
        status: Set(Some(
            api_rust_loco::models::order_status::OrderStatus::Pending.to_i32(),
        )),
        payment_status: Set(Some(
            api_rust_loco::models::order_status::PaymentStatus::Unpaid.to_i32(),
        )),
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
        status: Set(Some(
            api_rust_loco::models::payment_gateway_status::PaymentAttemptStatus::Pending.to_i16()
                as i32,
        )),
        intent: Set(
            api_rust_loco::models::payment_gateway_status::PaymentIntent::Purchase.to_i16(),
        ),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    // 2. Insert a failed Webhook Event
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

    let event = payment_gateway_events::ActiveModel {
        payment_gateway_id: Set(gateway.id),
        event_type: Set("order.paid".to_string()),
        external_event_id: Set("or_pagarme_123".to_string()),
        status: Set(PaymentGatewayEventStatus::Failed.to_i16()),
        signature_valid: Set(true),
        payload: Set(payload.to_string()),
        processed_at: Set(None),
        failure_message: Set(Some("Temporary DB lock error".to_string())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    // 3. Run the worker
    assert!(Worker::perform_later(&boot.app_context, WorkerArgs {})
        .await
        .is_ok());

    // 4. Validate results
    let updated_event = payment_gateway_events::Entity::find_by_id(event.id)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        updated_event.status,
        PaymentGatewayEventStatus::Processed.to_i16()
    );
    assert!(updated_event.processed_at.is_some());

    let updated_payment =
        api_rust_loco::models::_entities::payments::Entity::find_by_id(payment.id)
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
    assert_eq!(
        updated_payment.status,
        Some(
            api_rust_loco::models::payment_gateway_status::PaymentAttemptStatus::Captured.to_i16()
                as i32
        )
    );

    let updated_order = api_rust_loco::models::_entities::orders::Entity::find_by_id(order.id)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        updated_order.payment_status,
        Some(api_rust_loco::models::order_status::PaymentStatus::Paid.to_i32())
    );
}
