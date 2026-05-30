// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

mod config;
mod infra;

use config::ArkenDriverConfig;
use infra::get_chrome_driver::fetch_latest_chrome_driver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ArkenDriverConfig::load()?;
    fetch_latest_chrome_driver(&config.chrome_driver_download_path).await?;

    Ok(())
}
