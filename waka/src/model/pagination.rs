//! Contains the shared pagination metadata returned by WakaTime list endpoints.

use serde::{Deserialize, Serialize};

/// Pagination metadata returned by WakaTime list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number.
    pub page: Option<u32>,
    /// Total number of pages available.
    pub total_pages: Option<u32>,
    /// Total number of items across all pages.
    pub total: Option<u32>,
}
