//! # WakaTime API client
//!
//! A client for the [WakaTime](https://wakatime.com/) API, covering every
//! documented endpoint: coding statistics, summaries, durations, heartbeats,
//! goals, insights, leaderboards, organizations, and the write operations.
//!
//! Authenticate with an API key ([`WakaTimeClientBuilder::new_with_api_key`])
//! or an OAuth 2.0 access token ([`WakaTimeClientBuilder::new_with_bearer_token`]).
//!
//! ```no_run
//! use waka::{SummariesOptions, WakaTimeClientBuilder};
//!
//! # async fn test() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = std::env::var("WAKATIME_API_KEY")?;
//! let client = WakaTimeClientBuilder::new_with_api_key(api_key)
//!     .with_user("current")
//!     .build()?;
//!
//! let summary = client
//!     .summaries("2023-01-01", "2023-01-08", SummariesOptions::default())
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]

mod api_error;
mod builder_error;
pub mod model;

pub use crate::api_error::ApiError;
pub use crate::builder_error::BuilderError;
use base64::Engine;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use query_string_builder::QueryString;
use reqwest::header::HeaderValue;
use reqwest::{Client, ClientBuilder, Response, header};
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "https://wakatime.com/api/v1/";
const CURRENT_USER: &str = "current";

/// Characters that must be percent-encoded in a URL path segment (RFC 3986),
/// including `/` so that user-provided values cannot extend the path.
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b'/')
    .add(b'\\');

/// Percent-encodes a user-provided value for use as a single URL path segment.
fn encode_path_segment(segment: &str) -> std::borrow::Cow<'_, str> {
    utf8_percent_encode(segment, PATH_SEGMENT).into()
}

/// A builder for [`WakaTimeClient`] instances.
#[derive(Default)]
pub struct WakaTimeClientBuilder {
    /// The value of the `Authorization` header.
    auth_header: String,
    /// The optional user to use.
    user: Option<String>,
    /// The optional base URL to use instead of the default WakaTime API URL.
    base_url: Option<String>,
    /// The optional request timeout.
    timeout: Option<std::time::Duration>,
}

impl WakaTimeClientBuilder {
    /// Authenticates with an API key using HTTP Basic auth.
    /// See [wakatime.com/api-key](https://wakatime.com/api-key).
    pub fn new_with_api_key<S: AsRef<str>>(api_key: S) -> Self {
        Self {
            auth_header: format!(
                "Basic {api_key}",
                api_key = base64::engine::general_purpose::STANDARD.encode(api_key.as_ref())
            ),
            ..Default::default()
        }
    }

    /// Authenticates with an OAuth 2.0 access token using Bearer auth.
    /// See [wakatime.com/developers#authentication](https://wakatime.com/developers#authentication).
    pub fn new_with_bearer_token<S: AsRef<str>>(token: S) -> Self {
        Self {
            auth_header: format!("Bearer {token}", token = token.as_ref()),
            ..Default::default()
        }
    }

    /// Specifies a user to focus on. If unspecified, `current` is used.
    ///
    /// The value is percent-encoded when building the client, so it can be
    /// passed verbatim.
    pub fn with_user<S: AsRef<str>>(mut self, user: S) -> Self {
        self.user = Some(user.as_ref().to_string());
        self
    }

    /// Sets a timeout for each request, from connecting until the response
    /// body has finished. If unspecified, no timeout applies.
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Overrides the base URL of the WakaTime API, e.g. for testing against a mock server.
    /// If unspecified, `https://wakatime.com/api/v1/` is used.
    pub fn with_base_url<S: AsRef<str>>(mut self, base_url: S) -> Self {
        let base_url = base_url.as_ref();
        // Ensure a trailing slash so path concatenation stays correct.
        self.base_url = Some(if base_url.ends_with('/') {
            base_url.to_string()
        } else {
            format!("{base_url}/")
        });
        self
    }

    /// Builds the [`WakaTimeClient`].
    ///
    /// Fails if the authentication value is not a valid header value or the
    /// underlying HTTP client cannot be constructed.
    pub fn build(self) -> Result<WakaTimeClient, BuilderError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str(&self.auth_header)?);

        let mut builder = ClientBuilder::new().default_headers(headers);
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        let client = builder.build()?;

        let user = self.user.unwrap_or_else(|| CURRENT_USER.to_string());
        Ok(WakaTimeClient {
            client,
            user: encode_path_segment(&user).into_owned(),
            base_url: self.base_url.unwrap_or_else(|| BASE_URL.to_string()),
        })
    }
}

/// A client for accessing the WakaTime API.
pub struct WakaTimeClient {
    /// The HTTP client to use.
    client: Client,
    /// The user to use.
    user: String,
    /// The base URL of the API.
    base_url: String,
}

