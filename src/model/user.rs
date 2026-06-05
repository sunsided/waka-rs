//! Contains the models for [`WakaTimeClient::user`](crate::WakaTimeClient::user).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique id of user.
    pub id: String,
    /// Users public username.
    pub username: Option<String>,
    /// Display name of this user taken from full_name or @username; defaults to "Anonymous User".
    pub display_name: Option<String>,
    /// Full name of user.
    pub full_name: Option<String>,
    /// Email address of user.
    pub email: Option<String>,
    /// URL of photo for this user.
    pub photo: Option<String>,
    /// Whether this user's email should be shown on the public leaderboard.
    pub is_email_public: Option<bool>,
    /// Whether this user's email address has been verified with a confirmation email.
    pub is_email_confirmed: Option<bool>,
    /// User's timezone in Olson Country/Region format.
    pub timezone: Option<String>,
    /// Time of most recent heartbeat received in ISO 8601 format.
    pub last_heartbeat_at: Option<String>,
    /// User-agent string from the last plugin used.
    pub last_plugin: Option<String>,
    /// Name of editor last used.
    pub last_plugin_name: Option<String>,
    /// Name of last project coded in.
    pub last_project: Option<String>,
    /// Users subscription plan.
    pub plan: Option<String>,
    /// Website of user.
    pub website: Option<String>,
    /// Website of user without protocol part.
    pub human_readable_website: Option<String>,
    /// Location of user.
    pub location: Option<String>,
    /// Time when user was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Time when user was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
    /// Whether this user's photo should be shown on the public leaderboard.
    pub photo_public: Option<bool>,
    /// Whether this user is hireable.
    pub is_hireable: Option<bool>,
    /// Whether this user has access to premium features.
    pub has_premium_features: Option<bool>,
    /// Whether this user's languages used should be shown publicly.
    pub languages_used_public: Option<bool>,
    /// Whether this user's coding activity should be shown publicly.
    pub logged_time_public: Option<bool>,
}
