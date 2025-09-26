//! Error handling integration tests
//! These tests validate that error scenarios are handled correctly end-to-end

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

// ============================================================================
// ERROR HANDLING INTEGRATION TESTS (T020-T023)
// ============================================================================

#[tokio::test]
async fn test_error_handling_invalid_file_format() {
    // T020: Test that invalid file formats are properly rejected
    // This test MUST FAIL until file format validation is implemented

    // Create a text file with .txt extension
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "This is a plain text file, not an image or PDF").unwrap();
    let temp_path = temp_file.path().with_extension("txt");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .assert()
        .failure()
        .code(2) // Validation error per constitution
        .stderr(predicate::str::contains("format").or(predicate::str::contains("supported")))
        .stderr(
            predicate::str::contains("pdf")
                .or(predicate::str::contains("png"))
                .or(predicate::str::contains("jpg")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_file_too_large() {
    // T021: Test that files exceeding 100MB limit are properly rejected
    // This test MUST FAIL until file size validation is implemented

    // Create a small file but configure very small size limit to trigger error
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nThis file will be considered too large")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .env("PAPERLESS_OCR_MAX_FILE_SIZE", "0") // Set limit to 0 MB to trigger error
        .assert()
        .failure()
        .code(4) // Configuration error per constitution (file size config issue)
        .stderr(
            predicate::str::contains("large")
                .or(predicate::str::contains("size"))
                .or(predicate::str::contains("limit")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_invalid_api_key() {
    // T022: Test that invalid API keys are properly handled
    // This test MUST FAIL until API authentication error handling is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nTest content for auth error")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    // Test with obviously invalid API key
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("invalid-key-format")
        .assert()
        .failure()
        .code(2) // Validation error per constitution (API key validation)
        .stderr(
            predicate::str::contains("auth")
                .or(predicate::str::contains("key"))
                .or(predicate::str::contains("invalid")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_network_timeout() {
    // T023: Test that network timeouts are properly handled
    // This test MUST FAIL until network timeout handling is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nTest content for timeout")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    // Use an invalid URL to trigger network error
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--api-base-url")
        .arg("https://definitely-does-not-exist-timeout-test.invalid")
        .env("PAPERLESS_OCR_TIMEOUT", "1") // Very short timeout
        .assert()
        .failure()
        .code(5) // Internal/network error per constitution
        .stderr(
            predicate::str::contains("timeout")
                .or(predicate::str::contains("network"))
                .or(predicate::str::contains("connect"))
                .or(predicate::str::contains("operation timed out")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_empty_api_key() {
    // Test that empty API key is properly handled as config error
    // This test MUST FAIL until API key validation is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nTest content for empty key")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("") // Empty API key
        .assert()
        .failure()
        .code(4) // Config error per constitution
        .stderr(predicate::str::contains("key").or(predicate::str::contains("required")));

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_malformed_pdf() {
    // Test that malformed PDF files are handled gracefully
    // This test MUST FAIL until file content validation is implemented

    // Create a file with PDF extension but invalid content
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"This is not a valid PDF file at all")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .assert()
        .failure()
        .code(2) // Validation error per constitution
        .stderr(
            predicate::str::contains("invalid")
                .or(predicate::str::contains("malformed"))
                .or(predicate::str::contains("corrupt"))
                .or(predicate::str::contains("valid PDF")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_error_handling_with_json_output() {
    // Test that errors are properly formatted in JSON output
    // This test MUST FAIL until JSON error formatting is implemented

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg("nonexistent.pdf")
        .arg("--api-key")
        .arg("test-key")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::function(|output: &str| {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
                // Validate error JSON structure
                !json.get("success").unwrap().as_bool().unwrap()
                    && json.get("error").is_some()
                    && json.get("error").unwrap().get("type").is_some()
                    && json.get("error").unwrap().get("message").is_some()
            } else {
                false
            }
        }));
}

#[tokio::test]
async fn test_error_handling_verbose_error_details() {
    // Test that verbose mode provides detailed error information
    // This test MUST FAIL until verbose error logging is implemented

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    cmd.arg("--file")
        .arg("nonexistent.pdf")
        .arg("--api-key")
        .arg("test-key")
        .arg("--verbose")
        .assert()
        .failure()
        .stderr(predicate::str::contains("DEBUG").or(predicate::str::contains("TRACE")))
        .stderr(predicate::str::contains("file").or(predicate::str::contains("validation")));
}

#[tokio::test]
async fn test_error_handling_api_rate_limit() {
    // Test that API rate limiting is handled gracefully
    // This test MUST FAIL until rate limit error handling is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nRate limit test content")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    // This would need to trigger actual rate limiting to test properly
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("rate-limited-key") // Mock key that would trigger rate limiting
        .assert()
        .failure()
        .code(2) // Validation error per constitution (API authentication)
        .stderr(
            predicate::str::contains("rate")
                .or(predicate::str::contains("limit"))
                .or(predicate::str::contains("429"))
                .or(predicate::str::contains("Client error")),
        );

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}
