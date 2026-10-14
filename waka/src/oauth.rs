//! OAuth 2.0 authorization for the WakaTime API.
//!
//! WakaTime API keys are write-only: they can send coding activity, but
//! reading stats, summaries and other data requires an OAuth 2.0 access token.
//! This module implements the authorization code flow:
//!
//! 1. Register an app at [wakatime.com/apps](https://wakatime.com/apps) to get
//!    a client ID and secret, and add your redirect URI to its settings.
//! 2. Send the user to [`OAuthClient::authorize_url`].
//! 3. WakaTime redirects back to your redirect URI with a `code` query parameter;
//!    exchange it for a [`Token`] with [`OAuthClient::exchange_code`].
//! 4. Authenticate API calls with [`WakaTimeClientBuilder::new_with_oauth_token`](crate::WakaTimeClientBuilder::new_with_oauth_token).
//!
//! ```no_run
//! use waka::WakaTimeClientBuilder;
//! use waka::oauth::{AuthorizeOptions, OAuthClient, Scope};
//!
//! # async fn test() -> Result<(), Box<dyn std::error::Error>> {
//! let oauth = OAuthClient::new("client-id", "client-secret", "https://example.com/callback")?;
//! let url = oauth.authorize_url(AuthorizeOptions {
//!     scopes: &[Scope::ReadStats, Scope::ReadSummaries],
//!     state: Some("random-state"),
//!     ..Default::default()
//! });
//! println!("Open {url} and paste the code from the redirect:");
//! # let code = "";
//! let token = oauth.exchange_code(code).await?;
//! let client = WakaTimeClientBuilder::new_with_oauth_token(&token).build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Documentation
//! * [Authentication](https://wakatime.com/developers#authentication)

use crate::BuilderError;
use query_string_builder::QueryString;
use reqwest::{Client, ClientBuilder, Response, header};
use serde::{Deserialize, Deserializer, Serialize};

static OAUTH_BASE_URL: &str = "https://wakatime.com/oauth/";

/// An OAuth 2.0 scope, limiting what an access token may be used for.
///
/// ## Documentation
/// * [Scopes](https://wakatime.com/developers#scopes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Scope {
    /// Access the user's summaries and status bar, including all sub-scopes.
    ReadSummaries,
    /// Access the categories in the user's summaries.
    ReadSummariesCategories,
    /// Access the dependencies in the user's summaries.
    ReadSummariesDependencies,
    /// Access the editors in the user's summaries.
    ReadSummariesEditors,
    /// Access the languages in the user's summaries.
    ReadSummariesLanguages,
    /// Access the machines in the user's summaries.
    ReadSummariesMachines,
    /// Access the operating systems in the user's summaries.
    ReadSummariesOperatingSystems,
    /// Access the projects in the user's summaries.
    ReadSummariesProjects,
    /// Access the user's stats, including all sub-scopes.
    ReadStats,
    /// Access the best day in the user's stats.
    ReadStatsBestDay,
    /// Access the categories in the user's stats.
    ReadStatsCategories,
    /// Access the dependencies in the user's stats.
    ReadStatsDependencies,
    /// Access the editors in the user's stats.
    ReadStatsEditors,
    /// Access the languages in the user's stats.
    ReadStatsLanguages,
    /// Access the machines in the user's stats.
    ReadStatsMachines,
    /// Access the operating systems in the user's stats.
    ReadStatsOperatingSystems,
    /// Access the projects in the user's stats.
    ReadStatsProjects,
    /// Access the user's goals.
    ReadGoals,
    /// Access the user's organizations and their dashboards.
    ReadOrgs,
    /// Modify the user's organizations.
    WriteOrgs,
    /// Access the user's private leaderboards.
    ReadPrivateLeaderboards,
    /// Modify the user's private leaderboards.
    WritePrivateLeaderboards,
    /// Access the user's heartbeats, durations, commits and data dumps.
    ReadHeartbeats,
    /// Create, modify and delete the user's heartbeats and external durations.
    WriteHeartbeats,
    /// Access the user's email address and private profile.
    Email,
}

