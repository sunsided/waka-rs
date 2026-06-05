//! Contains the models for [`WakaTimeClient::private_leaderboards`](crate::WakaTimeClient::private_leaderboards).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateLeaderboards {
    /// The private leaderboards the user is a member of.
    pub data: Vec<PrivateLeaderboard>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateLeaderboard {
    /// Unique id of this leaderboard.
    pub id: String,
    /// Display name of this leaderboard.
    pub name: Option<String>,
    /// Whether the user can invite more members.
    pub has_available_seat: Option<bool>,
    /// Number of members on this leaderboard.
    pub members_count: Option<u32>,
    /// Number of members with timezones set; others are not ranked.
    pub members_with_timezones_count: Option<u32>,
    /// The time range of this leaderboard, e.g. `last_7_days`.
    pub time_range: Option<String>,
    /// Time when this leaderboard was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this leaderboard was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
}
