# FileObserver Guide

This guide covers the FileObserver system in k8s-maestro for watching filesystem changes and serving file content via caching and HTTP.

## Overview

FileObserver is a component that monitors filesystem changes and serves file content through multiple modes:

- **Channel Mode**: Publishes file change events to a channel for consumers
- **Cache Mode**: Caches file content in memory for fast retrieval
- **HTTP Service**: Exposes file content via REST API endpoints

Use FileObserver when you need to:
- Track file creation, modification, and deletion events
- Cache frequently accessed files in memory
- Serve files via HTTP to external consumers

## FileObserverBuilder

The `FileObserverBuilder` provides a fluent API for configuring FileObserver instances.

### Builder API

```rust
use k8s_maestro::steps::observers::FileObserverBuilder;

let observer = FileObserverBuilder::new()
    .watch_path("/data/files")
    .with_channel_mode(true)
    .with_cache_mode(true, Some(cache_config))
    .with_http_service(true, 8080)
    .with_filters(filters)
    .build()?;
```

### Methods

| Method | Description |
|--------|-------------|
| `new()` | Create a new builder instance |
| `watch_path(path)` | Set the path to watch for file changes |
| `with_channel_mode(enabled)` | Enable or disable channel mode |
| `with_cache_mode(enabled, config)` | Enable or disable cache mode with optional config |
| `with_http_service(enabled, port)` | Enable or disable HTTP service on port |
| `with_filters(filters)` | Apply file filters |
| `build()` | Build the FileObserverSidecar |

### Configuration Options

The builder requires:
- `watch_path` - Must be provided
- At least one mode enabled (channel, cache, or http_service)

## FileEvent

FileEvent represents filesystem change events with three event types.

### Event Types

```rust
use k8s_maestro::steps::observers::FileEvent;
use chrono::Utc;

// Created event - fired when a new file is detected
let created_event = FileEvent::Created(FileMetadata {
    filename: "data.txt".to_string(),
    path: "/data/files/data.txt".to_string(),
    mime_type: "text/plain".to_string(),
    size: 1024,
    created_at: Utc::now(),
    modified_at: Utc::now(),
});

// Modified event - fired when a file changes
let modified_event = FileEvent::Modified {
    filename: "data.txt".to_string(),
    path: "/data/files/data.txt".to_string(),
    size: 2048,
    modified_at: Utc::now(),
};

// Deleted event - fired when a file is removed
let deleted_event = FileEvent::Deleted {
    filename: "data.txt".to_string(),
    path: "/data/files/data.txt".to_string(),
    deleted_at: Utc::now(),
};
```

### FileMetadata

```rust
use k8s_maestro::steps::observers::FileMetadata;

let metadata = FileMetadata::new(
    "document.pdf".to_string(),
    "/data/files/document.pdf".to_string(),
    "application/pdf".to_string(),
    524288,
    Utc::now(),
    Utc::now(),
);

println!("File: {}", metadata.filename);
println!("Path: {}", metadata.path);
println!("Size: {} bytes", metadata.size);
println!("MIME Type: {}", metadata.mime_type);
```

### FileContent

```rust
use k8s_maestro::steps::observers::{FileContent, FileMetadata};

let content = FileContent::new(metadata, b"file content here".to_vec());

// Access metadata
let meta = &content.metadata;
println!("Filename: {}", meta.filename);

// Access raw bytes
let bytes = &content.content;
println!("Content length: {}", bytes.len());
```

## TieredCache

The TieredCache provides an in-memory LRU cache for file content with TTL support.

### MemoryCacheConfig

```rust
use k8s_maestro::steps::observers::MemoryCacheConfig;

let config = MemoryCacheConfig {
    max_size_mb: 100,      // Max cache size in MB
    max_files: 500,         // Max number of files in cache
    ttl_seconds: 3600,      // Time-to-live in seconds (1 hour)
};

// Or use defaults
let default_config = MemoryCacheConfig::default();
```

### Default Configuration

