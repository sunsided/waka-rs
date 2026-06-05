//! Command-line argument definitions.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line client for the WakaTime API.
#[derive(Debug, Parser)]
#[command(name = "waka-cli", version, about, propagate_version = true)]
pub struct Cli {
    /// WakaTime API key; see <https://wakatime.com/api-key>
    #[arg(long, env = "WAKATIME_API_KEY", global = true, hide_env_values = true)]
    pub api_key: Option<String>,

    /// OAuth 2.0 access token, used as Bearer auth
    #[arg(
        long,
        env = "WAKATIME_BEARER_TOKEN",
        global = true,
        hide_env_values = true,
        conflicts_with = "api_key"
    )]
    pub bearer_token: Option<String>,

    /// The user to query; defaults to "current"
    #[arg(long, env = "WAKATIME_USER", global = true)]
    pub user: Option<String>,

    /// Override the API base URL, e.g. for a mock server
    #[arg(long, env = "WAKATIME_BASE_URL", global = true)]
    pub base_url: Option<String>,

    /// Request timeout in seconds
    #[arg(long, env = "WAKATIME_TIMEOUT", global = true)]
    pub timeout: Option<u64>,

    /// Print the raw API response as JSON to stdout
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// The available subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show the user's profile
    User,

    /// Total time logged since the account was created
    AllTime {
        /// Only show stats for this project
        #[arg(long)]
        project: Option<String>,
    },

    /// Coding activity stats for a time range
    Stats {
        /// Time range: last_7_days, last_30_days, last_6_months, last_year, all_time or a year like 2024
        range: String,
        /// Keystroke timeout preference in minutes
        #[arg(long)]
        keystroke_timeout: Option<u32>,
        /// Only count heartbeats triggered by writes
        #[arg(long)]
        writes_only: bool,
        /// Show more detailed stats limited to this project
        #[arg(long)]
        project: Option<String>,
    },

    /// Aggregate stats of all WakaTime users for a time range
    StatsAggregated {
        /// Time range, e.g. last_7_days
        range: String,
    },

    /// Daily coding activity summaries for a date range
    Summaries {
        /// Start date, e.g. 2026-01-01
        start: String,
        /// End date, e.g. 2026-01-08
        end: String,
        /// Only show summaries for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Keystroke timeout preference in minutes
        #[arg(long)]
        keystroke_timeout: Option<u32>,
        /// Only count heartbeats triggered by writes
        #[arg(long)]
        writes_only: bool,
        /// Timezone for the given dates
        #[arg(long)]
        timezone: Option<String>,
        /// Alternative way to supply start and end dates, e.g. "Last 7 Days"
        #[arg(long)]
        range: Option<String>,
    },

    /// Coding activity for a single day as an array of durations
    Durations {
        /// The date, e.g. 2026-06-01
        date: String,
        /// Only show durations for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Keystroke timeout preference in minutes
        #[arg(long)]
        keystroke_timeout: Option<u32>,
        /// Only count heartbeats triggered by writes
        #[arg(long)]
        writes_only: bool,
        /// Timezone for the given date
        #[arg(long)]
        timezone: Option<String>,
        /// Primary key to slice durations by; defaults to entity
        #[arg(long)]
        slice_by: Option<String>,
    },

    /// An insight about the user's coding activity
    Insights {
        /// Insight type: weekdays, days, best_day, daily_average, projects, languages, editors,
        /// categories, machines or operating_systems
        insight_type: String,
        /// Time range, e.g. last_7_days
        range: String,
        /// Keystroke timeout preference in minutes
        #[arg(long)]
        keystroke_timeout: Option<u32>,
        /// Only count heartbeats triggered by writes
        #[arg(long)]
        writes_only: bool,
        /// Filter to a day of the week, either 0-6 or a weekday name
        #[arg(long)]
        weekday: Option<String>,
    },

    /// List the user's projects
    Projects {
        /// Filter project names by a search term
        #[arg(long)]
        query: Option<String>,
        /// Page number
        #[arg(long, conflicts_with = "all")]
        page: Option<u32>,
        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// List commits of a project, including the time spent on them
    Commits {
        /// The project name
        project: String,
        /// Filter commits by author
        #[arg(long)]
        author: Option<String>,
        /// Filter commits to a branch
        #[arg(long)]
        branch: Option<String>,
        /// Page number
        #[arg(long, conflicts_with = "all")]
        page: Option<u32>,
        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// Show a single commit of a project, including the time spent on it
    Commit {
        /// The project name
        project: String,
        /// The commit hash
        hash: String,
        /// Filter the commit to a branch
        #[arg(long)]
        branch: Option<String>,
    },

    /// List the user's goals
    Goals,

    /// Show a single goal
    Goal {
        /// The goal id
        id: String,
    },

    /// The public leaderboard of users ranked by coding activity
    Leaders {
        /// Filter leaders by language
        #[arg(long)]
        language: Option<String>,
        /// Filter leaders by the hireable badge
        #[arg(long)]
        hireable: bool,
        /// Filter leaders by a two-character country code
        #[arg(long)]
        country_code: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },

    /// List the user's private leaderboards
    Leaderboards,

    /// The leaders of a private leaderboard
    Leaderboard {
        /// The leaderboard id
        board: String,
        /// Filter leaders by language
        #[arg(long)]
        language: Option<String>,
        /// Filter leaders by a two-character country code
        #[arg(long)]
        country_code: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },

    /// List the machines the user has logged coding activity from
    MachineNames,

    /// List the plugin user agents the user has logged coding activity from
    UserAgents,

    /// List the editors with WakaTime plugins
    Editors {
        /// Include editors with unreleased plugins
        #[arg(long)]
        unreleased: bool,
    },

    /// List the programming languages known to WakaTime
    ProgramLanguages,

    /// WakaTime service metadata, e.g. server IP addresses
    Meta,

    /// Today's coding activity, as used by editor status bars
    StatusBar,

    /// Inspect, send or delete heartbeats
    Heartbeats {
        #[command(subcommand)]
        command: HeartbeatsCommand,
    },

    /// Inspect, log or delete external durations
    ExternalDurations {
        #[command(subcommand)]
        command: ExternalDurationsCommand,
    },

    /// Inspect or request data dump exports
    DataDumps {
        #[command(subcommand)]
        command: DataDumpsCommand,
    },

    /// Inspect or modify custom rules
    CustomRules {
        #[command(subcommand)]
        command: CustomRulesCommand,
    },

    /// Organizations and their dashboards
    Org {
        #[command(subcommand)]
        command: OrgCommand,
    },
}

/// Subcommands for heartbeats.
#[derive(Debug, Subcommand)]
pub enum HeartbeatsCommand {
    /// List the heartbeats of a single day
    List {
        /// The date, e.g. 2026-06-01
        date: String,
    },
    /// Send a single heartbeat representing coding activity
    Send {
        /// The entity the heartbeat is logging time against, e.g. a file path or domain
        #[arg(long)]
        entity: String,
        /// Type of entity: file, app, url or domain
        #[arg(long = "type", value_name = "TYPE")]
        entity_type: String,
        /// UNIX epoch timestamp; defaults to now
        #[arg(long)]
        time: Option<f64>,
        /// Category, e.g. coding, debugging or building
        #[arg(long)]
        category: Option<String>,
        /// Project name
        #[arg(long)]
        project: Option<String>,
        /// Branch name
        #[arg(long)]
        branch: Option<String>,
        /// Language name
        #[arg(long)]
        language: Option<String>,
        /// Total number of lines in the entity
        #[arg(long)]
        lines: Option<u64>,
        /// Current cursor line number
        #[arg(long)]
        lineno: Option<u64>,
        /// Current cursor column position
        #[arg(long)]
        cursorpos: Option<u64>,
        /// Mark the heartbeat as triggered by a write
        #[arg(long)]
        is_write: bool,
    },
    /// Send multiple heartbeats from a JSON array; at most 25 per request
    SendBulk {
        /// Path to a JSON file with an array of heartbeats; reads stdin when omitted or "-"
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Delete heartbeats; all ids must be from the given day
    Delete {
        /// The date the heartbeats belong to
        date: String,
        /// The heartbeat ids to delete
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

/// Subcommands for external durations.
#[derive(Debug, Subcommand)]
pub enum ExternalDurationsCommand {
    /// List the external durations of a single day
    List {
        /// The date, e.g. 2026-06-01
        date: String,
        /// Only show durations for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Timezone for the given date
        #[arg(long)]
        timezone: Option<String>,
    },
    /// Log time spent in an external app, e.g. a meeting or code review
    Send {
        /// Unique id of this duration on the external provider
        #[arg(long)]
        external_id: String,
        /// The entity this duration is logging time against
        #[arg(long)]
        entity: String,
        /// Type of entity: file, app, event, url or domain
        #[arg(long = "type", value_name = "TYPE")]
        entity_type: String,
        /// Start of the duration as UNIX epoch
        #[arg(long)]
        start: f64,
        /// End of the duration as UNIX epoch
        #[arg(long)]
        end: f64,
        /// Category, e.g. coding, meeting or code reviewing
        #[arg(long)]
        category: Option<String>,
        /// Project name
        #[arg(long)]
        project: Option<String>,
        /// Branch name
        #[arg(long)]
        branch: Option<String>,
        /// Language name
        #[arg(long)]
        language: Option<String>,
        /// Additional metadata; max 2083 characters
        #[arg(long)]
        meta: Option<String>,
    },
    /// Log multiple external durations from a JSON array; at most 1000 per request
    SendBulk {
        /// Path to a JSON file with an array of external durations; reads stdin when omitted or "-"
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Delete external durations; all ids must be from the given day
    Delete {
        /// The date the durations belong to
        date: String,
        /// The external duration ids to delete
        #[arg(required = true)]
        ids: Vec<String>,
    },
}

/// Subcommands for data dumps.
#[derive(Debug, Subcommand)]
pub enum DataDumpsCommand {
    /// List the user's data dump exports and their status
    List,
    /// Request a data dump export
    Create {
        /// Type of export: heartbeats or daily
        dump_type: String,
        /// Do not send an email when the export is ready
        #[arg(long)]
        no_email: bool,
    },
}

/// Subcommands for custom rules.
#[derive(Debug, Subcommand)]
pub enum CustomRulesCommand {
    /// List the user's custom rules
    List,
    /// Replace the user's custom rules with rules from a JSON array
    Set {
        /// Path to a JSON file with an array of custom rules; reads stdin when omitted or "-"
        #[arg(long)]
        file: Option<PathBuf>,
    },
    /// Delete a custom rule
    Delete {
        /// The rule id
        id: String,
    },
    /// Progress of a custom rules background job
    Progress {
        /// The background job id
        job_id: String,
    },
    /// Clear the progress of a finished custom rules job
    ClearProgress,
}

/// Subcommands for organizations.
#[derive(Debug, Subcommand)]
pub enum OrgCommand {
    /// List the organizations the user belongs to
    List,
    /// List the dashboards of an organization
    Dashboards {
        /// The organization id
        org: String,
    },
    /// List the members of an organization's dashboard
    Members {
        /// The organization id
        org: String,
        /// The dashboard id
        dashboard: String,
    },
    /// A dashboard's coding activity for a single day as durations
    Durations {
        /// The organization id
        org: String,
        /// The dashboard id
        dashboard: String,
        /// The date, e.g. 2026-06-01
        date: String,
        /// Only show durations for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Primary key to slice durations by; defaults to entity
        #[arg(long)]
        slice_by: Option<String>,
    },
    /// A dashboard's coding activity for a single day as a summary
    Summaries {
        /// The organization id
        org: String,
        /// The dashboard id
        dashboard: String,
        /// The date, e.g. 2026-06-01
        date: String,
        /// Only show summaries for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
    },
    /// A dashboard member's coding activity for a single day as durations
    MemberDurations {
        /// The organization id
        org: String,
        /// The dashboard id
        dashboard: String,
        /// The member id
        member: String,
        /// The date, e.g. 2026-06-01
        date: String,
        /// Only show durations for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Primary key to slice durations by; defaults to entity
        #[arg(long)]
        slice_by: Option<String>,
    },
    /// A dashboard member's coding activity for a date range as daily summaries
    MemberSummaries {
        /// The organization id
        org: String,
        /// The dashboard id
        dashboard: String,
        /// The member id
        member: String,
        /// Start date, e.g. 2026-01-01
        start: String,
        /// End date, e.g. 2026-01-08
        end: String,
        /// Only show summaries for this project
        #[arg(long)]
        project: Option<String>,
        /// Comma separated list of branch names
        #[arg(long)]
        branches: Option<String>,
        /// Alternative way to supply start and end dates, e.g. "Last 7 Days"
        #[arg(long)]
        range: Option<String>,
    },
    /// List the custom rules of an organization
    CustomRules {
        /// The organization id
        org: String,
    },
}
