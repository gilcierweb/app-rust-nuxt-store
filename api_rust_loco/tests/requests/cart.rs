use api_rust_loco::app::App;
use api_rust_loco::models::_entities::products;
use loco_rs::testing::prelude::*;
use sea_orm::EntityTrait;
use serial_test::serial;

use super::prepare_data;
use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_cart() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        let csrf = prepare_data::init_csrf(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);
        request.add_header(csrf_key, csrf_value);

        prepare_data::init_user_login(&request, &ctx).await;

        let res = request.get("/api/carts/").await;
        assert_eq!(res.status_code(), 200);

        let body = res.json::<serde_json::Value>();
        assert!(body.get("id").is_some(), "Cart should have an id");
        assert!(body.get("items").is_some(), "Cart should have items array");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_add_item_to_cart() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        let csrf = prepare_data::init_csrf(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);
        request.add_header(csrf_key, csrf_value);

        prepare_data::init_user_login(&request, &ctx).await;

        let product = products::Entity::find()
            .one(&ctx.db)
            .await
            .unwrap()
            .expect("Seeded product should exist");

        let payload = serde_json::json!({
            "product_id": product.id,
            "product_variant_id": null,
            "quantity": 2,
            "price": "49.90"
        });

        let res = request.post("/api/carts/add_item").json(&payload).await;
        assert_eq!(res.status_code(), 200);

        let body = res.json::<serde_json::Value>();
        let items = body["items"].as_array().unwrap();
        assert!(!items.is_empty(), "Cart should have items after adding");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_clear_cart() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        let csrf = prepare_data::init_csrf(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);
        request.add_header(csrf_key, csrf_value);

        prepare_data::init_user_login(&request, &ctx).await;

        let product = products::Entity::find()
            .one(&ctx.db)
            .await
            .unwrap()
            .expect("Seeded product should exist");

        let add_payload = serde_json::json!({
            "product_id": product.id,
            "product_variant_id": null,
            "quantity": 1,
            "price": "29.90"
        });

        request.post("/api/carts/add_item").json(&add_payload).await;

        let res = request.delete("/api/carts/clear").await;
        assert_eq!(res.status_code(), 200);

        let body = res.json::<serde_json::Value>();
        assert_eq!(body["cleared"], true);
    })
    .await;
}
