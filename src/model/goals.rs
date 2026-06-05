//! Contains the models for [`WakaTimeClient::goals`](crate::WakaTimeClient::goals)
//! and [`WakaTimeClient::goal`](crate::WakaTimeClient::goal).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::goals`](crate::WakaTimeClient::goals).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goals {
    /// The goals of the user.
    pub data: Vec<Goal>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// A single goal; only backed by cached data, similar to the Status Bar endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedGoal {
    /// Time when this response was cached in ISO 8601 format.
    pub cached_at: Option<String>,
    /// The goal.
    pub data: Goal,
}

/// A single coding goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Unique id of this goal.
    pub id: String,
    /// Status of the goal averaged over the chart range: `success` or `fail`.
    pub average_status: Option<String>,
    /// Per-range progress towards the goal.
    pub chart_data: Option<Vec<GoalChartData>>,
    /// Time when this goal was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Status of the goal accumulated over the chart range.
    pub cumulative_status: Option<String>,
    /// Custom title of this goal, if set.
    pub custom_title: Option<String>,
    /// Goal interval: `day` or `week`.
    pub delta: Option<String>,
    /// Editors this goal is limited to.
    pub editors: Option<Vec<String>>,
    /// Weekday names ignored by this goal.
    pub ignore_days: Option<Vec<String>>,
    /// Whether days without coding activity are ignored.
    pub ignore_zero_days: Option<bool>,
    /// Percent the goal should improve by, if it is an improvement goal.
    pub improve_by_percent: Option<f64>,
    /// Whether the authorized user owns this goal.
    pub is_current_user_owner: Option<bool>,
    /// Whether this goal is enabled.
    pub is_enabled: Option<bool>,
    /// Whether this goal is inverse, i.e. coding less than the target.
    pub is_inverse: Option<bool>,
    /// Whether this goal is currently snoozed.
    pub is_snoozed: Option<bool>,
    /// Whether progress on this goal is tweeted.
    pub is_tweeting: Option<bool>,
    /// Languages this goal is limited to.
    pub languages: Option<Vec<String>>,
    /// Time when this goal was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
    /// The owner of this goal.
    pub owner: Option<GoalOwner>,
    /// Projects this goal is limited to.
    pub projects: Option<Vec<String>>,
    /// Human-readable description of the goal range.
    pub range_text: Option<String>,
    /// Target seconds per delta interval.
    pub seconds: Option<u64>,
    /// Users this goal is shared with.
    pub shared_with: Option<Vec<GoalSharedWith>>,
    /// Time until which this goal is snoozed in ISO 8601 format.
    pub snooze_until: Option<String>,
    /// Current status of this goal: `success`, `fail`, `ignored` or `pending`.
    pub status: Option<String>,
    /// A number between 0 and 100 where 100 means the status is up to date.
    pub status_percent_calculated: Option<u32>,
    /// Users subscribed to this goal's progress.
    pub subscribers: Option<Vec<GoalSubscriber>>,
    /// Human-readable title of this goal.
    pub title: Option<String>,
    /// Type of goal.
    pub r#type: Option<String>,
}

/// Progress towards a goal for a single range in the goal's chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalChartData {
    /// Number of seconds coded during this range.
    pub actual_seconds: Option<f64>,
    /// Coding activity during this range in human-readable format.
    pub actual_seconds_text: Option<String>,
    /// Target seconds for this range.
    pub goal_seconds: Option<f64>,
    /// Target for this range in human-readable format.
    pub goal_seconds_text: Option<String>,
    /// The time range this chart entry covers.
    pub range: Option<GoalRange>,
    /// Status of this range: `success`, `fail`, `pending` or `ignored`.
    pub range_status: Option<String>,
    /// Reason for the range status.
    pub range_status_reason: Option<String>,
    /// Short reason for the range status.
    pub range_status_reason_short: Option<String>,
}

/// A time range within a goal's chart data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRange {
    /// This day as Date string in YEAR-MONTH-DAY format; only set when delta is `day`.
    pub date: Option<String>,
    /// End of this range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Start of this range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// This range in human-readable format.
    pub text: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
}

/// The owner of a goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalOwner {
    /// Unique id of the owner.
    pub id: String,
    /// Display name of the owner.
    pub display_name: Option<String>,
    /// Email address of the owner.
    pub email: Option<String>,
    /// Full name of the owner.
    pub full_name: Option<String>,
    /// URL of the owner's photo.
    pub photo: Option<String>,
    /// Username of the owner.
    pub username: Option<String>,
}

/// A user a goal is shared with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSharedWith {
    /// Unique id of the share invitation.
    pub id: Option<String>,
    /// Display name of the user.
    pub display_name: Option<String>,
    /// Email address of the user.
    pub email: Option<String>,
    /// Full name of the user.
    pub full_name: Option<String>,
    /// URL of the user's photo.
    pub photo: Option<String>,
    /// Status of the share invitation.
    pub status: Option<String>,
    /// Unique id of the user.
    pub user_id: Option<String>,
    /// Username of the user.
    pub username: Option<String>,
}

/// A user subscribed to a goal's progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSubscriber {
    /// Display name of the subscriber.
    pub display_name: Option<String>,
    /// Email address of the subscriber.
    pub email: Option<String>,
    /// How often the subscriber receives goal progress emails.
    pub email_frequency: Option<String>,
    /// Full name of the subscriber.
    pub full_name: Option<String>,
    /// Unique id of the subscriber.
    pub user_id: Option<String>,
    /// Username of the subscriber.
    pub username: Option<String>,
}
