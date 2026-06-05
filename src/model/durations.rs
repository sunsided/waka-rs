//! Contains the models for [`WakaTimeClient::durations`](crate::WakaTimeClient::durations).

use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::durations`](crate::WakaTimeClient::durations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Durations {
    /// The durations for the requested day.
    pub data: Vec<Duration>,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: String,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: String,
    /// Timezone used in Olson Country/Region format.
    pub timezone: String,
}

/// A single period of logged coding activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    /// Project name for this duration, if any.
    pub project: Option<String>,
    /// Start of this duration as UNIX epoch; numbers after decimal point are fractions of a second.
    pub time: f64,
    /// Length of this duration in seconds.
    pub duration: f64,
    /// Color of this duration as hex string, if set.
    pub color: Option<String>,
}
