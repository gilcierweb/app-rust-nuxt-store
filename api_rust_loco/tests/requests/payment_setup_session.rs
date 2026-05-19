use api_rust_loco::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::with_api_key;

#[tokio::test]
#[serial]
async fn account_payment_setup_sessions_require_authentication() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);

        let list = request.get("/api/account/payment-setup-sessions").await;
        assert_ne!(list.status_code(), 200);

        let create = request
            .post("/api/account/payment-setup-sessions")
            .json(&serde_json::json!({
                "payment_method_id": 1,
            }))
            .await;
        assert_ne!(create.status_code(), 200);

        let get_one = request.get("/api/account/payment-setup-sessions/1").await;
        assert_ne!(get_one.status_code(), 200);

        let complete = request
            .post("/api/account/payment-setup-sessions/1/complete")
            .json(&serde_json::json!({}))
            .await;
        assert_ne!(complete.status_code(), 200);
    })
    .await;
}
