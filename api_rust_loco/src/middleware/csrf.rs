use axum::{
    extract::{Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use loco_rs::{app::AppContext, environment::Environment, prelude::*};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

pub const BACKEND_CSRF_COOKIE: &str = "backend_csrf";
pub const BACKEND_CSRF_HEADER: &str = "x-backend-csrf-token";

#[derive(Debug, Serialize)]
pub struct CsrfTokenResponse {
    pub token: String,
    pub header_name: String,
}

fn is_protected_method(method: &Method) -> bool {
    matches!(
        method,
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE
    )
}

fn is_exempt_path(path: &str) -> bool {
    matches!(path, "/api/auth/csrf")
}

fn build_csrf_cookie(ctx: &AppContext, token: &str) -> Cookie<'static> {
    Cookie::build((BACKEND_CSRF_COOKIE, token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(if ctx.environment == Environment::Production {
            SameSite::None
        } else {
            SameSite::Lax
        })
        .secure(ctx.environment == Environment::Production)
        .build()
}

pub fn ensure_csrf_cookie(ctx: &AppContext, jar: CookieJar) -> (CookieJar, String) {
    if let Some(existing_token) = jar
        .get(BACKEND_CSRF_COOKIE)
        .map(|cookie| cookie.value().to_string())
    {
        return (jar, existing_token);
    }

    let token = Uuid::new_v4().to_string();
    let jar = jar.add(build_csrf_cookie(ctx, &token));
    (jar, token)
}

pub async fn csrf_token(
    State(ctx): State<AppContext>,
    jar: CookieJar,
) -> Result<(CookieJar, Response)> {
    let (jar, token) = ensure_csrf_cookie(&ctx, jar);
    let payload = CsrfTokenResponse {
        token,
        header_name: BACKEND_CSRF_HEADER.to_string(),
    };

    Ok((jar, format::json(payload)?))
}

pub async fn csrf_guard(State(_ctx): State<AppContext>, req: Request, next: Next) -> Response {
    let path = req.uri().path();

    if !path.starts_with("/api/") || req.method() == Method::OPTIONS {
        return next.run(req).await;
    }

    if !is_protected_method(req.method()) || is_exempt_path(path) {
        return next.run(req).await;
    }

    let jar = CookieJar::from_headers(req.headers());
    let cookie_token = jar.get(BACKEND_CSRF_COOKIE).map(|cookie| cookie.value());
    let header_token = req
        .headers()
        .get(BACKEND_CSRF_HEADER)
        .and_then(|value| value.to_str().ok());

    match (cookie_token, header_token) {
        (Some(cookie), Some(header)) if cookie == header => next.run(req).await,
        (None, _) => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "csrf_token_missing",
                "description": "Backend CSRF cookie not found"
            })),
        )
            .into_response(),
        (_, None) => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "csrf_token_missing",
                "description": "Backend CSRF header not found"
            })),
        )
            .into_response(),
        _ => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "csrf_token_invalid",
                "description": "Backend CSRF token mismatch"
            })),
        )
            .into_response(),
    }
}
