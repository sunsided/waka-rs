//! Contains the models for [`WakaTimeClient::org_dashboard_summaries`](crate::WakaTimeClient::org_dashboard_summaries)
//! and [`WakaTimeClient::org_dashboard_member_summaries`](crate::WakaTimeClient::org_dashboard_member_summaries).

use crate::model::org_dashboards::DashboardMember;
use crate::model::summaries::{
    DailyAverageSummary, SummaryBranch, SummaryEditor, SummaryEntity, SummaryGrandTotal,
    SummaryLanguage, SummaryOperatingSystem, SummaryProject, SummaryRange,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgDashboardSummaries {
    /// Per-member summaries for the requested day.
    pub data: Vec<OrgSummary>,
    /// Cumulative total over all members.
    pub cumulative_total: Option<OrgCumulativeTotal>,
}

/// A single day of coding activity of one dashboard member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgSummary {
    /// The member this summary belongs to.
    pub member: Option<DashboardMember>,
    pub grand_total: SummaryGrandTotal,
    pub projects: Option<Vec<SummaryProject>>,
    pub languages: Option<Vec<SummaryLanguage>>,
    pub editors: Option<Vec<SummaryEditor>>,
    pub operating_systems: Option<Vec<SummaryOperatingSystem>>,
    /// Included only when the project url parameter is used.
    pub branches: Option<Vec<SummaryBranch>>,
    /// Included only when the project url parameter is used.
    pub entities: Option<Vec<SummaryEntity>>,
    pub range: SummaryRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMemberSummaries {
    /// The member's summaries, one per day in the requested range.
    pub data: Vec<OrgSummary>,
    /// Start of time range as ISO 8601 UTC datetime.
    pub start: Option<String>,
    /// End of time range as ISO 8601 UTC datetime.
    pub end: Option<String>,
    /// Privacy of the member's personal profile: `visible` or `hidden`.
    pub default_personal_privacy: Option<String>,
    /// Cumulative total over the date range.
    pub cumulative_total: Option<OrgCumulativeTotal>,
    /// Daily averages over the date range.
    pub daily_average: Option<DailyAverageSummary>,
}

/// Cumulative coding activity total of an org dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCumulativeTotal {
    /// Cumulative number of seconds.
    pub seconds: f64,
    /// Cumulative total in human-readable format.
    pub text: Option<String>,
}
