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
    /// Today's coding activity.
    pub data: StatusBarData,
    /// Whether the user has access to team features.
    pub has_team_features: Option<bool>,
}

/// A single day of coding activity in the same format as a
/// [`Summary`](crate::model::summaries::Summary), with all breakdowns optional
/// since the cached status bar response may omit them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarData {
    /// Total coding activity for today.
    pub grand_total: SummaryGrandTotal,
    /// Coding activity broken down per category.
    pub categories: Option<Vec<SummaryCategory>>,
    /// Coding activity broken down per project.
    pub projects: Option<Vec<SummaryProject>>,
    /// Coding activity broken down per language.
    pub languages: Option<Vec<SummaryLanguage>>,
    /// Coding activity broken down per editor.
    pub editors: Option<Vec<SummaryEditor>>,
    /// Coding activity broken down per operating system.
    pub operating_systems: Option<Vec<SummaryOperatingSystem>>,
    /// Coding activity broken down per dependency.
    pub dependencies: Option<Vec<SummaryDependency>>,
    /// Coding activity broken down per machine.
    pub machines: Option<Vec<SummaryMachine>>,
    /// Time range covered by this data.
    pub range: SummaryRange,
}
