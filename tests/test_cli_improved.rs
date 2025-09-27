//! CLI behavior tests - Improved version using test utilities
//!
//! This file demonstrates improved CLI testing patterns using the common test utilities.
//! It shows how to use TestConfig for consistent CLI setup, TestFile for automatic cleanup,
//! and contract validation for output verification.

mod common;

use common::config::presets;
use common::fixtures::{create_invalid_file, create_test_png};
use common::performance::{measure_performance_async, stress, Benchmark};
use common::*;
use predicates::prelude::*;
use std::time::Duration;

// ============================================================================
// CLI SMOKE TESTS WITH IMPROVED UTILITIES
// ============================================================================

#[tokio::test]
async fn test_cli_smoke_help_command() {
    // Use test utilities for consistent command creation
    let mut cmd = cli::create_test_command();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A command-line tool for extracting text"))
        .stdout(predicate::str::contains("--file"))
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("--verbose"));
}

#[tokio::test]
async fn test_cli_smoke_version_command() {
    let mut cmd = cli::create_test_command();

    cmd.arg("--version").assert().success().stdout(predicate::str::contains("paperless-ngx-ocr2")).stdout(predicate::str::contains("0.1.0"));
}

#[tokio::test]
async fn test_cli_smoke_no_args_shows_help() {
    let mut cmd = cli::create_test_command();

    cmd.assert().success().stdout(predicate::str::contains("OCR CLI tool that uploads PDF/image files"));
}

// ============================================================================
// CLI FILE ARGUMENT TESTS WITH IMPROVED UTILITIES
// ============================================================================

#[tokio::test]
async fn test_cli_file_argument_nonexistent_file() {
    // Use TestConfig for consistent setup
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
    // Use TestFile for temporary PDF
    let test_file = create_test_pdf("Valid PDF content");
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file").arg(test_file.path()).assert().failure().code(5); // Should fail with network error (code 5)
                                                                        // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_file_argument_valid_image() {
    // Use TestFile for temporary PNG
    let test_file = create_test_png();
    let config = presets::invalid_api_key();
    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file").arg(test_file.path()).assert().failure().code(5); // Should fail with network error (code 5)
                                                                        // Automatic cleanup on drop
}

// ============================================================================
// CLI JSON OUTPUT TESTS WITH CONTRACT VALIDATION
// ============================================================================

#[tokio::test]
async fn test_cli_json_output_flag() {
    // Use TestFile and JSON config preset
    let test_file = create_test_pdf("Sample PDF content");
    let config = presets::json_output().with_api_key("test-api-key");

    let mut cmd = cli::create_configured_command(&config);

    // Test JSON output
    let output = cmd.arg("--file").arg(test_file.path()).output().expect("Failed to execute command");

    if output.status.success() {
        // Validate JSON structure using contract validation
        let stdout = String::from_utf8(output.stdout).unwrap();
        let _json: serde_json::Value = serde_json::from_str(&stdout).expect("Output should be valid JSON when --json flag is used");

        // Use contract validation
        validate_json_contract(&stdout, ContractType::CliOutput).expect("JSON output should conform to CLI output contract");
    }
    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_json_output_error_format() {
    let config = presets::json_output();

    let mut cmd = cli::create_configured_command(&config);

    cmd.arg("--file").arg("nonexistent.pdf").assert().failure().stdout(predicate::function(|output: &str| {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
            // Validate error JSON structure using contract validation
            validate_json_contract(output, ContractType::CliOutput).is_ok() && !json.get("success").unwrap().as_bool().unwrap() && json.get("error").is_some()
        } else {
            false
        }
    }));
}

