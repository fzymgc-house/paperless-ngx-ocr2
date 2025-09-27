//! Performance testing utilities
//!
//! This module provides utilities for measuring and validating performance
//! in tests, including timing constraints and memory usage monitoring.

#![allow(dead_code)]

use std::time::{Duration, Instant};

/// A performance test that validates execution time
#[derive(Debug)]
pub struct PerformanceTest {
    name: String,
    start: Instant,
    max_duration: Duration,
}

impl PerformanceTest {
    /// Creates a new performance test with a maximum duration constraint
    pub fn new(name: &str, max_duration: Duration) -> Self {
        Self { name: name.to_string(), start: Instant::now(), max_duration }
    }

    /// Asserts that the test completed within the maximum duration
    pub fn assert_within_time(self) {
        let elapsed = self.start.elapsed();
        assert!(elapsed <= self.max_duration, "Performance test '{}' took {:?}, expected <= {:?}", self.name, elapsed, self.max_duration);
    }

    /// Gets the elapsed time so far
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Checks if the test is still within the time limit
    pub fn is_within_time(&self) -> bool {
        self.start.elapsed() <= self.max_duration
    }
}

/// Measures the performance of a function and asserts it completes within the time limit
pub fn measure_performance<F, R>(name: &str, max_duration: Duration, f: F) -> R
where
    F: FnOnce() -> R,
{
    let test = PerformanceTest::new(name, max_duration);
    let result = f();
    test.assert_within_time();
    result
}

/// Measures the performance of an async function and asserts it completes within the time limit
pub async fn measure_performance_async<F, Fut, R>(name: &str, max_duration: Duration, f: F) -> R
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = R>,
{
    let test = PerformanceTest::new(name, max_duration);
    let result = f().await;
    test.assert_within_time();
    result
}

/// Performance test configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub fast_threshold: Duration,
    pub normal_threshold: Duration,
    pub slow_threshold: Duration,
    pub very_slow_threshold: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            fast_threshold: Duration::from_millis(10),
            normal_threshold: Duration::from_millis(100),
            slow_threshold: Duration::from_millis(1000),
            very_slow_threshold: Duration::from_secs(10),
        }
    }
}

impl PerformanceConfig {
    /// Creates a new performance configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the fast threshold
    pub fn with_fast_threshold(mut self, threshold: Duration) -> Self {
        self.fast_threshold = threshold;
        self
    }

    /// Sets the normal threshold
    pub fn with_normal_threshold(mut self, threshold: Duration) -> Self {
        self.normal_threshold = threshold;
        self
    }

    /// Sets the slow threshold
    pub fn with_slow_threshold(mut self, threshold: Duration) -> Self {
        self.slow_threshold = threshold;
        self
    }

    /// Sets the very slow threshold
    pub fn with_very_slow_threshold(mut self, threshold: Duration) -> Self {
        self.very_slow_threshold = threshold;
        self
    }
}

/// Performance benchmark that runs a function multiple times and measures statistics
pub struct Benchmark {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
}

impl Benchmark {
    /// Creates a new benchmark
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), iterations: 100, warmup_iterations: 10 }
    }

    /// Sets the number of iterations
    pub fn iterations(mut self, count: usize) -> Self {
        self.iterations = count;
        self
    }

    /// Sets the number of warmup iterations
    pub fn warmup_iterations(mut self, count: usize) -> Self {
        self.warmup_iterations = count;
        self
    }

    /// Runs the benchmark and returns statistics
    pub fn run<F>(self, f: F) -> BenchmarkResults
    where
        F: Fn(),
    {
        let mut times = Vec::new();

        // Warmup iterations
        for _ in 0..self.warmup_iterations {
            f();
        }

        // Actual benchmark iterations
        for _ in 0..self.iterations {
            let start = Instant::now();
            f();
            let elapsed = start.elapsed();
            times.push(elapsed);
        }

        BenchmarkResults::new(self.name, times)
    }
}

/// Results from a benchmark run
#[derive(Debug)]
pub struct BenchmarkResults {
    pub name: String,
    pub iterations: usize,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
    pub median_time: Duration,
    pub p95_time: Duration,
    pub p99_time: Duration,
}