| Parameter | Default Value |
|-----------|---------------|
| max_size_mb | 50 |
| max_files | 100 |
| ttl_seconds | 3600 |

### CacheStats

```rust
use k8s_maestro::steps::observers::{CacheStats, TieredCache};

let cache = TieredCache::new(config);

// Get statistics
let stats = cache.stats().await;

println!("Memory usage: {} bytes", stats.memory_usage_bytes);
println!("File count: {}", stats.file_count);
println!("Hit count: {}", stats.hit_count);
println!("Miss count: {}", stats.miss_count);
println!("Eviction count: {}", stats.eviction_count);
```

### Cache Operations

```rust
use k8s_maestro::steps::observers::{TieredCache, FileContent, FileMetadata};

// Put content into cache
cache.put(
    "/data/files/test.txt".to_string(),
    FileContent::new(metadata, b"content".to_vec()),
).await?;

// Get content from cache
let content = cache.get("/data/files/test.txt").await;
if let Some(file_content) = content {
    println!("Found: {}", file_content.metadata.filename);
} else {
    println!("Not found in cache");
}

// Delete from cache
cache.delete("/data/files/test.txt").await?;

// List all cached files
let files = cache.list().await;

// Evict expired entries
let evicted = cache.evict_expired().await?;
println!("Evicted {} expired entries", evicted);
```

### Cache Behavior

- **LRU Eviction**: Least recently used entries are evicted when capacity is reached
- **TTL Expiration**: Entries expire after their TTL seconds
- **Size Limits**: Single files larger than max_size_mb are rejected
- **Automatic Cleanup**: Expired entries are removed on access

## HTTP Service

FileHttpService provides REST API endpoints for accessing cached file content.

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/files` | List all cached files |
| GET | `/files/:path` | Get file content |
| GET | `/files/:path/metadata` | Get file metadata |
| HEAD | `/files/:path` | Check if file exists |

### Starting the HTTP Service

```rust
use k8s_maestro::steps::observers::{FileHttpService, TieredCache};
use std::sync::Arc;

let cache = Arc::new(TieredCache::new(config));
let http_service = FileHttpService::new(cache);

// Start on port 8080
http_service.start(8080).await?;
```

### API Examples

```bash
# List all files
curl http://localhost:8080/files

# Get file content
curl http://localhost:8080/files/data/test.txt

# Get file metadata
curl http://localhost:8080/files/data/test.txt/metadata

