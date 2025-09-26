//! # Paperless NGX OCR2
//!
//! A CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction.
//! Supports TOML configuration, 12-factor app principles, and provides both
//! human-readable and JSON output formats.

pub mod api;
pub mod cache;
pub mod cli;
pub mod config;
pub mod credentials;
pub mod error;
pub mod file;
pub mod metrics;
pub mod ocr;

pub use cache::{generate_file_hash, CacheManager, FileCacheKey, OCRCacheKey, GLOBAL_CACHE};
pub use config::{Config, RetryPolicy};
pub use credentials::APICredentials;
pub use error::{Error, Result};
pub use file::FileUpload;
pub use metrics::{APIMetrics, FileMetrics, MetricsCollector, GLOBAL_METRICS};
pub use ocr::OCRResult;

/// Initialize the application with proper logging configuration
pub fn init_app() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "paperless_ngx_ocr2=info".into()),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_writer(std::io::stderr)
        .init();

    tracing::debug!("Application initialized");
    Ok(())
}

/// Initialize logging configuration
pub fn init_logging(verbose: bool) -> Result<()> {
    let log_level = if verbose {
        "paperless_ngx_ocr2=debug"
    } else {
        "paperless_ngx_ocr2=info"
    };

    let log_format = std::env::var("RUST_LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| log_level.into()),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_writer(std::io::stderr);

    if log_format == "json" {
        subscriber.json().init();
    } else {
        subscriber.init();
    }

    tracing::debug!("Logging initialized with level: {}", log_level);
    Ok(())
}
