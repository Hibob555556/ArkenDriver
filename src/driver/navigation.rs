// File: src/driver/navigation.rs
// Author: Cayden Lunt
// Date: 05/30/2026
// License: MIT
// Version: 1.0.0

// This module contains navigation commands for the driver, including
// POST /session/{id}/url, GET /session/{id}/url, and GET /session/{id}/title.

use serde::{Deserialize, Serialize};

use super::{Driver, DriverResult};

#[derive(Debug, Serialize)]
struct NavigateRequest<'a> {
    url: &'a str,
}

#[derive(Debug, Deserialize)]
struct WebDriverValueResponse<T> {
    value: T,
}

impl Driver {
    pub async fn navigate(&self, url: &str) -> DriverResult<()> {
        let session_id = self.session_id()?;
        let request = NavigateRequest { url };

        self.client()
            .post(format!("{}/session/{}/url", self.base_url(), session_id))
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn current_url(&self) -> DriverResult<String> {
        let session_id = self.session_id()?;
        let response = self
            .client()
            .get(format!("{}/session/{}/url", self.base_url(), session_id))
            .send()
            .await?
            .error_for_status()?
            .json::<WebDriverValueResponse<String>>()
            .await?;

        Ok(response.value)
    }

    pub async fn title(&self) -> DriverResult<String> {
        let session_id = self.session_id()?;
        let response = self
            .client()
            .get(format!("{}/session/{}/title", self.base_url(), session_id))
            .send()
            .await?
            .error_for_status()?
            .json::<WebDriverValueResponse<String>>()
            .await?;

        Ok(response.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_navigation_request() {
        let request = NavigateRequest {
            url: "https://example.com",
        };
        let json = serde_json::to_value(request).unwrap();

        assert_eq!(json["url"], "https://example.com");
    }

    #[test]
    fn deserializes_string_value_response() {
        let response: WebDriverValueResponse<String> =
            serde_json::from_str(r#"{"value":"https://example.com/"}"#).unwrap();

        assert_eq!(response.value, "https://example.com/");
    }
}
