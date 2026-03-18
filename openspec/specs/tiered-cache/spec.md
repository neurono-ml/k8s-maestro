## ADDED Requirements

### Requirement: Two-tier cache architecture
The system SHALL implement a two-tier cache with memory (L1) and disk (L2) tiers.

#### Scenario: Memory tier provides fast access
- **WHEN** a file is accessed from cache
- **THEN** the system SHALL first check the memory tier for fastest retrieval

#### Scenario: Disk tier provides persistence
- **WHEN** a file is not in memory tier
- **THEN** the system SHALL check the disk tier backed by PVC

### Requirement: Memory tier configuration
The system SHALL support configurable memory tier limits.

#### Scenario: Memory size limit
- **WHEN** memory tier reaches max_size_mb
- **THEN** the system SHALL spill oldest files to disk tier

#### Scenario: Memory file count limit
- **WHEN** memory tier reaches max_files count
- **THEN** the system SHALL spill oldest files to disk tier

#### Scenario: Memory TTL expiration
- **WHEN** a file in memory tier exceeds its TTL
- **THEN** the system SHALL evict the file on next access

### Requirement: Disk tier configuration
The system SHALL support configurable disk tier with PVC backing.

#### Scenario: PVC creation for persistence
- **WHEN** disk tier is enabled with PVC configuration
- **THEN** the system SHALL create or use the specified PVC for file storage

#### Scenario: Disk size limit
- **WHEN** disk tier reaches max_size_mb
- **THEN** the system SHALL evict files based on spill policy

#### Scenario: Disk file count limit
- **WHEN** disk tier reaches max_files count
- **THEN** the system SHALL evict files based on spill policy

#### Scenario: Disk TTL expiration
- **WHEN** a file on disk tier exceeds its TTL
- **THEN** the system SHALL delete the file on next cleanup

### Requirement: Spill policy
The system SHALL support configurable eviction policies for cache spill.

#### Scenario: LRU eviction
- **WHEN** spill_policy is LRU and tier is full
- **THEN** the system SHALL evict the least recently used file

#### Scenario: FIFO eviction
- **WHEN** spill_policy is FIFO and tier is full
- **THEN** the system SHALL evict the oldest file by insertion time

### Requirement: Cache operations
The system SHALL provide standard cache operations.

#### Scenario: Get file from cache
- **WHEN** get(path) is called
- **THEN** the system SHALL return Option<FileContent> checking memory then disk

#### Scenario: Put file to cache
- **WHEN** put(path, content) is called
- **THEN** the system SHALL store in memory tier, spilling to disk if needed

#### Scenario: Delete file from cache
- **WHEN** delete(path) is called
- **THEN** the system SHALL remove from both memory and disk tiers

#### Scenario: List all cached files
- **WHEN** list() is called
- **THEN** the system SHALL return Vec<FileMetadata> for all files in both tiers

#### Scenario: Evict expired files
- **WHEN** evict_expired() is called
- **THEN** the system SHALL remove all files past TTL and return count of evicted files

### Requirement: Cache statistics
The system SHALL provide cache usage statistics.

#### Scenario: Stats include tier metrics
- **WHEN** stats() is called
- **THEN** the system SHALL return CacheStats with memory_usage, disk_usage, file_count, hit_rate, and eviction_count
