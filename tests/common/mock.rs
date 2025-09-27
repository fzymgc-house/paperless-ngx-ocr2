//! Mock API service for testing
//!
//! This module provides a mock HTTP server using the httpmock crate that can be used during testing
//! to avoid making external API calls and speed up test execution.

use httpmock::{Method, Mock, MockServer};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Mock API server for testing
pub struct MockApiServer {
    /// httpmock server instance
    server: MockServer,
    /// Request counter
    request_count: Arc<AtomicU16>,
}

impl MockApiServer {
    /// Create a new mock API server
    pub fn new() -> Self {
        Self { server: MockServer::start(), request_count: Arc::new(AtomicU16::new(0)) }
    }

    /// Start the mock server (already started by httpmock)
    pub async fn start(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // Reset request counter
        self.request_count.store(0, Ordering::Relaxed);

        // Convert HTTP URL to HTTPS for application compatibility
        let http_url = self.server.base_url();
        let https_url = http_url.replace("http://", "https://");
        Ok(https_url)
    }

    /// Stop the mock server
    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // httpmock handles cleanup automatically when dropped
        Ok(())
    }

    /// Get the server port
    pub fn port(&self) -> u16 {
        self.server.port()
    }

    /// Get the request count
    pub fn request_count(&self) -> u16 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Set up mock for timeout testing
    pub async fn setup_timeout_mock(&mut self, delay_ms: u64) {
        self.server.mock(|when, then| {
            when.method(Method::POST).path("/v1/files");
            then.delay(Duration::from_millis(delay_ms))
                .status(200)
                .body(r#"{"id": "file-mock123", "object": "file", "bytes": 1024, "created_at": 1640995200, "filename": "test.pdf", "purpose": "ocr"}"#);
        });

        self.request_count.store(0, Ordering::Relaxed);
    }

    /// Set up mock for error testing
    pub async fn setup_error_mock(&mut self, status_code: u16) {
        self.server.mock(|when, then| {
            when.method(Method::POST).path("/v1/files");
            then.status(status_code).body(r#"{"error": {"type": "api", "message": "Mock API Error", "details": "This is a mock error for testing"}}"#);
        });

        self.request_count.store(0, Ordering::Relaxed);
    }

    /// Set up mock for network error testing
    pub async fn setup_network_error_mock(&mut self) {
        // For network errors, we can set up a mock that returns connection errors
        // This is handled by httpmock automatically for invalid URLs
        self.request_count.store(0, Ordering::Relaxed);
    }

    /// Set up mock for OCR response
    pub async fn setup_ocr_mock(&mut self, content: &str) {
        self.server.mock(|when, then| {
            when.method(Method::POST).path("/v1/chat/completions");
            then.status(200).body(format!(
                r#"{{
                        "id": "chatcmpl-mock123",
                        "object": "chat.completion",
                        "created": 1640995200,
                        "model": "mistral-ocr-latest",
                        "choices": [{{
                            "index": 0,
                            "message": {{
                                "role": "assistant",
                                "content": "{}"
                            }},
                            "finish_reason": "stop"
                        }}],
                        "usage": {{
                            "prompt_tokens": 10,
                            "completion_tokens": 20,
                            "total_tokens": 30
                        }}
                    }}"#,
                content
            ));
        });

        self.request_count.store(0, Ordering::Relaxed);
    }
}

impl Default for MockApiServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MockApiServer {
    fn drop(&mut self) {
        // httpmock handles cleanup automatically
    }
}
