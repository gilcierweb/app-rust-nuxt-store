use std::sync::{Arc, OnceLock};
use std::time::Duration;

use moka::sync::Cache;
use serde_json::Value;

use crate::controllers::dashboard::DashboardResponse;
use crate::models::_entities::{categories, posts, profiles};
use crate::models::products::ProductWithCategory;
use crate::views::auth::CurrentResponse;

static CURRENT_CACHE: OnceLock<Cache<String, Arc<CurrentResponse>>> = OnceLock::new();
static PRODUCTS_CACHE: OnceLock<Cache<String, Arc<Vec<ProductWithCategory>>>> = OnceLock::new();
static PRODUCT_DETAIL_CACHE: OnceLock<Cache<String, Arc<ProductWithCategory>>> = OnceLock::new();
static POSTS_CACHE: OnceLock<Cache<String, Arc<Vec<posts::Model>>>> = OnceLock::new();
static POST_DETAIL_CACHE: OnceLock<Cache<String, Arc<posts::Model>>> = OnceLock::new();
static PROFILES_CACHE: OnceLock<Cache<String, Arc<Vec<profiles::Model>>>> = OnceLock::new();
static PROFILE_DETAIL_CACHE: OnceLock<Cache<String, Arc<profiles::Model>>> = OnceLock::new();
static CATEGORIES_CACHE: OnceLock<Cache<&'static str, Arc<Vec<categories::Model>>>> =
    OnceLock::new();
static DASHBOARD_CACHE: OnceLock<Cache<&'static str, Arc<DashboardResponse>>> = OnceLock::new();
static JSON_CACHE: OnceLock<Cache<String, Arc<Value>>> = OnceLock::new();

pub fn current_cache() -> &'static Cache<String, Arc<CurrentResponse>> {
    CURRENT_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(2))
            .max_capacity(1024)
            .build()
    })
}

pub fn products_cache() -> &'static Cache<String, Arc<Vec<ProductWithCategory>>> {
    PRODUCTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(32)
            .build()
    })
}

pub fn product_detail_cache() -> &'static Cache<String, Arc<ProductWithCategory>> {
    PRODUCT_DETAIL_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(256)
            .build()
    })
}

pub fn posts_cache() -> &'static Cache<String, Arc<Vec<posts::Model>>> {
    POSTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
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
            .time_to_live(Duration::from_secs(5))
            .max_capacity(32)
            .build()
    })
}

pub fn dashboard_cache() -> &'static Cache<&'static str, Arc<DashboardResponse>> {
    DASHBOARD_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(16)
            .build()
    })
}

pub fn json_cache() -> &'static Cache<String, Arc<Value>> {
    JSON_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(10))
            .max_capacity(512)
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

pub fn invalidate_catalog_caches() {
    invalidate_products_cache();
    invalidate_categories_cache();
    invalidate_dashboard_cache();
}
