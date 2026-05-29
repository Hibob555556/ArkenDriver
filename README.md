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
