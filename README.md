# ArkenDriver

ArkenDriver is a browser automation framework built from scratch in Rust that communicates directly with the W3C WebDriver protocol. The project is designed to explore automation framework architecture, protocol-level browser control, and scalable test execution without relying on existing Selenium client implementations.

## Features

- Direct WebDriver protocol communication
- Browser session management
- Element discovery and interaction
- Navigation and page control
- Explicit wait strategies
- Strongly typed Rust APIs
- Extensible architecture for reporting and test execution
- Cross-browser support through WebDriver-compatible drivers

## Goals

- Learn the WebDriver protocol from the ground up
- Build a production-quality automation framework in Rust
- Explore framework architecture and API design
- Implement custom reporting and execution pipelines
- Provide a foundation for UI automation and testing

## Roadmap

### Phase 1 - Core WebDriver Client
- [ ] Session creation and management
- [ ] Navigation commands
- [ ] Element lookup
- [ ] Element interaction
- [ ] Screenshot support

### Phase 2 - Framework Features
- [ ] Wait utilities
- [ ] Locator abstractions
- [ ] Page Object Model support
- [ ] Configuration system
- [ ] Logging

### Phase 3 - Test Execution
- [ ] Test runner
- [ ] Parallel execution
- [ ] Reporting engine
- [ ] CI/CD integration

## Configuration

ArkenDriver reads configuration from `ArkenDriver.config` in the project root.

```json
{
  "ChromeDriverDownloadPath": "C:/Users/Cayden/Repos/ArkenDriver/src/chromedriver/",
  "Platform": "windows"
}
```

### Options

- `ChromeDriverDownloadPath`: Directory where ChromeDriver should be downloaded and extracted.
- `Platform`: ChromeDriver platform to download. Supported values are `windows`, `linux`, and `mac`.

If `Platform` is omitted, ArkenDriver defaults to `windows`.

Platform mappings:

- `windows`: downloads the `win64` ChromeDriver package and installs `chromedriver.exe`.
- `linux`: downloads the `linux64` ChromeDriver package and installs `chromedriver`.
- `mac`: downloads `mac-arm64` on Apple Silicon or `mac-x64` otherwise, and installs `chromedriver`.

## Git Hooks

This repo includes a Husky-style Git hook in `.githooks/pre-commit`.

Enable it once per clone:

```sh
git config core.hooksPath .githooks
```

The pre-commit hook runs:

- `cargo fmt --check`
- `cargo clippy --locked -- -D warnings`
- Fast unit tests that do not call the ChromeDriver download endpoints

Full network-based tests still run in CI with `cargo test --locked`.

## Example

```rust
let driver = Driver::new("http://localhost:4444");

driver.start()?;
driver.navigate("https://example.com")?;

let button = driver.find(By::Id("submit"))?;
button.click()?;

driver.quit()?;
```

## License

MIT License
