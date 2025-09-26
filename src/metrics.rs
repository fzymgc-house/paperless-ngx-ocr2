//! Metrics collection for monitoring API usage and performance

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Performance metrics for API operations
#[derive(Debug, Clone)]
pub struct APIMetrics {
    /// Number of successful API calls
    pub successful_calls: u64,
    /// Number of failed API calls
    pub failed_calls: u64,
    /// Total time spent on API calls
    pub total_duration: Duration,
    /// Average response time
    pub average_response_time: Duration,
    /// Total bytes uploaded
    pub total_bytes_uploaded: u64,
    /// Total bytes downloaded
    pub total_bytes_downloaded: u64,
    /// Number of retries performed
    pub total_retries: u64,
    /// Rate limit hits
    pub rate_limit_hits: u64,
}

impl Default for APIMetrics {
    fn default() -> Self {
        Self {
            successful_calls: 0,
            failed_calls: 0,
            total_duration: Duration::ZERO,
            average_response_time: Duration::ZERO,
            total_bytes_uploaded: 0,
            total_bytes_downloaded: 0,
            total_retries: 0,
            rate_limit_hits: 0,
        }
    }
}

impl APIMetrics {
    /// Record a successful API call
    pub fn record_success(&mut self, duration: Duration, bytes_uploaded: u64, bytes_downloaded: u64) {
        self.successful_calls += 1;
        self.total_duration += duration;
        self.total_bytes_uploaded += bytes_uploaded;
        self.total_bytes_downloaded += bytes_downloaded;
        self.update_average_response_time();
    }

    /// Record a failed API call
    pub fn record_failure(&mut self, duration: Duration) {
        self.failed_calls += 1;
        self.total_duration += duration;
        self.update_average_response_time();
    }

    /// Record a retry
    pub fn record_retry(&mut self) {
        self.total_retries += 1;
    }

    /// Record a rate limit hit
    pub fn record_rate_limit_hit(&mut self) {
        self.rate_limit_hits += 1;
    }

    /// Update average response time
    fn update_average_response_time(&mut self) {
        let total_calls = self.successful_calls + self.failed_calls;
        if total_calls > 0 {
            self.average_response_time = Duration::from_millis(
                self.total_duration.as_millis() as u64 / total_calls
            );
        }
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        let total_calls = self.successful_calls + self.failed_calls;
        if total_calls == 0 {
            0.0
        } else {
            (self.successful_calls as f64 / total_calls as f64) * 100.0
        }
    }

    /// Get total API calls
    pub fn total_calls(&self) -> u64 {
        self.successful_calls + self.failed_calls
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Global metrics collector
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<APIMetrics>>,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(APIMetrics::default())),
        }
    }

    /// Record a successful API call
    pub async fn record_success(&self, duration: Duration, bytes_uploaded: u64, bytes_downloaded: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.record_success(duration, bytes_uploaded, bytes_downloaded);
    }

    /// Record a failed API call
    pub async fn record_failure(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.record_failure(duration);
    }

    /// Record a retry
    pub async fn record_retry(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.record_retry();
    }

    /// Record a rate limit hit
    pub async fn record_rate_limit_hit(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.record_rate_limit_hit();
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> APIMetrics {
        self.metrics.read().await.clone()
    }

    /// Reset all metrics
    pub async fn reset(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.reset();
    }

    /// Get metrics summary as JSON
    pub async fn get_metrics_json(&self) -> serde_json::Value {
        let metrics = self.get_metrics().await;
        serde_json::json!({
            "successful_calls": metrics.successful_calls,
            "failed_calls": metrics.failed_calls,
            "total_calls": metrics.total_calls(),
            "success_rate_percent": metrics.success_rate(),
            "average_response_time_ms": metrics.average_response_time.as_millis(),
            "total_duration_ms": metrics.total_duration.as_millis(),
            "total_bytes_uploaded": metrics.total_bytes_uploaded,
            "total_bytes_downloaded": metrics.total_bytes_downloaded,
            "total_retries": metrics.total_retries,
            "rate_limit_hits": metrics.rate_limit_hits
        })
    }

    /// Log metrics summary
    pub async fn log_summary(&self) {
        let metrics = self.get_metrics().await;
        if metrics.total_calls() > 0 {
            tracing::info!(
                "API Metrics Summary: {} calls, {:.1}% success rate, avg response: {}ms, {} bytes uploaded, {} bytes downloaded, {} retries, {} rate limit hits",
                metrics.total_calls(),
                metrics.success_rate(),
                metrics.average_response_time.as_millis(),
                metrics.total_bytes_uploaded,
                metrics.total_bytes_downloaded,
                metrics.total_retries,
                metrics.rate_limit_hits
            );
        }
    }
}

