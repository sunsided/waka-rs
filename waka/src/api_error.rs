use crate::ErrorsResponse;

/// Formats the API's error messages for appending to a [`Display`](std::fmt::Display) message.
fn fmt_errors(errors: &Option<ErrorsResponse>) -> String {
    let Some(errors) = errors else {
        return String::new();
    };
    let mut messages: Vec<&str> = errors.errors.iter().map(String::as_str).collect();
    if let Some(error) = &errors.error {
        messages.push(error);
    }
    if messages.is_empty() {
        String::new()
    } else {
        format!(": {}", messages.join("; "))
    }
}

/// An error returned by the WakaTime API or while communicating with it.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    /// An unhandled HTTP status code was returned.
    #[error("An unspecified error occurred: {code}{errors}", code = .0, errors = fmt_errors(.1))]
    Unspecified(u16, Option<ErrorsResponse>),
    /// The response body did not match the expected format.
    #[error("The API returned an unexpected format: {0}")]
    InvalidFormat(reqwest::Error),
    /// Authentication failed (HTTP 401); check the API key.
    #[error("Authorization failed{}", fmt_errors(.0))]
    Unauthorized(Option<ErrorsResponse>),
    /// The requested resource requires a paid plan (HTTP 402).
    #[error("Payment required{}", fmt_errors(.0))]
    PaymentRequired(Option<ErrorsResponse>),
    /// Access to the requested resource is forbidden (HTTP 403).
    #[error("Access forbidden{}", fmt_errors(.0))]
    Forbidden(Option<ErrorsResponse>),
    /// The requested resource does not exist (HTTP 404).
    #[error("Resource not found{}", fmt_errors(.0))]
    NotFound(Option<ErrorsResponse>),
    /// The rate limit was exceeded (HTTP 429).
    #[error("Rate limit exceeded{}{}", retry_after.map(|s| format!(", retry after {s} seconds")).unwrap_or_default(), fmt_errors(errors))]
    RateLimited {
        /// The number of seconds to wait before retrying, if provided
        /// by the `Retry-After` response header.
        retry_after: Option<u64>,
        /// The error messages returned by the API, if any.
        errors: Option<ErrorsResponse>,
    },
    /// The request timed out.
    #[error("The request timed out: {0}")]
    Timeout(reqwest::Error),
    /// The HTTP call itself failed.
    #[error("The call failed: {0}")]
    ServerError(reqwest::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            Self::Timeout(value)
        } else {
            Self::ServerError(value)
        }
    }
}
