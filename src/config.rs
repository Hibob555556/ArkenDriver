// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

// This module defines the configuration structure for ArkenDriver, which is loaded from a JSON file named
// ArkenDriver.config. It includes fields for the ChromeDriver download path and the target platform
// (Windows, Linux, or Mac). The configuration is deserialized using Serde, and a default platform is provided
// if not specified.
use serde::Deserialize;
use std::path::{Path, PathBuf};

// Configuration for ArkenDriver, loaded from ArkenDriver.config
#[derive(Debug, Deserialize)]
pub struct ArkenDriverConfig {
    #[serde(rename = "ChromeDriverDownloadPath")]
    pub chrome_driver_download_path: PathBuf,

    #[serde(rename = "Platform", default)]
    pub platform: ChromeDriverPlatform,
}

// Supported platforms for ChromeDriver
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChromeDriverPlatform {
    #[default]
    Windows,
    Linux,
    Mac,
}

// Implementation to load configuration from ArkenDriver.config
impl ArkenDriverConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("ArkenDriver.config");
        let config_text = std::fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&config_text)?;

        Ok(config)
    }
}
