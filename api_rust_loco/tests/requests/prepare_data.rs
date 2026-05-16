use api_rust_loco::{models::users, views::auth::LoginResponse};
use axum::http::{HeaderName, HeaderValue};
use loco_rs::{app::AppContext, TestServer};
use serde::Deserialize;

const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "1234";
const BACKEND_CSRF_HEADER: &str = "x-backend-csrf-token";

#[derive(Debug, Deserialize)]
struct CsrfResponse {
    token: String,
}

pub struct LoggedInUser {
    pub user: users::Model,
    pub token: String,
}

pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let csrf = init_csrf(request).await;
    let (csrf_key, csrf_value) = csrf_header(&csrf);

    let register_payload = serde_json::json!({
        "name": "loco",
        "email": USER_EMAIL,
        "password": USER_PASSWORD
    });

    //Creating a new user
    request
        .post("/api/auth/register")
        .add_header(csrf_key.clone(), csrf_value.clone())
        .json(&register_payload)
        .await;
    let user = users::Model::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verification_token,
    });

    request
        .post("/api/auth/verify")
        .add_header(csrf_key.clone(), csrf_value.clone())
        .json(&verify_payload)
        .await;

    let response = request
        .post("/api/auth/login")
        .add_header(csrf_key, csrf_value)
        .save_cookies()
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}

pub async fn init_csrf(request: &TestServer) -> String {
    let response = request.get("/api/auth/csrf").save_cookies().await;
    let payload: CsrfResponse = serde_json::from_str(&response.text()).unwrap();
    payload.token
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}

pub fn csrf_header(token: &str) -> (HeaderName, HeaderValue) {
    let csrf_header_value = HeaderValue::from_str(token).unwrap();
    (
        HeaderName::from_static(BACKEND_CSRF_HEADER),
        csrf_header_value,
    )
}
