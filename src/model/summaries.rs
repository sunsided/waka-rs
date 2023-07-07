use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summaries {
    pub data: Vec<Summary>,
    pub cumulative_total: CumulativeTotalSummary,
    pub daily_average: DailyAverageSummary,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: String,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub grand_total: SummaryGrandTotal,
    pub categories: Vec<SummaryCategory>,
    pub projects: Vec<SummaryProject>,
    pub languages: Vec<SummaryLanguage>,
    pub editors: Vec<SummaryEditor>,
    pub operating_systems: Vec<SummaryOperatingSystem>,
    pub dependencies: Vec<SummaryDependency>,
    pub machines: Vec<SummaryMachine>,
    /// Included only when project url parameter used.
    pub branches: Option<Vec<SummaryBranch>>,
    /// Included only when project url parameter used.
    pub entities: Option<Vec<SummaryEntity>>,
    pub range: SummaryRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryCategory {
    /// Name of category, for ex: Coding or Debugging.
    pub name: String,
    /// Total coding activity as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this category.
    pub percent: f32,
    /// Total coding activity for this category in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this category.
    pub hours: u32,
    /// Minutes portion of coding activity for this category.
    pub minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryProject {
    /// Project name.
    pub name: String,
    /// Total coding activity as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this project.
    pub percent: f32,
    /// Total coding activity for this project in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this project.
    pub hours: u32,
    /// Minutes portion of coding activity for this project.
    pub minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryLanguage {
    /// Language name.
    pub name: String,
    /// Total coding activity spent in this language as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this language.
    pub percent: f32,
    /// Total coding activity for this language in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this language.
    pub hours: u32,
    /// Minutes portion of coding activity for this language.
    pub minutes: u8,
    /// Seconds portion of coding activity for this language.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryEditor {
    /// Editor name.
    pub name: String,
    /// Total coding activity spent in this editor as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this editor.
    pub percent: f32,
    /// Total coding activity for this editor in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this editor.
    pub hours: u32,
    /// Minutes portion of coding activity for this editor.
    pub minutes: u8,
    /// Seconds portion of coding activity for this editor.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryOperatingSystem {
    /// OS name.
    pub name: String,
    /// Total coding activity spent in this OS as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this OS.
    pub percent: f32,
    /// Total coding activity for this OS in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this OS.
    pub hours: u32,
    /// Minutes portion of coding activity for this OS.
    pub minutes: u8,
    /// Seconds portion of coding activity for this OS.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryDependency {
    /// Dependency name.
    pub name: String,
    /// Total coding activity spent in this dependency as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this dependency.
    pub percent: f32,
    /// Total coding activity for this dependency in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this dependency.
    pub hours: u32,
    /// Minutes portion of coding activity for this dependency.
    pub minutes: u8,
    /// Seconds portion of coding activity for this dependency.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMachine {
    /// Machine hostname and ip address.
    pub name: String,
    /// Unique id of this machine.
    pub machine_name_id: String,
    /// Total coding activity spent on this machine as seconds.
    pub total_seconds: f32,
    /// Percent of time spent on this machine.
    pub percent: f32,
    /// Total coding activity for this machine in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this machine.
    pub hours: u32,
    /// Minutes portion of coding activity for this machine.
    pub minutes: u8,
    /// Seconds portion of coding activity for this machine.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryBranch {
    /// Branch name.
    pub name: String,
    /// Total coding activity spent in this branch as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this branch.
    pub percent: f32,
    /// Total coding activity for this branch in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this branch.
    pub hours: u32,
    /// Minutes portion of coding activity for this branch.
    pub minutes: u8,
    /// Seconds portion of coding activity for this branch.
    pub seconds: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryGrandTotal {
    /// Total coding activity in digital clock format.
    pub digital: String,
    /// Hours portion of coding activity for this entity.
    pub hours: u32,
    /// Minutes portion of coding activity for this entity.
    pub minutes: u8,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Total coding activity as seconds.
    pub total_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryEntity {
    /// Entity name.
    pub name: String,
    /// Total coding activity spent in this entity as seconds.
    pub total_seconds: f32,
    /// Percent of time spent in this entity.
    pub percent: f32,
    /// Total coding activity for this entity in digital clock format.
    pub digital: String,
    /// Total coding activity in human readable format.
    pub text: String,
    /// Hours portion of coding activity for this entity.
    pub hours: u32,
    /// Minutes portion of coding activity for this entity.
    pub minutes: u8,
    /// Seconds portion of coding activity for this entity.
    pub seconds: u8,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CumulativeTotalSummary {
    /// Cumulative number of seconds over the date range of summaries.
    pub seconds: f32,
    /// Cumulative total coding activity in human readable format.
    pub text: String,
    /// Cumulative total as a decimal.
    pub decimal: String,
    /// Cumulative total in digital clock format.
    pub digital: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAverageSummary {
    /// Number of days in this range with no coding time logged.
    pub holidays: u32,
    /// Number of days in this range.
    pub days_including_holidays: u32,
    /// Number of days in this range excluding days with no activity.
    pub days_minus_holidays: u32,
    /// Average coding activity per day as seconds for the given range of time, excluding Other language.
    pub seconds: f32,
    /// Daily average, excluding Other language, as human readable string.
    pub text: String,
    /// Average coding activity per day as seconds for the given range of time.
    pub seconds_including_other_language: f32,
    /// Daily average as human readable string.
    pub text_including_other_language: String,
}
