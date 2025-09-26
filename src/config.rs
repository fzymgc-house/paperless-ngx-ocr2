//! Configuration management for the OCR CLI tool

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Jitter factor (0.0 to 1.0) to add randomness to delays
    pub jitter_factor: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            exponential_backoff: true,
            jitter_factor: 0.1,
        }
    }
}

impl RetryPolicy {
    /// Calculate delay for a given retry attempt
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let mut delay_ms = if self.exponential_backoff {
            self.base_delay_ms * 2_u64.pow(attempt - 1)
        } else {
            self.base_delay_ms
        };

        // Apply jitter
        if self.jitter_factor > 0.0 {
            let jitter =
                (delay_ms as f64 * self.jitter_factor * (rand::random::<f64>() - 0.5) * 2.0) as i64;
            delay_ms = (delay_ms as i64 + jitter).max(1) as u64;
        }

        // Cap at maximum delay
        delay_ms = delay_ms.min(self.max_delay_ms);

        Duration::from_millis(delay_ms)
    }

    /// Validate retry policy configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_retries > 10 {
            return Err(Error::Config("Max retries cannot exceed 10".to_string()));
        }

        if self.base_delay_ms == 0 {
            return Err(Error::Config(
                "Base delay must be greater than 0".to_string(),
            ));
        }

        if self.max_delay_ms < self.base_delay_ms {
            return Err(Error::Config("Max delay must be >= base delay".to_string()));
        }

        if self.jitter_factor < 0.0 || self.jitter_factor > 1.0 {
            return Err(Error::Config(
                "Jitter factor must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Mistral AI API key
    pub api_key: String,

    /// Mistral AI API base URL
    #[serde(default = "default_api_base_url")]
    pub api_base_url: String,

    /// Request timeout in seconds
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    /// Maximum file size in MB
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,

    /// Logging level
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Retry policy configuration
    #[serde(default = "default_retry_policy")]
    pub retry_policy: RetryPolicy,
}

fn default_api_base_url() -> String {
    "https://api.mistral.ai".to_string()
}

fn default_timeout_seconds() -> u64 {
    30
}

fn default_max_file_size_mb() -> u64 {
    100
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_retry_policy() -> RetryPolicy {
    RetryPolicy::default()
}

impl Config {
    /// Load configuration from file with environment variable overrides
    pub fn load() -> Result<Self> {
        // Load .env file first
        dotenv::dotenv().ok(); // Ignore errors if .env doesn't exist

        let mut config = Self::load_from_file()?;
        config.apply_env_overrides();
        config.validate()?;
        Ok(config)
    }

    /// Load configuration without validation (for CLI override scenarios)
    pub fn load_without_validation() -> Result<Self> {
        // Load .env file first
        dotenv::dotenv().ok(); // Ignore errors if .env doesn't exist

        let mut config = Self::load_from_file().unwrap_or_default();
        config.apply_env_overrides();
        Ok(config)
    }

    /// Load configuration from TOML file
    fn load_from_file() -> Result<Self> {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

            let config: Config = toml::from_str(&content)
                .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?;

            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Self::default())
        }
    }

    /// Load configuration from a specific file path
    pub fn load_from_path(path: &str) -> Result<Self> {
        // Load .env file first
        dotenv::dotenv().ok(); // Ignore errors if .env doesn't exist

        let config_path = PathBuf::from(path);

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

            let mut config: Config = toml::from_str(&content)
                .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?;

            config.apply_env_overrides();
            config.validate()?;
            Ok(config)
        } else {
            Err(Error::Config(format!("Config file not found: {}", path)))
        }
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        if let Ok(api_key) = env::var("PAPERLESS_OCR_API_KEY") {
            self.api_key = api_key;
        }

        if let Ok(api_base_url) = env::var("PAPERLESS_OCR_API_BASE_URL") {
            self.api_base_url = api_base_url;
        }

        if let Ok(timeout) = env::var("PAPERLESS_OCR_TIMEOUT") {
            if let Ok(timeout_val) = timeout.parse::<u64>() {
                self.timeout_seconds = timeout_val;
            }
        }

        if let Ok(max_size) = env::var("PAPERLESS_OCR_MAX_FILE_SIZE") {
            if let Ok(size_val) = max_size.parse::<u64>() {
                self.max_file_size_mb = size_val;
            }
        }

        if let Ok(log_level) = env::var("PAPERLESS_OCR_LOG_LEVEL") {
            self.log_level = log_level;
        }
    }

    /// Validate configuration according to data model rules
    pub fn validate(&self) -> Result<()> {
        // Validate API key
        if self.api_key.is_empty() {
            return Err(Error::Config("API key must not be empty".to_string()));
        }

        // Validate API base URL
        Url::parse(&self.api_base_url)
            .map_err(|_| Error::Config("API base URL must be a valid URL".to_string()))?;

        // Validate timeout range
        if self.timeout_seconds < 1 || self.timeout_seconds > 300 {
            return Err(Error::Config(
                "Timeout must be between 1 and 300 seconds".to_string(),
            ));
        }

        // Validate file size range
        if self.max_file_size_mb < 1 || self.max_file_size_mb > 100 {
            return Err(Error::Config(
                "Max file size must be between 1 and 100 MB".to_string(),
            ));
        }

        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(Error::Config(format!(
                "Log level must be one of: {}",
                valid_levels.join(", ")
            )));
        }

        // Validate retry policy
        self.retry_policy.validate()?;

        Ok(())
    }

    /// Get the default configuration file path
    /// Search order: current directory -> ~/.config/paperless-ngx-ocr2/
    fn get_config_path() -> PathBuf {
        // First try current directory
        let current_dir_config = PathBuf::from("config.toml");
        if current_dir_config.exists() {
            return current_dir_config;
        }

        // Then try XDG config directory
        if let Ok(config_dir) = env::var("XDG_CONFIG_HOME") {
            let xdg_config = PathBuf::from(config_dir)
                .join("paperless-ngx-ocr2")
                .join("config.toml");
            if xdg_config.exists() {
                return xdg_config;
            }
        }

        // Finally try ~/.config/paperless-ngx-ocr2/
        if let Ok(home_dir) = env::var("HOME") {
            let home_config = PathBuf::from(home_dir)
                .join(".config")
                .join("paperless-ngx-ocr2")
                .join("config.toml");
            if home_config.exists() {
                return home_config;
            }
        }

        // Return current directory as default (will be created if needed)
        PathBuf::from("config.toml")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(), // Will be set via env var or CLI arg
            api_base_url: default_api_base_url(),
            timeout_seconds: default_timeout_seconds(),
            max_file_size_mb: default_max_file_size_mb(),
            log_level: default_log_level(),
            retry_policy: default_retry_policy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let config = Config::default();

        assert_eq!(config.api_base_url, "https://api.mistral.ai");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_file_size_mb, 100);
        assert_eq!(config.log_level, "info");
        assert!(config.api_key.is_empty());
    }

    #[test]
    fn test_validation_valid_config() {
        let config = Config {
            api_key: "sk-test123456789".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 50,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_api_key() {
        let config = Config {
            api_key: "".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 50,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_url() {
        let config = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "not-a-valid-url".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 50,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_timeout_range() {
        // Test timeout too low
        let config_low = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 0,
            max_file_size_mb: 50,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };
        assert!(config_low.validate().is_err());

        // Test timeout too high
        let config_high = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 301,
            max_file_size_mb: 50,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };
        assert!(config_high.validate().is_err());
    }

    #[test]
    fn test_validation_file_size_range() {
        // Test file size too low
        let config_low = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 0,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };
        assert!(config_low.validate().is_err());

        // Test file size too high
        let config_high = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 101,
            log_level: "info".to_string(),
            retry_policy: RetryPolicy::default(),
        };
        assert!(config_high.validate().is_err());
    }

    #[test]
    fn test_validation_log_level() {
        let valid_levels = ["error", "warn", "info", "debug", "trace"];

        for level in valid_levels {
            let config = Config {
                api_key: "sk-test123".to_string(),
                api_base_url: "https://api.mistral.ai".to_string(),
                timeout_seconds: 30,
                max_file_size_mb: 50,
                log_level: level.to_string(),
                retry_policy: RetryPolicy::default(),
            };
            assert!(
                config.validate().is_ok(),
                "Level '{}' should be valid",
                level
            );
        }

        // Test invalid log level
        let config_invalid = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 50,
            log_level: "invalid".to_string(),
            retry_policy: RetryPolicy::default(),
        };
        assert!(config_invalid.validate().is_err());
    }
}
