//! # WakaTime API client
//!
//! A client to fetch your coding statistics from [WakaTime](https://wakatime.com/) given your API key.
//!
//! This is a work in progress and the API may change over time.
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

mod api_error;
mod builder_error;
pub mod model;

pub use crate::api_error::ApiError;
pub use crate::builder_error::BuilderError;
use base64::Engine;
use query_string_builder::QueryString;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, ClientBuilder, Response};
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "https://wakatime.com/api/v1/";
const CURRENT_USER: &str = "current";

/// A builder for [`WakaTimeClient`] instances.
#[derive(Default)]
pub struct WakaTimeClientBuilder {
    /// The API key, base-64 encoded.
    api_key_base64: String,
    /// The optional user to use.
    user: Option<String>,
    /// The optional base URL to use instead of the default WakaTime API URL.
    base_url: Option<String>,
}

impl WakaTimeClientBuilder {
    /// See [wakatime.com/api-key](https://wakatime.com/api-key).
    pub fn new_with_api_key<S: AsRef<str>>(api_key: S) -> Self {
        Self {
            api_key_base64: base64::engine::general_purpose::STANDARD.encode(api_key.as_ref()),
            ..Default::default()
        }
    }

    /// Specifies a user to focus on. If unspecified, `current` is used.
    pub fn with_user<S: AsRef<str>>(mut self, user: S) -> Self {
        self.user = Some(user.as_ref().to_string());
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

    pub fn build(self) -> Result<WakaTimeClient, BuilderError> {
        let mut headers = header::HeaderMap::new();
        let authorize = format!("Basic {api_key}", api_key = self.api_key_base64);
        headers.insert("authorization", HeaderValue::from_str(&authorize)?);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(WakaTimeClient {
            client,
            user: self.user.unwrap_or_else(|| CURRENT_USER.to_string()),
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
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

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
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
    }

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

    /// ## Documentation
    /// * [Stats](https://wakatime.com/developers#stats)
    pub async fn stats<'a>(
        &self,
        range: &str,
        options: StatsOptions<'a>,
    ) -> Result<model::Stats, ApiError> {
        let qs = options.into_query_string();
        let url = format!(
            "{base_url}users/{user}/stats/{range}{qs}",
            base_url = self.base_url,
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r: DataWrapper<model::Stats>| r.data).await
    }

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
            200 => match response.json::<TIn>().await {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorsResponse {
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    data: T,
}

trait IntoQueryString {
    fn into_query_string(self) -> QueryString;
}

#[derive(Debug, Default, Clone)]
pub struct AllTimesSinceTodayOptions<'a> {
    pub project: Option<&'a str>,
}

impl<'a> IntoQueryString for AllTimesSinceTodayOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic().with_opt_value("project", self.project)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommitOptions<'a> {
    pub branch: Option<&'a str>,
}

impl<'a> IntoQueryString for CommitOptions<'a> {
    fn into_query_string(self) -> QueryString {
        QueryString::dynamic().with_opt_value("branch", self.branch)
    }
}

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

#[derive(Debug, Default, Clone)]
pub struct SummariesOptions<'a> {
    pub project: Option<&'a str>,
    pub branches: Option<&'a str>,
    pub timeout: Option<u32>,
    pub writes_only: Option<bool>,
    pub timezone: Option<&'a str>,
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