impl WakaTimeClient {
    /// Fetches the total time logged since the account was created.
    ///
    /// ## Documentation
    /// * [All Time Since Today](https://wakatime.com/developers#all_time_since_today)
    pub async fn all_time_since_today<'a>(
        &self,
        options: AllTimesSinceTodayOptions<'a>,
    ) -> Result<model::AllTimeSinceToday, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/all_time_since_today{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::AllTimeSinceToday>| r.data).await
    }

    /// Fetches a single commit of a project, including the time spent on it.
    ///
    /// ## Documentation
    /// * [Commits](https://wakatime.com/developers#commits)
    pub async fn commit<'a>(
        &self,
        project: &str,
        hash: &str,
        options: CommitOptions<'a>,
    ) -> Result<model::Commits, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/projects/{project}/commits/{hash}{qs}",
            base_url = self.base_url,
            user = self.user,
            project = encode_path_segment(project),
            hash = encode_path_segment(hash)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's coding activity for the given time range as daily summaries.
    ///
    /// ## Documentation
    /// * [Summaries](https://wakatime.com/developers#summaries)
    pub async fn summaries<'a>(
        &self,
        start: &str,
        end: &str,
        options: SummariesOptions<'a>,
    ) -> Result<model::Summaries, ApiError> {
        let qs = options
            .into_query_string()
            .with_value("start", start)
            .with_value("end", end);
        let url = format!(
            "{base_url}users/{user}/summaries{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches one page of commits of a project, including the time spent on them.
    ///
    /// ## Documentation
    /// * [Commits](https://wakatime.com/developers#commits)
    pub async fn commits<'a>(
        &self,
        project: &str,
        options: CommitsOptions<'a>,
    ) -> Result<model::CommitsPage, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/projects/{project}/commits{qs}",
            base_url = self.base_url,
            user = self.user,
            project = encode_path_segment(project)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's coding activity for the given day as an array of durations.
    ///
    /// ## Documentation
    /// * [Durations](https://wakatime.com/developers#durations)
    pub async fn durations<'a>(
        &self,
        date: &str,
        options: DurationsOptions<'a>,
    ) -> Result<model::Durations, ApiError> {
        let qs = options.into_query_string().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/durations{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's heartbeats sent from plugins for the given day.
    ///
    /// ## Documentation
    /// * [Heartbeats](https://wakatime.com/developers#heartbeats)
    pub async fn heartbeats(&self, date: &str) -> Result<model::Heartbeats, ApiError> {
        let qs = QueryString::dynamic().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/heartbeats{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Sends a single heartbeat representing coding activity.
    ///
    /// ## Documentation
    /// * [Heartbeats](https://wakatime.com/developers#heartbeats)
    pub async fn send_heartbeat(
        &self,
        heartbeat: &model::HeartbeatInput,
    ) -> Result<model::CreatedHeartbeat, ApiError> {
        let url = format!(
            "{base_url}users/{user}/heartbeats",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.post(url).json(heartbeat).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::CreatedHeartbeat>| r.data).await
    }

    /// Sends multiple heartbeats at once; at most 25 per request.
    ///
    /// The per-heartbeat results are returned as raw JSON since their shape
    /// is not fully documented.
    ///
    /// ## Documentation
    /// * [Heartbeats](https://wakatime.com/developers#heartbeats)
    pub async fn send_heartbeats(
        &self,
        heartbeats: &[model::HeartbeatInput],
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!(
            "{base_url}users/{user}/heartbeats.bulk",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.post(url).json(heartbeats).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Deletes the given heartbeats; all ids must be from the given day.
    ///
    /// ## Documentation
    /// * [Heartbeats](https://wakatime.com/developers#heartbeats)
    pub async fn delete_heartbeats(&self, date: &str, ids: &[&str]) -> Result<(), ApiError> {
        let url = format!(
            "{base_url}users/{user}/heartbeats.bulk",
            base_url = self.base_url,
            user = self.user
        );
        let body = serde_json::json!({ "date": date, "ids": ids });
        let response = self.client.delete(url).json(&body).send().await?;
        Self::deserialize_as(response, |_: serde_json::Value| ()).await
    }

    /// Logs time spent in an external app, e.g. a meeting or code review.
    ///
    /// ## Documentation
    /// * [External Durations](https://wakatime.com/developers#external_durations)
    pub async fn send_external_duration(
        &self,
        duration: &model::ExternalDurationInput,
    ) -> Result<model::ExternalDuration, ApiError> {
        let url = format!(
            "{base_url}users/{user}/external_durations",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.post(url).json(duration).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::ExternalDuration>| r.data).await
    }

    /// Logs multiple external durations at once; at most 1000 per request.
    ///
    /// The per-duration results are returned as raw JSON since their shape
    /// is not fully documented.
    ///
    /// ## Documentation
    /// * [External Durations](https://wakatime.com/developers#external_durations)
    pub async fn send_external_durations(
        &self,
        durations: &[model::ExternalDurationInput],
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!(
            "{base_url}users/{user}/external_durations.bulk",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.post(url).json(durations).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Deletes the given external durations; all ids must be from the given day.
    ///
    /// ## Documentation
    /// * [External Durations](https://wakatime.com/developers#external_durations)
    pub async fn delete_external_durations(
        &self,
        date: &str,
        ids: &[&str],
    ) -> Result<(), ApiError> {
        let url = format!(
            "{base_url}users/{user}/external_durations.bulk",
            base_url = self.base_url,
            user = self.user
        );
        let body = serde_json::json!({ "date": date, "ids": ids });
        let response = self.client.delete(url).json(&body).send().await?;
        Self::deserialize_as(response, |_: serde_json::Value| ()).await
    }

    /// Requests a data dump export; the API emails the user when it is ready.
    ///
    /// ## Documentation
    /// * [Data Dumps](https://wakatime.com/developers#data_dumps)
    pub async fn create_data_dump(
        &self,
        dump_type: &str,
        email_when_finished: Option<bool>,
    ) -> Result<model::DataDump, ApiError> {
        let url = format!(
            "{base_url}users/{user}/data_dumps",
            base_url = self.base_url,
            user = self.user
        );
        let mut body = serde_json::json!({ "type": dump_type });
        if let Some(email) = email_when_finished {
            body["email_when_finished"] = serde_json::Value::Bool(email);
        }
        let response = self.client.post(url).json(&body).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::DataDump>| r.data).await
    }

    /// Replaces the user's custom rules with the given rules.
    ///
    /// ## Documentation
    /// * [Custom Rules](https://wakatime.com/developers#custom_rules)
    pub async fn set_custom_rules(
        &self,
        rules: &[model::CustomRuleInput],
    ) -> Result<model::CustomRulesChanges, ApiError> {
        let url = format!(
            "{base_url}users/{user}/custom_rules",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.put(url).json(rules).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::CustomRulesChanges>| r.data).await
    }

    /// Deletes a custom rule.
    ///
    /// ## Documentation
    /// * [Custom Rules](https://wakatime.com/developers#custom_rules)
    pub async fn delete_custom_rule(&self, rule_id: &str) -> Result<(), ApiError> {
        let url = format!(
            "{base_url}users/{user}/custom_rules/{rule_id}",
            base_url = self.base_url,
            user = self.user,
            rule_id = encode_path_segment(rule_id)
        );
        let response = self.client.delete(url).send().await?;
        Self::deserialize_as(response, |_: serde_json::Value| ()).await
    }

    /// Clears the progress of a finished custom rules job.
    ///
    /// ## Documentation
    /// * [Custom Rules Progress](https://wakatime.com/developers#custom_rules_progress)
    pub async fn delete_custom_rules_progress(&self) -> Result<(), ApiError> {
        let url = format!(
            "{base_url}users/{user}/custom_rules_progress",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.delete(url).send().await?;
        Self::deserialize_as(response, |_: serde_json::Value| ()).await
    }

    /// Fetches the user's custom rules for projects, branches, and entities.
    ///
    /// ## Documentation
    /// * [Custom Rules](https://wakatime.com/developers#custom_rules)
    pub async fn custom_rules(&self) -> Result<model::CustomRules, ApiError> {
        let url = format!(
            "{base_url}users/{user}/custom_rules",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the progress of a custom rules background job.
    ///
    /// ## Documentation
    /// * [Custom Rules Progress](https://wakatime.com/developers#custom_rules_progress)
    pub async fn custom_rules_progress(
        &self,
        job_id: &str,
    ) -> Result<model::CustomRulesProgress, ApiError> {
        let qs = QueryString::dynamic().with_value("job_id", job_id);
        let url = format!(
            "{base_url}users/{user}/custom_rules_progress{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::CustomRulesProgress>| {
            r.data
        })
        .await
    }

    /// Fetches the user's data dump exports and their status.
    ///
    /// ## Documentation
    /// * [Data Dumps](https://wakatime.com/developers#data_dumps)
    pub async fn data_dumps(&self) -> Result<model::DataDumps, ApiError> {
        let url = format!(
            "{base_url}users/{user}/data_dumps",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the list of editors with WakaTime plugins.
    ///
    /// ## Documentation
    /// * [Editors](https://wakatime.com/developers#editors)
    pub async fn editors(&self, options: EditorsOptions) -> Result<model::Editors, ApiError> {
        let qs = options.into_query_string();
        let url = format!("{base_url}editors{qs}", base_url = self.base_url);
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's external durations for the given day.
    ///
    /// ## Documentation
    /// * [External Durations](https://wakatime.com/developers#external_durations)
    pub async fn external_durations<'a>(
        &self,
        date: &str,
        options: ExternalDurationsOptions<'a>,
    ) -> Result<model::ExternalDurations, ApiError> {
        let qs = options.into_query_string().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/external_durations{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's goals.
    ///
    /// ## Documentation
    /// * [Goals](https://wakatime.com/developers#goals)
    pub async fn goals(&self) -> Result<model::Goals, ApiError> {
        let url = format!(
            "{base_url}users/{user}/goals",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches a single goal of the user.
    ///
    /// ## Documentation
    /// * [Goal](https://wakatime.com/developers#goal)
    pub async fn goal(&self, goal: &str) -> Result<model::CachedGoal, ApiError> {
        let url = format!(
            "{base_url}users/{user}/goals/{goal}",
            base_url = self.base_url,
            user = self.user,
            goal = encode_path_segment(goal)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches an insight about the user's coding activity for the given time range.
    ///
    /// ## Documentation
    /// * [Insights](https://wakatime.com/developers#insights)
    pub async fn insights<'a>(
        &self,
        insight_type: impl std::fmt::Display,
        range: impl std::fmt::Display,
        options: InsightsOptions<'a>,
    ) -> Result<model::Insight, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/insights/{insight_type}/{range}{qs}",
            base_url = self.base_url,
            user = self.user,
            insight_type = encode_path_segment(&insight_type.to_string()),
            range = encode_path_segment(&range.to_string())
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::Insight>| r.data).await
    }

    /// Fetches the public leaderboard of users ranked by coding activity.
    ///
    /// ## Documentation
    /// * [Leaders](https://wakatime.com/developers#leaders)
    pub async fn leaders<'a>(
        &self,
        options: LeadersOptions<'a>,
    ) -> Result<model::Leaders, ApiError> {
        let qs = options.into_query_string();
        let url = format!("{base_url}leaders{qs}", base_url = self.base_url);
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the machines the user has logged coding activity from.
    ///
    /// ## Documentation
    /// * [Machine Names](https://wakatime.com/developers#machine_names)
    pub async fn machine_names(&self) -> Result<model::MachineNames, ApiError> {
        let url = format!(
            "{base_url}users/{user}/machine_names",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches WakaTime service metadata, e.g. the IP addresses used by the service.
    ///
    /// ## Documentation
    /// * [Meta](https://wakatime.com/developers#meta)
    pub async fn meta(&self) -> Result<model::Meta, ApiError> {
        let url = format!("{base_url}meta", base_url = self.base_url);
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::Meta>| r.data).await
    }

    /// Fetches the organizations the user belongs to.
    ///
    /// ## Documentation
    /// * [Orgs](https://wakatime.com/developers#orgs)
    pub async fn orgs(&self) -> Result<model::Orgs, ApiError> {
        let url = format!(
            "{base_url}users/{user}/orgs",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the custom rules of an organization.
    ///
    /// ## Documentation
    /// * [Org Custom Rules](https://wakatime.com/developers#org_custom_rules)
    pub async fn org_custom_rules(&self, org: &str) -> Result<model::OrgCustomRules, ApiError> {
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/custom_rules",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the dashboards of an organization.
    ///
    /// ## Documentation
    /// * [Org Dashboards](https://wakatime.com/developers#org_dashboards)
    pub async fn org_dashboards(&self, org: &str) -> Result<model::OrgDashboards, ApiError> {
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the members of an organization's dashboard.
    ///
    /// ## Documentation
    /// * [Org Dashboard Members](https://wakatime.com/developers#org_dashboard_members)
    pub async fn org_dashboard_members(
        &self,
        org: &str,
        dashboard: &str,
    ) -> Result<model::OrgDashboardMembers, ApiError> {
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards/{dashboard}/members",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org),
            dashboard = encode_path_segment(dashboard)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches a dashboard's coding activity for the given day as an array of durations.
    ///
    /// ## Documentation
    /// * [Org Dashboard Durations](https://wakatime.com/developers#org_dashboard_durations)
    pub async fn org_dashboard_durations<'a>(
        &self,
        org: &str,
        dashboard: &str,
        date: &str,
        options: OrgDurationsOptions<'a>,
    ) -> Result<model::OrgDashboardDurations, ApiError> {
        let qs = options.into_query_string().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards/{dashboard}/durations{qs}",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org),
            dashboard = encode_path_segment(dashboard)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches a dashboard's coding activity for the given day as a summary.
    ///
    /// ## Documentation
    /// * [Org Dashboard Summaries](https://wakatime.com/developers#org_dashboard_summaries)
    pub async fn org_dashboard_summaries<'a>(
        &self,
        org: &str,
        dashboard: &str,
        date: &str,
        options: OrgSummariesOptions<'a>,
    ) -> Result<model::OrgDashboardSummaries, ApiError> {
        let qs = options.into_query_string().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards/{dashboard}/summaries{qs}",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org),
            dashboard = encode_path_segment(dashboard)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches a dashboard member's coding activity for the given day as an array of durations.
    ///
    /// ## Documentation
    /// * [Org Dashboard Member Durations](https://wakatime.com/developers#org_dashboard_member_durations)
    pub async fn org_dashboard_member_durations<'a>(
        &self,
        org: &str,
        dashboard: &str,
        member: &str,
        date: &str,
        options: OrgDurationsOptions<'a>,
    ) -> Result<model::Durations, ApiError> {
        let qs = options.into_query_string().with_value("date", date);
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards/{dashboard}/members/{member}/durations{qs}",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org),
            dashboard = encode_path_segment(dashboard),
            member = encode_path_segment(member)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches a dashboard member's coding activity for the given time range as daily summaries.
    ///
    /// ## Documentation
    /// * [Org Dashboard Member Summaries](https://wakatime.com/developers#org_dashboard_member_summaries)
    pub async fn org_dashboard_member_summaries<'a>(
        &self,
        org: &str,
        dashboard: &str,
        member: &str,
        start: &str,
        end: &str,
        options: OrgMemberSummariesOptions<'a>,
    ) -> Result<model::OrgMemberSummaries, ApiError> {
        let qs = options
            .into_query_string()
            .with_value("start", start)
            .with_value("end", end);
        let url = format!(
            "{base_url}users/{user}/orgs/{org}/dashboards/{dashboard}/members/{member}/summaries{qs}",
            base_url = self.base_url,
            user = self.user,
            org = encode_path_segment(org),
            dashboard = encode_path_segment(dashboard),
            member = encode_path_segment(member)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's private leaderboards.
    ///
    /// ## Documentation
    /// * [Private Leaderboards](https://wakatime.com/developers#private_leaderboards)
    pub async fn private_leaderboards(&self) -> Result<model::PrivateLeaderboards, ApiError> {
        let url = format!(
            "{base_url}users/{user}/leaderboards",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the leaders of a private leaderboard.
    ///
    /// ## Documentation
    /// * [Private Leaderboards Leaders](https://wakatime.com/developers#private_leaderboards_leaders)
    pub async fn private_leaderboard_leaders<'a>(
        &self,
        board: &str,
        options: PrivateLeaderboardLeadersOptions<'a>,
    ) -> Result<model::Leaders, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/leaderboards/{board}{qs}",
            base_url = self.base_url,
            user = self.user,
            board = encode_path_segment(board)
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the list of programming languages known to WakaTime.
    ///
    /// ## Documentation
    /// * [Program Languages](https://wakatime.com/developers#program_languages)
    pub async fn program_languages(&self) -> Result<model::ProgramLanguages, ApiError> {
        let url = format!("{base_url}program_languages", base_url = self.base_url);
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches one page of the user's projects.
    ///
    /// ## Documentation
    /// * [Projects](https://wakatime.com/developers#projects)
    pub async fn projects<'a>(
        &self,
        options: ProjectsOptions<'a>,
    ) -> Result<model::Projects, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/projects{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches all projects across all pages.
    ///
    /// ## Documentation
    /// * [Projects](https://wakatime.com/developers#projects)
    pub async fn projects_all(
        &self,
        q: Option<&str>,
    ) -> Result<Vec<model::projects::ProjectSummary>, ApiError> {
        let mut all = Vec::new();
        let mut page = 1;
        loop {
            let response = self
                .projects(ProjectsOptions {
                    q,
                    page: Some(page),
                })
                .await?;
            let is_empty = response.data.is_empty();
            let total_pages = response.pagination.total_pages;
            all.extend(response.data);
            match total_pages {
                Some(total_pages) if page < total_pages => page += 1,
                Some(_) => break,
                // No pagination metadata: stop once a page comes back empty.
                None if is_empty => break,
                None => page += 1,
            }
        }
        Ok(all)
    }

    /// Fetches all commits of a project across all pages.
    ///
    /// ## Documentation
    /// * [Commits](https://wakatime.com/developers#commits)
    pub async fn commits_all<'a>(
        &self,
        project: &str,
        options: CommitsOptions<'a>,
    ) -> Result<Vec<model::commit::Commit>, ApiError> {
        let mut all = Vec::new();
        let mut page = options.page.unwrap_or(1);
        loop {
            let response = self
                .commits(
                    project,
                    CommitsOptions {
                        page: Some(page),
                        ..options.clone()
                    },
                )
                .await?;
            let is_empty = response.data.is_empty();
            let total_pages = response.pagination.total_pages;
            all.extend(response.data);
            match total_pages {
                Some(total_pages) if page < total_pages => page += 1,
                Some(_) => break,
                // No pagination metadata: stop once a page comes back empty.
                None if is_empty => break,
                None => page += 1,
            }
        }
        Ok(all)
    }

    /// Fetches the user's coding activity stats for the given time range.
    ///
    /// ## Documentation
    /// * [Stats](https://wakatime.com/developers#stats)
    pub async fn stats<'a>(
        &self,
        range: impl std::fmt::Display,
        options: StatsOptions<'a>,
    ) -> Result<model::Stats, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/stats/{range}{qs}",
            base_url = self.base_url,
            user = self.user,
            range = encode_path_segment(&range.to_string())
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::Stats>| r.data).await
    }

    /// Fetches aggregate stats of all WakaTime users for the given time range.
    ///
    /// ## Documentation
    /// * [Stats Aggregated](https://wakatime.com/developers#stats_aggregated)
    pub async fn stats_aggregated(
        &self,
        range: impl std::fmt::Display,
    ) -> Result<model::AggregatedStats, ApiError> {
        let url = format!(
            "{base_url}stats/{range}",
            base_url = self.base_url,
            range = encode_path_segment(&range.to_string())
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::AggregatedStats>| r.data).await
    }

    /// Fetches the user's coding activity for today, as used by editor status bars.
    ///
    /// ## Documentation
    /// * [Status Bar](https://wakatime.com/developers#status_bar)
    pub async fn status_bar_today(&self) -> Result<model::StatusBar, ApiError> {
        let url = format!(
            "{base_url}users/{user}/status_bar/today",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the plugin user agents the user has logged coding activity from.
    ///
    /// ## Documentation
    /// * [User Agents](https://wakatime.com/developers#user_agents)
    pub async fn user_agents(&self) -> Result<model::UserAgents, ApiError> {
        let url = format!(
            "{base_url}users/{user}/user_agents",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

    /// Fetches the user's profile.
    ///
    /// ## Documentation
    /// * [Users](https://wakatime.com/developers#users)
    pub async fn user(&self) -> Result<model::User, ApiError> {
        let url = format!(
            "{base_url}users/{user}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::User>| r.data).await
    }

    async fn deserialize_as<TIn, F, TOut>(response: Response, map: F) -> Result<TOut, ApiError>
    where
        TIn: for<'de> Deserialize<'de>,
        F: FnOnce(TIn) -> TOut,
    {
        match response.status().as_u16() {
            // Write endpoints respond with 201 Created or 202 Accepted;
            // 202 is also used when cached stats are being refreshed.
            200..=202 => match response.json::<TIn>().await {
                Ok(response) => Ok(map(response)),
                Err(e) => Err(ApiError::InvalidFormat(e)),
            },
            other => {
                let retry_after = response
                    .headers()
                    .get(header::RETRY_AFTER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                let errors = response.json::<ErrorsResponse>().await.ok();
                match other {
                    401 => Err(ApiError::Unauthorized(errors)),
                    402 => Err(ApiError::PaymentRequired(errors)),
                    403 => Err(ApiError::Forbidden(errors)),
                    404 => Err(ApiError::NotFound(errors)),
                    429 => Err(ApiError::RateLimited {
                        retry_after,
                        errors,
                    }),
                    other => Err(ApiError::Unspecified(other, errors)),
                }
            }
        }
    }
}

/// The error messages returned by the API; either as a list in `errors`
/// or as a single message in `error`, depending on the endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorsResponse {
    /// Error messages, when the API returns a list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    /// The error message, when the API returns a single one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// The `data` envelope used by some API responses. Implementation detail.
#[doc(hidden)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    data: T,
}

trait IntoQueryString {
    fn into_query_string(self) -> QueryString;
}

/// A time range accepted by [`WakaTimeClient::stats`], [`WakaTimeClient::insights`]
/// and [`WakaTimeClient::stats_aggregated`].
///
/// These methods also accept plain strings, e.g. `"last_7_days"` or `"2024-03"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Range {
    /// The last 7 days.
    Last7Days,
    /// The last 30 days.
    Last30Days,
    /// The last 6 months.
    Last6Months,
    /// The last year.
    LastYear,
    /// All time since the account was created.
    AllTime,
    /// A specific year, e.g. `2024`.
    Year(u16),
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Range::Last7Days => f.write_str("last_7_days"),
            Range::Last30Days => f.write_str("last_30_days"),
            Range::Last6Months => f.write_str("last_6_months"),
            Range::LastYear => f.write_str("last_year"),
            Range::AllTime => f.write_str("all_time"),
            Range::Year(year) => write!(f, "{year}"),
        }
    }
}

/// An insight type accepted by [`WakaTimeClient::insights`].
///
/// The method also accepts plain strings, e.g. `"weekdays"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InsightType {
    /// Coding activity per weekday.
    Weekdays,
    /// Coding activity per day.
    Days,
    /// The day with the most coding activity.
    BestDay,
    /// Average coding activity per day.
    DailyAverage,
    /// Coding activity per project.
    Projects,
    /// Coding activity per language.
    Languages,
    /// Coding activity per editor.
    Editors,
    /// Coding activity per category.
    Categories,
    /// Coding activity per machine.
    Machines,
    /// Coding activity per operating system.
    OperatingSystems,
}

impl std::fmt::Display for InsightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InsightType::Weekdays => "weekdays",
            InsightType::Days => "days",
            InsightType::BestDay => "best_day",
            InsightType::DailyAverage => "daily_average",
            InsightType::Projects => "projects",
            InsightType::Languages => "languages",
            InsightType::Editors => "editors",
            InsightType::Categories => "categories",
            InsightType::Machines => "machines",
            InsightType::OperatingSystems => "operating_systems",
        })
    }
}

/// Options for [`WakaTimeClient::all_time_since_today`].
#[derive(Debug, Default, Clone)]
pub struct AllTimesSinceTodayOptions<'a> {
    /// Only show stats for this project.
    pub project: Option<&'a str>,
}

impl<'a> IntoQueryString for AllTimesSinceTodayOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic().with_opt_value("project", self.project)
    }
}

/// Options for [`WakaTimeClient::commit`].
#[derive(Debug, Default, Clone)]
pub struct CommitOptions<'a> {
    /// Filter the commit to a branch; defaults to the repo's default branch name.
    pub branch: Option<&'a str>,
}

impl<'a> IntoQueryString for CommitOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic().with_opt_value("branch", self.branch)
    }
}

/// Options for [`WakaTimeClient::commits`] and [`WakaTimeClient::commits_all`].
#[derive(Debug, Default, Clone)]
pub struct CommitsOptions<'a> {
    /// Filter commits to those authored by the given author.
    pub author: Option<&'a str>,
    /// Filter commits to a branch; defaults to the repo's default branch name.
    pub branch: Option<&'a str>,
    /// Page number of commits.
    pub page: Option<u32>,
}

impl<'a> IntoQueryString for CommitsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("author", self.author)
            .with_opt_value("branch", self.branch)
            .with_opt_value("page", self.page.map(|v| v.to_string()))
    }
}

/// Options for [`WakaTimeClient::durations`].
#[derive(Debug, Default, Clone)]
pub struct DurationsOptions<'a> {
    /// Only show durations for this project.
    pub project: Option<&'a str>,
    /// Only show durations for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
    /// The keystroke timeout preference used when joining heartbeats into durations.
    pub timeout: Option<u32>,
    /// The writes_only preference.
    pub writes_only: Option<bool>,
    /// The timezone for the given date; defaults to the user's timezone.
    pub timezone: Option<&'a str>,
    /// Optional primary key to use when slicing durations; defaults to `entity`.
    pub slice_by: Option<&'a str>,
}

impl<'a> IntoQueryString for DurationsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
            .with_opt_value("timeout", self.timeout.map(|v| v.to_string()))
            .with_opt_value("writes_only", self.writes_only.map(|v| v.to_string()))
            .with_opt_value("timezone", self.timezone)
            .with_opt_value("slice_by", self.slice_by)
    }
}

/// Options for [`WakaTimeClient::editors`].
#[derive(Debug, Default, Clone)]
pub struct EditorsOptions {
    /// Include editors with unreleased plugins.
    pub unreleased: Option<bool>,
}

impl IntoQueryString for EditorsOptions {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic().with_opt_value("unreleased", self.unreleased.map(|v| v.to_string()))
    }
}

/// Options for [`WakaTimeClient::external_durations`].
#[derive(Debug, Default, Clone)]
pub struct ExternalDurationsOptions<'a> {
    /// Only show durations for this project.
    pub project: Option<&'a str>,
    /// Only show durations for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
    /// The timezone for the given date; defaults to the user's timezone.
    pub timezone: Option<&'a str>,
}

impl<'a> IntoQueryString for ExternalDurationsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
            .with_opt_value("timezone", self.timezone)
    }
}

