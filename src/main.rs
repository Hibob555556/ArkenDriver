// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

// This is the main entry point for the ArkenDriver application. It loads the configuration from ArkenDriver.config
// and then fetches the latest ChromeDriver based on the specified download path and platform. The application
// uses Tokio for asynchronous execution and handles errors gracefully by returning a Result type.
mod config;
mod infra;

// Re-exporting the configuration module for easier access in main.rs
pub use config::ArkenDriverConfig;
use infra::get_chrome_driver::fetch_latest_chrome_driver;

// Main function that initializes the application, loads configuration, and fetches the latest ChromeDriver
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ArkenDriverConfig::load()?;
    fetch_latest_chrome_driver(&config.chrome_driver_download_path, config.platform).await?;

    Ok(())
}
