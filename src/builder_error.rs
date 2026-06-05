use reqwest::header::InvalidHeaderValue;

/// An error that occurred while building a [`WakaTimeClient`](crate::WakaTimeClient).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BuilderError {
    /// An invalid header value was provided.
    #[error("An invalid header was provided: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),
    /// The underlying HTTP client could not be constructed.
    #[error("Failed to construct the HTTP client: {0}")]
    ClientError(#[from] reqwest::Error),
}
