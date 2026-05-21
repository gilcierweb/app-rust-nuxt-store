use api_rust_loco::{
    app::App,
    models::_entities::{cart_items, carts, categories, product_variants, products, users},
};
use loco_rs::testing::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
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

#[tokio::test]
#[serial]
async fn admin_can_list_inventory_with_filters_and_pagination() {
    request::<App, _, _>(|mut request, ctx| async move {
        with_api_key(&mut request);
        seed::<App>(&ctx).await.unwrap();
        let _csrf = login_admin(&request).await;

        let now = chrono::Utc::now();
        let admin = users::Model::find_by_email(&ctx.db, "admin@example.com")
            .await
            .unwrap();
        let category = categories::Entity::find()
            .one(&ctx.db)
            .await
            .unwrap()
            .expect("seeded category");

        let product = products::ActiveModel {
            name: Set(Some("Inventory Search Product".to_string())),
            slug: Set(Some("inventory-search-product".to_string())),
            sku: Set(Some("INV-PARENT".to_string())),
            price: Set(Some(Decimal::new(1000, 2))),
            active: Set(Some(true)),
            status: Set(Some(1)),
            category_id: Set(category.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let _low_variant = product_variants::ActiveModel {
            name: Set(Some("Low Variant".to_string())),
            sku: Set(Some("SKU-LOW".to_string())),
            inventory_quantity: Set(Some(3)),
            active: Set(Some(true)),
            product_id: Set(product.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let reserved_variant = product_variants::ActiveModel {
            name: Set(Some("Reserved Variant".to_string())),
            sku: Set(Some("SKU-RESERVED".to_string())),
            inventory_quantity: Set(Some(9)),
            active: Set(Some(true)),
            product_id: Set(product.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let _healthy_variant = product_variants::ActiveModel {
            name: Set(Some("Healthy Variant".to_string())),
            sku: Set(Some("SKU-HEALTHY".to_string())),
            inventory_quantity: Set(Some(20)),
            active: Set(Some(true)),
            product_id: Set(product.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let cart = carts::ActiveModel {
            user_id: Set(admin.id),
            session_id: Set(Some("admin-cart-session".to_string())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        cart_items::ActiveModel {
            cart_id: Set(cart.id),
            product_id: Set(product.id),
            product_variant_id: Set(reserved_variant.id),
            quantity: Set(Some(2)),
            price: Set(Some(Decimal::new(1000, 2))),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();

        let paged = request
            .get("/api/admin/inventory?page=1&page_size=2&low_stock_threshold=5")
            .await;
        let paged_body = paged.text();
        assert_eq!(paged.status_code(), 200, "unexpected body: {paged_body}");
        let paged_payload: Value = serde_json::from_str(&paged_body).unwrap();
        assert_eq!(paged_payload["page"], 1);
        assert_eq!(paged_payload["page_size"], 2);
        assert_eq!(paged_payload["items"].as_array().unwrap().len(), 2);
        assert!(paged_payload["summary"]["total_variants"].as_i64().unwrap() >= 3);

        let filtered = request
            .get("/api/admin/inventory?search=SKU-LOW&status=low&low_stock_threshold=5")
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
        assert_eq!(items[0]["sku"], "SKU-LOW");

        let reserved = request
            .get("/api/admin/inventory?status=reserved&low_stock_threshold=5")
            .await;
        let reserved_payload: Value = serde_json::from_str(&reserved.text()).unwrap();
        let reserved_items = reserved_payload["items"].as_array().unwrap();
        assert!(reserved_items
            .iter()
            .any(|item| item["sku"] == "SKU-RESERVED"));
    })
    .await;
}
