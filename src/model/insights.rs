//! Contains the models for [`WakaTimeClient::insights`](crate::WakaTimeClient::insights).

use serde::{Deserialize, Serialize};

/// An insight about the user's coding activity.
///
/// The shape of the actual insight payload varies with the requested insight
/// type, so it is captured as raw JSON in [`Insight::payload`], keyed by the
/// insight type (e.g. `weekdays`, `projects`, `best_day`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Time range of this insight, e.g. `last_7_days`.
    pub range: Option<String>,
    /// Time range of this insight in human-readable format.
    pub human_readable_range: Option<String>,
    /// Status of this insight in the cache: `ok` or `pending_update`.
    pub status: Option<String>,
    /// Whether this insight includes today's coding activity.
    pub is_including_today: Option<bool>,
    /// `true` if this insight is up to date; when `false`, it will be refreshed soon.
    pub is_up_to_date: Option<bool>,
    /// A number between 0 and 100 where 100 means the insight is up to date.
    pub percent_calculated: Option<u32>,
    /// Start of the range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// End of the range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
    /// Keystroke timeout setting in minutes used for this insight.
    pub timeout: Option<u32>,
    /// The writes_only setting used for this insight.
    pub writes_only: Option<bool>,
    /// Unique id of the user.
    pub user_id: Option<String>,
    /// Time when this insight was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this insight was last updated in ISO 8601 format.
    pub modified_at: Option<String>,
    /// The insight payload, keyed by the insight type.
    #[serde(flatten)]
    pub payload: serde_json::Map<String, serde_json::Value>,
}
