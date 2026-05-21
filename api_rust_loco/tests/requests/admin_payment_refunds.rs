use api_rust_loco::{
    app::App,
    models::_entities::{orders, payment_gateways, payment_methods, payment_refunds, payments},
    models::order_status::OrderStatus,
    models::payment_gateway_status::{PaymentAttemptStatus, PaymentRefundStatus},
    models::users,
    payment_gateways::drivers::MANUAL_DRIVER,
};
use loco_rs::testing::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde_json::Value;
use serial_test::serial;

use super::prepare_data;
use super::with_api_key;

async fn login_admin(request: &loco_rs::TestServer) -> String {
    let csrf = prepare_data::init_csrf(request).await;
    let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

    let response = request
        .post("/api/auth/login")
        .add_header(csrf_key, csrf_value)
        .save_cookies()
        .json(&serde_json::json!({
            "email": "admin@example.com",
            "password": "Password123!"
        }))
        .await;

    assert_eq!(response.status_code(), 200);
    csrf
}

async fn create_refund_fixture(ctx: &loco_rs::app::AppContext) -> payment_refunds::Model {
    let now = chrono::Utc::now();
    let admin = users::Model::find_by_email(&ctx.db, "admin@example.com")
        .await
        .unwrap();
    let gateway = payment_gateways::ActiveModel {
        code: Set("manual_refund_test".to_string()),
        name: Set("Manual Refund Test".to_string()),
        driver: Set(MANUAL_DRIVER.to_string()),
        environment: Set(1),
        status: Set(1),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    let method = payment_methods::ActiveModel {
        payment_gateway_id: Set(Some(gateway.id)),
        code: Set(Some("manual_pix".to_string())),
        name: Set(Some("Manual Pix".to_string())),
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

    let order = orders::ActiveModel {
        user_id: Set(admin.id),
        status: Set(Some(OrderStatus::Pending.to_i32())),
        total_amount: Set(Some(Decimal::new(1500, 2))),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    let payment = payments::ActiveModel {
        order_id: Set(order.id),
        payment_method_id: Set(method.id),
        amount: Set(Some(Decimal::new(1500, 2))),
        currency: Set(Some("BRL".to_string())),
        status: Set(Some(PaymentAttemptStatus::Refunded.to_i16() as i32)),
        number: Set(Some("PAY-REFUND-001".to_string())),
        external_payment_id: Set(Some("pay_ext_ref_001".to_string())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    payment_refunds::ActiveModel {
        payment_id: Set(payment.id),
        amount: Set(Decimal::new(500, 2)),
        currency: Set("BRL".to_string()),
        status: Set(PaymentRefundStatus::Succeeded.to_i16()),
        reason: Set(Some(1)),
        external_refund_id: Set(Some("ref_001".to_string())),
        idempotency_key: Set("refund_test_key_001".to_string()),
        processed_at: Set(Some(now.naive_utc())),
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
async fn admin_can_list_payment_refunds() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let _csrf = login_admin(&request).await;
        create_refund_fixture(&ctx).await;

        let response = request.get("/api/admin/payment-refunds").await;
        let body = response.text();
        assert_eq!(response.status_code(), 200, "unexpected body: {body}");

        let items: Vec<Value> =
            serde_json::from_str(&body).unwrap_or_else(|error| panic!("{error}: {body}"));
        assert!(!items.is_empty());
        assert_eq!(items[0]["payment_number"], "PAY-REFUND-001");
        assert_eq!(items[0]["gateway_name"], "Manual Refund Test");
        assert_eq!(items[0]["refund"]["external_refund_id"], "ref_001");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_get_payment_refund_detail() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let _csrf = login_admin(&request).await;
        let refund = create_refund_fixture(&ctx).await;

        let response = request
            .get(&format!("/api/admin/payment-refunds/{}", refund.id))
            .await;
        let body = response.text();
        assert_eq!(response.status_code(), 200, "unexpected body: {body}");

        let item: Value =
            serde_json::from_str(&body).unwrap_or_else(|error| panic!("{error}: {body}"));
        assert_eq!(item["refund"]["id"], refund.id);
        assert_eq!(item["payment"]["number"], "PAY-REFUND-001");
        assert_eq!(item["gateway_name"], "Manual Refund Test");
    })
    .await;
}
