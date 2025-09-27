//! Test configuration utilities
//!
//! This module provides utilities for managing test configurations,
//! including API keys, endpoints, and other test-specific settings.

#![allow(dead_code)]

use assert_cmd::Command;
use std::collections::HashMap;

/// Test configuration for API and CLI settings
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub api_key: String,
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub max_file_size_mb: u64,
    pub log_level: String,
    pub json_output: bool,
    pub verbose: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            api_key: "test-api-key".to_string(),
            api_base_url: "https://api.test.com".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 100,
            log_level: "info".to_string(),
            json_output: false,
            verbose: false,
        }
    }
}

impl TestConfig {
    /// Creates a new test configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key
    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = key.to_string();
        self
    }

    /// Sets the API base URL
    pub fn with_api_base_url(mut self, url: &str) -> Self {
        self.api_base_url = url.to_string();
        self
    }

    /// Sets the timeout in seconds
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout_seconds = timeout;
        self
    }

    /// Sets the max file size in MB
    pub fn with_max_file_size(mut self, size_mb: u64) -> Self {
        self.max_file_size_mb = size_mb;
        self
    }

    /// Sets the log level
    pub fn with_log_level(mut self, level: &str) -> Self {
        self.log_level = level.to_string();
        self
    }

    /// Enables JSON output
    pub fn with_json_output(mut self, enabled: bool) -> Self {
        self.json_output = enabled;
        self
    }

    /// Enables verbose logging
    pub fn with_verbose(mut self, enabled: bool) -> Self {
        self.verbose = enabled;
        self
    }

    /// Applies this configuration to a CLI command
    pub fn apply_to_command(&self, cmd: &mut Command) {
        cmd.arg("--api-key")
            .arg(&self.api_key)
            .arg("--api-base-url")
            .arg(&self.api_base_url)
            .env("PAPERLESS_OCR_TIMEOUT", self.timeout_seconds.to_string())
            .env(
                "PAPERLESS_OCR_MAX_FILE_SIZE",
                self.max_file_size_mb.to_string(),
            )
            .env("PAPERLESS_OCR_LOG_LEVEL", &self.log_level);

        if self.json_output {
            cmd.arg("--json");
        }

        if self.verbose {
            cmd.arg("--verbose");
        }
    }

    /// Creates environment variables from this configuration
    pub fn to_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("PAPERLESS_OCR_API_KEY".to_string(), self.api_key.clone());
        vars.insert(
            "PAPERLESS_OCR_API_BASE_URL".to_string(),
            self.api_base_url.clone(),
        );
        vars.insert(
            "PAPERLESS_OCR_TIMEOUT".to_string(),
            self.timeout_seconds.to_string(),
        );
        vars.insert(
            "PAPERLESS_OCR_MAX_FILE_SIZE".to_string(),
            self.max_file_size_mb.to_string(),
        );
        vars.insert(
            "PAPERLESS_OCR_LOG_LEVEL".to_string(),
            self.log_level.clone(),
        );
        vars
    }
}

/// Predefined test configurations for common scenarios
pub mod presets {
    use super::TestConfig;

    /// Configuration for testing with invalid API key
    pub fn invalid_api_key() -> TestConfig {
        TestConfig::new().with_api_key("invalid-key")
    }

    /// Configuration for testing with timeout
    pub fn with_timeout(timeout_seconds: u64) -> TestConfig {
        TestConfig::new().with_timeout(timeout_seconds)
    }

    /// Configuration for testing with JSON output
    pub fn json_output() -> TestConfig {
        TestConfig::new().with_json_output(true)
    }

    /// Configuration for testing with verbose logging
    pub fn verbose() -> TestConfig {
        TestConfig::new().with_verbose(true)
    }

    /// Configuration for testing with debug logging
    pub fn debug() -> TestConfig {
        TestConfig::new().with_log_level("debug")
    }

    /// Configuration for testing large files
    pub fn large_files() -> TestConfig {
        TestConfig::new().with_max_file_size(200)
    }

    /// Configuration for testing network timeouts
    pub fn network_timeout() -> TestConfig {
        TestConfig::new()
            .with_timeout(2)
            .with_api_base_url("https://httpbin.org/delay/10")
    }

    /// Configuration for testing invalid endpoints
    pub fn invalid_endpoint() -> TestConfig {
        TestConfig::new().with_api_base_url("https://invalid-endpoint.invalid")
    }

    /// Configuration for testing localhost endpoints
    pub fn localhost(port: u16) -> TestConfig {
        TestConfig::new().with_api_base_url(&format!("https://localhost:{}", port))
    }
}

/// Test configuration builder for more complex scenarios
pub struct TestConfigBuilder {
    config: TestConfig,
}

impl TestConfigBuilder {
    /// Creates a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: TestConfig::default(),
        }
    }

    /// Sets the API key
    pub fn api_key(mut self, key: &str) -> Self {
        self.config.api_key = key.to_string();
        self
    }

    /// Sets the API base URL
    pub fn api_base_url(mut self, url: &str) -> Self {
        self.config.api_base_url = url.to_string();
        self
    }

    /// Sets the timeout in seconds
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.config.timeout_seconds = timeout;
        self
    }

    /// Sets the max file size in MB
    pub fn max_file_size(mut self, size_mb: u64) -> Self {
        self.config.max_file_size_mb = size_mb;
        self
    }

    /// Sets the log level
    pub fn log_level(mut self, level: &str) -> Self {
        self.config.log_level = level.to_string();
        self
    }

    /// Enables JSON output
    pub fn json_output(mut self, enabled: bool) -> Self {
        self.config.json_output = enabled;
        self
    }

    /// Enables verbose logging
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.config.verbose = enabled;
        self
    }

    /// Builds the final configuration
    pub fn build(self) -> TestConfig {
        self.config
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TestConfig::default();
        assert_eq!(config.api_key, "test-api-key");
        assert_eq!(config.api_base_url, "https://api.test.com");
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_config_builder() {
        let config = TestConfigBuilder::new()
            .api_key("custom-key")
            .timeout(60)
            .json_output(true)
            .build();

        assert_eq!(config.api_key, "custom-key");
        assert_eq!(config.timeout_seconds, 60);
        assert!(config.json_output);
    }

    #[test]
    fn test_preset_configs() {
        let invalid_config = presets::invalid_api_key();
        assert_eq!(invalid_config.api_key, "invalid-key");

        let timeout_config = presets::with_timeout(10);
        assert_eq!(timeout_config.timeout_seconds, 10);

        let json_config = presets::json_output();
        assert!(json_config.json_output);
    }

    #[test]
    fn test_env_vars() {
        let config = TestConfig::new().with_api_key("test-key").with_timeout(45);

        let env_vars = config.to_env_vars();
        assert_eq!(
            env_vars.get("PAPERLESS_OCR_API_KEY"),
            Some(&"test-key".to_string())
        );
        assert_eq!(
            env_vars.get("PAPERLESS_OCR_TIMEOUT"),
            Some(&"45".to_string())
        );
    }
}
