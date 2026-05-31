// File: src/driver/session.rs
// Author: Cayden Lunt
// Date: 05/30/2026
// License: MIT
// Version: 1.0.0

// this module contains POST /session, and DELETE/session{id}. This
// will be used to create and delete sessions for the driver. This
// will also include the session struct and any related logic.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Driver, DriverError, DriverResult};

#[derive(Debug, Serialize)]
struct NewSessionRequest {
    capabilities: CapabilitiesRequest,
}

#[derive(Debug, Serialize)]
struct CapabilitiesRequest {
    #[serde(rename = "alwaysMatch")]
    always_match: BrowserCapabilities,
}

#[derive(Debug, Serialize)]
struct BrowserCapabilities {
    #[serde(rename = "browserName")]
    browser_name: String,
}

#[derive(Debug, Deserialize)]
struct NewSessionResponse {
    value: NewSessionValue,
}

#[derive(Debug, Deserialize)]
struct NewSessionValue {
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    #[allow(dead_code)]
    capabilities: Option<Value>,
}

impl Driver {
    pub async fn start(&mut self) -> DriverResult<()> {
        let request = NewSessionRequest {
            capabilities: CapabilitiesRequest {
                always_match: BrowserCapabilities {
                    browser_name: "chrome".to_string(),
                },
            },
        };

        let response = self
            .client()
            .post(format!("{}/session", self.base_url()))
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<NewSessionResponse>()
            .await?;

        let session_id = response.value.session_id.ok_or_else(|| {
            DriverError::InvalidResponse("new session response did not include sessionId".into())
        })?;

        self.set_session_id(session_id);
        Ok(())
    }

    pub async fn quit(&mut self) -> DriverResult<()> {
        let session_id = self.session_id()?.to_string();

        self.client()
            .delete(format!("{}/session/{}", self.base_url(), session_id))
            .send()
            .await?
            .error_for_status()?;

        self.clear_session_id();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_new_session_response() {
        let response: NewSessionResponse = serde_json::from_str(
            r#"{"value":{"sessionId":"abc-123","capabilities":{"browserName":"chrome"}}}"#,
        )
        .unwrap();

        assert_eq!(response.value.session_id.as_deref(), Some("abc-123"));
    }
}
