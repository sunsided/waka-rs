//! Contains the models for [`WakaTimeClient::commits`](crate::WakaTimeClient::commits).

use crate::model::commit::{Commit, Project};
use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsPage {
    /// The commits on this page.
    ///
    /// The live API returns this under the `commits` key while the docs
    /// show `data`; both are accepted.
    #[serde(alias = "commits")]
    pub data: Vec<Commit>,
    /// Author filter applied to this page, if any.
    pub author: Option<String>,
    /// Branch name containing the commits.
    pub branch: Option<String>,
    pub project: Option<Project>,
    /// Project's sync status.
    pub status: Option<String>,
    /// Number of the next page, if any.
    pub next_page: Option<u32>,
    /// URL of the next page, if any.
    pub next_page_url: Option<String>,
    /// Number of the previous page, if any.
    pub prev_page: Option<u32>,
    /// URL of the previous page, if any.
    pub prev_page_url: Option<String>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}
