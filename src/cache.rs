//! Caching implementation for repeated API calls

use crate::error::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Cache entry with expiration
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: Instant,
    pub expires_at: Instant,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(data: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            expires_at: now + ttl,
        }
    }

    /// Check if the entry is expired
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }

    /// Get remaining TTL
    pub fn remaining_ttl(&self) -> Duration {
        let now = Instant::now();
        if now > self.expires_at {
            Duration::ZERO
        } else {
            self.expires_at - now
        }
    }
}

/// Cache key for file uploads (based on file content hash)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileCacheKey {
    pub file_hash: String,
    pub purpose: String,
}

/// Cache key for OCR requests (based on file ID and model)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OCRCacheKey {
    pub file_id: String,
    pub model: String,
}

/// Generic cache implementation
#[derive(Debug)]
pub struct Cache<K, V> 
where 
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    entries: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    default_ttl: Duration,
    max_entries: usize,
}

impl<K, V> Cache<K, V>
where 
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache with default TTL and max entries
    pub fn new(default_ttl: Duration, max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
            max_entries,
        }
    }

    /// Get an entry from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut entries = self.entries.write().await;
        
        if let Some(entry) = entries.get(key) {
            if entry.is_expired() {
                entries.remove(key);
                None
            } else {
                Some(entry.data.clone())
            }
        } else {
            None
        }
    }

    /// Put an entry into the cache
    pub async fn put(&self, key: K, value: V) -> Result<()> {
        let mut entries = self.entries.write().await;
        
        // Check if we need to evict entries
        if entries.len() >= self.max_entries {
            self.evict_expired_entries(&mut entries).await;
            
            // If still at max capacity, remove oldest entry
            if entries.len() >= self.max_entries {
                if let Some(oldest_key) = self.find_oldest_entry(&entries).await {
                    entries.remove(&oldest_key);
                }
            }
        }

        let entry = CacheEntry::new(value, self.default_ttl);
        entries.insert(key, entry);
        Ok(())
    }

    /// Put an entry with custom TTL
    pub async fn put_with_ttl(&self, key: K, value: V, ttl: Duration) -> Result<()> {
        let mut entries = self.entries.write().await;
        
        // Check if we need to evict entries
        if entries.len() >= self.max_entries {
            self.evict_expired_entries(&mut entries).await;
            
            // If still at max capacity, remove oldest entry
            if entries.len() >= self.max_entries {
                if let Some(oldest_key) = self.find_oldest_entry(&entries).await {
                    entries.remove(&oldest_key);
                }
            }
        }

        let entry = CacheEntry::new(value, ttl);
        entries.insert(key, entry);
        Ok(())
    }

    /// Remove an entry from the cache
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut entries = self.entries.write().await;
        entries.remove(key).map(|entry| entry.data)
    }

    /// Clear all entries
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let entries = self.entries.read().await;
        let mut total_entries = 0;
        let mut expired_entries = 0;
        let mut total_size_bytes = 0;

        for entry in entries.values() {
            total_entries += 1;
            if entry.is_expired() {
                expired_entries += 1;
            }
            // Estimate size (this is approximate)
            total_size_bytes += std::mem::size_of::<CacheEntry<V>>();
        }

        CacheStats {
            total_entries,
            expired_entries,
            active_entries: total_entries - expired_entries,
            max_entries: self.max_entries,
            estimated_size_bytes: total_size_bytes,
        }
    }

    /// Evict expired entries
    async fn evict_expired_entries(&self, entries: &mut HashMap<K, CacheEntry<V>>) {
        entries.retain(|_, entry| !entry.is_expired());
    }

    /// Find the oldest entry (least recently created)
    async fn find_oldest_entry(&self, entries: &HashMap<K, CacheEntry<V>>) -> Option<K> {
        entries
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
    pub max_entries: usize,
    pub estimated_size_bytes: usize,
}

/// File upload cache (caches file upload responses)
pub type FileUploadCache = Cache<FileCacheKey, crate::api::files::FileUploadResponse>;

/// OCR result cache (caches OCR responses)
pub type OCRResultCache = Cache<OCRCacheKey, crate::api::ocr::OCRResponse>;

/// Cache manager for all caches
#[derive(Debug)]
pub struct CacheManager {
    pub file_upload_cache: FileUploadCache,
    pub ocr_result_cache: OCRResultCache,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new() -> Self {
        Self {
            file_upload_cache: FileUploadCache::new(Duration::from_secs(3600), 100), // 1 hour TTL, 100 entries
            ocr_result_cache: OCRResultCache::new(Duration::from_secs(7200), 200), // 2 hours TTL, 200 entries
        }
    }

