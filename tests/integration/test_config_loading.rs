//! Integration tests for configuration loading

use assert_cmd::Command;
use predicates::prelude::*;
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
    
    // Create a test PDF file
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::write(&pdf_file, b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/MediaBox [0 0 612 792]\n>>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer\n<<\n/Size 4\n/Root 1 0 R\n>>\nstartxref\n174\n%%EOF").unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command - it should fail with Validation error but show env vars were loaded
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
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
    
    // Create a test PDF file
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::write(&pdf_file, b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/MediaBox [0 0 612 792]\n>>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer\n<<\n/Size 4\n/Root 1 0 R\n>>\nstartxref\n174\n%%EOF").unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command - should use current directory config (priority)
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
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
    
    // Create a test PDF file
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::write(&pdf_file, b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/MediaBox [0 0 612 792]\n>>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer\n<<\n/Size 4\n/Root 1 0 R\n>>\nstartxref\n174\n%%EOF").unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command with --config flag
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
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
    
    // Create a test PDF file
    let pdf_file = temp_dir.path().join("test.pdf");
    fs::write(&pdf_file, b"%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/MediaBox [0 0 612 792]\n>>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer\n<<\n/Size 4\n/Root 1 0 R\n>>\nstartxref\n174\n%%EOF").unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Run the command with non-existent config file
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    let assert = cmd
        .arg("--file")
        .arg("test.pdf")
        .arg("--config")
        .arg("nonexistent_config.toml")
        .assert();
    
    // Should fail with config error
    assert.failure().stderr(predicate::str::contains("Config file not found"));
}
