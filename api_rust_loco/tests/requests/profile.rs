use api_rust_loco::{app::App, models::_entities::profiles};
use loco_rs::testing::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde_json::Value;
use serial_test::serial;

use super::prepare_data;
use super::with_api_key;

#[tokio::test]
#[serial]
async fn can_get_profiles() {
    request::<App, _, _>(|mut request, _ctx| async move {
        with_api_key(&mut request);
        let res = request.get("/api/profiles/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

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

#[tokio::test]
#[serial]
async fn admin_can_list_profiles_with_search_and_pagination() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let _headers = prepare_data::csrf_header(&csrf);

        for index in 0..3 {
            let now = chrono::Utc::now();
            profiles::ActiveModel {
                first_name: Set(Some(format!("Search{index}"))),
                last_name: Set(Some("Customer".to_string())),
                full_name: Set(Some(format!("Search {index} Customer"))),
                username: Set(Some(format!("search_customer_{index}"))),
                user_id: Set(1),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .unwrap();
        }

        let paged = request.get("/api/admin/profiles?page=1&page_size=2").await;
        assert_eq!(paged.status_code(), 200);
        let paged_payload: Value = serde_json::from_str(&paged.text()).unwrap();
        assert_eq!(paged_payload["page"], 1);
        assert_eq!(paged_payload["page_size"], 2);
        assert_eq!(paged_payload["items"].as_array().unwrap().len(), 2);

        let filtered = request
            .get("/api/admin/profiles?search=search_customer_1")
            .await;
        assert_eq!(filtered.status_code(), 200);
        let filtered_payload: Value = serde_json::from_str(&filtered.text()).unwrap();
        let items = filtered_payload["items"].as_array().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["username"], "search_customer_1");
    })
    .await;
}
