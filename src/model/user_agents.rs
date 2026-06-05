//! Contains the models for [`WakaTimeClient::user_agents`](crate::WakaTimeClient::user_agents).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgents {
    /// The plugin user agents of the user.
    pub data: Vec<UserAgent>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgent {
    /// Unique id of this user agent.
    pub id: String,
    /// The full user agent string.
    pub value: Option<String>,
    /// Editor name parsed from the user agent.
    pub editor: Option<String>,
    /// Plugin version parsed from the user agent.
    pub version: Option<String>,
    /// Operating system parsed from the user agent.
    pub os: Option<String>,
    /// Whether this user agent is a browser extension.
    pub is_browser_extension: Option<bool>,
    /// Whether this user agent is a desktop app.
    pub is_desktop_app: Option<bool>,
    /// Time when this user agent was first seen in ISO 8601 format.
    pub first_seen_at: Option<String>,
    /// Time when this user agent was last seen in ISO 8601 format.
    pub last_seen_at: Option<String>,
    /// Time when this user agent was created in ISO 8601 format.
    pub created_at: Option<String>,
}
