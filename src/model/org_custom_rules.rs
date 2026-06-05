//! Contains the models for [`WakaTimeClient::org_custom_rules`](crate::WakaTimeClient::org_custom_rules).

use crate::model::custom_rules::CustomRuleDestination;
use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::org_custom_rules`](crate::WakaTimeClient::org_custom_rules).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCustomRules {
    /// The custom rules of the organization.
    pub data: Vec<OrgCustomRule>,
}

/// A rule that changes or deletes coding activity of org members.
///
/// Like a [`CustomRule`](crate::model::custom_rules::CustomRule), but the
/// documented org variant carries no id or timestamps, so all fields are
/// optional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCustomRule {
    /// Unique id of this rule, if provided.
    pub id: Option<String>,
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
}
