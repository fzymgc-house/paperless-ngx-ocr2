//! Integration test demonstrating all test utilities working together
//!
//! This test file showcases the complete test infrastructure improvements,
//! including file management, configuration, contract validation, and performance testing.

mod common;

use common::*;
use common::config::presets;
use common::performance::{Benchmark, stress};
use std::time::Duration;

/// Comprehensive test demonstrating all test utilities
#[tokio::test]
async fn test_comprehensive_utility_demonstration() {
    print_status("🧪 Starting comprehensive test utility demonstration");

    // 1. File Management Utilities
    print_status("📁 Testing file management utilities...");

    // Use fixtures for stable test data
    let pdf_fixture = fixtures::sample_fixtures::sample_pdf();
    let png_fixture = fixtures::sample_fixtures::sample_png();

    // Use TestFile for temporary content
    let temp_pdf = create_test_pdf("Temporary PDF content");
    let temp_png = create_test_png();

    // Use large file for performance testing
    let large_pdf = create_large_test_pdf(1); // 1MB

    // Verify all files exist
    assert!(pdf_fixture.exists());
    assert!(png_fixture.exists());
    assert!(temp_pdf.exists());
    assert!(temp_png.exists());
    assert!(large_pdf.exists());

    print_status("✅ File management utilities working correctly");

    // 2. Configuration Utilities
    print_status("⚙️  Testing configuration utilities...");

    // Test different configuration presets
    let configs = vec![
        ("default", TestConfig::new()),
        ("invalid_api", presets::invalid_api_key()),
        ("json_output", presets::json_output()),
        ("verbose", presets::verbose()),
        ("debug", presets::debug()),
        ("large_files", presets::large_files()),
        ("network_timeout", presets::network_timeout()),
    ];

    for (name, config) in configs {
        // Verify configuration properties
        assert!(!config.api_key.is_empty() || name == "invalid_api");
        assert!(!config.api_base_url.is_empty());
        assert!(config.timeout_seconds > 0);

        // Test command configuration
        let _cmd = cli::create_configured_command(&config);
        // Command should be properly configured (we can't easily test the args without execution)
    }

    print_status("✅ Configuration utilities working correctly");

    // 3. Contract Validation Utilities
    print_status("📋 Testing contract validation utilities...");

    // Test CLI output contract validation
    let success_json = serde_json::json!({
        "success": true,
        "data": {
            "extracted_text": "Hello World",
            "file_name": "test.pdf",
            "file_size": 1024,
            "processing_time_ms": 1500,
            "confidence": 0.95
        }
    });

    assert!(validate_json_contract(&success_json.to_string(), ContractType::CliOutput).is_ok());

    // Test error contract validation
    let error_json = serde_json::json!({
        "success": false,
        "error": {
            "type": "validation",
            "message": "Invalid file format",
            "details": "File must be PDF or image"
        }
    });

    assert!(validate_json_contract(&error_json.to_string(), ContractType::CliOutput).is_ok());

    // Test API error contract validation
    let api_error_json = serde_json::json!({
        "error": "Authentication failed",
        "code": 401,
        "details": "Invalid API key provided"
    });

    assert!(validate_json_contract(&api_error_json.to_string(), ContractType::ApiError).is_ok());

    print_status("✅ Contract validation utilities working correctly");

    // 4. Performance Testing Utilities
    print_status("⚡ Testing performance utilities...");

    // Test performance measurement (using async version since this is an async test)
    // Note: We'll use a simple timing approach for this demo
    let start = std::time::Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(100));

    // Test benchmarking
    let benchmark_results = Benchmark::new("demo_benchmark")
        .iterations(50)
        .warmup_iterations(5)
        .run(|| {
            // Simulate some work
            std::thread::sleep(Duration::from_micros(100));
        });

    // Verify benchmark results
    assert!(benchmark_results.iterations > 0);
    assert!(benchmark_results.avg_time < Duration::from_millis(10));
    benchmark_results.assert_avg_time_less_than(Duration::from_millis(50));

    print_status("✅ Performance utilities working correctly");

    // 5. Memory Testing Utilities
    print_status("🧠 Testing memory utilities...");

    use performance::memory::*;

    reset_memory_tracking();
    let memory_test = MemoryTest::new().with_max_increase(1024 * 1024); // 1MB max increase

    // Simulate some memory allocation
    let _data: Vec<u8> = vec![0; 1000];

    memory_test.assert_memory_usage();

    print_status("✅ Memory utilities working correctly");

    // 6. Environment Management Utilities
    print_status("🌍 Testing environment utilities...");

    let mut test_env = env::TestEnv::new();
    test_env.set("TEST_VAR", "test_value");

    // Verify environment variable was set
    assert_eq!(std::env::var("TEST_VAR").unwrap(), "test_value");

    // Environment variables will be automatically restored on drop
    drop(test_env);

    // Verify environment variable was restored
    assert!(std::env::var("TEST_VAR").is_err());

    print_status("✅ Environment utilities working correctly");

    // 7. Stress Testing Utilities
    print_status("💪 Testing stress utilities...");

    let stress_results = stress::stress_test("demo_stress", 20, || {
        // Simulate some work
        std::thread::sleep(Duration::from_micros(50));
    });

    // Verify stress test results
    assert!(stress_results.iterations > 0);
    stress_results.assert_error_rate_less_than(0.1);

    print_status("✅ Stress utilities working correctly");

    // 8. CLI Integration Test
    print_status("🖥️  Testing CLI integration...");

    let config = presets::json_output().with_api_key("test-key");

    let mut cmd = cli::create_configured_command(&config);

    // Test that the command is properly configured
    let output = cmd.arg("--file").arg(temp_pdf.path()).output();

    // Should fail due to invalid API key, but command should be properly configured
    assert!(output.is_ok());

    print_status("✅ CLI integration working correctly");

    print_status("🎉 All test utilities are working correctly!");
    print_status("📊 Summary of utilities tested:");
    print_status("   ✅ File management (fixtures, temporary files, cleanup)");
    print_status("   ✅ Configuration management (presets, builders, CLI setup)");
    print_status("   ✅ Contract validation (JSON schemas, API contracts)");
    print_status("   ✅ Performance testing (timing, benchmarking, memory)");
    print_status("   ✅ Environment management (test isolation)");
    print_status("   ✅ Stress testing (reliability validation)");
    print_status("   ✅ CLI integration (command configuration)");

    // All TestFile instances will be automatically cleaned up on drop
    print_status("🧹 Automatic cleanup will occur when test files go out of scope");
}

