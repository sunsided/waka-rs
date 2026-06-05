//! Contains the models for [`WakaTimeClient::heartbeats`](crate::WakaTimeClient::heartbeats).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeats {
    /// The heartbeats for the requested day.
    pub data: Vec<Heartbeat>,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Unique id of this heartbeat.
    pub id: String,
    /// The entity that the heartbeat is logging time against, such as an absolute file path or domain.
    pub entity: String,
    /// Type of entity; can be `file`, `app`, or `domain`.
    pub r#type: String,
    /// UNIX epoch timestamp; numbers after decimal point are fractions of a second.
    pub time: f64,
    /// Project name, if any.
    pub project: Option<String>,
    /// Branch name, if any.
    pub branch: Option<String>,
    /// Language name, if any.
    pub language: Option<String>,
    /// Category for this activity, e.g. `coding`, `debugging` or `building`.
    pub category: Option<String>,
    /// `true` if this heartbeat was triggered from writing to a file.
    pub is_write: Option<bool>,
    /// Total number of lines in the entity (when entity type is file).
    pub lines: Option<u64>,
    /// Current line row number of cursor (when entity type is file).
    pub lineno: Option<u64>,
    /// Current cursor column position (when entity type is file).
    pub cursorpos: Option<u64>,
    /// Comma-separated list of dependencies detected from entity file (when entity type is file).
    pub dependencies: Option<Vec<String>>,
    /// Unique id of the machine which generated this heartbeat.
    pub machine_name_id: Option<String>,
    /// Unique id of the user.
    pub user_id: Option<String>,
    /// Time when this heartbeat was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Unique id of the user agent which generated this heartbeat.
    pub user_agent_id: Option<String>,
}
