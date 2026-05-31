// File: src/driver/mod.rs
// Author: Cayden Lunt
// Date: 05/30/2026
// License: MIT
// Version: 1.0.0

// This module exports Driver and will wire each of the submodules
// together. It will also re-export the error module for easier access
// to the framework error type.

// The driver module will contain the main logic for the driver, including
// the session management and navigation logic. It will also include any
// utility functions or data structures needed by the driver.
pub mod error;
pub mod navigation;
pub mod session;

// re-export the error module for easier access to the framework error type.
pub use error::{DriverError, DriverResult};

// The main driver struct used to send commands to a WebDriver server.
#[derive(Debug)]
pub struct Driver {
    client: reqwest::Client,
    base_url: String,
    session_id: Option<String>,
}

impl Driver {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into().trim().trim_end_matches('/').to_string(),
            session_id: None,
        }
    }

    pub(crate) fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn session_id(&self) -> DriverResult<&str> {
        self.session_id
            .as_deref()
            .ok_or_else(|| DriverError::MissingSession)
    }

    pub(crate) fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }

    pub(crate) fn clear_session_id(&mut self) {
        self.session_id = None;
    }
}
