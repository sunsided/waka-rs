use crate::ErrorsResponse;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApiError {
    Unspecified(u16, Option<ErrorsResponse>),
    InvalidFormat(reqwest::Error),
    Unauthorized(Option<ErrorsResponse>),
    ServerError(reqwest::Error),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unspecified(code, _) => write!(f, "An unspecified error occurred: {code}"),
            ApiError::Unauthorized(_) => write!(f, "Authorization failed"),
            ApiError::ServerError(e) => write!(f, "The call failed: {e}"),
            ApiError::InvalidFormat(e) => write!(f, "The API returned an unexpected format: {e}"),
        }
    }
}

impl Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ServerError(value)
    }
}
