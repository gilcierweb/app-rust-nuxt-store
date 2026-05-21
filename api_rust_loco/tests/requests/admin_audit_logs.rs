use api_rust_loco::{app::App, models::_entities::users};
use loco_rs::testing::prelude::*;
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
async fn admin_can_list_and_filter_audit_logs() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

        let create_response = request
            .post("/api/admin/users")
            .add_header(csrf_key.clone(), csrf_value.clone())
            .json(&serde_json::json!({
                "name": "Audited User",
                "email": "audited-user@example.com",
                "password": "Password123!",
                "role": "support",
                "active": true
            }))
            .await;

        assert_eq!(create_response.status_code(), 200);

        let response = request
            .get("/api/admin/audit-logs?action=user.create")
            .await;
        assert_eq!(response.status_code(), 200);

        let payload: Value = serde_json::from_str(&response.text()).unwrap();
        let items = payload["items"].as_array().unwrap();
        assert!(!items.is_empty());
        assert_eq!(items[0]["action"], "user.create");
        assert_eq!(items[0]["actor_email"], "admin@example.com");
        assert_eq!(items[0]["resource_type"], "user");
        assert_eq!(items[0]["resource_label"], "audited-user@example.com");

        let role_response = request
            .post("/api/admin/rbac/roles")
            .add_header(csrf_key.clone(), csrf_value.clone())
            .json(&serde_json::json!({
                "name": "auditor",
                "resource_type": "store",
                "resource_id": 7
            }))
            .await;
        assert_eq!(role_response.status_code(), 200);
        let created_role: Value = serde_json::from_str(&role_response.text()).unwrap();
        let role_id = created_role["id"].as_i64().unwrap() as i32;

        let customer = users::Model::find_by_email(&ctx.db, "customer@example.com")
            .await
            .unwrap();

        let assign_response = request
            .post("/api/admin/rbac/assignments")
            .add_header(csrf_key.clone(), csrf_value.clone())
            .json(&serde_json::json!({
                "user_id": customer.id,
                "role_id": role_id
            }))
            .await;
        assert_eq!(assign_response.status_code(), 200);

        let settings_response = request
            .put("/api/admin/settings")
            .add_header(csrf_key, csrf_value)
            .json(&serde_json::json!({
                "settings": [
                    {
                        "namespace": "general",
                        "key": "store_name",
                        "value": "Audited Store"
                    }
                ]
            }))
            .await;
        assert_eq!(settings_response.status_code(), 200);

        let role_logs = request
            .get("/api/admin/audit-logs?action=role.assign")
            .await;
        assert_eq!(role_logs.status_code(), 200);
        let role_payload: Value = serde_json::from_str(&role_logs.text()).unwrap();
        let role_items = role_payload["items"].as_array().unwrap();
        assert!(!role_items.is_empty());
        assert_eq!(role_items[0]["action"], "role.assign");
        assert_eq!(role_items[0]["resource_type"], "role_assignment");

        let settings_logs = request
            .get("/api/admin/audit-logs?action=settings.update")
            .await;
        assert_eq!(settings_logs.status_code(), 200);
        let settings_payload: Value = serde_json::from_str(&settings_logs.text()).unwrap();
        let settings_items = settings_payload["items"].as_array().unwrap();
        assert!(!settings_items.is_empty());
        assert_eq!(settings_items[0]["action"], "settings.update");
        assert_eq!(settings_items[0]["resource_type"], "admin_settings");
    })
    .await;
}
