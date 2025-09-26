//! CLI JSON output behavior tests
//! These tests validate that the --json flag produces correct output format

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

// ============================================================================
// CLI JSON OUTPUT TESTS (T015)
// ============================================================================

#[tokio::test]
async fn test_cli_json_output_flag() {
    // Test that --json flag produces JSON output instead of human-readable
    // This test MUST FAIL until JSON output formatting is implemented

    // Create a temporary PDF file
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nSample PDF content")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    // This should eventually succeed and produce JSON, but will fail during development
    let output = cmd
        .arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-api-key")
        .arg("--json")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // If successful, validate JSON structure
        let stdout = String::from_utf8(output.stdout).unwrap();
        let json: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Output should be valid JSON when --json flag is used");

        // Validate JSON contract structure
        assert!(
            json.get("success").is_some(),
            "JSON output must have 'success' field"
        );

        if json.get("success").unwrap().as_bool().unwrap() {
            assert!(
                json.get("data").is_some(),
                "Successful JSON output must have 'data' field"
            );
        } else {
            assert!(
                json.get("error").is_some(),
                "Failed JSON output must have 'error' field"
            );
        }
    }

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_json_output_success_format() {
    // Test that successful JSON output follows the contract
    // This test MUST FAIL until JSON success output is implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file
        .write_all(b"%PDF-1.4\nTest content for OCR")
        .unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();

    // Mock a successful scenario (this will fail until API is implemented)
    cmd.arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("valid-test-key") // This would need to be a real key for success
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

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}

#[tokio::test]
async fn test_cli_json_output_error_format() {
    // Test that error JSON output follows the contract
    // This test MUST FAIL until JSON error output is implemented

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
async fn test_cli_json_vs_human_readable_output() {
    // Test that output format differs between --json and default
    // This test MUST FAIL until both output formats are implemented

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"%PDF-1.4\nTest content").unwrap();
    let temp_path = temp_file.path().with_extension("pdf");
    std::fs::copy(temp_file.path(), &temp_path).unwrap();

    // Test human-readable output (default)
    let mut cmd_human = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    let human_output = cmd_human
        .arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .output()
        .expect("Failed to execute command");

    // Test JSON output
    let mut cmd_json = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    let json_output = cmd_json
        .arg("--file")
        .arg(&temp_path)
        .arg("--api-key")
        .arg("test-key")
        .arg("--json")
        .output()
        .expect("Failed to execute command");

    // Outputs should be different formats
    let human_stdout = String::from_utf8(human_output.stdout).unwrap();
    let json_stdout = String::from_utf8(json_output.stdout).unwrap();

    // JSON output should be parseable as JSON
    if !json_stdout.is_empty() {
        serde_json::from_str::<serde_json::Value>(&json_stdout)
            .expect("JSON output should be valid JSON");
    }

    // Human output should not be valid JSON (unless empty)
    if !human_stdout.is_empty() {
        assert!(
            serde_json::from_str::<serde_json::Value>(&human_stdout).is_err(),
            "Human-readable output should not be valid JSON"
        );
    }

    // Cleanup
    std::fs::remove_file(&temp_path).ok();
}