#[tokio::test]
async fn test_cli_json_vs_human_readable_output() {
    // Use TestFile for consistent test data
    let test_file = create_test_pdf("Test content");
    let config = TestConfig::new().with_api_key("test-key");

    // Test human-readable output (default)
    let mut cmd_human = cli::create_configured_command(&config);
    let human_output = cmd_human.arg("--file").arg(test_file.path()).output().expect("Failed to execute command");

    // Test JSON output
    let json_config = config.with_json_output(true);
    let mut cmd_json = cli::create_configured_command(&json_config);
    let json_output = cmd_json.arg("--file").arg(test_file.path()).output().expect("Failed to execute command");

    // Outputs should be different formats
    let human_stdout = String::from_utf8(human_output.stdout).unwrap();
    let json_stdout = String::from_utf8(json_output.stdout).unwrap();

    // JSON output should be parseable as JSON
    if !json_stdout.is_empty() {
        serde_json::from_str::<serde_json::Value>(&json_stdout).expect("JSON output should be valid JSON");
    }

    // Human output should not be valid JSON (unless empty)
    if !human_stdout.is_empty() {
        assert!(serde_json::from_str::<serde_json::Value>(&human_stdout).is_err(), "Human-readable output should not be valid JSON");
    }
    // Automatic cleanup on drop
}

// ============================================================================
// CLI PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_cli_performance_with_large_file() {
    // Create a larger test file
    let large_file = create_large_test_pdf(1); // 1MB
    let config = presets::invalid_api_key();

    // Measure CLI performance
    measure_performance_async("cli_large_file", Duration::from_secs(5), || async {
        let mut cmd = cli::create_configured_command(&config);
        let _output = cmd.arg("--file").arg(large_file.path()).output();
    })
    .await;
    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_benchmark() {
    let test_file = create_test_pdf("Benchmark test content");
    let config = presets::invalid_api_key();

    // Run CLI benchmark
    let results = Benchmark::new("cli_benchmark").iterations(50).warmup_iterations(5).run(|| {
        let mut cmd = cli::create_configured_command(&config);
        let _output = cmd.arg("--file").arg(test_file.path()).output();
    });

    // Assert performance requirements
    results.assert_avg_time_less_than(Duration::from_millis(500));
    results.print_results();
    // Automatic cleanup on drop
}

// ============================================================================
// CLI CONFIGURATION TESTS
// ============================================================================

#[tokio::test]
async fn test_cli_with_different_configs() {
    let test_file = create_test_pdf("Config test content");

    // Test different configuration presets
    let configs =
        vec![("invalid_api", presets::invalid_api_key()), ("timeout", presets::with_timeout(5)), ("verbose", presets::verbose()), ("debug", presets::debug())];

    for (config_name, config) in configs {
        let mut cmd = cli::create_configured_command(&config);
        let output = cmd.arg("--file").arg(test_file.path()).output().expect("Failed to execute command");

        // All should fail but with different error patterns
        assert!(!output.status.success(), "Config '{}' should fail", config_name);
    }
    // Automatic cleanup on drop
}

#[tokio::test]
async fn test_cli_environment_variables() {
    // Test environment variable handling
    let mut test_env = env::TestEnv::new();
    test_env.set("PAPERLESS_OCR_API_KEY", "env-test-key");
    test_env.set("PAPERLESS_OCR_TIMEOUT", "45");

    let test_file = create_test_pdf("Env test content");
    let mut cmd = cli::create_test_command();

    cmd.arg("--file").arg(test_file.path()).assert().failure(); // Should fail due to invalid key, but env vars should be loaded

    // Environment variables are automatically restored on drop
    // Automatic cleanup on drop
}

// ============================================================================
// CLI STRESS TESTS
// ============================================================================

#[tokio::test]
async fn test_cli_stress_test() {
    let test_file = create_test_pdf("Stress test content");
    let config = presets::invalid_api_key();

    // Run stress test for CLI
    let results = stress::stress_test("cli_stress", 50, || {
        let mut cmd = cli::create_configured_command(&config);
        let _output = cmd.arg("--file").arg(test_file.path()).output();
    });

    // Assert stress test results
    results.assert_error_rate_less_than(0.1); // < 10% error rate
    results.print_results();
    // Automatic cleanup on drop
}
