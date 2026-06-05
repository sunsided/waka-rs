//! Contains the models for [`WakaTimeClient::custom_rules`](crate::WakaTimeClient::custom_rules)
//! and [`WakaTimeClient::custom_rules_progress`](crate::WakaTimeClient::custom_rules_progress).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRules {
    /// The custom rules of the user.
    pub data: Vec<CustomRule>,
    /// Id of the background job applying rule changes, if one is running.
    pub job_id: Option<String>,
}

/// A rule that changes or deletes coding activity matching its source condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    /// Unique id of this rule.
    pub id: String,
    /// What the rule does with matching activity: `change` or `delete`.
    pub action: Option<String>,
    /// The attribute matched against, e.g. `project` or `entity`.
    pub source: Option<String>,
    /// How the source is matched: `equals`, `contains`, `starts with` or `ends with`.
    pub operation: Option<String>,
    /// The value the source is matched against.
    pub source_value: Option<String>,
    /// The attributes modified when the rule matches.
    pub destinations: Option<Vec<CustomRuleDestination>>,
    /// Order in which this rule is applied.
    pub priority: Option<i64>,
    /// Time when this rule was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this rule was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRuleDestination {
    /// Unique id of this destination.
    pub id: Option<String>,
    /// The attribute being modified.
    pub destination: Option<String>,
    /// The new value for the attribute.
    pub destination_value: Option<String>,
}

/// Progress of a background job applying custom rule changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRulesProgress {
    /// Progress of the job as a number between 0 and 100.
    pub progress: Option<u32>,
    /// Id of the background job.
    pub job_id: Option<String>,
}
