use api_rust_loco::models::users;
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

    assert_eq!(response.status_code(), 200, "Login request should succeed");

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
    }
}

pub async fn init_csrf(request: &TestServer) -> String {
    let response = request.get("/api/auth/csrf").save_cookies().await;
    let payload: CsrfResponse = serde_json::from_str(&response.text()).unwrap();
    payload.token
}

pub fn csrf_header(token: &str) -> (HeaderName, HeaderValue) {
    let csrf_header_value = HeaderValue::from_str(token).unwrap();
    (
        HeaderName::from_static(BACKEND_CSRF_HEADER),
        csrf_header_value,
    )
}
