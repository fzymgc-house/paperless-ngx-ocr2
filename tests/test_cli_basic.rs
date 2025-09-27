//! Basic CLI behavior tests
//! These tests validate that the CLI interface works correctly

mod common;

use common::*;
use common::config::presets;
use predicates::prelude::*;

// ============================================================================
// CLI SMOKE TESTS (T013)
// ============================================================================

#[tokio::test]
async fn test_cli_smoke_help_command() {
    // Test that the CLI binary can be executed and shows help
    // This test MUST FAIL until CLI help functionality is properly implemented

    let mut cmd = cli::create_test_command();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A command-line tool for extracting text",
        ))
        .stdout(predicate::str::contains("--file"))
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--verbose"));
}

#[tokio::test]
async fn test_cli_smoke_version_command() {
    // Test that the CLI binary shows version information
    // This test MUST FAIL until CLI version functionality is implemented

    let mut cmd = cli::create_test_command();

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("paperless-ngx-ocr2"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[tokio::test]
async fn test_cli_smoke_no_args_shows_help() {
    // Test that running without arguments shows help
    // This test MUST FAIL until CLI argument validation is implemented

    let mut cmd = cli::create_test_command();

    cmd.assert()
        .success() // Should succeed and show help since file is optional
        .stdout(predicate::str::contains(
            "OCR CLI tool that uploads PDF/image files",
        ));
}

// ============================================================================
// CLI FILE ARGUMENT TESTS (T014)
// ============================================================================

#[tokio::test]
async fn test_cli_file_argument_required() {
    // Test that --file argument is required for OCR processing (not for completions)
    // This test MUST FAIL until CLI file argument validation is implemented

    let mut cmd = cli::create_test_command();

    // When no file is provided and no completions are requested, should show help
    cmd.assert()
        .success() // Should succeed and show help since file is optional
        .stdout(predicate::str::contains(
            "OCR CLI tool that uploads PDF/image files",
        ));
}

#[tokio::test]
async fn test_cli_file_argument_nonexistent_file() {
    // Test that CLI properly handles nonexistent files
    // This test MUST FAIL until file validation is implemented

    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg("nonexistent_file.pdf")
        .assert()
        .failure()
        .code(3) // I/O error per constitution
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

#[tokio::test]
async fn test_cli_file_argument_invalid_format() {
    // Test that CLI rejects invalid file formats
    // This test MUST FAIL until file format validation is implemented

    // Use TestFile for automatic cleanup
    let invalid_file = create_invalid_file();
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg(invalid_file.path())
        .assert()
        .failure()
        .code(2) // Validation error per constitution
        .stderr(predicate::str::contains("format").or(predicate::str::contains("supported")));
    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_file_argument_valid_pdf() {
    // Test that CLI accepts valid PDF files (should fail at API call stage)
    // This test MUST FAIL until file validation and API integration is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_pdf("Valid PDF content");
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Should fail with network error, not validation error
                  // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_file_argument_valid_image() {
    // Test that CLI accepts valid image files
    // This test MUST FAIL until file validation and API integration is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_png();
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .code(5); // Should fail with network error, not validation error
                  // Automatic cleanup on drop
}
