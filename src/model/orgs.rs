//! Contains the models for [`WakaTimeClient::orgs`](crate::WakaTimeClient::orgs).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orgs {
    /// The organizations the user is a member of.
    pub data: Vec<Org>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org {
    /// Unique id of this organization.
    pub id: String,
    /// Name of this organization.
    pub name: Option<String>,
    /// Privacy of projects on new dashboards: `visible` or `hidden`.
    pub default_project_privacy: Option<String>,
    /// Number of pending invites.
    pub invited_people_count: Option<u32>,
    /// Number of pending invites in human-readable format.
    pub invited_people_count_human_readable: Option<String>,
    /// Whether durations are visible on this org's dashboards.
    pub is_duration_visible: Option<bool>,
    /// Number of members.
    pub people_count: Option<u32>,
    /// Number of members in human-readable format.
    pub people_count_human_readable: Option<String>,
    /// Keystroke timeout preference of this organization.
    pub timeout: Option<u32>,
    /// Timezone preference of this organization.
    pub timezone: Option<String>,
    /// The writes_only preference of this organization.
    pub writes_only: Option<bool>,
    /// Whether the authorized user may list this org's dashboards.
    pub can_current_user_list_dashboards: Option<bool>,
    /// Time when this organization was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this organization was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
}
