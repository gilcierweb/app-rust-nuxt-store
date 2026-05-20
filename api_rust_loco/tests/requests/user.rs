use api_rust_loco::{app::App, models::users};
use loco_rs::testing::prelude::*;
use sea_orm::EntityTrait;
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

    assert_eq!(response.status_code(), 200, "Admin login should succeed");
    csrf
}

#[tokio::test]
#[serial]
async fn admin_can_create_and_update_user() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

        let create_response = request
            .post("/api/admin/users")
            .add_header(csrf_key.clone(), csrf_value.clone())
            .json(&serde_json::json!({
                "name": "Support Person",
                "email": "support-person@example.com",
                "password": "Password123!",
                "role": "support",
                "active": true
            }))
            .await;

        assert_eq!(create_response.status_code(), 200);
        let created: Value = serde_json::from_str(&create_response.text()).unwrap();
        assert_eq!(created["email"], "support-person@example.com");
        assert_eq!(created["role"], "support");
        assert_eq!(created["active"], true);

        let user = users::Model::find_by_email(&ctx.db, "support-person@example.com")
            .await
            .unwrap();
        assert!(user.email_verified_at.is_some());
        assert_eq!(
            user.roles(&ctx.db).await.unwrap(),
            vec!["support".to_string()]
        );

        let update_response = request
            .patch(&format!("/api/admin/users/{}", user.id))
            .add_header(csrf_key, csrf_value)
            .json(&serde_json::json!({
                "name": "Readonly Person",
                "email": "readonly-person@example.com",
                "role": "viewer",
                "active": false
            }))
            .await;

        assert_eq!(update_response.status_code(), 200);
        let updated: Value = serde_json::from_str(&update_response.text()).unwrap();
        assert_eq!(updated["name"], "Readonly Person");
        assert_eq!(updated["email"], "readonly-person@example.com");
        assert_eq!(updated["role"], "viewer");
        assert_eq!(updated["active"], false);

        let user = users::Entity::find_by_id(user.id)
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();
        assert!(user.email_verified_at.is_none());
        assert_eq!(
            user.roles(&ctx.db).await.unwrap(),
            vec!["viewer".to_string()]
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_cannot_remove_own_admin_role() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

        let admin = users::Model::find_by_email(&ctx.db, "admin@example.com")
            .await
            .unwrap();

        let response = request
            .patch(&format!("/api/admin/users/{}", admin.id))
            .add_header(csrf_key, csrf_value)
            .json(&serde_json::json!({
                "name": "Admin User",
                "email": "admin@example.com",
                "role": "viewer",
                "active": true
            }))
            .await;

        assert_eq!(response.status_code(), 400);
        assert!(admin.has_role(&ctx.db, "admin").await.unwrap());
    })
    .await;
}
