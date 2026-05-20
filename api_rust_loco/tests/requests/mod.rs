mod admin_rbac;
mod auth;
mod prepare_data;

use axum::http::{HeaderName, HeaderValue};
use loco_rs::TestServer;

const TEST_API_KEY: &str = "test-api-key";
const API_KEY_ENV: &str = "API_PROTECTION_API_KEY";

pub fn with_api_key(request: &mut TestServer) {
    std::env::set_var(API_KEY_ENV, TEST_API_KEY);
    request.add_header(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_static(TEST_API_KEY),
    );
}

pub mod address;
pub mod cart;
pub mod category;
pub mod coupon;
pub mod dashboard;
pub mod order;
pub mod payment;
pub mod post;
pub mod product;
pub mod profile;
pub mod review;
pub mod shipping;
pub mod user;
pub mod variant;
pub mod wishlist;

pub mod payment_webhook;

pub mod payment_setup_session;
