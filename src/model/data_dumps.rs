//! Contains the models for [`WakaTimeClient::data_dumps`](crate::WakaTimeClient::data_dumps).

use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::data_dumps`](crate::WakaTimeClient::data_dumps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDumps {
    /// The data dump exports of the user.
    pub data: Vec<DataDump>,
}

/// An export of the user's coding activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDump {
    /// Unique id of this data dump.
    pub id: String,
    /// Type of export: `heartbeats` or `daily`.
    pub r#type: Option<String>,
    /// Human-readable status of this export.
    pub status: Option<String>,
    /// Percent complete as a number between 0 and 100.
    pub percent_complete: Option<f64>,
    /// URL to download the export, once completed.
    pub download_url: Option<String>,
    /// Whether this export is still being processed.
    pub is_processing: Option<bool>,
    /// Whether this export appears to be stuck.
    pub is_stuck: Option<bool>,
    /// Whether this export has failed.
    pub has_failed: Option<bool>,
    /// Time when the download expires in ISO 8601 format.
    pub expires: Option<String>,
    /// Time when this export was requested in ISO 8601 format.
    pub created_at: Option<String>,
}
