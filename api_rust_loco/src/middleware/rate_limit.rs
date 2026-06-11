use axum::{
    extract::{Request, State},
    http::{header::RETRY_AFTER, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use loco_rs::{app::AppContext, auth::jwt};
use redis;
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

static CONFIG: std::sync::OnceLock<RateLimitConfig> = std::sync::OnceLock::new();

fn get_config() -> &'static RateLimitConfig {
    CONFIG.get_or_init(config_from_env)
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
    let member = format!("{}:{}", now, uuid::Uuid::new_v4());

    let script = redis::Script::new(r#"
        redis.call('ZREMRANGEBYSCORE', KEYS[1], 0, ARGV[1])
        local count = redis.call('ZCARD', KEYS[1])
        if count < tonumber(ARGV[2]) then
            redis.call('ZADD', KEYS[1], ARGV[3], ARGV[4])
            redis.call('EXPIRE', KEYS[1], tonumber(ARGV[5]))
            return { 1, count, 0 }
        else
            local oldest = redis.call('ZRANGE', KEYS[1], 0, 0, 'WITHSCORES')
            if oldest and oldest[2] then
                return { 0, count, tonumber(oldest[2]) }
            else
                return { 0, count, 0 }
            end
        end
    "#);

    let result: (i32, usize, u64) = script
        .key(&redis_key)
        .arg(window_start)
        .arg(limit)
        .arg(now)
        .arg(&member)
        .arg(window.as_secs() as usize)
        .invoke_async(conn)
        .await?;

    let (allowed_flag, count, oldest_score) = result;

    if allowed_flag == 1 {
        let remaining = limit.saturating_sub(count + 1);
        Ok(RateLimitState {
            allowed: true,
            limit,
            remaining,
            retry_after: 0,
            scope,
        })
    } else {
        let retry_after = if oldest_score > 0 {
            let remaining_ms = window_ms.saturating_sub(now.saturating_sub(oldest_score));
            (remaining_ms / 1000).max(1)
        } else {
            1
        };

        Ok(RateLimitState {
            allowed: false,
            limit,
            remaining: 0,
            retry_after,
            scope,
        })
    }
}

static REDIS_MANAGER: tokio::sync::OnceCell<Option<redis::aio::ConnectionManager>> = tokio::sync::OnceCell::const_new();

async fn get_connection_manager(redis_url: &str) -> Option<redis::aio::ConnectionManager> {
    REDIS_MANAGER.get_or_init(|| async {
        match redis::Client::open(redis_url) {
            Ok(client) => match client.get_tokio_connection_manager().await {
                Ok(conn) => Some(conn),
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
    })
    .await
    .clone()
}

pub async fn rate_limit_guard(State(ctx): State<AppContext>, req: Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS || !is_protected_path(req.uri().path()) {
        return next.run(req).await;
    }

    let config = get_config();
    if !config.enabled {
        return next.run(req).await;
    }

    let ip_key = format!("ip:{}", client_ip(req.headers()));
    let user_key = user_key(&ctx, req.headers());

    let mut conn = match get_connection_manager(&config.redis_url).await {
        Some(conn) => conn,
        None => {
            tracing::warn!("Redis connection failed, allowing request (rate limiter disabled)");
            return next.run(req).await;
        }
    };

    if let Some(user_key) = user_key {
        // Run IP and user checks in parallel to halve the Redis round-trip overhead.
        let ip_limit = config.ip_requests;
        let ip_window = config.ip_window;
        let user_limit = config.user_requests;
        let user_window = config.user_window;
        let mut conn2 = conn.clone();

        let (ip_result, user_result) = tokio::join!(
            check_rate_limit_redis(&mut conn, &ip_key, ip_limit, ip_window, "ip"),
            check_rate_limit_redis(&mut conn2, &user_key, user_limit, user_window, "user"),
        );

        let ip_state = match ip_result {
            Ok(state) => state,
            Err(e) => {
                tracing::error!(error = ?e, "Redis IP rate limit check failed");
                return next.run(req).await;
            }
        };

        if !ip_state.allowed {
            return rate_limited_response(ip_state);
        }

        let user_state = match user_result {
            Ok(state) => state,
            Err(e) => {
                tracing::error!(error = ?e, "Redis user rate limit check failed");
                return next.run(req).await;
            }
        };

        if !user_state.allowed {
            return rate_limited_response(user_state);
        }
    } else {
        // Unauthenticated: only check IP limit.
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
