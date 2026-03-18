use anyhow::Result;
use tokio::sync::broadcast;

use crate::steps::observers::{FileContent, FileEvent, FileMetadata, TieredCache};

pub fn get_file_content(_path: &str, _cache: &TieredCache) -> Result<FileContent> {
    todo!("Implement get_file_content")
}

pub fn list_observed_files(_cache: &TieredCache) -> Result<Vec<FileMetadata>> {
    todo!("Implement list_observed_files")
}

pub fn subscribe_to_events() -> Result<broadcast::Receiver<FileEvent>> {
    todo!("Implement subscribe_to_events")
}