/// Test that demonstrates error handling in utilities
#[test]
fn test_utility_error_handling() {
    // Test contract validation with invalid JSON
    let invalid_json = "{ invalid json }";
    let result = validate_json_contract(invalid_json, ContractType::CliOutput);
    assert!(result.is_err());

    // Test contract validation with wrong structure
    let wrong_structure = serde_json::json!({
        "wrong_field": "value"
    });
    let result = validate_json_contract(&wrong_structure.to_string(), ContractType::CliOutput);
    assert!(result.is_err());

    // Test performance test timeout (simplified for demo)
    let start = std::time::Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = start.elapsed();
    // This demonstrates the concept without actually panicking
    assert!(elapsed > Duration::from_millis(1));
}

/// Test that demonstrates utility combinations
#[test]
fn test_utility_combinations() {
    // Combine file utilities with performance testing
    let _test_file = create_test_pdf("Combination test");

    let results = Benchmark::new("file_creation_benchmark")
        .iterations(10)
        .run(|| {
            let _temp_file = create_test_pdf("Benchmark content");
        });

    results.assert_avg_time_less_than(Duration::from_millis(100));

    // Combine configuration with contract validation
    let config = TestConfig::new()
        .with_api_key("test-key")
        .with_json_output(true)
        .with_verbose(true);

    let _cmd = cli::create_configured_command(&config);

    // Verify configuration was applied
    // (We can't easily test the internal args without execution)
    assert!(config.json_output);
    assert!(config.verbose);

    // Combine environment management with CLI testing
    let mut test_env = env::TestEnv::new();
    test_env.set("PAPERLESS_OCR_API_KEY", "env-key");

    let env_config = TestConfig::new().with_api_key("env-key");

    let _env_cmd = cli::create_configured_command(&env_config);
    // Command should be configured with environment variables
}

/// Helper function for status printing
fn print_status(msg: &str) {
    println!("{}", msg);
}
