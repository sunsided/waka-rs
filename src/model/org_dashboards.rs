//! Contains the models for [`WakaTimeClient::org_dashboards`](crate::WakaTimeClient::org_dashboards)
//! and [`WakaTimeClient::org_dashboard_members`](crate::WakaTimeClient::org_dashboard_members).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgDashboards {
    /// The dashboards of the organization.
    pub data: Vec<OrgDashboard>,
    /// Number of the next page, if any.
    pub next_page: Option<u32>,
    /// Number of the previous page, if any.
    pub prev_page: Option<u32>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgDashboard {
    /// Unique id of this dashboard.
    pub id: String,
    /// Name of this dashboard.
    pub full_name: Option<String>,
    /// The user who created this dashboard.
    pub created_by: Option<String>,
    /// Timezone of this dashboard in Olson Country/Region format.
    pub timezone: Option<String>,
    /// Whether the timezone has been changed from the default.
    pub has_changed_timezone: Option<bool>,
    /// Number of members on this dashboard.
    pub members_count: Option<u32>,
    /// Number of members in human-readable format.
    pub members_count_human_readable: Option<String>,
    /// Whether the authorized user is a member of this dashboard.
    pub is_current_user_member: Option<bool>,
    /// Whether viewing this dashboard is restricted.
    pub is_viewing_restricted: Option<bool>,
    /// Whether manually logged time is hidden from this dashboard.
    pub is_manual_time_hidden: Option<bool>,
    /// Whether the authorized user may view this dashboard.
    pub can_current_user_view: Option<bool>,
    /// Whether the authorized user may request to view this dashboard.
    pub can_current_user_request_to_view: Option<bool>,
    /// Whether the authorized user may request to join this dashboard.
    pub can_current_user_request_to_join: Option<bool>,
    /// Whether the authorized user may add members to this dashboard.
    pub can_current_user_add_members: Option<bool>,
    /// Whether the authorized user may remove members from this dashboard.
    pub can_current_user_remove_members: Option<bool>,
    /// Whether the authorized user may delete this dashboard.
    pub can_current_user_delete: Option<bool>,
    /// Time when this dashboard was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this dashboard was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgDashboardMembers {
    /// The members of the dashboard.
    pub data: Vec<DashboardMember>,
    /// Number of the next page, if any.
    pub next_page: Option<u32>,
    /// Number of the previous page, if any.
    pub prev_page: Option<u32>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMember {
    /// Unique id of this user.
    pub id: String,
    /// Email address of this user.
    pub email: Option<String>,
    /// Full name of this user.
    pub full_name: Option<String>,
    /// When `true`, this member's coding activity is hidden from the dashboard.
    pub is_view_only: Option<bool>,
    /// URL of this user's photo.
    pub photo: Option<String>,
    /// Username of this user.
    pub username: Option<String>,
    /// Privacy of this member's personal profile: `visible` or `hidden`.
    pub default_personal_privacy: Option<String>,
}
