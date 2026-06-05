//! Contains the models for [`WakaTimeClient::summaries`](crate::WakaTimeClient::summaries).

use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::summaries`](crate::WakaTimeClient::summaries).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summaries {
    /// The summaries, one per day in the requested range.
    pub data: Vec<Summary>,
    /// Cumulative total over the date range.
    pub cumulative_total: CumulativeTotalSummary,
    /// Daily averages over the date range.
    pub daily_average: DailyAverageSummary,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: String,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: String,
}

/// A single day of coding activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// Total coding activity for this day.
    pub grand_total: SummaryGrandTotal,
    /// Coding activity broken down per category.
    pub categories: Vec<SummaryCategory>,
    /// Coding activity broken down per project.
    pub projects: Vec<SummaryProject>,
    /// Coding activity broken down per language.
    pub languages: Vec<SummaryLanguage>,
    /// Coding activity broken down per editor.
    pub editors: Vec<SummaryEditor>,
    /// Coding activity broken down per operating system.
    pub operating_systems: Vec<SummaryOperatingSystem>,
    /// Coding activity broken down per dependency.
    pub dependencies: Vec<SummaryDependency>,
    /// Coding activity broken down per machine.
    pub machines: Vec<SummaryMachine>,
    /// Included only when project url parameter used.
    pub branches: Option<Vec<SummaryBranch>>,
    /// Included only when project url parameter used.
    pub entities: Option<Vec<SummaryEntity>>,
    /// Time range covered by this summary.
    pub range: SummaryRange,
}

/// Coding activity broken down for a single category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryCategory {
    /// Name of category, for ex: Coding or Debugging.
    pub name: String,
    /// Total coding activity as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this category.
    pub percent: f64,
    /// Total coding activity for this category in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this category.
    pub hours: u32,
    /// Minutes portion of coding activity for this category.
    pub minutes: u32,
}

/// Coding activity broken down for a single project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryProject {
    /// Project name.
    pub name: String,
    /// Total coding activity as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this project.
    pub percent: f64,
    /// Total coding activity for this project in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this project.
    pub hours: u32,
    /// Minutes portion of coding activity for this project.
    pub minutes: u32,
}

/// Coding activity broken down for a single language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryLanguage {
    /// Language name.
    pub name: String,
    /// Total coding activity spent in this language as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this language.
    pub percent: f64,
    /// Total coding activity for this language in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this language.
    pub hours: u32,
    /// Minutes portion of coding activity for this language.
    pub minutes: u32,
    /// Seconds portion of coding activity for this language.
    pub seconds: u32,
}

/// Coding activity broken down for a single editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryEditor {
    /// Editor name.
    pub name: String,
    /// Total coding activity spent in this editor as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this editor.
    pub percent: f64,
    /// Total coding activity for this editor in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this editor.
    pub hours: u32,
    /// Minutes portion of coding activity for this editor.
    pub minutes: u32,
    /// Seconds portion of coding activity for this editor.
    pub seconds: u32,
}

/// Coding activity broken down for a single operating system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryOperatingSystem {
    /// OS name.
    pub name: String,
    /// Total coding activity spent in this OS as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this OS.
    pub percent: f64,
    /// Total coding activity for this OS in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this OS.
    pub hours: u32,
    /// Minutes portion of coding activity for this OS.
    pub minutes: u32,
    /// Seconds portion of coding activity for this OS.
    pub seconds: u32,
}

/// Coding activity broken down for a single dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryDependency {
    /// Dependency name.
    pub name: String,
    /// Total coding activity spent in this dependency as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this dependency.
    pub percent: f64,
    /// Total coding activity for this dependency in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this dependency.
    pub hours: u32,
    /// Minutes portion of coding activity for this dependency.
    pub minutes: u32,
    /// Seconds portion of coding activity for this dependency.
    pub seconds: u32,
}

/// Coding activity broken down for a single machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMachine {
    /// Machine hostname and ip address.
    pub name: String,
    /// Unique id of this machine.
    pub machine_name_id: String,
    /// Total coding activity spent on this machine as seconds.
    pub total_seconds: f64,
    /// Percent of time spent on this machine.
    pub percent: f64,
    /// Total coding activity for this machine in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this machine.
    pub hours: u32,
    /// Minutes portion of coding activity for this machine.
    pub minutes: u32,
    /// Seconds portion of coding activity for this machine.
    pub seconds: u32,
}

/// Coding activity broken down for a single branch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryBranch {
    /// Branch name.
    pub name: String,
    /// Total coding activity spent in this branch as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this branch.
    pub percent: f64,
    /// Total coding activity for this branch in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this branch.
    pub hours: u32,
    /// Minutes portion of coding activity for this branch.
    pub minutes: u32,
    /// Seconds portion of coding activity for this branch.
    pub seconds: u32,
}

/// Total coding activity for a summary day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryGrandTotal {
    /// Total coding activity in digital clock format.
    pub digital: String,
    /// Hours portion of coding activity for this entity.
    pub hours: u32,
    /// Minutes portion of coding activity for this entity.
    pub minutes: u32,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Total coding activity as seconds.
    pub total_seconds: f64,
}

/// Coding activity broken down for a single entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryEntity {
    /// Entity name.
    pub name: String,
    /// Total coding activity spent in this entity as seconds.
    pub total_seconds: f64,
    /// Percent of time spent in this entity.
    pub percent: f64,
    /// Total coding activity for this entity in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this entity.
    pub hours: u32,
    /// Minutes portion of coding activity for this entity.
    pub minutes: u32,
    /// Seconds portion of coding activity for this entity.
    pub seconds: u32,
}

/// The time range covered by a single summary day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryRange {
    /// This day as Date string in YEAR-MONTH-DAY format.
    pub date: String,
    /// Start of this day as ISO 8601 UTC datetime.
    pub start: String,
    /// End of this day as ISO 8601 UTC datetime.
    pub end: String,
    /// This day in human-readable format relative to the current day.
    pub text: String,
    /// Timezone used in Olson Country/Region format.
    pub timezone: String,
}

/// Cumulative coding activity total over the date range of summaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CumulativeTotalSummary {
    /// Cumulative number of seconds over the date range of summaries.
    pub seconds: f64,
    /// Cumulative total coding activity in human readable format.
    pub text: String,
    /// Cumulative total as a decimal.
    pub decimal: String,
    /// Cumulative total in digital clock format.
    pub digital: String,
}

/// Daily average coding activity over the date range of summaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAverageSummary {
    /// Number of days in this range with no coding time logged.
    pub holidays: u32,
    /// Number of days in this range.
    pub days_including_holidays: u32,
    /// Number of days in this range excluding days with no activity.
    pub days_minus_holidays: u32,
    /// Average coding activity per day as seconds for the given range of time, excluding Other language.
    pub seconds: f64,
    /// Daily average, excluding Other language, as human readable string.
    pub text: String,
    /// Average coding activity per day as seconds for the given range of time.
    pub seconds_including_other_language: f64,
    /// Daily average as human readable string.
    pub text_including_other_language: String,
}
