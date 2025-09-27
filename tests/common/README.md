# Test Utilities Documentation

This directory contains reusable test utilities and helpers for the `paperless-ngx-ocr2` project. These utilities provide consistent patterns for test file management, configuration, contract validation, and performance testing.

## Modules

### `fixtures.rs` - Test File Management

Provides utilities for creating and managing test files with automatic cleanup.

#### Key Types

- `TestFile`: A test file that automatically cleans up on drop
- `fixtures::*`: Access to predefined test fixtures

#### Usage Examples

```rust
use tests::common::fixtures::*;

#[test]
fn test_with_fixture() {
    let fixture = fixtures::sample_pdf();
    // Use fixture.path() for testing
    // Automatic cleanup on drop
}

#[test]
fn test_with_temporary_file() {
    let test_file = create_test_pdf("Test content");
    // Use test_file.path() for testing
    // Automatic cleanup on drop
}

#[test]
fn test_with_large_file() {
    let large_file = create_large_test_pdf(10); // 10MB
    // Use for performance testing
}
```

#### Available Fixtures

- `fixtures::sample_pdf()` - Valid PDF (274 bytes)
- `fixtures::sample_png()` - Valid PNG (1x1 pixel)
- `fixtures::corrupted_pdf()` - Invalid PDF content
- `fixtures::invalid_txt()` - Text file with wrong extension
- `fixtures::test_itinerary_pdf()` - Larger PDF for integration tests

### `config.rs` - Test Configuration

Provides utilities for managing test configurations and CLI command setup.

#### Key Types

- `TestConfig`: Configuration for API and CLI settings
- `TestConfigBuilder`: Builder pattern for complex configurations
- `presets::*`: Predefined configurations for common scenarios

#### Usage Examples

```rust
use tests::common::config::*;

#[test]
fn test_with_default_config() {
    let config = TestConfig::new();
    let mut cmd = create_configured_command(&config);
    // Command is configured with test settings
}

#[test]
fn test_with_custom_config() {
    let config = TestConfigBuilder::new()
        .api_key("custom-key")
        .timeout(60)
        .json_output(true)
        .build();
    
    let mut cmd = create_configured_command(&config);
}

#[test]
fn test_with_preset() {
    let config = presets::invalid_api_key();
    // Pre-configured for testing invalid API scenarios
}
```

#### Available Presets

- `presets::invalid_api_key()` - Invalid API key for error testing
- `presets::with_timeout(secs)` - Custom timeout configuration
- `presets::json_output()` - JSON output enabled
- `presets::verbose()` - Verbose logging enabled
- `presets::debug()` - Debug logging enabled
- `presets::large_files()` - Configuration for large file testing
- `presets::network_timeout()` - Network timeout testing
- `presets::invalid_endpoint()` - Invalid endpoint testing
- `presets::localhost(port)` - Localhost endpoint testing

### `contracts.rs` - Contract Validation

Provides utilities for validating API contracts and CLI output against expected schemas.

#### Key Types

- `ContractType`: Enum of available contract types
- Various validation functions for different contract types

#### Usage Examples

```rust
use tests::common::contracts::*;

#[test]
fn test_cli_output_validation() {
    let json_str = r#"{"success": true, "data": {...}}"#;
    let result = validate_json_contract(json_str, ContractType::CliOutput);
    assert!(result.is_ok());
}

#[test]
fn test_api_error_validation() {
    let json_str = r#"{"error": "Authentication failed", "code": 401}"#;
    let result = validate_json_contract(json_str, ContractType::ApiError);
    assert!(result.is_ok());
}
```

#### Available Contract Types

- `ContractType::CliOutput` - CLI JSON output format
- `ContractType::ApiError` - API error responses
- `ContractType::FileUploadRequest` - File upload requests
- `ContractType::FileUploadResponse` - File upload responses
- `ContractType::OcrRequest` - OCR API requests
- `ContractType::OcrResponse` - OCR API responses

### `performance.rs` - Performance Testing

Provides utilities for measuring and validating performance in tests.

