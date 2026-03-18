use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::steps::observers::{FileFilter, MemoryCacheConfig, TieredCache};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObserverModes {
    pub channel: bool,
    pub cache: bool,
    pub http_service: bool,
}

#[derive(Debug, Clone)]
pub struct FileObserverSidecar {
    pub watch_path: String,
    pub modes: ObserverModes,
    pub cache: Option<TieredCache>,
    pub filters: FileFilter,
    pub http_port: Option<u16>,
}

pub struct FileObserverBuilder {
    watch_path: Option<String>,
    modes: ObserverModes,
    cache_config: Option<MemoryCacheConfig>,
    filters: Option<FileFilter>,
    http_port: Option<u16>,
}

impl FileObserverBuilder {
    pub fn new() -> Self {
        Self {
            watch_path: None,
            modes: ObserverModes::default(),
            cache_config: None,
            filters: None,
            http_port: None,
        }
    }

    pub fn watch_path(mut self, path: impl Into<String>) -> Self {
        self.watch_path = Some(path.into());
        self
    }

    pub fn with_channel_mode(mut self, enabled: bool) -> Self {
        self.modes.channel = enabled;
        self
    }

    pub fn with_cache_mode(mut self, enabled: bool, config: Option<MemoryCacheConfig>) -> Self {
        self.modes.cache = enabled;
        self.cache_config = config;
        self
    }

    pub fn with_http_service(mut self, enabled: bool, port: u16) -> Self {
        self.modes.http_service = enabled;
        self.http_port = Some(port);
        self
    }

    pub fn with_filters(mut self, filters: FileFilter) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn build(self) -> Result<FileObserverSidecar> {
        let watch_path = self
            .watch_path
            .ok_or_else(|| anyhow::anyhow!("watch_path is required but not provided"))?;

        if !self.modes.channel && !self.modes.cache && !self.modes.http_service {
            bail!("at least one observer mode must be enabled");
        }

        let cache = if self.modes.cache {
            let config = self.cache_config.unwrap_or_default();
            Some(TieredCache::new(config))
        } else {
            None
        };

        let filters = self.filters.unwrap_or_default();

        Ok(FileObserverSidecar {
            watch_path,
            modes: self.modes,
            cache,
            filters,
            http_port: self.http_port,
        })
    }
}

impl Default for FileObserverBuilder {
    fn default() -> Self {
        Self::new()
    }
}