/// Global metrics collector instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_METRICS: MetricsCollector = MetricsCollector::new();
}

/// Helper macro to measure API call duration
#[macro_export]
macro_rules! measure_api_call {
    ($metrics:expr, $bytes_uploaded:expr, $bytes_downloaded:expr, $operation:block) => {{
        let start = std::time::Instant::now();
        let result = $operation;
        let duration = start.elapsed();
        
        match result {
            Ok(_) => {
                $metrics.record_success(duration, $bytes_uploaded, $bytes_downloaded).await;
            }
            Err(_) => {
                $metrics.record_failure(duration).await;
            }
        }
        
        result
    }};
}

/// Performance monitoring for file operations
#[derive(Debug, Clone)]
pub struct FileMetrics {
    /// Number of files processed
    pub files_processed: u64,
    /// Total file size processed
    pub total_file_size: u64,
    /// Average file size
    pub average_file_size: u64,
    /// Total processing time
    pub total_processing_time: Duration,
    /// Average processing time per file
    pub average_processing_time: Duration,
}

impl Default for FileMetrics {
    fn default() -> Self {
        Self {
            files_processed: 0,
            total_file_size: 0,
            average_file_size: 0,
            total_processing_time: Duration::ZERO,
            average_processing_time: Duration::ZERO,
        }
    }
}

impl FileMetrics {
    /// Record file processing
    pub fn record_file_processed(&mut self, file_size: u64, processing_time: Duration) {
        self.files_processed += 1;
        self.total_file_size += file_size;
        self.total_processing_time += processing_time;
        
        if self.files_processed > 0 {
            self.average_file_size = self.total_file_size / self.files_processed;
            self.average_processing_time = Duration::from_millis(
                self.total_processing_time.as_millis() as u64 / self.files_processed
            );
        }
    }

    /// Get processing throughput (bytes per second)
    pub fn throughput_bytes_per_second(&self) -> f64 {
        if self.total_processing_time.as_secs() > 0 {
            self.total_file_size as f64 / self.total_processing_time.as_secs() as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_metrics_collection() {
        let metrics = MetricsCollector::new();
        
        // Record some test data
        metrics.record_success(Duration::from_millis(100), 1024, 512).await;
        metrics.record_success(Duration::from_millis(200), 2048, 1024).await;
        metrics.record_failure(Duration::from_millis(150)).await;
        metrics.record_retry().await;
        metrics.record_rate_limit_hit().await;
        
        let collected_metrics = metrics.get_metrics().await;
        
        assert_eq!(collected_metrics.successful_calls, 2);
        assert_eq!(collected_metrics.failed_calls, 1);
        assert_eq!(collected_metrics.total_retries, 1);
        assert_eq!(collected_metrics.rate_limit_hits, 1);
        assert_eq!(collected_metrics.total_bytes_uploaded, 3072);
        assert_eq!(collected_metrics.total_bytes_downloaded, 1536);
        
        // Test success rate calculation
        assert!((collected_metrics.success_rate() - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_file_metrics() {
        let mut file_metrics = FileMetrics::default();
        
        file_metrics.record_file_processed(1024, Duration::from_millis(100));
        file_metrics.record_file_processed(2048, Duration::from_millis(200));
        
        assert_eq!(file_metrics.files_processed, 2);
        assert_eq!(file_metrics.total_file_size, 3072);
        assert_eq!(file_metrics.average_file_size, 1536);
        
        let throughput = file_metrics.throughput_bytes_per_second();
        assert!(throughput >= 0.0);
    }

    #[tokio::test]
    async fn test_metrics_json_output() {
        let metrics = MetricsCollector::new();
        metrics.record_success(Duration::from_millis(100), 1024, 512).await;
        
        let json = metrics.get_metrics_json().await;
        
        assert!(json.get("successful_calls").is_some());
        assert!(json.get("success_rate_percent").is_some());
        assert!(json.get("average_response_time_ms").is_some());
    }
}
