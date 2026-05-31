// File: src/infra/chrome_driver_process.rs
// Author: Cayden Lunt
// Date: 05/30/2026
// License: MIT
// Version: 1.0.0

// This module manages the local ChromeDriver process. It starts ChromeDriver,
// waits until the WebDriver server is ready, exposes the server base URL, and
// stops the process when it is no longer needed.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use crate::config::ChromeDriverPlatform;

const DEFAULT_CHROME_DRIVER_URL: &str = "http://127.0.0.1:9515";

pub struct ChromeDriverProcess {
    child: Child,
    base_url: String,
}

impl ChromeDriverProcess {
    pub fn start(
        chrome_driver_dir: &Path,
        platform: ChromeDriverPlatform,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = chrome_driver_path(chrome_driver_dir, platform);
        let child = Command::new(path)
            .arg("--port=9515")
            .arg("--disable-build-check")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(Self {
            child,
            base_url: DEFAULT_CHROME_DRIVER_URL.to_string(),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn wait_until_ready(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let status_url = format!("{}/status", self.base_url);

        for _ in 0..20 {
            if let Ok(response) = client.get(&status_url).send().await {
                if response.status().is_success() {
                    return Ok(());
                }
            }

            tokio::time::sleep(Duration::from_millis(250)).await;
        }

        Err("ChromeDriver did not become ready in time".into())
    }

    pub fn stop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for ChromeDriverProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

fn chrome_driver_path(chrome_driver_dir: &Path, platform: ChromeDriverPlatform) -> PathBuf {
    chrome_driver_dir.join(chrome_driver_executable_name(platform))
}

fn chrome_driver_executable_name(platform: ChromeDriverPlatform) -> &'static str {
    match platform {
        ChromeDriverPlatform::Windows => "chromedriver.exe",
        ChromeDriverPlatform::Linux | ChromeDriverPlatform::Mac => "chromedriver",
    }
}
