use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCacheConfig {
    pub max_size_mb: u64,
    pub max_files: usize,
    pub ttl_seconds: u64,
}

impl Default for MemoryCacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 50,
            max_files: 100,
            ttl_seconds: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilter {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub max_file_size_mb: u64,
}

impl Default for FileFilter {
    fn default() -> Self {
        Self {
            include_patterns: vec![],
            exclude_patterns: vec![],
            max_file_size_mb: 100,
        }
    }
}
