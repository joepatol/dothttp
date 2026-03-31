use std::fmt;

#[derive(Debug)]
pub enum RunnerError {
    HttpError(reqwest::Error),
    Other(String),
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunnerError::HttpError(e) => write!(f, "HTTP error: {e}"),
            RunnerError::Other(msg) => write!(f, "Runner error: {msg}"),
        }
    }
}

impl std::error::Error for RunnerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RunnerError::HttpError(e) => Some(e),
            RunnerError::Other(_) => None,
        }
    }
}

impl From<reqwest::Error> for RunnerError {
    fn from(e: reqwest::Error) -> Self {
        RunnerError::HttpError(e)
    }
}
