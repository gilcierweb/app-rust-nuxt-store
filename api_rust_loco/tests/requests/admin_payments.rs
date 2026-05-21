use api_rust_loco::{
    app::App,
    models::_entities::{orders, payment_gateways, payment_methods, payments, users},
    models::order_status::OrderStatus,
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

async fn create_payment_fixture(
    ctx: &loco_rs::app::AppContext,
    suffix: &str,
    gateway_name: &str,
    currency: &str,
    status: i32,
) -> payments::Model {
    let now = chrono::Utc::now();
    let admin = users::Model::find_by_email(&ctx.db, "admin@example.com")
        .await
        .unwrap();

    let gateway = payment_gateways::ActiveModel {
        code: Set(format!("gateway_{suffix}")),
        name: Set(gateway_name.to_string()),
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
        code: Set(Some(format!("method_{suffix}"))),
        name: Set(Some(format!("Method {suffix}"))),
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
        currency: Set(Some(currency.to_string())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .unwrap();

    payments::ActiveModel {
        order_id: Set(order.id),
        payment_method_id: Set(method.id),
        amount: Set(Some(Decimal::new(1500, 2))),
        currency: Set(Some(currency.to_string())),
        status: Set(Some(status)),
        number: Set(Some(format!("PAY-{suffix}"))),
        external_payment_id: Set(Some(format!("external-{suffix}"))),
        transaction_id: Set(Some(format!("txn-{suffix}"))),
        idempotency_key: Set(Some(format!("idem-{suffix}"))),
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
async fn admin_can_list_payments_with_filters_and_pagination() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let _csrf = login_admin(&request).await;

        create_payment_fixture(&ctx, "ONE", "Gateway One", "BRL", 6).await;
        create_payment_fixture(&ctx, "TWO", "Gateway Two", "USD", 5).await;
        create_payment_fixture(&ctx, "THREE", "Gateway One", "BRL", 6).await;

        let paged = request.get("/api/admin/payments?page=1&page_size=2").await;
        let paged_body = paged.text();
        assert_eq!(paged.status_code(), 200, "unexpected body: {paged_body}");
        let paged_payload: Value = serde_json::from_str(&paged_body).unwrap();
        assert_eq!(paged_payload["page"], 1);
        assert_eq!(paged_payload["page_size"], 2);
        assert_eq!(paged_payload["items"].as_array().unwrap().len(), 2);
        assert!(paged_payload["total"].as_u64().unwrap() >= 3);
        assert!(paged_payload["currencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "BRL"));

        let filtered = request
            .get("/api/admin/payments?search=external-THREE&status=6&currency=BRL")
            .await;
        let filtered_body = filtered.text();
        assert_eq!(
            filtered.status_code(),
            200,
            "unexpected body: {filtered_body}"
        );
        let filtered_payload: Value = serde_json::from_str(&filtered_body).unwrap();
        let items = filtered_payload["items"].as_array().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["external_payment_id"], "external-THREE");
        assert_eq!(items[0]["status"], 6);
        assert_eq!(items[0]["currency"], "BRL");
    })
    .await;
}
