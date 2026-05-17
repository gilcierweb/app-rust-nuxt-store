use axum::{
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

const API_KEY_HEADER: &str = "x-api-key";
const API_KEY_ENV: &str = "API_PROTECTION_API_KEY";

pub async fn api_key_guard(mut req: axum::extract::Request, next: Next) -> Response {
    let path = req.uri().path();

    // Only protect API routes.
    if !path.starts_with("/api/") {
        return next.run(req).await;
    }

    // Allow preflight; it is not an authenticated API call.
    if req.method() == Method::OPTIONS {
        return next.run(req).await;
    }

    let configured = std::env::var(API_KEY_ENV)
        .ok()
        .filter(|v| !v.trim().is_empty());
    let Some(configured_key) = configured else {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "api_key_not_configured",
                "description": format!("Set {API_KEY_ENV} to enable API key protection.")
            })),
        )
            .into_response();
    };

    let provided = req
        .headers()
        .get(API_KEY_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let Some(provided_key) = provided else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "api_key_missing",
                "description": format!("Missing required header: {API_KEY_HEADER}")
            })),
        )
            .into_response();
    };

    if provided_key != configured_key {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "api_key_invalid",
                "description": "Invalid API key"
            })),
        )
            .into_response();
    }

    // Avoid accidental header forwarding to downstream services/loggers.
    req.headers_mut().remove(API_KEY_HEADER);
    next.run(req).await
}
