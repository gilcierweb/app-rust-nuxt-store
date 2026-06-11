use axum::{
    extract::{Request, State},
    http::{header::RETRY_AFTER, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use loco_rs::{app::AppContext, auth::jwt};
use redis::AsyncCommands;
use serde_json::json;
use std::time::Duration;

const DEFAULT_IP_REQUESTS: usize = 120;
const DEFAULT_IP_WINDOW_SECONDS: u64 = 60;
const DEFAULT_USER_REQUESTS: usize = 300;
const DEFAULT_USER_WINDOW_SECONDS: u64 = 60;

const IP_LIMIT_ENV: &str = "API_RATE_LIMIT_IP_REQUESTS";
const IP_WINDOW_ENV: &str = "API_RATE_LIMIT_IP_WINDOW_SECONDS";
const USER_LIMIT_ENV: &str = "API_RATE_LIMIT_USER_REQUESTS";
const USER_WINDOW_ENV: &str = "API_RATE_LIMIT_USER_WINDOW_SECONDS";
const ENABLED_ENV: &str = "API_RATE_LIMIT_ENABLED";
const REDIS_URL_ENV: &str = "REDIS_URL";

#[derive(Clone, Debug)]
struct RateLimitConfig {
    enabled: bool,
    ip_requests: usize,
    ip_window: Duration,
    user_requests: usize,
    user_window: Duration,
    redis_url: String,
}

#[derive(Debug, PartialEq, Eq)]
struct RateLimitState {
    allowed: bool,
    limit: usize,
    remaining: usize,
    retry_after: u64,
    scope: &'static str,
}

fn is_protected_path(path: &str) -> bool {
    path == "/api"
        || path.starts_with("/api/")
        || path == "/api-docs"
        || path.starts_with("/api-docs/")
}

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            !matches!(normalized.as_str(), "0" | "false" | "no" | "off")
        })
        .unwrap_or(default)
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn env_duration(name: &str, default_seconds: u64) -> Duration {
    let seconds = std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_seconds);
    Duration::from_secs(seconds)
}

fn config_from_env() -> RateLimitConfig {
    RateLimitConfig {
        enabled: env_bool(ENABLED_ENV, true),
        ip_requests: env_usize(IP_LIMIT_ENV, DEFAULT_IP_REQUESTS),
        ip_window: env_duration(IP_WINDOW_ENV, DEFAULT_IP_WINDOW_SECONDS),
        user_requests: env_usize(USER_LIMIT_ENV, DEFAULT_USER_REQUESTS),
        user_window: env_duration(USER_WINDOW_ENV, DEFAULT_USER_WINDOW_SECONDS),
        redis_url: std::env::var(REDIS_URL_ENV)
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
    }
}

fn client_ip(headers: &HeaderMap) -> String {
    for header_name in ["cf-connecting-ip", "x-real-ip", "x-forwarded-for"] {
        if let Some(value) = headers
            .get(header_name)
            .and_then(|value| value.to_str().ok())
        {
            if let Some(ip) = value
                .split(',')
                .map(str::trim)
                .find(|candidate| !candidate.is_empty())
            {
                return ip.chars().take(128).collect();
            }
        }
    }

    "unknown".to_string()
}

fn user_key(ctx: &AppContext, headers: &HeaderMap) -> Option<String> {
    let token = CookieJar::from_headers(headers)
        .get("auth_token")
        .map(|cookie| cookie.value().to_string())?;
    let jwt_secret = ctx.config.get_jwt_config().ok()?;
    let claims = jwt::JWT::new(&jwt_secret.secret).validate(&token).ok()?;

    Some(format!("user:{}", claims.claims.pid))
}

fn rate_limited_response(state: RateLimitState) -> Response {
    let mut response = (
        StatusCode::TOO_MANY_REQUESTS,
        Json(json!({
            "error": "rate_limit_exceeded",
            "description": "Too many requests",
            "scope": state.scope,
            "limit": state.limit,
            "retry_after": state.retry_after
        })),
    )
        .into_response();

    let headers = response.headers_mut();
    headers.insert(
        RETRY_AFTER,
        HeaderValue::from_str(&state.retry_after.to_string())
            .unwrap_or_else(|_| HeaderValue::from_static("1")),
    );
    headers.insert(
        HeaderName::from_static("x-ratelimit-limit"),
        HeaderValue::from_str(&state.limit.to_string())
            .unwrap_or_else(|_| HeaderValue::from_static("0")),
    );
    headers.insert(
        HeaderName::from_static("x-ratelimit-remaining"),
        HeaderValue::from_str(&state.remaining.to_string())
            .unwrap_or_else(|_| HeaderValue::from_static("0")),
    );
    headers.insert(
        HeaderName::from_static("x-ratelimit-scope"),
        HeaderValue::from_static(state.scope),
    );

    response
}

