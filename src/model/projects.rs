//! Contains the models for [`WakaTimeClient::projects`](crate::WakaTimeClient::projects).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::projects`](crate::WakaTimeClient::projects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projects {
    /// The projects of the user.
    pub data: Vec<ProjectSummary>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// A single project of the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    /// Unique id of project.
    pub id: String,
    /// Project name.
    pub name: String,
    /// Associated repository if connected, otherwise `null`.
    pub repository: Option<serde_json::Value>,
    /// Associated project badge if enabled, otherwise `null`.
    pub badge: Option<serde_json::Value>,
    /// Project color as hex string, if set.
    pub color: Option<String>,
    /// Time when project was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when project last received a heartbeat in ISO 8601 format.
    pub last_heartbeat_at: Option<String>,
    /// Time when project last received a heartbeat in human-readable format.
    pub human_readable_last_heartbeat_at: Option<String>,
    /// Project relative URL.
    pub url: Option<String>,
    /// Project name URL-encoded.
    pub urlencoded_name: Option<String>,
    /// Whether this project has a shareable URL.
    pub has_public_url: Option<bool>,
}
