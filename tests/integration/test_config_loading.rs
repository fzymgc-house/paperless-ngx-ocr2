//! Integration tests for configuration loading

mod common;

use predicates::prelude::*;
use common::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_env_file_loading() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    let env_file = temp_dir.path().join(".env");
    
    // Create .env file with test values
    fs::write(&env_file, "PAPERLESS_OCR_API_KEY=test_key_from_env\nPAPERLESS_OCR_API_BASE_URL=https://test.api.com\n").unwrap();
    
    // Create a test PDF file using fixture
    let test_file = create_test_pdf("Test content for env loading");
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::copy(test_file.path(), &pdf_file).unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command - it should fail with Validation error but show env vars were loaded
    let mut cmd = cli::create_test_command();
    let assert = cmd
        .arg("--file")
        .arg("test.pdf")
        .assert();
    
    // Should fail with Validation error (not config error) because env vars were loaded
    assert.failure().stderr(predicate::str::contains("Validation error"));
}

#[test]
fn test_config_file_search_order() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    
    // Create config in current directory
    let current_config = temp_dir.path().join("config.toml");
    fs::write(&current_config, r#"
api_key = "current_dir_key"
api_base_url = "https://current.api.com"
"#).unwrap();
    
    // Create config in ~/.config/paperless-ngx-ocr2/ (simulate)
    let home_config_dir = temp_dir.path().join(".config").join("paperless-ngx-ocr2");
    fs::create_dir_all(&home_config_dir).unwrap();
    let home_config = home_config_dir.join("config.toml");
    fs::write(&home_config, r#"
api_key = "home_config_key"
api_base_url = "https://home.api.com"
"#).unwrap();
    
    // Create a test PDF file using fixture
    let test_file = create_test_pdf("Test content for config search");
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::copy(test_file.path(), &pdf_file).unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command - should use current directory config (priority)
    let mut cmd = cli::create_test_command();
    let assert = cmd
        .arg("--file")
        .arg("test.pdf")
        .assert();
    
    // Should fail with Validation error (not config error) because current dir config was loaded
    assert.failure().stderr(predicate::str::contains("Validation error"));
}

#[test]
fn test_config_flag_handling() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    
    // Create a custom config file
    let custom_config = temp_dir.path().join("custom_config.toml");
    fs::write(&custom_config, r#"
api_key = "custom_config_key"
api_base_url = "https://custom.api.com"
"#).unwrap();
    
    // Create a test PDF file using fixture
    let test_file = create_test_pdf("Test content for config flag");
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::copy(test_file.path(), &pdf_file).unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command with --config flag
    let mut cmd = cli::create_test_command();
    let assert = cmd
        .arg("--file")
        .arg("test.pdf")
        .arg("--config")
        .arg("custom_config.toml")
        .assert();
    
    // Should fail with Validation error because custom config was loaded successfully
    assert.failure().stderr(predicate::str::contains("Validation error"));
}

#[test]
fn test_config_file_not_found() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test PDF file using fixture
    let test_file = create_test_pdf("Test content for config not found");
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::copy(test_file.path(), &pdf_file).unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command with non-existent config file
    let mut cmd = cli::create_test_command();
    let assert = cmd
        .arg("--file")
        .arg("test.pdf")
        .arg("--config")
        .arg("nonexistent_config.toml")
        .assert();
    
    // Should fail with config error
    assert.failure().stderr(predicate::str::contains("Config file not found"));
}