impl BenchmarkResults {
    fn new(name: String, mut times: Vec<Duration>) -> Self {
        times.sort();

        let iterations = times.len();
        let min_time = times[0];
        let max_time = times[iterations - 1];

        let total_time: Duration = times.iter().sum();
        let avg_time = total_time / iterations as u32;

        let median_time = times[iterations / 2];
        let p95_time = times[(iterations * 95) / 100];
        let p99_time = times[(iterations * 99) / 100];

        Self { name, iterations, min_time, max_time, avg_time, median_time, p95_time, p99_time }
    }

    /// Asserts that the average time is within the specified threshold
    pub fn assert_avg_time_less_than(&self, threshold: Duration) {
        assert!(self.avg_time <= threshold, "Benchmark '{}' average time {:?} exceeded threshold {:?}", self.name, self.avg_time, threshold);
    }

    /// Asserts that the 95th percentile time is within the specified threshold
    pub fn assert_p95_time_less_than(&self, threshold: Duration) {
        assert!(self.p95_time <= threshold, "Benchmark '{}' 95th percentile time {:?} exceeded threshold {:?}", self.name, self.p95_time, threshold);
    }

    /// Asserts that the maximum time is within the specified threshold
    pub fn assert_max_time_less_than(&self, threshold: Duration) {
        assert!(self.max_time <= threshold, "Benchmark '{}' maximum time {:?} exceeded threshold {:?}", self.name, self.max_time, threshold);
    }

    /// Prints the benchmark results
    pub fn print_results(&self) {
        println!("Benchmark Results: {}", self.name);
        println!("  Iterations: {}", self.iterations);
        println!("  Min time: {:?}", self.min_time);
        println!("  Max time: {:?}", self.max_time);
        println!("  Average time: {:?}", self.avg_time);
        println!("  Median time: {:?}", self.median_time);
        println!("  95th percentile: {:?}", self.p95_time);
        println!("  99th percentile: {:?}", self.p99_time);
    }
}

/// Memory usage monitoring utilities
pub mod memory {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A simple memory allocator that tracks allocations
    pub struct TrackingAllocator {
        inner: System,
        allocated: AtomicUsize,
    }

    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = self.inner.alloc(layout);
            if !ptr.is_null() {
                self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
            self.inner.dealloc(ptr, layout);
        }
    }

    /// Global tracking allocator instance
    #[global_allocator]
    static GLOBAL: TrackingAllocator = TrackingAllocator { inner: System, allocated: AtomicUsize::new(0) };

    /// Gets the current allocated memory in bytes
    pub fn get_allocated_bytes() -> usize {
        GLOBAL.allocated.load(Ordering::Relaxed)
    }

    /// Resets the memory tracking counter
    pub fn reset_memory_tracking() {
        GLOBAL.allocated.store(0, Ordering::Relaxed);
    }

    /// Memory usage test helper
    pub struct MemoryTest {
        start_memory: usize,
        max_memory_increase: Option<usize>,
    }

    impl MemoryTest {
        /// Creates a new memory test
        pub fn new() -> Self {
            Self { start_memory: get_allocated_bytes(), max_memory_increase: None }
        }

        /// Sets the maximum allowed memory increase
        pub fn with_max_increase(mut self, max_bytes: usize) -> Self {
            self.max_memory_increase = Some(max_bytes);
            self
        }

        /// Asserts that memory usage is within acceptable limits
        pub fn assert_memory_usage(self) {
            let current_memory = get_allocated_bytes();
            let memory_increase = current_memory.saturating_sub(self.start_memory);

            if let Some(max_increase) = self.max_memory_increase {
                assert!(memory_increase <= max_increase, "Memory usage increased by {} bytes, expected <= {} bytes", memory_increase, max_increase);
            }
        }

        /// Gets the current memory increase
        pub fn get_memory_increase(&self) -> usize {
            let current_memory = get_allocated_bytes();
            current_memory.saturating_sub(self.start_memory)
        }
    }
}

/// Stress test utilities
pub mod stress {
    use super::*;