async fn check_rate_limit_redis(
    conn: &mut redis::aio::ConnectionManager,
    key: &str,
    limit: usize,
    window: Duration,
    scope: &'static str,
) -> Result<RateLimitState, redis::RedisError> {
    let now = chrono::Utc::now().timestamp_millis() as u64;
    let window_ms = window.as_millis() as u64;
    let window_start = now.saturating_sub(window_ms);
    let redis_key = format!("ratelimit:{key}");

    // Remove old entries outside the window
    let _: () = conn
        .zrembyscore(&redis_key, 0, window_start)
        .await?;

    // Count current entries in window
    let count: usize = conn.zcard(&redis_key).await?;

    if count >= limit {
        // Get oldest entry to calculate retry_after
        let oldest: Option<(String, u64)> = conn.zrange_withscores(&redis_key, 0, 0).await?;
        let retry_after = oldest
            .map(|(_, score)| {
                let remaining_ms = window_ms.saturating_sub(now.saturating_sub(score));
                (remaining_ms / 1000).max(1)
            })
            .unwrap_or(1);

        return Ok(RateLimitState {
            allowed: false,
            limit,
            remaining: 0,
            retry_after,
            scope,
        });
    }

    // Add current request
    let member = format!("{}:{}", now, uuid::Uuid::new_v4());
    let _: () = conn.zadd(&redis_key, &member, now).await?;

    // Set expiration on the key
    let _: () = conn
        .expire(&redis_key, window.as_secs() as usize)
        .await?;

    let remaining = limit.saturating_sub(count + 1);

    Ok(RateLimitState {
        allowed: true,
        limit,
        remaining,
        retry_after: 0,
        scope,
    })
}

static REDIS_MANAGER: std::sync::OnceLock<tokio::sync::Mutex<Option<redis::aio::ConnectionManager>>> = std::sync::OnceLock::new();

async fn get_connection_manager(redis_url: &str) -> Option<redis::aio::ConnectionManager> {
    let mutex = REDIS_MANAGER.get_or_init(|| tokio::sync::Mutex::new(None));
    
    {
        let guard = mutex.lock().await;
        if let Some(conn) = &*guard {
            return Some(conn.clone());
        }
    }
    
    let mut guard = mutex.lock().await;
    if let Some(conn) = &*guard {
        return Some(conn.clone());
    }
    
    match redis::Client::open(redis_url) {
        Ok(client) => match client.get_tokio_connection_manager().await {
            Ok(conn) => {
                *guard = Some(conn.clone());
                Some(conn)
            }
            Err(e) => {
                tracing::warn!("Redis connection failed: {}", e);
                None
            }
        },
        Err(e) => {
            tracing::warn!("Redis client open failed: {}", e);
            None
        }
    }
}

pub async fn rate_limit_guard(State(ctx): State<AppContext>, req: Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS || !is_protected_path(req.uri().path()) {
        return next.run(req).await;
    }

    let config = config_from_env();
    if !config.enabled {
        return next.run(req).await;
    }

    let ip_key = format!("ip:{}", client_ip(req.headers()));
    let user_key = user_key(&ctx, req.headers());

    let redis_url = config.redis_url.clone();
    let mut conn = match get_connection_manager(&redis_url).await {
        Some(conn) => conn,
        None => {
            tracing::warn!("Redis connection failed, allowing request (rate limiter disabled)");
            return next.run(req).await;
        }
    };

    // Check IP rate limit
    let ip_state = match check_rate_limit_redis(
        &mut conn,
        &ip_key,
        config.ip_requests,
        config.ip_window,
        "ip",
    )
    .await
    {
        Ok(state) => state,
        Err(e) => {
            tracing::error!(error = ?e, "Redis IP rate limit check failed");
            return next.run(req).await;
        }
    };

    if !ip_state.allowed {
        return rate_limited_response(ip_state);
    }

    // Check user rate limit if authenticated
    if let Some(user_key) = user_key {
        let user_state = match check_rate_limit_redis(
            &mut conn,
            &user_key,
            config.user_requests,
            config.user_window,
            "user",
        )
        .await
        {
            Ok(state) => state,
            Err(e) => {
                tracing::error!(error = ?e, "Redis user rate limit check failed");
                return next.run(req).await;
            }
        };

        if !user_state.allowed {
            return rate_limited_response(user_state);
        }
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::{client_ip, is_protected_path};
    use axum::http::{HeaderMap, HeaderValue};

    #[test]
    fn extracts_first_forwarded_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.10, 10.0.0.1"),
        );

        assert_eq!(client_ip(&headers), "203.0.113.10");
    }

    #[test]
    fn prefers_cloudflare_connecting_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("203.0.113.10"));
        headers.insert(
            "cf-connecting-ip",
            HeaderValue::from_static("198.51.100.20"),
        );

        assert_eq!(client_ip(&headers), "198.51.100.20");
    }

    #[test]
    fn protects_api_and_openapi_paths() {
        assert!(is_protected_path("/api"));
        assert!(is_protected_path("/api/products"));
        assert!(is_protected_path("/api-docs/openapi.json"));
        assert!(!is_protected_path("/uploads/image.png"));
    }
}
