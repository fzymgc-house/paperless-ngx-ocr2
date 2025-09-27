//! Common test utilities and helpers
//!
//! This module provides reusable utilities for testing across different test types.
//! It includes helpers for temporary file management, test configuration,
//! contract validation, and performance testing.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod config;
pub mod contracts;
pub mod fixtures;
pub mod performance;

// Re-export commonly used utilities
pub use config::{presets, TestConfig};
pub use contracts::{validate_json_contract, ContractType};
pub use fixtures::{create_corrupted_pdf, create_invalid_file, create_large_test_pdf, create_test_pdf, create_test_png, TestFile};
pub use performance::{measure_performance, measure_performance_async, stress, Benchmark};

/// Test utilities for temporary file management
pub mod temp_files {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::{NamedTempFile, TempDir};

    /// Creates a temporary PDF file with the given content
    /// Automatically cleans up on drop
    pub fn create_temp_pdf(content: &[u8]) -> TestFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content).unwrap();
        let path = temp_file.path().with_extension("pdf");
        fs::copy(temp_file.path(), &path).unwrap();
        TestFile::new(path)
    }

    /// Creates a temporary PNG file with minimal valid PNG content
    pub fn create_temp_png() -> TestFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Minimal PNG header
        temp_file.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]).unwrap();
        let path = temp_file.path().with_extension("png");
        fs::copy(temp_file.path(), &path).unwrap();
        TestFile::new(path)
    }

    /// Creates a temporary directory for test files
    /// Automatically cleans up on drop
    pub fn create_temp_dir() -> TempDir {
        TempDir::new().unwrap()
    }
}

/// Test utilities for CLI command execution
pub mod cli {
    use super::TestConfig;
    use assert_cmd::Command;

    /// Creates a configured CLI command with default test settings
    pub fn create_test_command() -> Command {
        Command::cargo_bin("paperless-ngx-ocr2").unwrap()
    }

    /// Creates a CLI command with test configuration applied
    pub fn create_configured_command(config: &TestConfig) -> Command {
        let mut cmd = create_test_command();
        config.apply_to_command(&mut cmd);
        cmd
    }
}

/// Test utilities for environment management
pub mod env {
    use std::collections::HashMap;
    use std::env;

    /// Manages environment variables during tests
    /// Automatically restores original values on drop
    pub struct TestEnv {
        original_values: HashMap<String, Option<String>>,
    }

    impl TestEnv {
        /// Creates a new test environment manager
        pub fn new() -> Self {
            Self { original_values: HashMap::new() }
        }

        /// Sets an environment variable, remembering the original value
        pub fn set(&mut self, key: &str, value: &str) {
            let original = env::var(key).ok();
            self.original_values.insert(key.to_string(), original);
            env::set_var(key, value);
        }

        /// Removes an environment variable, remembering if it existed
        pub fn remove(&mut self, key: &str) {
            let original = env::var(key).ok();
            self.original_values.insert(key.to_string(), original);
            env::remove_var(key);
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            for (key, original_value) in self.original_values.iter() {
                match original_value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }
}
