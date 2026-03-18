use anyhow::Result;
use chrono::{DateTime, Utc};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::steps::observers::{FileContent, FileMetadata, MemoryCacheConfig};

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub memory_usage_bytes: u64,
    pub file_count: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    content: FileContent,
    size_bytes: u64,
    created_at: DateTime<Utc>,
}

pub struct MemoryTier {
    cache: Arc<Mutex<LruCache<String, CacheEntry>>>,
    max_size_bytes: u64,
    current_size_bytes: Arc<Mutex<u64>>,
    ttl_seconds: u64,
    stats: Arc<RwLock<CacheStats>>,
}

impl Clone for MemoryTier {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            max_size_bytes: self.max_size_bytes,
            current_size_bytes: Arc::clone(&self.current_size_bytes),
            ttl_seconds: self.ttl_seconds,
            stats: Arc::clone(&self.stats),
        }
    }
}

impl std::fmt::Debug for MemoryTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryTier")
            .field("max_size_bytes", &self.max_size_bytes)
            .field("ttl_seconds", &self.ttl_seconds)
            .finish()
    }
}

impl MemoryTier {
    pub fn new(config: &MemoryCacheConfig, stats: Arc<RwLock<CacheStats>>) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(config.max_files).unwrap_or(NonZeroUsize::MIN),
            ))),
            max_size_bytes: config.max_size_mb * 1024 * 1024,
            current_size_bytes: Arc::new(Mutex::new(0)),
            ttl_seconds: config.ttl_seconds,
            stats,
        }
    }

    pub async fn get(&self, path: &str) -> Option<FileContent> {
        let mut cache = self.cache.lock().await;
        if let Some(entry) = cache.get_mut(path) {
            let now = Utc::now();
            let age = now.signed_duration_since(entry.created_at);

            if age.num_seconds() > self.ttl_seconds as i64 {
                let size = entry.size_bytes;
                cache.pop(path);
                let mut current_size = self.current_size_bytes.lock().await;
                *current_size -= size;
                self.increment_eviction().await;
                return None;
            }

            self.increment_hit().await;
            Some(entry.content.clone())
        } else {
            self.increment_miss().await;
            None
        }
    }

    pub async fn put(&self, path: String, content: FileContent) -> Result<()> {
        let size_bytes = content.content.len() as u64;

        if size_bytes > self.max_size_bytes {
            return Ok(());
        }

        let mut cache = self.cache.lock().await;
        let mut current_size = self.current_size_bytes.lock().await;

        while *current_size + size_bytes > self.max_size_bytes && !cache.is_empty() {
            if let Some((_, entry)) = cache.pop_lru() {
                *current_size -= entry.size_bytes;
                self.increment_eviction().await;
            }
        }

        let now = Utc::now();
        cache.put(
            path,
            CacheEntry {
                content,
                size_bytes,
                created_at: now,
            },
        );

        Ok(())
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let mut cache = self.cache.lock().await;
        if let Some(entry) = cache.pop(path) {
            let mut current_size = self.current_size_bytes.lock().await;
            *current_size -= entry.size_bytes;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<FileMetadata> {
        todo!()
    }

    pub async fn evict_expired(&self) -> Result<usize> {
        todo!()
    }

    pub async fn stats(&self) -> CacheStats {
        todo!()
    }

    async fn increment_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hit_count += 1;
    }

    async fn increment_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.miss_count += 1;
    }

    async fn increment_eviction(&self) {
        let mut stats = self.stats.write().await;
        stats.eviction_count += 1;
    }
}

pub struct TieredCache {
    memory: MemoryTier,
    stats: Arc<RwLock<CacheStats>>,
}

impl Clone for TieredCache {
    fn clone(&self) -> Self {
        Self {
            memory: self.memory.clone(),
            stats: Arc::clone(&self.stats),
        }
    }
}

impl std::fmt::Debug for TieredCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TieredCache")
            .field("memory", &self.memory)
            .finish()
    }
}

impl TieredCache {
    pub fn new(memory_config: MemoryCacheConfig) -> Self {
        let stats = Arc::new(RwLock::new(CacheStats {
            memory_usage_bytes: 0,
            file_count: 0,
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
        }));

        Self {
            memory: MemoryTier::new(&memory_config, stats.clone()),
            stats,
        }
    }

    pub async fn get(&self, path: &str) -> Option<FileContent> {
        self.memory.get(path).await
    }

    pub async fn put(&self, path: String, content: FileContent) -> Result<()> {
        self.memory.put(path, content).await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        self.memory.delete(path).await
    }

    pub async fn list(&self) -> Vec<FileMetadata> {
        todo!()
    }

    pub async fn evict_expired(&self) -> Result<usize> {
        todo!()
    }

    pub async fn stats(&self) -> CacheStats {
        todo!()
    }
}
