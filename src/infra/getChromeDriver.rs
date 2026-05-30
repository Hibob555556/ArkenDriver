const CHROME_DRIVER_BASE_URL: &str = "https://storage.googleapis.com/chrome-for-testing-public/{{REPLACE}}/win64/chromedriver-win64.zip";
const CHROME_DRIVER_VERSION_URL: &str =
    "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_STABLE";

/**
 * Fetches the latest ChromeDriver version, constructs the download URL, and downloads the ChromeDriver.
 * Returns the download URL of the latest ChromeDriver.
 */
pub async fn fetch_latest_chrome_driver(
    chrome_driver_dir: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let version = fetch_latest_chrome_driver_version().await?;
    let url = make_latest_chrome_driver_download_url(&version);
    download_chrome_driver(&url, chrome_driver_dir).await?;
    Ok(url)
}

/**
 * Fetches the latest ChromeDriver version from the specified URL.
 * Returns the version as a String.
 */
async fn fetch_latest_chrome_driver_version() -> Result<String, reqwest::Error> {
    let response = reqwest::get(CHROME_DRIVER_VERSION_URL).await?;
    let version = response.text().await?;
    Ok(version.trim().to_string())
}

/**
 * Constructs the download URL for the latest ChromeDriver using the provided version.
 * Returns the constructed URL as a String.
 */
fn make_latest_chrome_driver_download_url(version: &str) -> String {
    CHROME_DRIVER_BASE_URL.replace("{{REPLACE}}", version)
}

/**
 * Downloads the ChromeDriver from the specified URL and saves it to disk.
 * Returns a Result indicating success or failure of the download operation.
 */
async fn download_chrome_driver(
    url: &str,
    chrome_driver_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading ChromeDriver from: {url}");

    let zip_path = chrome_driver_dir.join("chromedriver.zip");

    // ensure the target directory exists
    std::fs::create_dir_all(chrome_driver_dir)?;

    // download the file and save it to disk
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    std::fs::write(&zip_path, &bytes)?;

    // unzip the downloaded file
    let zip_file = std::fs::File::open(&zip_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    archive.extract(chrome_driver_dir)?;

    // clean up the zip file after extraction
    std::fs::remove_file(&zip_path)?;

    // move chromdriver executable to the target directory
    let extracted_path = chrome_driver_dir
        .join("chromedriver-win64")
        .join("chromedriver.exe");
    let target_path = chrome_driver_dir.join("chromedriver.exe");
    std::fs::rename(extracted_path, target_path)?;
    let target_path = chrome_driver_dir.join("chromedriver-win64");
    std::fs::remove_dir_all(target_path)?;

    // For Unix-based systems (Linux and macOS), we need to set the executable permissions for the chromedriver file.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let chromedriver_path = "/chromedriver/chromedriver.exe";
        let mut permissions = std::fs::metadata(chromedriver_path)?.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(chromedriver_path, permissions)?;
    }

    // On Windows, the executable permissions are handled by the file system, so no additional steps are needed.
    #[cfg(windows)]
    {
        // On Windows, the executable permissions are handled by the file system, so no additional steps are needed.
    }

    // For macOS, we need to set the executable permissions for the chromedriver file.
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::PermissionsExt;
        let chromedriver_path = "/chromedriver/chromedriver";
        let mut permissions = std::fs::metadata(chromedriver_path)?.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(chromedriver_path, permissions)?;
    }

    Ok(())
}