    /// Get combined cache statistics
    pub async fn get_stats(&self) -> CombinedCacheStats {
        let file_stats = self.file_upload_cache.stats().await;
        let ocr_stats = self.ocr_result_cache.stats().await;

        CombinedCacheStats {
            file_upload_cache: file_stats.clone(),
            ocr_result_cache: ocr_stats.clone(),
            total_active_entries: file_stats.active_entries + ocr_stats.active_entries,
            total_estimated_size_bytes: file_stats.estimated_size_bytes + ocr_stats.estimated_size_bytes,
        }
    }

    /// Clear all caches
    pub async fn clear_all(&self) {
        self.file_upload_cache.clear().await;
        self.ocr_result_cache.clear().await;
    }
}

/// Combined cache statistics
#[derive(Debug, Clone)]
pub struct CombinedCacheStats {
    pub file_upload_cache: CacheStats,
    pub ocr_result_cache: CacheStats,
    pub total_active_entries: usize,
    pub total_estimated_size_bytes: usize,
}

/// Global cache manager instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_CACHE: CacheManager = CacheManager::new();
}

/// Helper function to generate file hash for caching
pub fn generate_file_hash(file_data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    file_data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(1), 10);
        
        // Test put and get
        cache.put("key1".to_string(), "value1".to_string()).await.unwrap();
        assert_eq!(cache.get(&"key1".to_string()).await, Some("value1".to_string()));
        
        // Test expiration
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_cache_max_entries() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(10), 2);
        
        cache.put("key1".to_string(), "value1".to_string()).await.unwrap();
        cache.put("key2".to_string(), "value2".to_string()).await.unwrap();
        cache.put("key3".to_string(), "value3".to_string()).await.unwrap(); // Should evict key1
        
        assert_eq!(cache.get(&"key1".to_string()).await, None);
        assert_eq!(cache.get(&"key2".to_string()).await, Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()).await, Some("value3".to_string()));
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache: Cache<String, String> = Cache::new(Duration::from_secs(1), 10);
        
        cache.put("key1".to_string(), "value1".to_string()).await.unwrap();
        cache.put("key2".to_string(), "value2".to_string()).await.unwrap();
        
        let stats = cache.stats().await;
        assert_eq!(stats.active_entries, 2);
        assert_eq!(stats.total_entries, 2);
        
        tokio::time::sleep(Duration::from_millis(1100)).await;
        let stats_after_expiry = cache.stats().await;
        assert_eq!(stats_after_expiry.active_entries, 0);
    }

    #[test]
    fn test_file_hash_generation() {
        let data1 = b"test data";
        let data2 = b"test data";
        let data3 = b"different data";
        
        let hash1 = generate_file_hash(data1);
        let hash2 = generate_file_hash(data2);
        let hash3 = generate_file_hash(data3);
        
        assert_eq!(hash1, hash2); // Same data should produce same hash
        assert_ne!(hash1, hash3); // Different data should produce different hash
    }

    #[tokio::test]
    async fn test_cache_manager() {
        let manager = CacheManager::new();
        
        let file_key = FileCacheKey {
            file_hash: "test_hash".to_string(),
            purpose: "ocr".to_string(),
        };
        
        let ocr_key = OCRCacheKey {
            file_id: "test_file_id".to_string(),
            model: "mistral-large".to_string(),
        };
        
        // Test file upload cache
        let upload_response = crate::api::files::FileUploadResponse {
            id: "test_id".to_string(),
            object: "file".to_string(),
            bytes: 1024,
            created_at: 1234567890,
            filename: "test.pdf".to_string(),
            purpose: "ocr".to_string(),
            status: None,
        };
        
        manager.file_upload_cache.put(file_key.clone(), upload_response.clone()).await.unwrap();
        assert!(manager.file_upload_cache.get(&file_key).await.is_some());
        
        // Test OCR result cache
        let ocr_response = crate::api::ocr::OCRResponse {
            model: "mistral-large".to_string(),
            pages: vec![],
            usage_info: crate::api::ocr::UsageInfo {
                pages_processed: 1,
                doc_size_bytes: 1024,
            },
            document_annotation: None,
        };
        
        manager.ocr_result_cache.put(ocr_key.clone(), ocr_response.clone()).await.unwrap();
        assert!(manager.ocr_result_cache.get(&ocr_key).await.is_some());
        
        // Test combined stats
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_active_entries, 2);
    }
}
