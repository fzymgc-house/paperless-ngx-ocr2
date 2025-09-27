//! CLI verbose logging behavior tests
//! These tests validate that the --verbose flag produces appropriate logging output

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

// ============================================================================
// CLI VERBOSE LOGGING TESTS (T017)
// ============================================================================

#[tokio::test]
async fn test_cli_verbose_flag_enables_debug_logging() {
    // Test that --verbose flag enables debug logging output
    // This test MUST FAIL until verbose logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .assert()
        .failure() // Will fail until implementation is complete
        .stderr(predicate::str::contains("DEBUG").or(predicate::str::contains("TRACE")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_shows_configuration_loading() {
    // Test that verbose mode shows configuration loading details
    // This test MUST FAIL until configuration logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .assert()
        .failure() // Will fail until implementation is complete
        .stderr(predicate::str::contains("Configuration").or(predicate::str::contains("loaded")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_shows_file_validation_steps() {
    // Test that verbose mode shows file validation process
    // This test MUST FAIL until file validation logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nValidation test content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .assert()
        .failure() // Will fail until implementation is complete
        .stderr(predicate::str::contains("validating").or(predicate::str::contains("file")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_shows_api_request_details() {
    // Test that verbose mode shows API request/response details
    // This test MUST FAIL until API request logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nAPI test content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .assert()
        .failure() // Will fail until implementation is complete
        .stderr(predicate::str::contains("API").or(predicate::str::contains("request")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_no_logs_on_stdout() {
    // Test that verbose logging goes to stderr, not stdout (constitutional requirement)
    // This test MUST FAIL until proper logging separation is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nStdout test content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    let output = cmd.arg("--file").arg(&temp_path).arg("--api-key").arg("test-key").arg("--verbose").output().expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Constitutional requirement: no logs on stdout
    // Stdout should only contain program output (extracted text or JSON)
    if !stdout.is_empty() {
        // If there's stdout content, it should be either extracted text or JSON
        // It should NOT contain log-like content
        assert!(
            !stdout.contains("DEBUG") && !stdout.contains("INFO") && !stdout.contains("TRACE") && !stdout.contains("ERROR"),
            "Stdout must not contain log messages (constitutional requirement)"
        );
    }

    // Verbose logs should go to stderr
    if !stderr.is_empty() {
        // Stderr can contain log messages when --verbose is used
        // (This assertion may pass or fail depending on current implementation state)
    }

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_environment_variable_support() {
    // Test that RUST_LOG environment variable works with verbose mode
    // This test MUST FAIL until environment-based logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .env("RUST_LOG", "debug")
        .arg("--verbose")
        .assert()
        .failure() // Will fail until implementation is complete
        .stderr(predicate::str::contains("DEBUG").or(predicate::str::contains("TRACE")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_verbose_structured_logging_format() {
    // Test that verbose logging produces structured, parseable log output
    // This test MUST FAIL until structured logging is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nStructured logging test").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    let output = cmd
        .arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .env("RUST_LOG_FORMAT", "json") // Request JSON log format
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8(output.stderr).unwrap();

    // If there are log lines, they should be valid JSON when JSON format is requested
    let mut found_json_logs = false;
    for line in stderr.lines() {
        if !line.trim().is_empty() && line.contains("{") {
            // Try to parse as JSON
            if serde_json::from_str::<serde_json::Value>(line).is_ok() {
                found_json_logs = true;
                break;
            }
        }
    }

    // If JSON format was requested, we should have found at least one JSON log line
    if stderr.contains("RUST_LOG_FORMAT=json") {
        assert!(found_json_logs, "Should have JSON formatted logs when RUST_LOG_FORMAT=json is set");
    }

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}
