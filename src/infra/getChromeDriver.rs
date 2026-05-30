// src/infra/getChromeDriver.rs
// This module is responsible for fetching the latest ChromeDriver version, constructing the download URL, and downloading the ChromeDriver to a specified directory.
// It includes functions to fetch the latest version, construct the download URL, and handle the download and extraction of the ChromeDriver. Additionally, it contains unit tests for each of these functions to ensure they work as expected.
// The module uses the `reqwest` crate for making HTTP requests and the `zip` crate for handling ZIP file extraction. It also includes error handling to manage potential issues during the download and extraction process.
// Note: The download URL and version URL are specific to the ChromeDriver for Windows (win64). If support for other platforms is needed, additional logic will be required to handle different URLs and file formats.

// Author: Cayden
// Date: 05/29/2026
// License: MIT
// Version: 1.0.0

use std::path::Path;
use std::process::Command;

use crate::config::ChromeDriverPlatform;

const CHROME_DRIVER_BASE_URL: &str =
    "https://storage.googleapis.com/chrome-for-testing-public/{{VERSION}}/{{PLATFORM}}/chromedriver-{{PLATFORM}}.zip";
const CHROME_DRIVER_VERSION_URL: &str =
    "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_STABLE";

/**
 * Fetches the latest ChromeDriver version, constructs the download URL, and downloads the ChromeDriver.
 * Returns the download URL of the latest ChromeDriver.
 */
pub async fn fetch_latest_chrome_driver(
    chrome_driver_dir: &Path,
    platform: ChromeDriverPlatform,
) -> Result<String, Box<dyn std::error::Error>> {
    let version = fetch_latest_chrome_driver_version().await?;

    if is_chrome_driver_current(chrome_driver_dir, platform, &version)? {
        println!("ChromeDriver {version} is already installed.");
        return Ok(make_latest_chrome_driver_download_url(&version, platform));
    }

    let url = make_latest_chrome_driver_download_url(&version, platform);
    download_chrome_driver(&url, chrome_driver_dir, platform).await?;
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
fn make_latest_chrome_driver_download_url(version: &str, platform: ChromeDriverPlatform) -> String {
    CHROME_DRIVER_BASE_URL
        .replace("{{VERSION}}", version)
        .replace("{{PLATFORM}}", chrome_driver_platform_package(platform))
}

fn is_chrome_driver_current(
    chrome_driver_dir: &Path,
    platform: ChromeDriverPlatform,
    latest_version: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(installed_version) = get_installed_chrome_driver_version(chrome_driver_dir, platform)?
    else {
        return Ok(false);
    };

    Ok(installed_version == latest_version)
}

fn get_installed_chrome_driver_version(
    chrome_driver_dir: &Path,
    platform: ChromeDriverPlatform,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let chrome_driver_path = chrome_driver_dir.join(chrome_driver_executable_name(platform));

    if !chrome_driver_path.exists() {
        return Ok(None);
    }

    let output = match Command::new(chrome_driver_path).arg("--version").output() {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(parse_chrome_driver_version(&stdout).map(str::to_string))
}

fn parse_chrome_driver_version(output: &str) -> Option<&str> {
    output.split_whitespace().nth(1)
}

fn chrome_driver_platform_package(platform: ChromeDriverPlatform) -> &'static str {
    match platform {
        ChromeDriverPlatform::Windows => "win64",
        ChromeDriverPlatform::Linux => "linux64",
        ChromeDriverPlatform::Mac => {
            if cfg!(target_arch = "aarch64") {
                "mac-arm64"
            } else {
                "mac-x64"
            }
        }
    }
}

fn chrome_driver_executable_name(platform: ChromeDriverPlatform) -> &'static str {
    match platform {
        ChromeDriverPlatform::Windows => "chromedriver.exe",
        ChromeDriverPlatform::Linux | ChromeDriverPlatform::Mac => "chromedriver",
    }
}

/**
 * Downloads the ChromeDriver from the specified URL and saves it to disk.
 * Returns a Result indicating success or failure of the download operation.
 */
