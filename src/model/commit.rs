//! Contains the models for [`WakaTimeClient::commit`](crate::WakaTimeClient::commit).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commits {
    pub commit: Commit,
    /// Branch name containing the commit.
    pub branch: String,
    pub project: Project,
    /// Project's sync status.
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// URL of author's avatar image.
    pub author_avatar_url: String,
    /// Time when commit was authored in ISO 8601 format.
    pub author_date: String,
    /// Email address of author.
    pub author_email: String,
    /// Unique id of the author's WakaTime user, when matched.
    pub author_id: Option<String>,
    /// Link to author's profile on GitHub, Bitbucket, GitLab, etc.
    pub author_html_url: String,
    /// Name of author.
    pub author_name: String,
    /// API url for author's profile.
    pub author_url: String,
    /// Author's username.
    pub author_username: String,
    /// Branch name, for ex: `main`.
    pub branch: String,
    /// URL of committer's avatar image.
    pub committer_avatar_url: String,
    /// Commit time in ISO 8601 format.
    pub committer_date: String,
    /// Email address of committer.
    pub committer_email: String,
    /// Link to committer's profile on GitHub, Bitbucket, GitLab, etc.
    pub committer_html_url: String,
    /// Name of committer.
    pub committer_name: String,
    /// API url for committer's profile.
    pub committer_url: String,
    /// Committer's username.
    pub committer_username: String,
    /// Time commit was synced in ISO 8601 format.
    pub created_at: String,
    /// Revision control hash of this commit.
    pub hash: String,
    /// Link to an html page with details about current commit.
    pub html_url: String,
    /// Commit time in human-readable format.
    pub human_readable_date: Option<String>,
    /// Commit time relative to the current time, e.g. `2 days ago`.
    pub human_readable_natural_date: Option<String>,
    /// Time coded in editor for this commit.
    pub human_readable_total: String,
    /// Time coded in editor for this commit.
    pub human_readable_total_with_seconds: String,
    /// Unique id of commit.
    pub id: String,
    /// Whether the author was matched to a WakaTime user.
    pub is_author_found: Option<bool>,
    /// Author's description of this commit.
    pub message: String,
    /// `refs/heads/main`.
    pub r#ref: String,
    /// Time coded in editor for this commit.
    pub total_seconds: f64,
    /// Truncated revision control hash of this commit.
    pub truncated_hash: String,
    /// API url with details about current commit.
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique id of project.
    pub id: String,
    /// Project name.
    pub name: String,
    /// Project privacy setting.
    pub privacy: Option<String>,
    pub repository: Repository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Associated repository badge if enabled, otherwise `null`.
    pub badge: Option<serde_json::Value>,
    /// Time when this repository was created in ISO 8601 format.
    pub created_at: Option<String>,
    /// Default branch if given for this repo.
    pub default_branch: String,
    /// remote repository description.
    pub description: String,
    /// Number of repo forks if available.
    pub fork_count: u64,
    /// Username and repo name, ex: `wakatime/wakadump`.
    pub full_name: String,
    /// Homepage of repository.
    pub homepage: Option<String>,
    /// HTML url for repository.
    pub html_url: Option<String>,
    /// Unique id of repository.
    pub id: String,
    /// URL of the repository provider's icon image.
    pub image_icon_url: Option<String>,
    /// Whether this repo is a fork or original.
    pub is_fork: bool,
    /// Whether this repo is private or public.
    pub is_private: bool,
    /// Last time this repo was synced with remote provider ISO 8601 format.
    pub last_synced_at: Option<String>,
    /// Time when this repository was last modified in ISO 8601 format.
    pub modified_at: Option<String>,
    /// Repository name.
    pub name: String,
    /// Remote provider of repository, ex: `github`.
    pub provider: String,
    /// Number of repo stars if available.
    pub star_count: u64,
    /// API url of remote repository.
    pub url: String,
    /// Repository name URL-encoded.
    pub urlencoded_name: Option<String>,
    /// Name of the WakaTime project connected to this repository.
    pub wakatime_project_name: Option<String>,
    /// Number of watchers of repo if available.
    pub watch_count: u64,
}
