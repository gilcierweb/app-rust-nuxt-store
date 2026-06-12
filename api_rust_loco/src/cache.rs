use std::sync::{Arc, OnceLock};
use std::time::Duration;

use moka::sync::Cache;
use serde_json::Value;

use crate::controllers::dashboard::DashboardResponse;
use crate::controllers::post::PostListItem;
use crate::models::_entities::{categories, posts, profiles};
use crate::models::products::ProductWithCategory;
use crate::views::auth::CurrentResponse;

static CURRENT_CACHE: OnceLock<Cache<String, Arc<CurrentResponse>>> = OnceLock::new();
static PRODUCTS_CACHE: OnceLock<Cache<String, Arc<Vec<ProductWithCategory>>>> = OnceLock::new();
static PRODUCT_DETAIL_CACHE: OnceLock<Cache<String, Arc<ProductWithCategory>>> = OnceLock::new();
static POSTS_CACHE: OnceLock<Cache<String, Arc<Vec<PostListItem>>>> = OnceLock::new();
static POST_DETAIL_CACHE: OnceLock<Cache<String, Arc<posts::Model>>> = OnceLock::new();
static PROFILES_CACHE: OnceLock<Cache<String, Arc<Vec<profiles::Model>>>> = OnceLock::new();
static PROFILE_DETAIL_CACHE: OnceLock<Cache<String, Arc<profiles::Model>>> = OnceLock::new();
static CATEGORIES_CACHE: OnceLock<Cache<&'static str, Arc<Vec<categories::Model>>>> =
    OnceLock::new();
static DASHBOARD_CACHE: OnceLock<Cache<&'static str, Arc<DashboardResponse>>> = OnceLock::new();
static JSON_CACHE: OnceLock<Cache<String, Arc<Value>>> = OnceLock::new();
/// Caches JWT blacklist lookups to avoid a DB round-trip on every authenticated request.
/// TTL of 5 s: a revoked token will be blocked within at most 5 s of logout.
static BLACKLIST_CACHE: OnceLock<Cache<String, bool>> = OnceLock::new();
/// In-memory rate-limit result cache keyed by "<scope>:<key>".
/// TTL of 1 s: avoids hitting Redis on every request for the same user/IP
/// while still converging to accurate counts within one sliding window tick.
static RATE_LIMIT_CACHE: OnceLock<Cache<String, bool>> = OnceLock::new();

pub fn current_cache() -> &'static Cache<String, Arc<CurrentResponse>> {
    CURRENT_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(1024)
            .build()
    })
}

pub fn products_cache() -> &'static Cache<String, Arc<Vec<ProductWithCategory>>> {
    PRODUCTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(64)
            .build()
    })
}

pub fn product_detail_cache() -> &'static Cache<String, Arc<ProductWithCategory>> {
    PRODUCT_DETAIL_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(15))
            .max_capacity(256)
            .build()
    })
}

pub fn posts_cache() -> &'static Cache<String, Arc<Vec<PostListItem>>> {
    POSTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(30))
            .max_capacity(64)
            .build()
    })
}

pub fn post_detail_cache() -> &'static Cache<String, Arc<posts::Model>> {
    POST_DETAIL_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(256)
            .build()
    })
}

pub fn profiles_cache() -> &'static Cache<String, Arc<Vec<profiles::Model>>> {
    PROFILES_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(64)
            .build()
    })
}

pub fn profile_detail_cache() -> &'static Cache<String, Arc<profiles::Model>> {
    PROFILE_DETAIL_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(256)
            .build()
    })
}

pub fn categories_cache() -> &'static Cache<&'static str, Arc<Vec<categories::Model>>> {
    CATEGORIES_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(32)
            .build()
    })
}

pub fn dashboard_cache() -> &'static Cache<&'static str, Arc<DashboardResponse>> {
    DASHBOARD_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(16)
            .build()
    })
}

/// In-memory cache for JWT blacklist lookups.
/// Avoids a DB round-trip on every authenticated request.
/// TTL of 5 s: revoked tokens are blocked within 5 s of logout.
pub fn blacklist_cache() -> &'static Cache<String, bool> {
    BLACKLIST_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(4096)
            .build()
    })
}

pub fn json_cache() -> &'static Cache<String, Arc<Value>> {
    JSON_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(30))
            .max_capacity(512)
            .build()
    })
}

/// In-memory cache for rate-limit "allowed" decisions.
/// Avoids a Redis round-trip on every request from the same user/IP within a 5 s window.
/// The worst case is 5 extra requests per key per window, negligible vs 300 req/min limit.
pub fn rate_limit_cache() -> &'static Cache<String, bool> {
    RATE_LIMIT_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(8192)
            .build()
    })
}

pub fn invalidate_json_cache(key: &str) {
    json_cache().invalidate(key);
}

pub fn invalidate_json_cache_with_prefix(prefix: &str) {
    let prefix = prefix.to_string();
    let _ = json_cache().invalidate_entries_if(move |k, _| k.starts_with(&prefix));
}

pub fn invalidate_products_cache() {
    products_cache().invalidate_all();
    product_detail_cache().invalidate_all();
}

pub fn invalidate_posts_cache() {
    posts_cache().invalidate_all();
    post_detail_cache().invalidate_all();
}

pub fn invalidate_profiles_cache() {
    profiles_cache().invalidate_all();
    profile_detail_cache().invalidate_all();
}

pub fn invalidate_categories_cache() {
    categories_cache().invalidate("list");
}

pub fn invalidate_dashboard_cache() {
    dashboard_cache().invalidate("stats");
}

pub fn invalidate_blacklist_cache(token_hash: &str) {
    blacklist_cache().invalidate(token_hash);
}

pub fn invalidate_rate_limit_cache(key: &str) {
    rate_limit_cache().invalidate(key);
}

pub fn invalidate_catalog_caches() {
    invalidate_products_cache();
    invalidate_categories_cache();
    invalidate_dashboard_cache();
}
