use api_rust_loco::{
    app::App,
    models::_entities::{roles, users_roles},
    models::users,
};
use loco_rs::testing::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
async fn admin_can_manage_resource_scoped_role_assignment() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

        let create_response = request
            .post("/api/admin/rbac/roles")
            .add_header(csrf_key.clone(), csrf_value.clone())
            .json(&serde_json::json!({
                "name": "manager",
                "resource_type": "store",
                "resource_id": 42
            }))
            .await;
        assert_eq!(create_response.status_code(), 200);

        let created: Value = serde_json::from_str(&create_response.text()).unwrap();
        let role_id = created["id"].as_i64().unwrap() as i32;
        assert_eq!(created["resource_type"], "store");
        assert_eq!(created["resource_id"], 42);

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

        let assignment_exists = users_roles::Entity::find()
            .filter(users_roles::Column::UserId.eq(customer.id))
            .filter(users_roles::Column::RoleId.eq(role_id))
            .one(&ctx.db)
            .await
            .unwrap()
            .is_some();
        assert!(assignment_exists);

        let remove_response = request
            .delete(&format!(
                "/api/admin/rbac/assignments/{}/{}",
                customer.id, role_id
            ))
            .add_header(csrf_key, csrf_value)
            .await;
        assert_eq!(remove_response.status_code(), 200);

        let assignment_exists = users_roles::Entity::find()
            .filter(users_roles::Column::UserId.eq(customer.id))
            .filter(users_roles::Column::RoleId.eq(role_id))
            .one(&ctx.db)
            .await
            .unwrap()
            .is_some();
        assert!(!assignment_exists);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_cannot_remove_own_admin_assignment() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let csrf = login_admin(&request).await;
        let (csrf_key, csrf_value) = prepare_data::csrf_header(&csrf);

        let admin = users::Model::find_by_email(&ctx.db, "admin@example.com")
            .await
            .unwrap();
        let admin_role = roles::Entity::find()
            .filter(roles::Column::Name.eq("admin"))
            .filter(roles::Column::ResourceType.is_null())
            .filter(roles::Column::ResourceId.is_null())
            .one(&ctx.db)
            .await
            .unwrap()
            .unwrap();

        let response = request
            .delete(&format!(
                "/api/admin/rbac/assignments/{}/{}",
                admin.id, admin_role.id
            ))
            .add_header(csrf_key, csrf_value)
            .await;

        assert_eq!(response.status_code(), 400);
        assert!(admin.has_role(&ctx.db, "admin").await.unwrap());
    })
    .await;
}
