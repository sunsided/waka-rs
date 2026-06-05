//! Contains the models for [`WakaTimeClient::leaders`](crate::WakaTimeClient::leaders).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::leaders`](crate::WakaTimeClient::leaders).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaders {
    /// The authorized user's rank, if they are on the leaderboard.
    pub current_user: Option<CurrentUserRank>,
    /// The ranked users.
    pub data: Vec<LeaderboardEntry>,
    /// Language filter applied to this leaderboard, if any.
    pub language: Option<String>,
    /// Hireable filter applied to this leaderboard, if any.
    pub is_hireable: Option<bool>,
    /// Country code filter applied to this leaderboard, if any.
    pub country_code: Option<String>,
    /// Time when this leaderboard was last updated in ISO 8601 format.
    pub modified_at: Option<String>,
    /// The time range of this leaderboard.
    pub range: Option<LeadersRange>,
    /// Keystroke timeout setting in minutes used for this leaderboard.
    pub timeout: Option<u32>,
    /// The writes_only setting used for this leaderboard.
    pub writes_only: Option<bool>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// The authorized user's position on a leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUserRank {
    /// Rank of the authorized user, or `null` if not on this leaderboard.
    pub rank: Option<u32>,
    /// Page containing the authorized user, or `null` if not on this leaderboard.
    pub page: Option<u32>,
    /// The authorized user.
    pub user: Option<LeaderUser>,
}

/// A single ranked user on a leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// Rank of this leader.
    pub rank: u32,
    /// Running total of this leader's coding activity.
    pub running_total: Option<RunningTotal>,
    /// The ranked user.
    pub user: LeaderUser,
}

/// A leader's running total of coding activity over the leaderboard range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningTotal {
    /// Total coding activity for this user as seconds.
    pub total_seconds: f64,
    /// Total coding activity for this user in human-readable format.
    pub human_readable_total: Option<String>,
    /// Average coding activity per day as seconds.
    pub daily_average: Option<f64>,
    /// Average coding activity per day in human-readable format.
    pub human_readable_daily_average: Option<String>,
    /// Coding activity per language.
    pub languages: Option<Vec<LeaderLanguage>>,
}

/// Coding activity in a single language for a leader.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderLanguage {
    /// Language name.
    pub name: String,
    /// Total coding activity in this language as seconds.
    pub total_seconds: f64,
}

/// A user appearing on a leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderUser {
    /// Unique id of the user.
    pub id: String,
    /// Email address of the user, if public.
    pub email: Option<String>,
    /// Username of the user.
    pub username: Option<String>,
    /// Full name of the user.
    pub full_name: Option<String>,
    /// Display name of the user.
    pub display_name: Option<String>,
    /// Website of the user.
    pub website: Option<String>,
    /// Website of the user without the protocol part.
    pub human_readable_website: Option<String>,
    /// Whether the user has the hireable badge.
    pub is_hireable: Option<bool>,
    /// Location of the user.
    pub city: Option<LeaderCity>,
    /// Whether the user's email is public.
    pub is_email_public: Option<bool>,
    /// Whether the user's photo is public.
    pub is_photo_public: Option<bool>,
    /// URL of the user's photo.
    pub photo: Option<String>,
}

/// The location of a leader.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderCity {
    /// Two-character country code.
    pub country_code: Option<String>,
    /// City name.
    pub name: Option<String>,
    /// State name.
    pub state: Option<String>,
    /// City title including state and country.
    pub title: Option<String>,
}

/// The time range covered by a leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadersRange {
    /// Start of the range as ISO 8601 UTC datetime.
    pub start_date: Option<String>,
    /// Start of the range in human-readable format.
    pub start_text: Option<String>,
    /// End of the range as ISO 8601 UTC datetime.
    pub end_date: Option<String>,
    /// End of the range in human-readable format.
    pub end_text: Option<String>,
    /// Name of the range, e.g. `last_7_days`.
    pub name: Option<String>,
    /// The range in human-readable format.
    pub text: Option<String>,
}
