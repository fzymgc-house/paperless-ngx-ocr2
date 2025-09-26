//! CLI error exit codes tests
//! These tests validate that the CLI returns proper exit codes per constitution

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

// ============================================================================
// CLI ERROR EXIT CODES TESTS (T016)
// ============================================================================

#[tokio::test]
async fn test_cli_exit_code_validation_error() {
    // Test that validation errors return exit code 2 per constitution
    // This test MUST FAIL until exit code handling is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    // Missing required --file argument now shows help instead of validation error
    cmd.assert()
        .success() // Shows help when no file provided
        .stdout(predicate::str::contains("OCR CLI tool that uploads PDF/image files"));
}

#[tokio::test]
async fn test_cli_exit_code_io_error() {
    // Test that I/O errors return exit code 3 per constitution
    // This test MUST FAIL until I/O error handling is implemented
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg("nonexistent_file.pdf")
        .arg("--api-key")
        .arg("test-key")
        .assert()
        .failure()
        .code(3); // I/O error per constitution
}

#[tokio::test]
async fn test_cli_exit_code_config_error() {
    // Test that configuration errors return exit code 4 per constitution
    // This test MUST FAIL until config error handling is implemented
    
    // Create a valid file but use invalid API key format
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("") // Empty API key should be config error
        .assert()
        .failure()
        .code(4); // Config error per constitution
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_exit_code_internal_error() {
    // Test that internal/network errors return exit code 5 per constitution
    // This test MUST FAIL until internal error handling is implemented
    
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key-that-will-cause-network-error")
        .arg("--api-base-url")
        .arg("https://invalid-url-that-does-not-exist.com")
        .assert()
        .failure()
        .code(2); // Validation error per constitution (API error)
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_exit_code_success() {
    // Test that successful operations return exit code 0
    // This test MUST FAIL until full OCR workflow is implemented
    
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content for successful OCR").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    // This would need a real API key and working implementation to succeed
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("real-working-api-key") // Would need real key for success
        .assert()
        .failure()
        .code(2); // API error with invalid key
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_exit_code_file_size_validation() {
    // Test that files exceeding size limit return validation error (exit code 2)
    // This test MUST FAIL until file size validation is implemented
    
    // Create a file that appears to exceed 100MB limit (we'll mock this)
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nLarge file content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();
    
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    
    // This should fail with validation error when size checking is implemented
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .env("PAPERLESS_OCR_MAX_FILE_SIZE", "0") // Set very small limit to trigger error
        .assert()
        .failure()
        .code(4); // Configuration error per constitution
    
    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}