/// Options for [`WakaTimeClient::insights`].
#[derive(Debug, Default, Clone)]
pub struct InsightsOptions<'a> {
    /// The keystroke timeout value used to calculate the insight.
    pub timeout: Option<u32>,
    /// The writes_only value used to calculate the insight.
    pub writes_only: Option<bool>,
    /// Filter to a specific day of the week, either 0-6 or a weekday name.
    pub weekday: Option<&'a str>,
}

impl<'a> IntoQueryString for InsightsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("timeout", self.timeout.map(|v| v.to_string()))
            .with_opt_value("writes_only", self.writes_only.map(|v| v.to_string()))
            .with_opt_value("weekday", self.weekday)
    }
}

/// Options for [`WakaTimeClient::leaders`].
#[derive(Debug, Default, Clone)]
pub struct LeadersOptions<'a> {
    /// Filter leaders by a specific language.
    pub language: Option<&'a str>,
    /// Filter leaders by the hireable badge.
    pub is_hireable: Option<bool>,
    /// Filter leaders by a two-character country code.
    pub country_code: Option<&'a str>,
    /// Page number of the leaderboard.
    pub page: Option<u32>,
}

impl<'a> IntoQueryString for LeadersOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("language", self.language)
            .with_opt_value("is_hireable", self.is_hireable.map(|v| v.to_string()))
            .with_opt_value("country_code", self.country_code)
            .with_opt_value("page", self.page.map(|v| v.to_string()))
    }
}