    /// Runs a function multiple times to stress test it
    pub fn stress_test<F>(name: &str, iterations: usize, f: F) -> StressTestResults
    where
        F: Fn(),
    {
        let mut results = StressTestResults::new(name.to_string());

        for i in 0..iterations {
            let start = Instant::now();
            f();
            let elapsed = start.elapsed();
            results.add_result(elapsed);

            // Print progress every 10% of iterations
            if (i + 1) % (iterations / 10).max(1) == 0 {
                println!("Stress test '{}': {}/{} iterations completed", name, i + 1, iterations);
            }
        }

        results
    }

    /// Results from a stress test
    #[derive(Debug)]
    pub struct StressTestResults {
        pub name: String,
        pub iterations: usize,
        pub successful_iterations: usize,
        pub failed_iterations: usize,
        pub total_time: Duration,
        pub avg_time: Duration,
        pub min_time: Duration,
        pub max_time: Duration,
        pub error_rate: f64,
    }

    impl StressTestResults {
        fn new(name: String) -> Self {
            Self {
                name,
                iterations: 0,
                successful_iterations: 0,
                failed_iterations: 0,
                total_time: Duration::ZERO,
                avg_time: Duration::ZERO,
                min_time: Duration::MAX,
                max_time: Duration::ZERO,
                error_rate: 0.0,
            }
        }

        fn add_result(&mut self, elapsed: Duration) {
            self.iterations += 1;
            self.successful_iterations += 1;
            self.total_time += elapsed;

            if elapsed < self.min_time {
                self.min_time = elapsed;
            }
            if elapsed > self.max_time {
                self.max_time = elapsed;
            }
        }

        fn finalize(&mut self) {
            if self.iterations > 0 {
                self.avg_time = self.total_time / self.iterations as u32;
                self.error_rate = self.failed_iterations as f64 / self.iterations as f64;
            }
        }

        /// Asserts that the error rate is within acceptable limits
        pub fn assert_error_rate_less_than(&self, max_error_rate: f64) {
            assert!(
                self.error_rate <= max_error_rate,
                "Stress test '{}' error rate {:.2}% exceeded threshold {:.2}%",
                self.name,
                self.error_rate * 100.0,
                max_error_rate * 100.0
            );
        }

        /// Asserts that the average time is within acceptable limits
        pub fn assert_avg_time_less_than(&self, threshold: Duration) {
            assert!(self.avg_time <= threshold, "Stress test '{}' average time {:?} exceeded threshold {:?}", self.name, self.avg_time, threshold);
        }

        /// Prints the stress test results
        pub fn print_results(&self) {
            println!("Stress Test Results: {}", self.name);
            println!("  Iterations: {}", self.iterations);
            println!("  Successful: {}", self.successful_iterations);
            println!("  Failed: {}", self.failed_iterations);
            println!("  Error rate: {:.2}%", self.error_rate * 100.0);
            println!("  Total time: {:?}", self.total_time);
            println!("  Average time: {:?}", self.avg_time);
            println!("  Min time: {:?}", self.min_time);
            println!("  Max time: {:?}", self.max_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_test() {
        let test = PerformanceTest::new("test", Duration::from_millis(100));
        thread::sleep(Duration::from_millis(10));
        test.assert_within_time();
    }

    #[test]
    fn test_measure_performance() {
        let result = measure_performance("test", Duration::from_millis(100), || {
            thread::sleep(Duration::from_millis(10));
            42
        });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_benchmark() {
        let results = Benchmark::new("test").iterations(10).warmup_iterations(2).run(|| {
            thread::sleep(Duration::from_millis(1));
        });

        assert_eq!(results.iterations, 10);
        results.assert_avg_time_less_than(Duration::from_millis(50));
    }

    #[test]
    fn test_memory_tracking() {
        use memory::*;

        reset_memory_tracking();
        let test = MemoryTest::new();

        // Allocate some memory
        let _vec: Vec<u8> = vec![0; 1000];

        test.assert_memory_usage();
    }

    #[test]
    fn test_stress_test() {
        let results = stress::stress_test("test", 10, || {
            thread::sleep(Duration::from_millis(1));
        });

        assert_eq!(results.iterations, 10);
        results.assert_error_rate_less_than(0.1);
    }
}
