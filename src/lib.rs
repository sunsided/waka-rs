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
//! let client = WakaTimeClientBuilder::with_api_key(api_key)
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
pub mod model;

use crate::api_error::ApiError;
use base64::Engine;
use query_string_builder::QueryString;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, ClientBuilder, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

static BASE_URL: &'static str = "https://wakatime.com/api/v1/";

#[derive(Default)]
pub struct WakaTimeClientBuilder {
    api_key_base64: String,
    user: Option<String>,
}

impl WakaTimeClientBuilder {
    /// See [wakatime.com/api-key](https://wakatime.com/api-key).
    pub fn with_api_key<S: AsRef<str>>(api_key: S) -> Self {
        Self {
            api_key_base64: base64::engine::general_purpose::STANDARD.encode(api_key.as_ref()),
            ..Default::default()
        }
    }

    pub fn with_user<S: AsRef<str>>(mut self, user: S) -> Self {
        self.user = Some(user.as_ref().to_string());
        self
    }

    pub fn build(self) -> Result<WakaTimeClient, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        let authorize = format!("Basic {api_key}", api_key = self.api_key_base64);
        headers.insert("authorization", HeaderValue::from_str(&authorize)?);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(WakaTimeClient {
            client,
            user: self.user.unwrap_or("current".to_string()),
        })
    }
}

pub struct WakaTimeClient {
    client: Client,
    user: String,
}

impl WakaTimeClient {
    /// ## Documentation
    /// * [All Time Since Today](https://wakatime.com/developers#all_time_since_today)
    pub async fn all_time_since_today<'a>(
        &self,
        options: AllTimesSinceTodayOptions<'a>,
    ) -> Result<model::all_times_since_today::AllTimeSinceToday, ApiError> {
        let qs = QueryString::new().with_opt_value("project", options.project);
        let url = format!(
            "{BASE_URL}users/{user}/all_time_since_today{qs}",
            user = self.user
        );
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(
            response,
            |r: DataWrapper<model::all_times_since_today::AllTimeSinceToday>| r.data,
        )
        .await
    }

    /// ## Documentation
    /// * [Commits](https://wakatime.com/developers#commits)
    pub async fn commit<'a>(
        &self,
        project: &str,
        hash: &str,
        options: CommitOptions<'a>,
    ) -> Result<model::commit::CommitResponse, ApiError> {
        let qs = QueryString::new().with_opt_value("branch", options.branch);
        let url = format!(
            "{BASE_URL}users/{user}/projects/{project}/commits/{hash}{qs}",
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
    ) -> Result<model::summaries::Summaries, ApiError> {
        let timeout = options.timeout.map(|v| v.to_string());
        let writes_only = options.writes_only.map(|v| v.to_string());
        let qs = QueryString::new()
            .with_value("start", start)
            .with_value("end", end)
            .with_opt_value("project", options.project)
            .with_opt_value("branches", options.branches)
            .with_opt_value("timeout", timeout.as_deref())
            .with_opt_value("writes_only", writes_only.as_deref())
            .with_opt_value("timezone", options.timezone)
            .with_opt_value("range", options.range);
        let url = format!("{BASE_URL}users/{user}/summaries{qs}", user = self.user);
        let response = self.client.get(url).send().await?;
        Self::deserialize_as(response, |r| r).await
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
                let errors = response.json::<ErrorsResponse>().await.ok();
                match other {
                    401 => Err(ApiError::Unauthorized(errors)),
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

#[derive(Debug, Default, Clone)]
pub struct AllTimesSinceTodayOptions<'a> {
    pub project: Option<&'a str>,
}

#[derive(Debug, Default, Clone)]
pub struct CommitOptions<'a> {
    pub branch: Option<&'a str>,
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
