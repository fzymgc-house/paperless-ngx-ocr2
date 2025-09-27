//! Mistral AI API client modules
//!
//! This module contains the API client implementation for Mistral AI services.
//! Documentation: https://docs.mistral.ai/api/
//!
//! The client supports:
//! - Files API for uploading documents
//! - OCR API for text extraction
//! - Authentication with Bearer tokens
//! - Retry logic and error handling

use crate::credentials::APICredentials;
use crate::error::{Error, Result};
use reqwest::{Client, Response};
use std::time::Duration;
use tokio::time::sleep;

pub mod auth;
pub mod error;
pub mod files;
pub mod ocr;

/// Base API client for Mistral AI
#[derive(Debug, Clone)]
pub struct MistralClient {
    client: Client,
    pub credentials: APICredentials,
    base_url: String,
}

impl MistralClient {
    /// Create a new Mistral AI API client with compression support
    pub fn new(credentials: APICredentials, timeout_seconds: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .user_agent(format!("paperless-ngx-ocr2/{}", env!("CARGO_PKG_VERSION")))
            .gzip(true) // Enable gzip compression
            .brotli(true) // Enable brotli compression
            .deflate(true) // Enable deflate compression
            .build()
            .map_err(|e| Error::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, base_url: credentials.api_base_url.clone(), credentials })
    }

    /// Get the HTTP client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the authorization header
    pub fn auth_header(&self) -> String {
        self.credentials.get_auth_header()
    }

    /// Build a full URL for an endpoint
    pub fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'))
    }

    /// Handle API response and convert errors
    pub async fn handle_response(response: Response) -> Result<Response> {
        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else {
            let status_code = status.as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as JSON error response
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let Some(error_msg) = error_json.get("error").and_then(|e| e.as_str()) {
                    Err(Error::from_http_status(status_code, error_msg.to_string()))
                } else {
                    Err(Error::from_http_status(status_code, error_text))
                }
            } else {
                Err(Error::from_http_status(status_code, error_text))
            }
        }
    }

    /// Log API request details (for debugging)
    pub fn log_request(&self, method: &str, url: &str) {
        tracing::debug!("API Request: {} {} (auth: {})", method, url, self.credentials.redacted_key());
    }

    /// Log API response details (for debugging)
    pub fn log_response(&self, status: u16, response_size: Option<usize>) {
        if let Some(size) = response_size {
            tracing::debug!("API Response: {} ({} bytes)", status, size);
        } else {
            tracing::debug!("API Response: {}", status);
        }
    }

    /// Log API response details with compression info
    pub fn log_response_with_compression(&self, status: u16, response_size: Option<usize>, content_encoding: Option<&str>) {
        if let Some(size) = response_size {
            if let Some(encoding) = content_encoding {
                tracing::debug!("API Response: {} ({} bytes, compressed with {})", status, size, encoding);
            } else {
                tracing::debug!("API Response: {} ({} bytes, uncompressed)", status, size);
            }
        } else {
            tracing::debug!("API Response: {}", status);
        }
    }

    /// Execute request with retry logic for rate limits
    pub async fn execute_with_retry<F, Fut>(&self, request_fn: F) -> Result<Response>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<Response>>,
    {
        const MAX_RETRIES: u32 = 3;
        const BASE_DELAY_MS: u64 = 1000; // 1 second base delay

        for attempt in 0..=MAX_RETRIES {
            match request_fn().await {
                Ok(response) => {
                    let status = response.status();

                    // Check if it's a rate limit error (HTTP 429)
                    if status == 429 {
                        if attempt < MAX_RETRIES {
                            let delay_ms = BASE_DELAY_MS * 2_u64.pow(attempt); // Exponential backoff
                            tracing::warn!("Rate limit hit (HTTP 429), retrying in {}ms (attempt {}/{})", delay_ms, attempt + 1, MAX_RETRIES);
                            sleep(Duration::from_millis(delay_ms)).await;
                            continue;
                        } else {
                            return Err(Error::from_http_status(429, "Rate limit exceeded after 3 retries".to_string()));
                        }
                    }

                    return Ok(response);
                }
                Err(e) => {
                    // Check if it's a rate limit error by checking the error message
                    if let Error::Api(ref api_error) = e {
                        if (api_error.contains("429") || api_error.contains("rate limit")) && attempt < MAX_RETRIES {
                            let delay_ms = BASE_DELAY_MS * 2_u64.pow(attempt);
                            tracing::warn!("Rate limit hit (HTTP 429), retrying in {}ms (attempt {}/{})", delay_ms, attempt + 1, MAX_RETRIES);
                            sleep(Duration::from_millis(delay_ms)).await;
                            continue;
                        }
                    }

                    return Err(e);
                }
            }
        }

        unreachable!()
    }
}
