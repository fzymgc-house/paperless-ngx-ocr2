//! Mock API service for testing
//!
//! This module provides a simple mock HTTP server that can be used during testing
//! to avoid making external API calls and speed up test execution.

use std::io::{Read, Write};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;

/// Mock API server for testing
pub struct MockApiServer {
    /// Server handle
    server_handle: Option<tokio::task::JoinHandle<()>>,
    /// Server port
    port: AtomicU16,
    /// Request counter
    request_count: Arc<AtomicU16>,
    /// Response delay in milliseconds
    delay_ms: RwLock<u64>,
    /// Response status code
    status_code: RwLock<u16>,
    /// Response body
    response_body: RwLock<String>,
    /// Whether to simulate network errors
    simulate_error: RwLock<bool>,
}

impl MockApiServer {
    /// Create a new mock API server
    pub fn new() -> Self {
        Self {
            server_handle: None,
            port: AtomicU16::new(0),
            request_count: Arc::new(AtomicU16::new(0)),
            delay_ms: RwLock::new(0),
            status_code: RwLock::new(200),
            response_body: RwLock::new(
                r#"{"id": "file-mock123", "object": "file", "bytes": 1024, "created_at": 1640995200, "filename": "test.pdf", "purpose": "ocr"}"#.to_string(),
            ),
            simulate_error: RwLock::new(false),
        }
    }

    /// Start the mock server
    pub async fn start(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let port = self.find_available_port().await?;

        let delay_ms = Arc::new(*self.delay_ms.read().await);
        let status_code = Arc::new(*self.status_code.read().await);
        let response_body = Arc::new(self.response_body.read().await.clone());
        let simulate_error = Arc::new(*self.simulate_error.read().await);
        let request_count = Arc::clone(&self.request_count);

        // Create a simple HTTP server using tokio
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server_handle = tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
            println!("Mock server listening on {}", addr);

            loop {
                let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");

                let delay_ms = delay_ms.clone();
                let status_code = status_code.clone();
                let response_body = response_body.clone();
                let simulate_error = simulate_error.clone();
                let request_count = request_count.clone();

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    // Read request (simplified - just read until we see the end)
                    let n = match socket.read(&mut buffer).await {
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(_) => return,
                    };

                    let request_str = String::from_utf8_lossy(&buffer[..n]);

                    // Simple request parsing - check if it's a files or ocr request
                    let _is_files_request = request_str.contains("/v1/files");
                    let _is_ocr_request = request_str.contains("/v1/chat/completions");

                    // Simulate delay
                    if *delay_ms > 0 {
                        tokio::time::sleep(Duration::from_millis(*delay_ms)).await;
                    }

                    // Increment request counter
                    request_count.fetch_add(1, Ordering::Relaxed);

                    // Check if we should simulate an error
                    if *simulate_error {
                        let response = b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
                        socket.write_all(response).await.ok();
                        return;
                    }

                    let status = *status_code;
                    let body = (*response_body).clone();

                    let response = format!("HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", status, body.len(), body);

                    socket.write_all(response.as_bytes()).await.ok();
                });
            }
        });

        self.server_handle = Some(server_handle);
        self.port.store(port, Ordering::Relaxed);

        // Wait a bit for server to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(format!("http://127.0.0.1:{}", port))
    }

    /// Stop the mock server
    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
        Ok(())
    }

    /// Get the server port
    pub fn port(&self) -> u16 {
        self.port.load(Ordering::Relaxed)
    }

    /// Get the request count
    pub fn request_count(&self) -> u16 {
        self.request_count.load(Ordering::Relaxed)
    }

    /// Set response delay in milliseconds
    pub async fn set_delay(&self, delay_ms: u64) {
        *self.delay_ms.write().await = delay_ms;
    }

    /// Set response status code
    pub async fn set_status_code(&self, status_code: u16) {
        *self.status_code.write().await = status_code;
    }

    /// Set response body
    pub async fn set_response_body(&self, body: String) {
        *self.response_body.write().await = body;
    }

    /// Set whether to simulate errors
    pub async fn set_simulate_error(&self, simulate_error: bool) {
        *self.simulate_error.write().await = simulate_error;
    }

    /// Find an available port
    async fn find_available_port(&self) -> Result<u16, Box<dyn std::error::Error>> {
        // Use port 0 to let the OS assign an available port
        match tokio::net::TcpListener::bind("127.0.0.1:0").await {
            Ok(listener) => {
                let addr = listener.local_addr()?;
                Ok(addr.port())
            }
            Err(_) => Err("Failed to find available port".into()),
        }
    }

    /// Set up mock for timeout testing
    pub async fn setup_timeout_mock(&self, delay_ms: u64) {
        self.set_delay(delay_ms).await;
        self.set_status_code(200).await;
        self.set_simulate_error(false).await;
        self.set_response_body(
            r#"{"id": "file-mock123", "object": "file", "bytes": 1024, "created_at": 1640995200, "filename": "test.pdf", "purpose": "ocr"}"#.to_string(),
        )
        .await;
    }

    /// Set up mock for error testing
    pub async fn setup_error_mock(&self, status_code: u16) {
        self.set_delay(0).await;
        self.set_status_code(status_code).await;
        self.set_simulate_error(false).await;
        self.set_response_body(r#"{"error": {"type": "api", "message": "Mock API Error", "details": "This is a mock error for testing"}}"#.to_string()).await;
    }

    /// Set up mock for network error testing
    pub async fn setup_network_error_mock(&self) {
        self.set_delay(0).await;
        self.set_simulate_error(true).await;
    }

    /// Set up mock for OCR response
    pub async fn setup_ocr_mock(&self, content: &str) {
        self.set_delay(0).await;
        self.set_status_code(200).await;
        self.set_simulate_error(false).await;
        let response = format!(
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
        );
        self.set_response_body(response).await;
    }
}

impl Default for MockApiServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MockApiServer {
    fn drop(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_mock_server_start_stop() {
        let mut server = MockApiServer::new();
        let _url = server.start().await.expect("Failed to start server");

        // Wait a bit for server to be ready
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert!(server.port() > 0);
        assert_eq!(server.request_count(), 0);

        server.stop().await.expect("Failed to stop server");
    }

    #[tokio::test]
    async fn test_mock_server_timeout_response() {
        let mut server = MockApiServer::new();
        server.setup_timeout_mock(100).await; // 100ms delay

        let url = server.start().await.expect("Failed to start server");
        let client = reqwest::Client::new();

        // Test that request times out appropriately
        let start = std::time::Instant::now();
        let result = timeout(Duration::from_millis(50), client.get(&url).send()).await;
        let elapsed = start.elapsed();

        assert!(result.is_err() || elapsed.as_millis() < 50);
        server.stop().await.expect("Failed to stop server");
    }

    #[tokio::test]
    async fn test_mock_server_error_response() {
        let mut server = MockApiServer::new();
        server.setup_error_mock(500).await;

        let url = server.start().await.expect("Failed to start server");
        let client = reqwest::Client::new();

        let response = client.get(&url).send().await.expect("Failed to send request");
        assert_eq!(response.status().as_u16(), 500);

        server.stop().await.expect("Failed to stop server");
    }
}