impl Scope {
    /// All scopes, in documentation order.
    pub const ALL: &'static [Scope] = &[
        Scope::ReadSummaries,
        Scope::ReadSummariesCategories,
        Scope::ReadSummariesDependencies,
        Scope::ReadSummariesEditors,
        Scope::ReadSummariesLanguages,
        Scope::ReadSummariesMachines,
        Scope::ReadSummariesOperatingSystems,
        Scope::ReadSummariesProjects,
        Scope::ReadStats,
        Scope::ReadStatsBestDay,
        Scope::ReadStatsCategories,
        Scope::ReadStatsDependencies,
        Scope::ReadStatsEditors,
        Scope::ReadStatsLanguages,
        Scope::ReadStatsMachines,
        Scope::ReadStatsOperatingSystems,
        Scope::ReadStatsProjects,
        Scope::ReadGoals,
        Scope::ReadOrgs,
        Scope::WriteOrgs,
        Scope::ReadPrivateLeaderboards,
        Scope::WritePrivateLeaderboards,
        Scope::ReadHeartbeats,
        Scope::WriteHeartbeats,
        Scope::Email,
    ];

    /// The scope's name as used by the API, e.g. `read_stats.languages`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Scope::ReadSummaries => "read_summaries",
            Scope::ReadSummariesCategories => "read_summaries.categories",
            Scope::ReadSummariesDependencies => "read_summaries.dependencies",
            Scope::ReadSummariesEditors => "read_summaries.editors",
            Scope::ReadSummariesLanguages => "read_summaries.languages",
            Scope::ReadSummariesMachines => "read_summaries.machines",
            Scope::ReadSummariesOperatingSystems => "read_summaries.operating_systems",
            Scope::ReadSummariesProjects => "read_summaries.projects",
            Scope::ReadStats => "read_stats",
            Scope::ReadStatsBestDay => "read_stats.best_day",
            Scope::ReadStatsCategories => "read_stats.categories",
            Scope::ReadStatsDependencies => "read_stats.dependencies",
            Scope::ReadStatsEditors => "read_stats.editors",
            Scope::ReadStatsLanguages => "read_stats.languages",
            Scope::ReadStatsMachines => "read_stats.machines",
            Scope::ReadStatsOperatingSystems => "read_stats.operating_systems",
            Scope::ReadStatsProjects => "read_stats.projects",
            Scope::ReadGoals => "read_goals",
            Scope::ReadOrgs => "read_orgs",
            Scope::WriteOrgs => "write_orgs",
            Scope::ReadPrivateLeaderboards => "read_private_leaderboards",
            Scope::WritePrivateLeaderboards => "write_private_leaderboards",
            Scope::ReadHeartbeats => "read_heartbeats",
            Scope::WriteHeartbeats => "write_heartbeats",
            Scope::Email => "email",
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Scope {
    type Err = UnknownScope;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Scope::ALL
            .iter()
            .copied()
            .find(|scope| scope.as_str() == s)
            .ok_or_else(|| UnknownScope(s.to_string()))
    }
}

/// The error returned when parsing an unknown [`Scope`] name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Unknown OAuth scope: {0}")]
pub struct UnknownScope(pub String);

/// Options for [`OAuthClient::authorize_url`].
#[derive(Debug, Default, Clone, Copy)]
pub struct AuthorizeOptions<'a> {
    /// The scopes to request. If empty, WakaTime grants only basic profile access.
    pub scopes: &'a [Scope],
    /// An unguessable value that WakaTime passes back to the redirect URI
    /// unchanged; compare it there to prevent cross-site request forgery.
    pub state: Option<&'a str>,
    /// Ask the user to approve the app again, even if they did before.
    pub force_approve: bool,
}

/// An OAuth 2.0 client for WakaTime's authorization code flow.
///
/// Holds the credentials of an app registered at
/// [wakatime.com/apps](https://wakatime.com/apps).
#[derive(Clone)]
pub struct OAuthClient {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    base_url: String,
}

impl std::fmt::Debug for OAuthClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthClient")
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .field("redirect_uri", &self.redirect_uri)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl OAuthClient {
    /// Creates a client for the app with the given credentials.
    ///
    /// The redirect URI must be one of the authorized redirect URLs in the
    /// app's settings; the same value is sent when authorizing and when
    /// exchanging or refreshing tokens.
    pub fn new<I, S, R>(
        client_id: I,
        client_secret: S,
        redirect_uri: R,
    ) -> Result<Self, BuilderError>
    where
        I: Into<String>,
        S: Into<String>,
        R: Into<String>,
    {
        Ok(Self {
            client: ClientBuilder::new().build()?,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            base_url: OAUTH_BASE_URL.to_string(),
        })
    }

    /// Overrides the base URL of the OAuth endpoints, e.g. for testing against a mock server.
    /// If unspecified, `https://wakatime.com/oauth/` is used.
    pub fn with_base_url<S: AsRef<str>>(mut self, base_url: S) -> Self {
        let base_url = base_url.as_ref();
        // Ensure a trailing slash so path concatenation stays correct.
        self.base_url = if base_url.ends_with('/') {
            base_url.to_string()
        } else {
            format!("{base_url}/")
        };
        self
    }

    /// The app's client ID.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// The redirect URI sent with every request.
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Builds the URL to send the user to for approving the app.
    ///
    /// After approval, WakaTime redirects to the redirect URI with `code` and
    /// `state` query parameters, or with `error` if the user declined.
    pub fn authorize_url(&self, options: AuthorizeOptions<'_>) -> String {
        let scope = options
            .scopes
            .iter()
            .map(Scope::as_str)
            .collect::<Vec<_>>()
            .join(",");
        let qs = QueryString::new()
            .with("client_id", &self.client_id)
            .with("response_type", "code")
            .with("redirect_uri", &self.redirect_uri)
            .with_opt("scope", (!scope.is_empty()).then_some(scope.as_str()))
            .with_opt("state", options.state)
            .with_opt("force_approve", options.force_approve.then_some("true"));
        format!("{base_url}authorize{qs}", base_url = self.base_url)
    }

    /// Exchanges the `code` received at the redirect URI for an access token.
    ///
    /// ## Documentation
    /// * [Authentication](https://wakatime.com/developers#authentication)
    pub async fn exchange_code(&self, code: &str) -> Result<Token, OAuthError> {
        self.request_token(&[("grant_type", "authorization_code"), ("code", code)])
            .await
    }

    /// Obtains a new access token using a refresh token.
    ///
    /// ## Documentation
    /// * [Authentication](https://wakatime.com/developers#authentication)
    pub async fn refresh(&self, refresh_token: &str) -> Result<Token, OAuthError> {
        self.request_token(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .await
    }

    /// Revokes an access or refresh token.
    ///
    /// WakaTime reports success even if the token was already revoked or expired.
    ///
    /// ## Documentation
    /// * [Authentication](https://wakatime.com/developers#authentication)
    pub async fn revoke(&self, token: &str) -> Result<(), OAuthError> {
        let response = self
            .client
            .post(format!("{base_url}revoke", base_url = self.base_url))
            .header(header::ACCEPT, "application/json")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("token", token),
            ])
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(OAuthError::rejected(response).await)
        }
    }

    async fn request_token(&self, grant: &[(&str, &str)]) -> Result<Token, OAuthError> {
        let mut form = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];
        form.extend_from_slice(grant);
        let response = self
            .client
            .post(format!("{base_url}token", base_url = self.base_url))
            .header(header::ACCEPT, "application/json")
            .form(&form)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(OAuthError::rejected(response).await);
        }
        response.json().await.map_err(OAuthError::InvalidResponse)
    }
}

