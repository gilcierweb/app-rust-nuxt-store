use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_payment_webhooks() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/webhooks/payments/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
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
