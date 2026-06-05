//! Contains the models for [`WakaTimeClient::org_dashboard_durations`](crate::WakaTimeClient::org_dashboard_durations).

use crate::model::durations::Duration;
use crate::model::org_dashboards::DashboardMember;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgDashboardDurations {
    /// Per-member durations for the requested day.
    pub data: Vec<MemberDurations>,
}

/// The durations of a single dashboard member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDurations {
    /// The member these durations belong to.
    pub member: DashboardMember,
    /// The member's durations for the requested day.
    pub durations: Vec<Duration>,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Timezone used in Olson Country/Region format.
    pub timezone: Option<String>,
}
