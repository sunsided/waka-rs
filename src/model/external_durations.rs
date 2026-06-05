//! Contains the models for [`WakaTimeClient::external_durations`](crate::WakaTimeClient::external_durations).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDurations {
    /// The external durations for the requested day.
    pub data: Vec<ExternalDuration>,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
}

/// A duration logged by an external integration, e.g. a meeting or code review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDuration {
    /// Unique id of this external duration.
    pub id: String,
    /// Unique id of this duration on the external provider.
    pub external_id: Option<String>,
    /// The entity that this duration is logging time against.
    pub entity: Option<String>,
    /// Type of entity; can be `file`, `app`, `event`, `url` or `domain`.
    pub r#type: Option<String>,
    /// The external app which created this activity.
    pub provider: Option<String>,
    /// Category for this activity, e.g. `coding`, `meeting` or `code reviewing`.
    pub category: Option<String>,
    /// Start of this duration as UNIX epoch.
    pub start_time: Option<f64>,
    /// End of this duration as UNIX epoch.
    pub end_time: Option<f64>,
    /// Project name, if any.
    pub project: Option<String>,
    /// Branch name, if any.
    pub branch: Option<String>,
    /// Language name, if any.
    pub language: Option<String>,
    /// Additional metadata for this duration.
    pub meta: Option<String>,
}
