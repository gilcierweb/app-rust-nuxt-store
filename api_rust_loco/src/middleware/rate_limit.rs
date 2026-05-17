use axum::{
    extract::{Request, State},
    http::{header::RETRY_AFTER, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use loco_rs::{app::AppContext, auth::jwt};
use serde_json::json;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

const DEFAULT_IP_REQUESTS: usize = 120;
const DEFAULT_IP_WINDOW_SECONDS: u64 = 60;
const DEFAULT_USER_REQUESTS: usize = 300;
const DEFAULT_USER_WINDOW_SECONDS: u64 = 60;

const IP_LIMIT_ENV: &str = "API_RATE_LIMIT_IP_REQUESTS";
const IP_WINDOW_ENV: &str = "API_RATE_LIMIT_IP_WINDOW_SECONDS";
const USER_LIMIT_ENV: &str = "API_RATE_LIMIT_USER_REQUESTS";
const USER_WINDOW_ENV: &str = "API_RATE_LIMIT_USER_WINDOW_SECONDS";
const ENABLED_ENV: &str = "API_RATE_LIMIT_ENABLED";

static RATE_LIMITER: OnceLock<Mutex<HashMap<String, VecDeque<Instant>>>> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
struct RateLimitConfig {
    enabled: bool,
    ip_requests: usize,
    ip_window: Duration,
    user_requests: usize,
    user_window: Duration,
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

fn check_window(
    entries: &mut VecDeque<Instant>,
    limit: usize,
    window: Duration,
    now: Instant,
    scope: &'static str,
) -> RateLimitState {
    while entries
        .front()
        .is_some_and(|seen_at| now.duration_since(*seen_at) >= window)
    {
        entries.pop_front();
    }

    if entries.len() >= limit {
        let retry_after = entries
            .front()
            .map(|oldest| {
                window
                    .saturating_sub(now.duration_since(*oldest))
                    .as_secs()
                    .max(1)
            })
            .unwrap_or(1);

        return RateLimitState {
            allowed: false,
            limit,
            remaining: 0,
            retry_after,
            scope,
        };
    }

    entries.push_back(now);
    RateLimitState {
        allowed: true,
        limit,
        remaining: limit.saturating_sub(entries.len()),
        retry_after: 0,
        scope,
    }
}

fn check_rate_limit(
    store: &mut HashMap<String, VecDeque<Instant>>,
    key: &str,
    limit: usize,
    window: Duration,
    now: Instant,
    scope: &'static str,
) -> RateLimitState {
    let state = check_window(
        store.entry(key.to_string()).or_default(),
        limit,
        window,
        now,
        scope,
    );

    store.retain(|_, entries| !entries.is_empty());
    state
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
    let now = Instant::now();

    let blocked_response = {
        let limiter = RATE_LIMITER.get_or_init(|| Mutex::new(HashMap::new()));
        let mut store = match limiter.lock() {
            Ok(store) => store,
            Err(_) => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "error": "rate_limiter_unavailable",
                        "description": "Rate limiter state is unavailable"
                    })),
                )
                    .into_response()
            }
        };

        let ip_state = check_rate_limit(
            &mut store,
            &ip_key,
            config.ip_requests,
            config.ip_window,
            now,
            "ip",
        );
        if !ip_state.allowed {
            Some(rate_limited_response(ip_state))
        } else if let Some(user_key) = user_key {
            let user_state = check_rate_limit(
                &mut store,
                &user_key,
                config.user_requests,
                config.user_window,
                now,
                "user",
            );
            if !user_state.allowed {
                Some(rate_limited_response(user_state))
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(response) = blocked_response {
        return response;
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::{check_rate_limit, client_ip, is_protected_path};
    use axum::http::{HeaderMap, HeaderValue};
    use std::{
        collections::{HashMap, VecDeque},
        time::{Duration, Instant},
    };

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

    #[test]
    fn blocks_when_window_is_full() {
        let mut store: HashMap<String, VecDeque<Instant>> = HashMap::new();
        let now = Instant::now();
        let window = Duration::from_secs(60);

        assert!(check_rate_limit(&mut store, "ip:test", 2, window, now, "ip").allowed);
        assert!(check_rate_limit(&mut store, "ip:test", 2, window, now, "ip").allowed);

        let blocked = check_rate_limit(&mut store, "ip:test", 2, window, now, "ip");
        assert!(!blocked.allowed);
        assert_eq!(blocked.remaining, 0);
        assert_eq!(blocked.scope, "ip");
    }

    #[test]
    fn allows_after_window_expires() {
        let mut store: HashMap<String, VecDeque<Instant>> = HashMap::new();
        let now = Instant::now();
        let window = Duration::from_secs(60);

        assert!(check_rate_limit(&mut store, "user:test", 1, window, now, "user").allowed);
        assert!(!check_rate_limit(&mut store, "user:test", 1, window, now, "user").allowed);

        let later = now + Duration::from_secs(61);
        assert!(check_rate_limit(&mut store, "user:test", 1, window, later, "user").allowed);
    }
}