# Check if file exists (HEAD request)
curl -I http://localhost:8080/files/data/test.txt
```

### Response Formats

**GET /files** (List files):
```json
[
  {
    "filename": "test.txt",
    "path": "/data/test.txt",
    "mime_type": "text/plain",
    "size": 1024,
    "created_at": "2024-01-15T10:00:00Z",
    "modified_at": "2024-01-15T10:00:00Z"
  }
]
```

**GET /files/:path** (File content):
Returns raw file content with headers:
- `content-type`: MIME type
- `content-length`: File size in bytes

**GET /files/:path/metadata** (Metadata):
```json
{
  "filename": "test.txt",
  "path": "/data/test.txt",
  "mime_type": "text/plain",
  "size": 1024,
  "created_at": "2024-01-15T10:00:00Z",
  "modified_at": "2024-01-15T10:00:00Z"
}
```

## Usage Examples

### Example 1: Basic FileObserver with Channel Mode

Watch a directory and receive file change events:

```rust
use k8s_maestro::steps::observers::FileObserverBuilder;
use k8s_maestro::steps::observers::FileEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create channel for events
    let (tx, mut rx) = mpsc::channel::<FileEvent>(100);

    // Build observer with channel mode
    let observer = FileObserverBuilder::new()
        .watch_path("/data/watch")
        .with_channel_mode(true)
        .build()?;

    println!("Watching: {}", observer.watch_path);

    // Spawn task to handle events
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                FileEvent::Created(meta) => {
                    println!("Created: {} at {}", meta.filename, meta.path);
                }
                FileEvent::Modified { filename, path, size, .. } => {
                    println!("Modified: {} ({} bytes)", filename, size);
                }
                FileEvent::Deleted { filename, path, .. } => {
                    println!("Deleted: {}", filename);
                }
            }
        }
    });

    // Keep the observer running
    tokio::signal::ctrl_c().await?;

    Ok(())
}
```

### Example 2: FileObserver with Cache Mode

Cache file content for fast retrieval:

```rust
use k8s_maestro::steps::observers::{FileObserverBuilder, MemoryCacheConfig, TieredCache, FileContent, FileMetadata};
use chrono::Utc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure cache
    let cache_config = MemoryCacheConfig {
        max_size_mb: 100,
        max_files: 200,
        ttl_seconds: 1800,  // 30 minutes
    };

    // Build observer with cache mode
    let observer = FileObserverBuilder::new()
        .watch_path("/data/cache")
        .with_cache_mode(true, Some(cache_config))
        .build()?;

    // Get the cache
    if let Some(cache) = &observer.cache {
        // Pre-populate cache with a file
        let metadata = FileMetadata::new(
            "example.txt".to_string(),
            "/data/cache/example.txt".to_string(),
            "text/plain".to_string(),
            1024,
            Utc::now(),
            Utc::now(),
        );

        cache.put(
            "/data/cache/example.txt".to_string(),
            FileContent::new(metadata, b"Hello, World!".to_vec()),
        ).await?;

        // Retrieve from cache
        if let Some(content) = cache.get("/data/cache/example.txt").await {
            println!("Retrieved: {}", String::from_utf8_lossy(&content.content));
        }

        // Get cache statistics
        let stats = cache.stats().await;
        println!("Cache hits: {}, misses: {}", stats.hit_count, stats.miss_count);
    }

    Ok(())
}
```

### Example 3: FileObserver with HTTP Service

Serve cached files via HTTP:

```rust
use k8s_maestro::steps::observers::{FileObserverBuilder, MemoryCacheConfig, FileHttpService, FileContent, FileMetadata};
use chrono::Utc;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure cache
    let cache_config = MemoryCacheConfig::default();

    // Build observer with cache and HTTP service
    let observer = FileObserverBuilder::new()
        .watch_path("/data/files")
        .with_cache_mode(true, Some(cache_config.clone()))
        .with_http_service(true, 8080)
        .build()?;

    // Get the cache for pre-populating
    if let Some(cache) = observer.cache {
        let cache = Arc::new(cache);

        // Pre-populate with some files
        let files = vec![
            ("/data/files/index.html", "<html><body>Hello</body></html>", "text/html"),
            ("/data/files/data.json", r#"{"key": "value"}"#, "application/json"),
        ];

        for (path, content, mime_type) in files {
            let metadata = FileMetadata::new(
                path.split('/').last().unwrap().to_string(),
                path.to_string(),
                mime_type.to_string(),
                content.len() as u64,
                Utc::now(),
                Utc::now(),
            );

            cache.put(
                path.to_string(),
                FileContent::new(metadata, content.as_bytes().to_vec()),
            ).await?;
        }

        // Start HTTP service
        let http_service = FileHttpService::new(cache);
        http_service.start(8080).await?;
    }

    Ok(())
}
```

### Example 4: FileObserver with Filters

Apply filters to include/exclude specific files:

```rust
use k8s_maestro::steps::observers::{FileObserverBuilder, FileFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure filters
    let filters = FileFilter {
        include_patterns: vec![
            "*.txt".to_string(),
            "*.json".to_string(),
        ],
        exclude_patterns: vec![
            "*.tmp".to_string(),
            ".git/*".to_string(),
        ],
        max_file_size_mb: 50,
    };

    let observer = FileObserverBuilder::new()
        .watch_path("/data/files")
        .with_channel_mode(true)
        .with_filters(filters)
        .build()?;

    println!("Watching: {}", observer.watch_path);
    println!("Filters: {:?}", observer.filters);

    Ok(())
}
```

## Combined Examples

### Example 5: FileObserver + Cache + HTTP Service

Combine all three modes for comprehensive file handling:

```rust
use k8s_maestro::steps::observers::{
    FileObserverBuilder,
    FileHttpService,
    MemoryCacheConfig,
    TieredCache,
    FileContent,
    FileMetadata,
};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::mpsc;
use k8s_maestro::steps::observers::FileEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure cache
    let cache_config = MemoryCacheConfig {
        max_size_mb: 200,
        max_files: 1000,
        ttl_seconds: 3600,
    };

    // Build observer with all modes enabled
    let observer = FileObserverBuilder::new()
        .watch_path("/data/files")
        .with_channel_mode(true)
        .with_cache_mode(true, Some(cache_config))
        .with_http_service(true, 8080)
        .build()?;

    println!("FileObserver configured:");
    println!("  - Watch path: {}", observer.watch_path);
    println!("  - Channel mode: {}", observer.modes.channel);
    println!("  - Cache mode: {}", observer.modes.cache);
    println!("  - HTTP service: {} on port {:?}", observer.modes.http_service, observer.http_port);

    // Set up event channel handling
    let (tx, mut rx) = mpsc::channel::<FileEvent>(100);

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                FileEvent::Created(meta) => {
                    println!("New file: {} ({} bytes)", meta.filename, meta.size);
                }
                FileEvent::Modified { filename, size, .. } => {
                    println!("Modified: {} ({} bytes)", filename, size);
                }
                FileEvent::Deleted { filename, .. } => {
                    println!("Deleted: {}", filename);
                }
            }
        }
    });

    // Start HTTP service if cache is available
    if let Some(cache) = observer.cache {
        let cache = Arc::new(cache);

        // Pre-populate cache
        let demo_files = vec![
            ("/data/files/config.json", r#"{"version": "1.0"}"#, "application/json"),
            ("/data/files/readme.md", "# Demo\n\nExample files." , "text/markdown"),
        ];

        for (path, content, mime) in demo_files {
            let meta = FileMetadata::new(
                path.split('/').last().unwrap().to_string(),
                path.to_string(),
                mime.to_string(),
                content.len() as u64,
                Utc::now(),
                Utc::now(),
            );

            cache.put(
                path.to_string(),
                FileContent::new(meta, content.as_bytes().to_vec()),
            ).await?;
        }

        // Start HTTP service
        let http_service = FileHttpService::new(cache);
        http_service.start(8080).await?;
    }

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;

    Ok(())
}
```

### Example 6: FileObserver + Ingress (External Access)

Expose the HTTP service via Kubernetes Ingress for external access:

```rust
use k8s_maestro::steps::observers::{FileObserverBuilder, MemoryCacheConfig};
use k8s_maestro::entities::volumes::EmptyDirVolumeBuilder;
use k8s_maestro::entities::MaestroContainer;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;

