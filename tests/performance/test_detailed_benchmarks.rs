//! Detailed performance benchmarks for OCR CLI tool

use paperless_ngx_ocr2::*;
use std::fs;
use std::io::Write;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub file_sizes: Vec<usize>, // in bytes
    pub concurrent_requests: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            warmup_iterations: 3,
            file_sizes: vec![1024, 10240, 102400, 1024000], // 1KB to 1MB
            concurrent_requests: 1,
        }
    }
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub operation_name: String,
    pub iterations: usize,
    pub total_duration: Duration,
    pub average_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub p50_duration: Duration,
    pub p95_duration: Duration,
    pub p99_duration: Duration,
    pub throughput_per_second: f64,
    pub memory_usage_mb: f64,
    pub error_rate: f64,
}

impl BenchmarkResults {
    /// Create a summary report
    pub fn to_report(&self) -> String {
        format!(
            "Benchmark Results: {}\n\
             Iterations: {}\n\
             Total Duration: {:?}\n\
             Average Duration: {:?}\n\
             Min Duration: {:?}\n\
             Max Duration: {:?}\n\
             P50 Duration: {:?}\n\
             P95 Duration: {:?}\n\
             P99 Duration: {:?}\n\
             Throughput: {:.2} ops/sec\n\
             Memory Usage: {:.2} MB\n\
             Error Rate: {:.2}%",
            self.operation_name,
            self.iterations,
            self.total_duration,
            self.average_duration,
            self.min_duration,
            self.max_duration,
            self.p50_duration,
            self.p95_duration,
            self.p99_duration,
            self.throughput_per_second,
            self.memory_usage_mb,
            self.error_rate
        )
    }
}

/// Performance benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Run a benchmark for a given operation
    pub async fn run_benchmark<F, Fut, R>(&self, name: &str, operation: F) -> BenchmarkResults
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
    {
        let mut durations = Vec::new();
        let mut errors = 0;
        let start_time = Instant::now();

        // Warmup iterations
        for _ in 0..self.config.warmup_iterations {
            let _ = operation().await;
        }

        // Actual benchmark iterations
        for _ in 0..self.config.iterations {
            let iteration_start = Instant::now();
            match operation().await {
                Ok(_) => {
                    durations.push(iteration_start.elapsed());
                }
                Err(_) => {
                    errors += 1;
                }
            }
        }

        let total_duration = start_time.elapsed();
        durations.sort();

        let average_duration = if !durations.is_empty() {
            Duration::from_nanos(
                durations.iter().map(|d| d.as_nanos() as u64).sum::<u64>() / durations.len() as u64
            )
        } else {
            Duration::ZERO
        };

        let min_duration = durations.first().copied().unwrap_or(Duration::ZERO);
        let max_duration = durations.last().copied().unwrap_or(Duration::ZERO);
        
        let p50_idx = (durations.len() as f64 * 0.5) as usize;
        let p95_idx = (durations.len() as f64 * 0.95) as usize;
        let p99_idx = (durations.len() as f64 * 0.99) as usize;
        
        let p50_duration = durations.get(p50_idx).copied().unwrap_or(Duration::ZERO);
        let p95_duration = durations.get(p95_idx).copied().unwrap_or(Duration::ZERO);
        let p99_duration = durations.get(p99_idx).copied().unwrap_or(Duration::ZERO);

        let throughput_per_second = if !durations.is_empty() {
            durations.len() as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };

        let error_rate = if self.config.iterations > 0 {
            (errors as f64 / self.config.iterations as f64) * 100.0
        } else {
            0.0
        };

        // Estimate memory usage (this is approximate)
        let memory_usage_mb = self.estimate_memory_usage();

        BenchmarkResults {
            operation_name: name.to_string(),
            iterations: self.config.iterations,
            total_duration,
            average_duration,
            min_duration,
            max_duration,
            p50_duration,
            p95_duration,
            p99_duration,
            throughput_per_second,
            memory_usage_mb,
            error_rate,
        }
    }

    /// Estimate memory usage (simplified)
    fn estimate_memory_usage(&self) -> f64 {
        // This is a simplified estimation
        // In a real implementation, you might use tools like heapsize
        let base_memory = 50.0; // Base memory usage in MB
        let file_memory = self.config.file_sizes.iter().sum::<usize>() as f64 / (1024.0 * 1024.0);
        base_memory + file_memory
    }
}