async fn download_chrome_driver(
    url: &str,
    chrome_driver_dir: &Path,
    platform: ChromeDriverPlatform,
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
    let platform_package = chrome_driver_platform_package(platform);
    let executable_name = chrome_driver_executable_name(platform);
    let extracted_path = chrome_driver_dir
        .join(format!("chromedriver-{platform_package}"))
        .join(executable_name);
    let target_path = chrome_driver_dir.join(executable_name);
    std::fs::rename(extracted_path, target_path)?;
    let target_path = chrome_driver_dir.join(format!("chromedriver-{platform_package}"));
    std::fs::remove_dir_all(target_path)?;

    // For Unix-based systems (Linux and macOS), we need to set the executable permissions for the chromedriver file.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let chromedriver_path = chrome_driver_dir.join(executable_name);
        let mut permissions = std::fs::metadata(chromedriver_path)?.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(chrome_driver_dir.join(executable_name), permissions)?;
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
        let chromedriver_path = chrome_driver_dir.join(executable_name);
        let mut permissions = std::fs::metadata(chromedriver_path)?.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(chrome_driver_dir.join(executable_name), permissions)?;
    }

    Ok(())
}

/**
 * Unit tests for the functions in this module.
 * Tests include fetching the latest ChromeDriver version, constructing the download URL, and downloading the ChromeDriver.
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests the fetch_latest_chrome_driver_version function to ensure it retrieves a non-empty version string.
     */
    #[tokio::test]
    async fn test_fetch_latest_chrome_driver_version() {
        let version = fetch_latest_chrome_driver_version().await.unwrap();
        assert!(!version.is_empty(), "Version should not be empty");
    }

    /**
     * Tests the make_latest_chrome_driver_download_url function to ensure it constructs a valid URL containing the version.
     */
    #[test]
    fn test_make_latest_chrome_driver_download_url() {
        let version = "123.0.1.2.3";
        let url = make_latest_chrome_driver_download_url(version, ChromeDriverPlatform::Windows);
        assert!(url.contains(version), "URL should contain the version");
        assert!(
            url.starts_with("https://storage.googleapis.com/"),
            "URL should start with the base URL"
        );
        assert!(
            url.ends_with("/win64/chromedriver-win64.zip"),
            "Windows URL should use the win64 package"
        );
    }

    #[test]
    fn test_make_linux_chrome_driver_download_url() {
        let url =
            make_latest_chrome_driver_download_url("123.0.1.2.3", ChromeDriverPlatform::Linux);

        assert!(
            url.ends_with("/linux64/chromedriver-linux64.zip"),
            "Linux URL should use the linux64 package"
        );
    }

    #[test]
    fn test_chrome_driver_executable_name() {
        assert_eq!(
            chrome_driver_executable_name(ChromeDriverPlatform::Windows),
            "chromedriver.exe"
        );
        assert_eq!(
            chrome_driver_executable_name(ChromeDriverPlatform::Linux),
            "chromedriver"
        );
        assert_eq!(
            chrome_driver_executable_name(ChromeDriverPlatform::Mac),
            "chromedriver"
        );
    }

    #[test]
    fn test_parse_chrome_driver_version() {
        let output = "ChromeDriver 125.0.6422.141 (4b1f04fb)";
        let version = parse_chrome_driver_version(output);

        assert_eq!(version, Some("125.0.6422.141"));
    }

    #[test]
    fn test_missing_chrome_driver_is_not_current() {
        let temp_dir = std::env::temp_dir().join("missing_chromedriver_test");
        let result =
            is_chrome_driver_current(&temp_dir, ChromeDriverPlatform::Windows, "125.0.6422.141")
                .unwrap();

        assert!(!result, "Missing ChromeDriver should not be current");
    }

    /**
     * Tests the download_chrome_driver function to ensure it successfully downloads the ChromeDriver and saves it to disk.
     * This test will create a temporary directory for the download and clean up after the test is complete.
     */
    #[tokio::test]
    async fn test_download_chrome_driver() {
        let version = fetch_latest_chrome_driver_version().await.unwrap();
        let url = make_latest_chrome_driver_download_url(&version, ChromeDriverPlatform::Windows);
        let response = reqwest::get(&url).await.unwrap();
        assert!(
            response.status().is_success(),
            "Download URL should be accessible"
        );
        let temp_dir = std::env::temp_dir().join("chromedriver_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let result = download_chrome_driver(&url, &temp_dir, ChromeDriverPlatform::Windows).await;
        assert!(result.is_ok(), "Download should succeed");
        let chromedriver_path = temp_dir.join("chromedriver.exe");
        assert!(
            chromedriver_path.exists(),
            "Chromedriver should exist after download"
        );
        std::fs::remove_dir_all(temp_dir).unwrap(); // Clean up after test
    }
}
