//! Contains the models for [`WakaTimeClient::program_languages`](crate::WakaTimeClient::program_languages).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramLanguages {
    /// The programming languages known to WakaTime.
    pub data: Vec<ProgramLanguage>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramLanguage {
    /// Unique id of this language.
    pub id: String,
    /// Name of this language.
    pub name: String,
    /// Alternative names of this language.
    pub aliases: Option<Vec<String>>,
    /// File extensions associated with this language.
    pub extensions: Option<Vec<String>>,
    /// Color of this language as hex string.
    pub color: Option<String>,
    /// Whether this language is a verified, well-known language.
    pub is_verified: Option<bool>,
    /// Time when this language was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when this language was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
}
