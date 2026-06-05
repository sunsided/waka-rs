//! Contains the models for [`WakaTimeClient::stats_aggregated`](crate::WakaTimeClient::stats_aggregated).

use serde::{Deserialize, Serialize};

/// Aggregate stats of all WakaTime users over a time range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStats {
    /// Aggregated total coding activity over all users.
    pub total: Option<AggregatedMeasures>,
    /// Aggregated daily averages over all users.
    pub daily_average: Option<AggregatedMeasures>,
    /// Aggregated coding activity per category.
    pub categories: Option<Vec<AggregatedEntry>>,
    /// Aggregated coding activity per editor.
    pub editors: Option<Vec<AggregatedEntry>>,
    /// Aggregated coding activity per language.
    pub languages: Option<Vec<AggregatedEntry>>,
    /// Aggregated coding activity per operating system.
    pub operating_systems: Option<Vec<AggregatedEntry>>,
}

/// Aggregated coding activity for a single entity, e.g. a language or editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedEntry {
    /// Name of the aggregated entity.
    pub name: String,
    /// Whether this entity is a verified, well-known entity.
    pub is_verified: Option<bool>,
    /// The aggregated measures for this entity.
    #[serde(flatten)]
    pub measures: AggregatedMeasures,
}

/// Statistical measures aggregated over all users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMeasures {
    /// Average over all users.
    pub average: Option<AggregatedValue>,
    /// Approximate number of users included.
    pub count: Option<AggregatedValue>,
    /// Maximum over all users.
    pub max: Option<AggregatedValue>,
    /// Median over all users.
    pub median: Option<AggregatedValue>,
    /// Sum over all users.
    pub sum: Option<AggregatedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedValue {
    /// The value as seconds; absent for counts.
    pub seconds: Option<f64>,
    /// The value in human-readable format.
    pub text: Option<String>,
}
