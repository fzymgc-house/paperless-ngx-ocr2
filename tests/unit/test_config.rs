//! Unit tests for configuration validation

use paperless_ngx_ocr2::{Config, Error};
use std::env;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_config_default_values() {
    let config = Config::default();
    
    assert_eq!(config.api_base_url, "https://api.mistral.ai");
    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.max_file_size_mb, 100);
    assert_eq!(config.log_level, "info");
    assert!(config.api_key.is_empty()); // Default is empty, set via CLI/env
}

#[test]
fn test_config_validation_valid() {
    let config = Config {
        api_key: "sk-test123456789".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_empty_api_key() {
    let config = Config {
        api_key: "".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    
    assert!(config.validate().is_err());
    assert!(matches!(config.validate().unwrap_err(), Error::Config(_)));
}

#[test]
fn test_config_validation_invalid_url() {
    let config = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "not-a-valid-url".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    
    assert!(config.validate().is_err());
    assert!(matches!(config.validate().unwrap_err(), Error::Config(_)));
}

#[test]
fn test_config_validation_timeout_range() {
    // Test timeout too low
    let config_low = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 0,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    assert!(config_low.validate().is_err());
    
    // Test timeout too high
    let config_high = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 301,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    assert!(config_high.validate().is_err());
    
    // Test valid timeout
    let config_valid = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 60,
        max_file_size_mb: 50,
        log_level: "info".to_string(),
    };
    assert!(config_valid.validate().is_ok());
}

#[test]
fn test_config_validation_file_size_range() {
    // Test file size too low
    let config_low = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 0,
        log_level: "info".to_string(),
    };
    assert!(config_low.validate().is_err());
    
    // Test file size too high
    let config_high = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 101,
        log_level: "info".to_string(),
    };
    assert!(config_high.validate().is_err());
}

#[test]
fn test_config_validation_log_level() {
    let valid_levels = ["error", "warn", "info", "debug", "trace"];
    
    for level in valid_levels {
        let config = Config {
            api_key: "sk-test123".to_string(),
            api_base_url: "https://api.mistral.ai".to_string(),
            timeout_seconds: 30,
            max_file_size_mb: 50,
            log_level: level.to_string(),
        };
        assert!(config.validate().is_ok(), "Level '{}' should be valid", level);
    }
    
    // Test invalid log level
    let config_invalid = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 30,
        max_file_size_mb: 50,
        log_level: "invalid".to_string(),
    };
    assert!(config_invalid.validate().is_err());
}

#[test]
fn test_config_toml_serialization() {
    let config = Config {
        api_key: "sk-test123".to_string(),
        api_base_url: "https://api.mistral.ai".to_string(),
        timeout_seconds: 60,
        max_file_size_mb: 50,
        log_level: "debug".to_string(),
    };
    
    // Test serialization to TOML
    let toml_str = toml::to_string(&config).expect("Should serialize to TOML");
    
    // Test deserialization from TOML
    let deserialized: Config = toml::from_str(&toml_str).expect("Should deserialize from TOML");
    
    assert_eq!(config.api_key, deserialized.api_key);
    assert_eq!(config.api_base_url, deserialized.api_base_url);
    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(config.max_file_size_mb, deserialized.max_file_size_mb);
    assert_eq!(config.log_level, deserialized.log_level);
}

#[test]
fn test_config_load_without_validation() {
    // Test loading config without validation (for CLI override scenarios)
    let result = Config::load_without_validation();
    
    // Should succeed even with default empty API key
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert_eq!(config.api_base_url, "https://api.mistral.ai");
    assert_eq!(config.timeout_seconds, 30);
}

#[tokio::test]
async fn test_config_environment_variable_overrides() {
    // Set test environment variables
    env::set_var("PAPERLESS_OCR_API_KEY", "env-test-key");
    env::set_var("PAPERLESS_OCR_API_BASE_URL", "https://env.test.url");
    env::set_var("PAPERLESS_OCR_TIMEOUT", "45");
    env::set_var("PAPERLESS_OCR_MAX_FILE_SIZE", "75");
    env::set_var("PAPERLESS_OCR_LOG_LEVEL", "debug");
    
    let mut config = Config::default();
    config.apply_env_overrides();
    
    assert_eq!(config.api_key, "env-test-key");
    assert_eq!(config.api_base_url, "https://env.test.url");
    assert_eq!(config.timeout_seconds, 45);
    assert_eq!(config.max_file_size_mb, 75);
    assert_eq!(config.log_level, "debug");
    
    // Cleanup
    env::remove_var("PAPERLESS_OCR_API_KEY");
    env::remove_var("PAPERLESS_OCR_API_BASE_URL");
    env::remove_var("PAPERLESS_OCR_TIMEOUT");
    env::remove_var("PAPERLESS_OCR_MAX_FILE_SIZE");
    env::remove_var("PAPERLESS_OCR_LOG_LEVEL");
}
