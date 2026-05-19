use axum::{
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

const API_KEY_HEADER: &str = "x-api-key";
const API_KEY_ENV: &str = "API_PROTECTION_API_KEY";

#[derive(Debug, PartialEq, Eq)]
enum ApiKeyError {
    MissingConfig,
    MissingHeader,
    Invalid,
}

fn constant_time_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let max_len = left.len().max(right.len());
    let mut diff = left.len() ^ right.len();

    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);
        diff |= (left_byte ^ right_byte) as usize;
    }

    diff == 0
}

fn validate_api_key(configured: Option<&str>, provided: Option<&str>) -> Result<(), ApiKeyError> {
    let configured = configured
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(ApiKeyError::MissingConfig)?;

    let provided = provided
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(ApiKeyError::MissingHeader)?;

    if !constant_time_eq(provided, configured) {
        return Err(ApiKeyError::Invalid);
    }

    Ok(())
}

fn is_protected_path(path: &str) -> bool {
    if path.starts_with("/api/webhooks/payments/") {
        return false;
    }

    path == "/api"
        || path.starts_with("/api/")
        || path == "/api-docs"
        || path.starts_with("/api-docs/")
}

pub async fn api_key_guard(mut req: axum::extract::Request, next: Next) -> Response {
    let path = req.uri().path();

    // Protect API routes and the generated OpenAPI document.
    if !is_protected_path(path) {
        return next.run(req).await;
    }

    // Allow preflight; it is not an authenticated API call.
    if req.method() == Method::OPTIONS {
        return next.run(req).await;
    }

    let configured = std::env::var(API_KEY_ENV).ok();
    let provided = req
        .headers()
        .get(API_KEY_HEADER)
        .and_then(|value| value.to_str().ok());

    match validate_api_key(configured.as_deref(), provided) {
        Ok(()) => {}
        Err(ApiKeyError::MissingConfig) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "error": "api_key_not_configured",
                    "description": format!("Set {API_KEY_ENV} to enable API key protection.")
                })),
            )
                .into_response()
        }
        Err(ApiKeyError::MissingHeader) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "api_key_missing",
                    "description": format!("Missing required header: {API_KEY_HEADER}")
                })),
            )
                .into_response()
        }
        Err(ApiKeyError::Invalid) => {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "api_key_invalid",
                    "description": "Invalid API key"
                })),
            )
                .into_response()
        }
    }

    // Avoid accidental header forwarding to downstream services/loggers.
    req.headers_mut().remove(API_KEY_HEADER);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::{is_protected_path, validate_api_key, ApiKeyError};

    #[test]
    fn rejects_missing_configuration() {
        assert_eq!(
            validate_api_key(None, Some("secret")),
            Err(ApiKeyError::MissingConfig)
        );
    }

    #[test]
    fn rejects_missing_header() {
        assert_eq!(
            validate_api_key(Some("secret"), None),
            Err(ApiKeyError::MissingHeader)
        );
    }

    #[test]
    fn rejects_invalid_key() {
        assert_eq!(
            validate_api_key(Some("secret"), Some("wrong")),
            Err(ApiKeyError::Invalid)
        );
    }

    #[test]
    fn accepts_valid_key() {
        assert_eq!(validate_api_key(Some("secret"), Some("secret")), Ok(()));
    }

    #[test]
    fn protects_api_and_openapi_paths() {
        assert!(is_protected_path("/api/products"));
        assert!(is_protected_path("/api"));
        assert!(is_protected_path("/api/docs"));
        assert!(is_protected_path("/api-docs"));
        assert!(is_protected_path("/api-docs/openapi.json"));
        assert!(!is_protected_path("/uploads/image.png"));
        assert!(!is_protected_path("/products"));
    }
}
