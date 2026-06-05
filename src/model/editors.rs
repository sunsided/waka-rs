//! Contains the models for [`WakaTimeClient::editors`](crate::WakaTimeClient::editors).

use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::editors`](crate::WakaTimeClient::editors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editors {
    /// The editors with WakaTime plugin support.
    pub data: Vec<Editor>,
}

/// An editor with WakaTime plugin support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editor {
    /// Unique id of this editor.
    pub id: String,
    /// Display name of this editor.
    pub name: String,
    /// Brand color of this editor as hex string.
    pub color: Option<String>,
    /// Website of this editor.
    pub website: Option<String>,
    /// Repository url of this editor's WakaTime plugin.
    pub repository: Option<String>,
    /// Latest version of this editor's WakaTime plugin.
    pub version: Option<String>,
    /// URL of the latest plugin version.
    pub version_url: Option<String>,
    /// URL of the plugin's release history.
    pub history_url: Option<String>,
    /// Whether the plugin for this editor has been released.
    pub released: Option<bool>,
    /// Whether this editor is hidden from the plugin list.
    pub hidden: Option<bool>,
}
