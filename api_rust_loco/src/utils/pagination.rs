#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Pagination parameters for list endpoints
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PaginationParams {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: i64,
    /// Items per page (max 100)
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

impl PaginationParams {
    /// Returns offset for database queries
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page).max(0)
    }

    /// Returns limit for database queries
    pub fn limit(&self) -> i64 {
        self.per_page.clamp(1, 100)
    }

    /// Returns the zero-based page index expected by SeaORM paginator.
    #[must_use]
    pub fn page_index(&self) -> u64 {
        self.page.max(1).saturating_sub(1) as u64
    }

    /// Returns the page size expected by SeaORM paginator.
    #[must_use]
    pub fn page_size(&self) -> u64 {
        self.limit() as u64
    }

    /// Creates pagination params with defaults
    pub fn new(page: i64, per_page: i64) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
        }
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            data,
            pagination: PaginationMeta {
                page,
                per_page,
                total,
                total_pages: total_pages.max(1),
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        }
    }
}

/// Pagination metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

/// Cursor-based pagination params
#[derive(Debug, Deserialize, Clone)]
pub struct CursorParams {
    pub cursor: Option<String>,
    #[serde(default = "default_per_page")]
    pub limit: i64,
}

impl CursorParams {
    pub fn limit(&self) -> i64 {
        self.limit.clamp(1, 100)
    }
}

/// Cursor-based paginated response
#[derive(Debug, Serialize)]
pub struct CursorResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl<T> CursorResponse<T> {
    pub fn new(data: Vec<T>, next_cursor: Option<String>, has_more: bool) -> Self {
        Self {
            data,
            next_cursor,
            has_more,
        }
    }
}