/// An OAuth 2.0 token issued by WakaTime.
///
/// Serializes losslessly, so it can be stored and loaded again; fields the
/// API returns beyond the ones below are kept in [`Token::extra`].
/// The [`Debug`] output redacts the access and refresh tokens.
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Token {
    /// The access token, sent as Bearer auth.
    pub access_token: String,
    /// The refresh token, for obtaining a new access token via [`OAuthClient::refresh`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// The token type, usually `bearer`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    /// The granted scopes, separated by commas or spaces; see [`Token::scopes`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The ID of the user who authorized the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// The number of seconds the access token is valid for, as of issuing.
    #[serde(
        default,
        deserialize_with = "lenient_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_in: Option<u64>,
    /// When the access token expires, as an ISO 8601 timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Any other fields of the token response.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl Token {
    /// The granted scope names, e.g. `["email", "read_stats"]`.
    pub fn scopes(&self) -> Vec<&str> {
        self.scope
            .as_deref()
            .unwrap_or_default()
            .split([',', ' '])
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Whether the given scope was granted.
    pub fn has_scope(&self, scope: Scope) -> bool {
        self.scopes().contains(&scope.as_str())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .field("token_type", &self.token_type)
            .field("scope", &self.scope)
            .field("uid", &self.uid)
            .field("expires_in", &self.expires_in)
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

/// Accepts a number of seconds given as an integer, float or numeric string.
fn lenient_seconds<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u64>, D::Error> {
    Ok(
        match Option::<serde_json::Value>::deserialize(deserializer)? {
            Some(serde_json::Value::Number(n)) => {
                n.as_u64().or_else(|| n.as_f64().map(|f| f as u64))
            }
            Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().ok().map(|f| f as u64),
            _ => None,
        },
    )
}

/// An error from WakaTime's OAuth endpoints.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OAuthError {
    /// The request could not be sent or the response could not be read.
    #[error("The OAuth request failed: {0}")]
    Http(#[from] reqwest::Error),
    /// WakaTime rejected the request, e.g. because the code or refresh token
    /// is invalid, or the client credentials or redirect URI do not match.
    #[error("The OAuth request was rejected ({status}){}", fmt_rejection(.error, .description))]
    Rejected {
        /// The HTTP status code.
        status: u16,
        /// The error code or message, e.g. `invalid_grant`.
        error: Option<String>,
        /// A human-readable description of the error.
        description: Option<String>,
    },
    /// The token response did not match the expected format.
    #[error("The OAuth token response has an unexpected format: {0}")]
    InvalidResponse(reqwest::Error),
}

impl OAuthError {
    async fn rejected(response: Response) -> Self {
        /// The error body of the OAuth endpoints; RFC 6749 style, or the
        /// `errors` list used by the rest of the API.
        #[derive(Deserialize)]
        struct ErrorBody {
            error: Option<String>,
            error_description: Option<String>,
            #[serde(default)]
            errors: Vec<String>,
        }

        let status = response.status().as_u16();
        let body = response.json::<ErrorBody>().await.ok();
        let (error, description) = match body {
            Some(body) => {
                let description = body
                    .error_description
                    .or_else(|| (!body.errors.is_empty()).then(|| body.errors.join("; ")));
                (body.error, description)
            }
            None => (None, None),
        };
        Self::Rejected {
            status,
            error,
            description,
        }
    }
}

fn fmt_rejection(error: &Option<String>, description: &Option<String>) -> String {
    match (error, description) {
        (Some(error), Some(description)) => format!(": {error}: {description}"),
        (Some(message), None) | (None, Some(message)) => format!(": {message}"),
        (None, None) => String::new(),
    }
}
