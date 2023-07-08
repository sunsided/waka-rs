use reqwest::header::InvalidHeaderValue;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BuilderError {
    InvalidHeader(InvalidHeaderValue),
    ClientError(reqwest::Error),
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeader(e) => write!(f, "An invalid header was provided: {e}"),
            Self::ClientError(e) => write!(f, "Failed to construct the HTTP client: {e}"),
        }
    }
}

impl Error for BuilderError {}

impl From<InvalidHeaderValue> for BuilderError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(value)
    }
}

impl From<reqwest::Error> for BuilderError {
    fn from(value: reqwest::Error) -> Self {
        Self::ClientError(value)
    }
}