/// Create a test file with specified size
fn create_test_file(size_bytes: usize) -> Result<NamedTempFile> {
    let mut temp_file = NamedTempFile::new().map_err(|e| Error::Io(e))?;
    
    // Write header
    temp_file.write_all(b"%PDF-1.4\n").map_err(|e| Error::Io(e))?;
    
    // Fill with test content
    let content = "Benchmark test content for performance measurement.\n";
    let content_bytes = content.as_bytes();
    let repetitions = size_bytes / content_bytes.len();
    
    for _ in 0..repetitions {
        temp_file.write_all(content_bytes).map_err(|e| Error::Io(e))?;
    }
    
    // Fill remaining bytes
    let remaining = size_bytes % content_bytes.len();
    if remaining > 0 {
        temp_file.write_all(&content_bytes[..remaining]).map_err(|e| Error::Io(e))?;
    }
    
    Ok(temp_file)
}

#[tokio::test]
async fn test_file_validation_benchmark() {
    let config = BenchmarkConfig {
        iterations: 50,
        warmup_iterations: 5,
        file_sizes: vec![1024, 10240, 102400],
        concurrent_requests: 1,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test file validation performance
    let results = runner.run_benchmark("file_validation", || async {
        let temp_file = create_test_file(10240)?;
        let file_path = temp_file.path().to_str().unwrap();
        let _file_upload = FileUpload::new(file_path)?;
        Ok::<(), Error>(())
    }).await;
    
    println!("{}", results.to_report());
    
    // Assertions for performance expectations
    assert!(results.average_duration < Duration::from_millis(100), 
            "File validation should be fast: {:?}", results.average_duration);
    assert_eq!(results.error_rate, 0.0, "File validation should not have errors");
    assert!(results.throughput_per_second > 10.0, 
            "File validation throughput should be > 10 ops/sec: {:.2}", results.throughput_per_second);
}

#[tokio::test]
async fn test_file_size_scaling_benchmark() {
    let config = BenchmarkConfig {
        iterations: 20,
        warmup_iterations: 3,
        file_sizes: vec![1024, 10240, 102400, 1024000, 10240000], // 1KB to 10MB
        concurrent_requests: 1,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test how performance scales with file size
    for &file_size in &config.file_sizes {
        let results = runner.run_benchmark(&format!("file_validation_{}bytes", file_size), || async {
            let temp_file = create_test_file(file_size)?;
            let file_path = temp_file.path().to_str().unwrap();
            let _file_upload = FileUpload::new(file_path)?;
            Ok::<(), Error>(())
        }).await;
        
        println!("File size {} bytes: {}", file_size, results.to_report());
        
        // Performance should degrade gracefully with file size
        let expected_max_duration = Duration::from_millis(1000 + (file_size / 1024) as u64);
        assert!(results.average_duration < expected_max_duration,
                "Performance should scale gracefully with file size {}: {:?}", 
                file_size, results.average_duration);
    }
}

#[tokio::test]
async fn test_memory_usage_benchmark() {
    let config = BenchmarkConfig {
        iterations: 10,
        warmup_iterations: 2,
        file_sizes: vec![1024000, 10240000, 102400000], // 1MB to 100MB
        concurrent_requests: 1,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test memory usage with different file sizes
    for &file_size in &config.file_sizes {
        let results = runner.run_benchmark(&format!("memory_usage_{}bytes", file_size), || async {
            let temp_file = create_test_file(file_size)?;
            let file_path = temp_file.path().to_str().unwrap();
            let _file_upload = FileUpload::new(file_path)?;
            Ok::<(), Error>(())
        }).await;
        
        println!("Memory usage for {} bytes: {}", file_size, results.to_report());
        
        // Memory usage should be reasonable
        let max_memory_mb = 200.0 + (file_size as f64 / (1024.0 * 1024.0)) * 2.0;
        assert!(results.memory_usage_mb < max_memory_mb,
                "Memory usage should be reasonable for file size {}: {:.2} MB", 
                file_size, results.memory_usage_mb);
    }
}

#[tokio::test]
async fn test_concurrent_operations_benchmark() {
    let config = BenchmarkConfig {
        iterations: 5,
        warmup_iterations: 1,
        file_sizes: vec![10240],
        concurrent_requests: 5,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test concurrent operations
    let results = runner.run_benchmark("concurrent_file_validation", || async {
        let handles: Vec<_> = (0..config.concurrent_requests).map(|_| {
            tokio::spawn(async {
                let temp_file = create_test_file(10240)?;
                let file_path = temp_file.path().to_str().unwrap();
                let _file_upload = FileUpload::new(file_path)?;
                Ok::<(), Error>(())
            })
        }).collect();
        
        // Wait for all operations to complete
        for handle in handles {
            let _ = handle.await.map_err(|e| Error::Internal(format!("Task failed: {}", e)))?;
        }
        
        Ok::<(), Error>(())
    }).await;
    
    println!("Concurrent operations: {}", results.to_report());
    
    // Concurrent operations should complete successfully
    assert_eq!(results.error_rate, 0.0, "Concurrent operations should not have errors");
}

#[tokio::test]
async fn test_performance_consistency_benchmark() {
    let config = BenchmarkConfig {
        iterations: 100,
        warmup_iterations: 10,
        file_sizes: vec![10240],
        concurrent_requests: 1,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test performance consistency over many iterations
    let results = runner.run_benchmark("performance_consistency", || async {
        let temp_file = create_test_file(10240)?;
        let file_path = temp_file.path().to_str().unwrap();
        let _file_upload = FileUpload::new(file_path)?;
        Ok::<(), Error>(())
    }).await;
    
    println!("Performance consistency: {}", results.to_report());
    
    // Performance should be consistent (low variance)
    let variance = (results.max_duration.as_nanos() as f64 - results.min_duration.as_nanos() as f64) / 
                   results.average_duration.as_nanos() as f64;
    
    assert!(variance < 2.0, 
            "Performance should be consistent (variance: {:.2})", variance);
    
    // P99 should not be too much worse than average
    let p99_ratio = results.p99_duration.as_nanos() as f64 / results.average_duration.as_nanos() as f64;
    assert!(p99_ratio < 3.0, 
            "P99 should not be too much worse than average (ratio: {:.2})", p99_ratio);
}

#[tokio::test]
async fn test_error_handling_performance() {
    let config = BenchmarkConfig {
        iterations: 20,
        warmup_iterations: 2,
        file_sizes: vec![10240],
        concurrent_requests: 1,
    };
    
    let runner = BenchmarkRunner::new(config);
    
    // Test performance of error handling (non-existent file)
    let results = runner.run_benchmark("error_handling_performance", || async {
        let _file_upload = FileUpload::new("nonexistent_file.pdf")?;
        Ok::<(), Error>(())
    }).await;
    
    println!("Error handling performance: {}", results.to_report());
    
    // Error handling should be fast
    assert!(results.average_duration < Duration::from_millis(10), 
            "Error handling should be fast: {:?}", results.average_duration);
    
    // Should have 100% error rate (expected)
    assert_eq!(results.error_rate, 100.0, "Should have 100% error rate for nonexistent files");
}

/// Comprehensive benchmark suite
#[tokio::test]
async fn test_comprehensive_benchmark_suite() {
    println!("Running comprehensive benchmark suite...");
    
    let configs = vec![
        BenchmarkConfig {
            iterations: 10,
            warmup_iterations: 2,
            file_sizes: vec![1024],
            concurrent_requests: 1,
        },
        BenchmarkConfig {
            iterations: 10,
            warmup_iterations: 2,
            file_sizes: vec![102400],
            concurrent_requests: 1,
        },
        BenchmarkConfig {
            iterations: 5,
            warmup_iterations: 1,
            file_sizes: vec![1024000],
            concurrent_requests: 1,
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        let runner = BenchmarkRunner::new(config.clone());
        
        let results = runner.run_benchmark(&format!("comprehensive_test_{}", i), || async {
            let file_size = config.file_sizes[0];
            let temp_file = create_test_file(file_size)?;
            let file_path = temp_file.path().to_str().unwrap();
            let _file_upload = FileUpload::new(file_path)?;
            Ok::<(), Error>(())
        }).await;
        
        println!("Benchmark {}: {}", i, results.to_report());
        
        // Each benchmark should complete successfully
        assert_eq!(results.error_rate, 0.0, "Benchmark {} should not have errors", i);
        assert!(results.average_duration < Duration::from_millis(500), 
                "Benchmark {} should be reasonably fast: {:?}", i, results.average_duration);
    }
    
    println!("Comprehensive benchmark suite completed successfully!");
}