#### Key Types

- `PerformanceTest`: Time-based performance validation
- `Benchmark`: Statistical performance benchmarking
- `memory::*`: Memory usage monitoring
- `stress::*`: Stress testing utilities

#### Usage Examples

```rust
use tests::common::performance::*;

#[test]
fn test_performance_validation() {
    measure_performance("file_validation", Duration::from_millis(100), || {
        // Code to measure
        validate_file("test.pdf");
    });
}

#[test]
fn test_benchmark() {
    let results = Benchmark::new("file_processing")
        .iterations(100)
        .run(|| {
            process_file("test.pdf");
        });
    
    results.assert_avg_time_less_than(Duration::from_millis(50));
}

#[test]
fn test_memory_usage() {
    use performance::memory::*;
    
    let test = MemoryTest::new()
        .with_max_increase(1024 * 1024); // 1MB max increase
    
    // Perform operations
    process_large_file();
    
    test.assert_memory_usage();
}

#[test]
fn test_stress_test() {
    let results = stress::stress_test("api_calls", 1000, || {
        make_api_call();
    });
    
    results.assert_error_rate_less_than(0.01); // < 1% error rate
}
```

## Best Practices

### File Management

1. **Use fixtures for stable test data**: Use predefined fixtures for consistent, small test files
2. **Use temporary files for dynamic content**: Create temporary files for test-specific content
3. **Prefer `TestFile` over manual cleanup**: Always use `TestFile` for automatic cleanup
4. **Use `TempDir` for directory-based tests**: For tests that need multiple files

### Configuration

1. **Use presets when possible**: Leverage predefined configurations for common scenarios
2. **Use builder pattern for complex configs**: Use `TestConfigBuilder` for multi-parameter configurations
3. **Apply configs consistently**: Use `apply_to_command()` for consistent CLI setup

### Contract Validation

1. **Validate all JSON outputs**: Always validate JSON responses against contracts
2. **Use appropriate contract types**: Choose the correct `ContractType` for validation
3. **Test both success and error cases**: Validate both positive and negative scenarios

### Performance Testing

1. **Set realistic thresholds**: Use appropriate time limits based on expected performance
2. **Use benchmarks for statistical validation**: Use `Benchmark` for multiple iterations
3. **Monitor memory usage**: Use memory tracking for memory-intensive tests
4. **Use stress tests for reliability**: Use stress testing for critical paths

## Migration Guide

### From Manual Cleanup to TestFile

**Before:**
```rust
#[test]
fn test_manual_cleanup() {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"content").unwrap();
    let path = temp_file.path().with_extension("pdf");
    fs::copy(temp_file.path(), &path).unwrap();
    
    // Test logic...
    
    fs::remove_file(&path).ok(); // Manual cleanup
}
```

**After:**
```rust
#[test]
fn test_auto_cleanup() {
    let test_file = create_test_pdf("content");
    // Test logic using test_file.path()
    // Automatic cleanup on drop
}
```

### From Manual Config to TestConfig

**Before:**
```rust
#[test]
fn test_manual_config() {
    let mut cmd = Command::cargo_bin("paperless-ngx-ocr2").unwrap();
    cmd.arg("--api-key").arg("test-key")
       .arg("--api-base-url").arg("https://api.test.com")
       .env("PAPERLESS_OCR_TIMEOUT", "30");
}
```

**After:**
```rust
#[test]
fn test_config_helper() {
    let config = TestConfig::new()
        .with_api_key("test-key")
        .with_api_base_url("https://api.test.com")
        .with_timeout(30);
    
    let mut cmd = create_configured_command(&config);
}
```

## Integration

To use these utilities in your tests, add this to your test file:

```rust
mod common;

use common::*;

#[test]
fn my_test() {
    let test_file = create_test_pdf("Test content");
    let config = presets::json_output();
    // ... rest of test
}
```

Or import specific modules:

```rust
use tests::common::fixtures::*;
use tests::common::config::*;
use tests::common::contracts::*;
use tests::common::performance::*;
```
