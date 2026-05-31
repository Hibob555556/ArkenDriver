// File: src/driver/error.rs
// Author: Cayden Lunt
// Date: 05/30/2026
// License: MIT
// Version: 1.0.0

// the error.rs file will provide a framework error type for HTTP
// errors within the ArkenDriver application. This will allow us
// to handle errors in a consistent way across the application.

#[derive(Debug)]
pub enum DriverError {
    /// Represents an error that occurs when an HTTP request fails.
    Http(reqwest::Error),

    /// Represents an error that occurs when a session is missing.
    MissingSession,

    /// Represents an error that occurs when a response is invalid.
    InvalidResponse(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::Http(err) => write!(f, "HTTP error: {}", err),
            DriverError::MissingSession => write!(f, "No active WebDriver session"),
            DriverError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl std::error::Error for DriverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DriverError::Http(err) => Some(err),
            DriverError::MissingSession | DriverError::InvalidResponse(_) => None,
        }
    }
}

impl From<reqwest::Error> for DriverError {
    fn from(err: reqwest::Error) -> Self {
        DriverError::Http(err)
    }
}

pub type DriverResult<T> = Result<T, DriverError>;