/// Options for [`WakaTimeClient::org_dashboard_durations`] and
/// [`WakaTimeClient::org_dashboard_member_durations`].
#[derive(Debug, Default, Clone)]
pub struct OrgDurationsOptions<'a> {
    /// Only show durations for this project.
    pub project: Option<&'a str>,
    /// Only show durations for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
    /// Optional primary key to use when slicing durations; defaults to `entity`.
    pub slice_by: Option<&'a str>,
}

impl<'a> IntoQueryString for OrgDurationsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
            .with_opt_value("slice_by", self.slice_by)
    }
}

/// Options for [`WakaTimeClient::org_dashboard_summaries`].
#[derive(Debug, Default, Clone)]
pub struct OrgSummariesOptions<'a> {
    /// Only show summaries for this project.
    pub project: Option<&'a str>,
    /// Only show summaries for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
}

impl<'a> IntoQueryString for OrgSummariesOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
    }
}

/// Options for [`WakaTimeClient::org_dashboard_member_summaries`].
#[derive(Debug, Default, Clone)]
pub struct OrgMemberSummariesOptions<'a> {
    /// Only show summaries for this project.
    pub project: Option<&'a str>,
    /// Only show summaries for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
    /// Alternative way to supply start and end dates, e.g. `Last 7 Days`.
    pub range: Option<&'a str>,
}

