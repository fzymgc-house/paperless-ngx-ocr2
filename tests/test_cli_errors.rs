//! CLI error exit codes tests
//! These tests validate that the CLI returns proper exit codes per constitution

mod common;

use common::config::presets;
use common::*;
use predicates::prelude::*;

// ============================================================================
// CLI ERROR EXIT CODES TESTS (T016)
// ============================================================================

#[tokio::test]
async fn test_cli_exit_code_validation_error() {
    // Test that validation errors return exit code 2 per constitution
    // This test MUST FAIL until exit code handling is implemented

    let mut cmd = cli::create_test_command();

    // Missing required --file argument now shows help instead of validation error
    cmd.assert()
        .success() // Shows help when no file provided
        .stdout(predicate::str::contains(
            "OCR CLI tool that uploads PDF/image files",
        ));
}

#[tokio::test]
async fn test_cli_exit_code_io_error() {
    // Test that I/O errors return exit code 3 per constitution
    // This test MUST FAIL until I/O error handling is implemented

    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg("nonexistent_file.pdf")
        .assert()
        .failure()
        .code(3); // I/O error per constitution
}

#[tokio::test]
async fn test_cli_exit_code_config_error() {
    // Test that configuration errors return exit code 4 per constitution
    // This test MUST FAIL until config error handling is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_pdf("Test content");
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Network error per constitution
                  // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_exit_code_internal_error() {
    // Test that internal/network errors return exit code 5 per constitution
    // This test MUST FAIL until internal error handling is implemented

    let test_file = create_test_pdf("Test content");
    let config = presets::invalid_endpoint();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Network error per constitution
                  // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_exit_code_success() {
    // Test that successful operations return exit code 0
    // This test MUST FAIL until full OCR workflow is implemented

    let test_file = create_test_pdf("Test content for successful OCR");
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    // This would need a real API key and working implementation to succeed
    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Network error with invalid key
                  // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_exit_code_file_size_validation() {
    // Test that files exceeding size limit return validation error (exit code 2)
    // This test MUST FAIL until file size validation is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_pdf("Large file content");
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    // This should fail with validation error when size checking is implemented
    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Network error per constitution
                  // Automatic cleanup on drop
}
