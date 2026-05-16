use std::sync::{Arc, OnceLock};
use std::time::Duration;

use moka::sync::Cache;

use crate::controllers::dashboard::DashboardResponse;
use crate::models::_entities::categories;
use crate::models::products::ProductWithCategory;
use crate::views::auth::CurrentResponse;

static CURRENT_CACHE: OnceLock<Cache<String, Arc<CurrentResponse>>> = OnceLock::new();
static PRODUCTS_CACHE: OnceLock<Cache<&'static str, Arc<Vec<ProductWithCategory>>>> =
    OnceLock::new();
static CATEGORIES_CACHE: OnceLock<Cache<&'static str, Arc<Vec<categories::Model>>>> =
    OnceLock::new();
static DASHBOARD_CACHE: OnceLock<Cache<&'static str, Arc<DashboardResponse>>> = OnceLock::new();

pub fn current_cache() -> &'static Cache<String, Arc<CurrentResponse>> {
    CURRENT_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(2))
            .max_capacity(1024)
            .build()
    })
}

pub fn products_cache() -> &'static Cache<&'static str, Arc<Vec<ProductWithCategory>>> {
    PRODUCTS_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(5))
            .max_capacity(32)
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

pub fn invalidate_products_cache() {
    products_cache().invalidate("list");
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