impl<'a> IntoQueryString for OrgMemberSummariesOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
            .with_opt_value("range", self.range)
    }
}

/// Options for [`WakaTimeClient::private_leaderboard_leaders`].
#[derive(Debug, Default, Clone)]
pub struct PrivateLeaderboardLeadersOptions<'a> {
    /// Filter leaders by a specific language.
    pub language: Option<&'a str>,
    /// Filter leaders by a two-character country code.
    pub country_code: Option<&'a str>,
    /// Page number of the leaderboard.
    pub page: Option<u32>,
}

impl<'a> IntoQueryString for PrivateLeaderboardLeadersOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("language", self.language)
            .with_opt_value("country_code", self.country_code)
            .with_opt_value("page", self.page.map(|v| v.to_string()))
    }
}

/// Options for [`WakaTimeClient::projects`].
#[derive(Debug, Default, Clone)]
pub struct ProjectsOptions<'a> {
    /// Filter project names by a search term.
    pub q: Option<&'a str>,
    /// Page number of projects.
    pub page: Option<u32>,
}

impl<'a> IntoQueryString for ProjectsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("q", self.q)
            .with_opt_value("page", self.page.map(|v| v.to_string()))
    }
}

/// Options for [`WakaTimeClient::stats`].
#[derive(Debug, Default, Clone)]
pub struct StatsOptions<'a> {
    /// The keystroke timeout value used to calculate these stats.
    pub timeout: Option<u32>,
    /// The writes_only value used to calculate these stats.
    pub writes_only: Option<bool>,
    /// Show more detailed stats limited to this project.
    pub project: Option<&'a str>,
}

