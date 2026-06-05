//! Contains the models for [`WakaTimeClient::stats`](crate::WakaTimeClient::stats).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Total coding activity as seconds for the given range of time.
    pub total_seconds: Option<f64>,
    /// Average coding activity per day as seconds for the given range of time, excluding Other language.
    pub daily_average: Option<f64>,
    /// Number of days in this range, including days with no coding time logged.
    pub days_including_holidays: Option<u32>,
    /// Number of days in this range excluding days with no coding time logged.
    pub days_minus_holidays: Option<u32>,
    /// Number of days in this range with no coding time logged.
    pub holidays: Option<u32>,
    /// Status of these stats in the cache, either `ok` or `pending_update`.
    pub status: String,
    /// `true` if these stats are up to date; when `false`, stats are missing or from an old time range and will be refreshed soon.
    pub is_up_to_date: bool,
    /// `true` if this user's coding activity is publicly visible.
    pub is_coding_activity_visible: Option<bool>,
    /// Time range of these stats, e.g. `last_7_days`.
    pub range: String,
    /// Time range of these stats in human-readable format.
    pub human_readable_range: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
    /// Coding activity per language.
    pub languages: Option<Vec<StatsAggregate>>,
    /// Coding activity per editor.
    pub editors: Option<Vec<StatsAggregate>>,
    /// Coding activity per operating system.
    pub operating_systems: Option<Vec<StatsAggregate>>,
    /// Coding activity per category, e.g. Coding or Debugging.
    pub categories: Option<Vec<StatsAggregate>>,
    /// Coding activity per project.
    pub projects: Option<Vec<StatsAggregate>>,
    /// Coding activity per machine.
    pub machines: Option<Vec<StatsAggregate>>,
    /// Coding activity per language dependency.
    pub dependencies: Option<Vec<StatsAggregate>>,
    /// The day with the most coding time logged in this range.
    pub best_day: Option<StatsBestDay>,
    /// Unique id of the user.
    pub user_id: Option<String>,
    /// The user's public username.
    pub username: Option<String>,
    /// Time when these stats were created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when these stats were last updated in ISO 8601 format.
    pub modified_at: Option<String>,
}

/// An aggregated coding activity entry, e.g. for a language, editor or project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsAggregate {
    /// Name of the aggregated entity, e.g. the language or editor name.
    pub name: String,
    /// Total coding activity as seconds.
    pub total_seconds: f64,
    /// Percent of total time spent in this entity.
    pub percent: Option<f64>,
    /// Total coding activity in digital clock format.
    pub digital: Option<String>,
    /// Total coding activity in human-readable format.
    pub text: Option<String>,
    /// Hours portion of the coding activity.
    pub hours: Option<u32>,
    /// Minutes portion of the coding activity.
    pub minutes: Option<u32>,
    /// Seconds portion of the coding activity.
    pub seconds: Option<u32>,
}

/// The day with the most coding time logged in the range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsBestDay {
    /// Day with most coding time logged as Date string in YEAR-MONTH-DAY format.
    pub date: String,
    /// Total coding activity in human-readable format.
    pub text: String,
    /// Number of seconds of coding activity, including other language, for the day with most coding time logged.
    pub total_seconds: f64,
}