// Configure the observer for the pod
let observer = FileObserverBuilder::new()
    .watch_path("/data/files")
    .with_cache_mode(true, Some(MemoryCacheConfig::default()))
    .with_http_service(true, 8080)
    .build()?;

// Create a container that runs the file observer service
let observer_container = MaestroContainer::new("myapp/file-observer:latest", "file-observer")
    .set_arguments(&vec!["/usr/bin/file-observer".to_string()])
    .with_volume(Box::new(
        EmptyDirVolumeBuilder::new("/data/files", "file-storage")
            .build()
    ))
    .set_resource_bounds(
        ResourceLimits::builder()
            .with_memory_limit("256Mi")
            .with_cpu_limit("500m")
            .build()
    );

// Create the Kubernetes job
let job = KubeJobStepBuilder::new()
    .with_name("file-observer-job")
    .with_namespace("default")
    .with_client(client)
    .add_container(Box::new(observer_container))
    .with_ingress("/files", "file-observer-ingress", "nginx")
    .build()?;

// Create the job in Kubernetes
job.create_job(false).await?;
println!("FileObserver job created");

// Wait for completion
job.wait().await?;
println!("FileObserver job completed");

// Clean up
job.delete_job(false).await?;
```

This example shows how to:
1. Configure FileObserver with cache and HTTP service modes
2. Create a container that runs the file observer
3. Expose the HTTP service via Kubernetes Ingress
4. Deploy to Kubernetes as a job

### Example 7: Multi-mode Observer with Statistics

Monitor cache performance and log statistics:

```rust
use k8s_maestro::steps::observers::{FileObserverBuilder, MemoryCacheConfig, CacheStats};
use std::sync::Arc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cache_config = MemoryCacheConfig {
        max_size_mb: 500,
        max_files: 2000,
        ttl_seconds: 600,  // 10 minutes
    };

    let observer = FileObserverBuilder::new()
        .watch_path("/data/large-storage")
        .with_cache_mode(true, Some(cache_config))
        .with_http_service(true, 8080)
        .build()?;

    if let Some(cache) = observer.cache {
        let cache = Arc::new(cache);

        // Periodic statistics reporting
        let cache_for_stats = cache.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(30));
            loop {
                ticker.tick().await;
                let stats = cache_for_stats.stats().await;
                println!(
                    "Cache Stats - Files: {}, Memory: {}MB, Hits: {}, Misses: {}, Evictions: {}",
                    stats.file_count,
                    stats.memory_usage_bytes / (1024 * 1024),
                    stats.hit_count,
                    stats.miss_count,
                    stats.eviction_count,
                );
            }
        });

        // Start HTTP service
        let http_service = FileHttpService::new(cache);
        http_service.start(8080).await?;
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

