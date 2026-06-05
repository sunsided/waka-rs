//! Contains the models for [`WakaTimeClient::status_bar_today`](crate::WakaTimeClient::status_bar_today).

use crate::model::summaries::{
    SummaryCategory, SummaryDependency, SummaryEditor, SummaryGrandTotal, SummaryLanguage,
    SummaryMachine, SummaryOperatingSystem, SummaryProject, SummaryRange,
};
use serde::{Deserialize, Serialize};

/// Today's coding activity for status bar display; only backed by cached data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBar {
    /// Time when this response was cached in ISO 8601 format.
    pub cached_at: Option<String>,
    pub data: StatusBarData,
    /// Whether the user has access to team features.
    pub has_team_features: Option<bool>,
}

/// A single day of coding activity in the same format as a
/// [`Summary`](crate::model::summaries::Summary), with all breakdowns optional
/// since the cached status bar response may omit them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarData {
    pub grand_total: SummaryGrandTotal,
    pub categories: Option<Vec<SummaryCategory>>,
    pub projects: Option<Vec<SummaryProject>>,
    pub languages: Option<Vec<SummaryLanguage>>,
    pub editors: Option<Vec<SummaryEditor>>,
    pub operating_systems: Option<Vec<SummaryOperatingSystem>>,
    pub dependencies: Option<Vec<SummaryDependency>>,
    pub machines: Option<Vec<SummaryMachine>>,
    pub range: SummaryRange,
}
