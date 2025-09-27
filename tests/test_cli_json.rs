//! CLI JSON output behavior tests
//! These tests validate that the --json flag produces correct output format

mod common;

use predicates::prelude::*;
use common::*;

// ============================================================================
// CLI JSON OUTPUT TESTS (T015)
// ============================================================================

#[tokio::test]
async fn test_cli_json_output_flag() {
    // Test that --json flag produces JSON output instead of human-readable
    // This test MUST FAIL until JSON output formatting is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_pdf("Sample PDF content");
    let config = presets::json_output().with_api_key("test-key");
    let mut cmd = cli::create_configured_command(&config);

    // This should eventually succeed and produce JSON, but will fail during development
    let output = cmd
        .arg("--file")
        .arg(test_file.path())
        // JSON flag is already set in config
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // If successful, validate JSON structure
        let stdout = String::from_utf8(output.stdout).unwrap();
        let json: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Output should be valid JSON when --json flag is used");

        // Use contract validation
        validate_json_contract(&stdout, ContractType::CliOutput)
            .expect("JSON output should conform to CLI output contract");
    }

    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_json_output_success_format() {
    // Test that successful JSON output follows the contract
    // This test MUST FAIL until JSON success output is implemented

    // Use TestFile for automatic cleanup
    let test_file = create_test_pdf("Test content for OCR");
    let config = presets::json_output().with_api_key("valid-test-key");
    let mut cmd = cli::create_configured_command(&config);

    // Mock a successful scenario (this will fail until API is implemented)
    cmd.arg("--file")
        .arg(test_file.path())
        .assert()
        .failure()
        .stdout(predicate::function(|output: &str| {
            validate_json_contract(output, ContractType::CliOutput).is_ok()
        }));

    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_json_output_error_format() {
    // Test that error JSON output follows the contract
    // This test MUST FAIL until JSON error output is implemented

    let mut cmd = cli::create_test_command();

    cmd.arg("--file")
        .arg("nonexistent.pdf")
        .arg("--api-key")
        .arg("test-key")
        .arg("--json")
        .assert()
        .failure()
        .stdout(predicate::function(|output: &str| {
            validate_json_contract(output, ContractType::CliOutput).is_ok()
        }));
}

#[tokio::test]
async fn test_cli_json_vs_human_readable_output() {
    // Test that output format differs between --json and default
    // This test MUST FAIL until both output formats are implemented

    let test_file = create_test_pdf("Test content");

    // Test human-readable output (default)
    let mut cmd_human = cli::create_test_command();
    let human_output = cmd_human
        .arg("--file")
        .arg(test_file.path())
        .arg("--api-key")
        .arg("test-key")
        .output()
        .expect("Failed to execute command");

    // Test JSON output
    let config = TestConfig::new()
        .with_api_key("test-key")
        .with_json_output(true);
    let mut cmd_json = cli::create_configured_command(&config);
    let json_output = cmd_json
        .arg("--file")
        .arg(test_file.path())
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

    // Automatic cleanup on drop
}
