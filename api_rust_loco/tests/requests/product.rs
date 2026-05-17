use api_rust_loco::app::App;
use axum::http::{HeaderName, HeaderValue};
use loco_rs::testing::prelude::*;
use serial_test::serial;

use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_products() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);
        let res = request.get("/api/products/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn rate_limits_products_by_ip() {
    request::<App, _, _>(|mut request, _ctx| async move {
        std::env::set_var("API_RATE_LIMIT_IP_REQUESTS", "1");
        std::env::set_var("API_RATE_LIMIT_IP_WINDOW_SECONDS", "60");
        std::env::set_var("API_RATE_LIMIT_USER_REQUESTS", "300");
        std::env::set_var("API_RATE_LIMIT_USER_WINDOW_SECONDS", "60");

        with_api_key(&mut request);
        request.add_header(
            HeaderName::from_static("x-forwarded-for"),
            HeaderValue::from_static("203.0.113.250"),
        );

        let first = request.get("/api/products/").await;
        assert_eq!(first.status_code(), 200);

        let second = request.get("/api/products/").await;
        assert_eq!(second.status_code(), 429);

        std::env::set_var("API_RATE_LIMIT_IP_REQUESTS", "120");
        std::env::set_var("API_RATE_LIMIT_IP_WINDOW_SECONDS", "60");
    })
    .await;
}
