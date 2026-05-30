// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArkenDriverConfig {
    #[serde(rename = "ChromeDriverDownloadPath")]
    pub chrome_driver_download_path: PathBuf,
}

impl ArkenDriverConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("ArkenDriver.config");
        let config_text = std::fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&config_text)?;

        Ok(config)
    }
}