impl<'a> IntoQueryString for StatsOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("timeout", self.timeout.map(|v| v.to_string()))
            .with_opt_value("writes_only", self.writes_only.map(|v| v.to_string()))
            .with_opt_value("project", self.project)
    }
}

/// Options for [`WakaTimeClient::summaries`].
#[derive(Debug, Default, Clone)]
pub struct SummariesOptions<'a> {
    /// Only show summaries for this project.
    pub project: Option<&'a str>,
    /// Only show summaries for these branches; comma separated list of branch names.
    pub branches: Option<&'a str>,
    /// The keystroke timeout preference used when joining heartbeats into durations.
    pub timeout: Option<u32>,
    /// The writes_only preference.
    pub writes_only: Option<bool>,
    /// The timezone for the given dates; defaults to the user's timezone.
    pub timezone: Option<&'a str>,
    /// Alternative way to supply start and end dates, e.g. `Last 7 Days`.
    pub range: Option<&'a str>,
}

impl<'a> IntoQueryString for SummariesOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic()
            .with_opt_value("project", self.project)
            .with_opt_value("branches", self.branches)
            .with_opt_value("timeout", self.timeout.map(|v| v.to_string()))
            .with_opt_value("writes_only", self.writes_only.map(|v| v.to_string()))
            .with_opt_value("timezone", self.timezone)
            .with_opt_value("range", self.range)
    }
}