This example demonstrates:
- High-capacity caching (500MB, 2000 files)
- Periodic cache statistics monitoring
- Real-time performance tracking

## FileFilter Configuration

### Include Patterns

```rust
use k8s_maestro::steps::observers::FileFilter;

let filter = FileFilter {
    include_patterns: vec![
        "*.txt".to_string(),
        "*.json".to_string(),
        "*.xml".to_string(),
    ],
    exclude_patterns: vec![],
    max_file_size_mb: 100,
};
```

### Exclude Patterns

```rust
use k8s_maestro::steps::observers::FileFilter;

let filter = FileFilter {
    include_patterns: vec![],
    exclude_patterns: vec![
        "*.tmp".to_string(),
        "*.bak".to_string(),
        ".git/*".to_string(),
        "node_modules/*".to_string(),
        "*.log".to_string(),
    ],
    max_file_size_mb: 50,
};
```

### Combined Filters

```rust
use k8s_maestro::steps::observers::FileFilter;

let filter = FileFilter {
    include_patterns: vec![
        "src/**/*.rs".to_string(),
        "tests/**/*.rs".to_string(),
        "Cargo.toml".to_string(),
    ],
    exclude_patterns: vec![
        "target/**/*.rs".to_string(),
        "*.generated.rs".to_string(),
    ],
    max_file_size_mb: 10,
};
```

## Performance Considerations

### Cache Sizing

| Use Case | max_size_mb | max_files | ttl_seconds |
|----------|-------------|-----------|-------------|
| Small files | 50 | 100 | 3600 |
| Large files | 500 | 500 | 600 |
| High throughput | 1000 | 2000 | 300 |
| Static content | 100 | 50 | 86400 |

### HTTP Service Tuning

- Use small TTL values for frequently changing files
- Set appropriate `max_file_size_mb` to prevent large file caching issues
- Monitor cache hit/miss ratios to optimize cache size
- Use Ingress with caching for static content distribution

### Best Practices

1. **Set appropriate cache limits**: Monitor memory usage and adjust `max_size_mb`
2. **Use TTL wisely**: Shorter TTL for dynamic content, longer for static
3. **Filter efficiently**: Use exclude patterns to avoid unnecessary processing
4. **Monitor statistics**: Track hit/miss ratios for optimization
5. **Handle errors gracefully**: Implement retry logic for file operations