use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTimeSinceToday {
    /// Total coding activity in decimal format.
    pub decimal: String,
    /// Total coding activity in digital clock format.
    pub digital: String,
    /// `true` if the stats are up to date; when `false`, a 202 response code is returned and stats will be refreshed soon.
    pub is_up_to_date: bool,
    /// A number between 0 and 100 where 100 means the stats are up to date including Today’s time.
    pub percent_calculated: Option<u8>,
    pub range: AllTimeSinceTodayRange,
    /// Total time logged since account created as human readable string.
    pub text: String,
    /// Keystroke timeout setting in minutes.
    pub timeout: u32,
    /// Total number of seconds logged since account created.
    pub total_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTimeSinceTodayRange {
    /// End of today as ISO 8601 UTC datetime.
    pub end: String,
    /// Today as Date string in YEAR-MONTH-DAY format.
    pub end_date: String,
    /// Today in human-readable format.
    pub end_text: String,
    /// Start of user created day as ISO 8601 UTC datetime.
    pub start: String,
    /// Day user was created in YEAR-MONTH-DAY format.
    pub start_date: String,
    /// Day user was created in human-readable format.
    pub start_text: String,
    /// Timezone used in Olson Country/Region format.
    pub timezone: String,
}
