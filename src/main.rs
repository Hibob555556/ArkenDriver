// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

// This is the main entry point for the ArkenDriver application. It loads the configuration from ArkenDriver.config
// and then fetches the latest ChromeDriver based on the specified download path and platform. The application
// uses Tokio for asynchronous execution and handles errors gracefully by returning a Result type.
mod config;
pub mod driver;
mod infra;

// Re-exporting the configuration module for easier access in main.rs
pub use config::ArkenDriverConfig;
use driver::Driver;
use infra::chrome_driver_installer::fetch_latest_chrome_driver;
use infra::chrome_driver_process::ChromeDriverProcess;

// Main function that initializes the application, loads configuration, and fetches the latest ChromeDriver
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ArkenDriverConfig::load()?;
    fetch_latest_chrome_driver(&config.chrome_driver_download_path, config.platform).await?;

    let chrome_driver =
        ChromeDriverProcess::start(&config.chrome_driver_download_path, config.platform)?;

    let session_result: Result<(), Box<dyn std::error::Error>> = async {
        chrome_driver.wait_until_ready().await?;

        let mut driver = Driver::new(chrome_driver.base_url());
        driver.start().await?;
        println!("Created ChromeDriver session.");

        driver.quit().await?;
        println!("Ended ChromeDriver session.");

        Ok(())
    }
    .await;

    session_result?;

    Ok(())
}
