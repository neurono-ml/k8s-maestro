use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileEvent {
    Created(FileMetadata),
    Modified {
        filename: String,
        path: String,
        size: u64,
        modified_at: DateTime<Utc>,
    },
    Deleted {
        filename: String,
        path: String,
        deleted_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetadata {
    pub filename: String,
    pub path: String,
    pub mime_type: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileContent {
    pub metadata: FileMetadata,
    pub content: Vec<u8>,
}

impl FileMetadata {
    pub fn new(
        filename: String,
        path: String,
        mime_type: String,
        size: u64,
        created_at: DateTime<Utc>,
        modified_at: DateTime<Utc>,
    ) -> Self {
        Self {
            filename,
            path,
            mime_type,
            size,
            created_at,
            modified_at,
        }
    }
}

impl FileContent {
    pub fn new(metadata: FileMetadata, content: Vec<u8>) -> Self {
        Self { metadata, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_file_event_created() {
        let metadata = FileMetadata::new(
            "test.txt".to_string(),
            "/data/test.txt".to_string(),
            "text/plain".to_string(),
            100,
            Utc::now(),
            Utc::now(),
        );
        let event = FileEvent::Created(metadata.clone());
        assert!(matches!(event, FileEvent::Created(_)));
    }

    #[test]
    fn test_file_metadata_new() {
        let metadata = FileMetadata::new(
            "test.txt".to_string(),
            "/data/test.txt".to_string(),
            "text/plain".to_string(),
            100,
            Utc::now(),
            Utc::now(),
        );
        assert_eq!(metadata.filename, "test.txt");
        assert_eq!(metadata.size, 100);
    }

    #[test]
    fn test_file_content_new() {
        let metadata = FileMetadata::new(
            "test.txt".to_string(),
            "/data/test.txt".to_string(),
            "text/plain".to_string(),
            100,
            Utc::now(),
            Utc::now(),
        );
        let content = FileContent::new(metadata.clone(), b"test data".to_vec());
        assert_eq!(content.metadata.filename, "test.txt");
        assert_eq!(content.content, b"test data");
    }
}
