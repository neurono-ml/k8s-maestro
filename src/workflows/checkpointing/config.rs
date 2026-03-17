use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckpointFrequency {
    OnStepCompletion,
    OnSuccess,
    Periodic(Duration),
}

impl Default for CheckpointFrequency {
    fn default() -> Self {
        Self::OnStepCompletion
    }
}

impl FromStr for CheckpointFrequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "on_step_completion" | "onstepcompletion" => Ok(Self::OnStepCompletion),
            "on_success" | "onsuccess" => Ok(Self::OnSuccess),
            _ => {
                if let Ok(duration) = parse_duration(s) {
                    Ok(Self::Periodic(duration))
                } else {
                    Err(format!("Invalid checkpoint frequency: {}", s))
                }
            }
        }
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim().to_lowercase();

    if s.ends_with('s') {
        let secs: i64 = s[..s.len() - 1]
            .parse()
            .map_err(|_| format!("Invalid seconds format: {}", s))?;
        Ok(Duration::seconds(secs))
    } else if s.ends_with('m') {
        let mins: i64 = s[..s.len() - 1]
            .parse()
            .map_err(|_| format!("Invalid minutes format: {}", s))?;
        Ok(Duration::minutes(mins))
    } else if s.ends_with('h') {
        let hours: i64 = s[..s.len() - 1]
            .parse()
            .map_err(|_| format!("Invalid hours format: {}", s))?;
        Ok(Duration::hours(hours))
    } else {
        Err(format!(
            "Invalid duration format: {}. Use format like '30s', '5m', '1h'",
            s
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckpointStorageConfig {
    Sqlite { namespace: String, pvc_size: String },
    Etcd { endpoints: Vec<String> },
    Redis { url: String },
    Postgres { connection_string: String },
}

impl Default for CheckpointStorageConfig {
    fn default() -> Self {
        Self::Sqlite {
            namespace: "default".to_string(),
            pvc_size: "1Gi".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetentionPolicy {
    pub max_age: Option<Duration>,
    pub max_count: Option<usize>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_age: Some(Duration::days(7)),
            max_count: Some(10),
        }
    }
}

impl RetentionPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    }

    pub fn with_max_count(mut self, count: usize) -> Self {
        self.max_count = Some(count);
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Some(count) = self.max_count {
            if count == 0 {
                return Err("max_count must be greater than 0".to_string());
            }
        }
        if let Some(duration) = self.max_age {
            if duration.num_seconds() <= 0 {
                return Err("max_age must be greater than 0".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointConfig {
    pub enabled: bool,
    pub frequency: CheckpointFrequency,
    pub storage: CheckpointStorageConfig,
    pub retention_policy: RetentionPolicy,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency: CheckpointFrequency::default(),
            storage: CheckpointStorageConfig::default(),
            retention_policy: RetentionPolicy::default(),
        }
    }
}

impl CheckpointConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_frequency(mut self, frequency: CheckpointFrequency) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn with_storage(mut self, storage: CheckpointStorageConfig) -> Self {
        self.storage = storage;
        self
    }

    pub fn with_retention_policy(mut self, policy: RetentionPolicy) -> Self {
        self.retention_policy = policy;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        self.retention_policy.validate()?;

        if self.enabled {
            match &self.storage {
                CheckpointStorageConfig::Sqlite {
                    namespace,
                    pvc_size,
                } => {
                    if namespace.is_empty() {
                        return Err("namespace cannot be empty for SQLite storage".to_string());
                    }
                    if pvc_size.is_empty() {
                        return Err("pvc_size cannot be empty for SQLite storage".to_string());
                    }
                }
                CheckpointStorageConfig::Etcd { endpoints } => {
                    if endpoints.is_empty() {
                        return Err("endpoints cannot be empty for Etcd storage".to_string());
                    }
                }
                CheckpointStorageConfig::Redis { url } => {
                    if url.is_empty() {
                        return Err("url cannot be empty for Redis storage".to_string());
                    }
                }
                CheckpointStorageConfig::Postgres { connection_string } => {
                    if connection_string.is_empty() {
                        return Err(
                            "connection_string cannot be empty for Postgres storage".to_string()
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn from_env() -> Result<Self, String> {
        let enabled = std::env::var("CHECKPOINTING_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let frequency_str = std::env::var("CHECKPOINTING_FREQUENCY")
            .unwrap_or_else(|_| "on_step_completion".to_string());
        let frequency = CheckpointFrequency::from_str(&frequency_str)?;

        let storage_type =
            std::env::var("CHECKPOINT_STORAGE_TYPE").unwrap_or_else(|_| "sqlite".to_string());
        let storage = match storage_type.to_lowercase().as_str() {
            "sqlite" => CheckpointStorageConfig::Sqlite {
                namespace: std::env::var("CHECKPOINT_SQLITE_NAMESPACE")
                    .unwrap_or_else(|_| "default".to_string()),
                pvc_size: std::env::var("CHECKPOINT_SQLITE_PVC_SIZE")
                    .unwrap_or_else(|_| "1Gi".to_string()),
            },
            "etcd" => CheckpointStorageConfig::Etcd {
                endpoints: std::env::var("CHECKPOINT_ETCD_ENDPOINTS")
                    .unwrap_or_else(|_| "http://localhost:2379".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            "redis" => CheckpointStorageConfig::Redis {
                url: std::env::var("CHECKPOINT_REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            },
            "postgres" => CheckpointStorageConfig::Postgres {
                connection_string: std::env::var("CHECKPOINT_POSTGRES_CONNECTION_STRING")
                    .unwrap_or_else(|_| "postgresql://localhost:5432/checkpoints".to_string()),
            },
            _ => return Err(format!("Unsupported storage type: {}", storage_type)),
        };

        let max_age = std::env::var("CHECKPOINT_RETENTION_MAX_AGE")
            .ok()
            .and_then(|s| parse_duration(&s).ok());

        let max_count = std::env::var("CHECKPOINT_RETENTION_MAX_COUNT")
            .ok()
            .and_then(|s| s.parse().ok());

        let retention_policy = RetentionPolicy { max_age, max_count };

        let config = Self {
            enabled,
            frequency,
            storage,
            retention_policy,
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_frequency_from_str() {
        assert_eq!(
            CheckpointFrequency::from_str("on_step_completion").unwrap(),
            CheckpointFrequency::OnStepCompletion
        );
        assert_eq!(
            CheckpointFrequency::from_str("on_success").unwrap(),
            CheckpointFrequency::OnSuccess
        );
        assert_eq!(
            CheckpointFrequency::from_str("30s").unwrap(),
            CheckpointFrequency::Periodic(Duration::seconds(30))
        );
        assert_eq!(
            CheckpointFrequency::from_str("5m").unwrap(),
            CheckpointFrequency::Periodic(Duration::minutes(5))
        );
        assert_eq!(
            CheckpointFrequency::from_str("1h").unwrap(),
            CheckpointFrequency::Periodic(Duration::hours(1))
        );
    }

    #[test]
    fn test_checkpoint_frequency_invalid() {
        assert!(CheckpointFrequency::from_str("invalid").is_err());
    }

    #[test]
    fn test_retention_policy_default() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.max_age, Some(Duration::days(7)));
        assert_eq!(policy.max_count, Some(10));
    }

    #[test]
    fn test_retention_policy_builder() {
        let policy = RetentionPolicy::new()
            .with_max_age(Duration::days(30))
            .with_max_count(20);
        assert_eq!(policy.max_age, Some(Duration::days(30)));
        assert_eq!(policy.max_count, Some(20));
    }

    #[test]
    fn test_retention_policy_validate() {
        let policy = RetentionPolicy::new();
        assert!(policy.validate().is_ok());

        let invalid_count = RetentionPolicy::new().with_max_count(0);
        assert!(invalid_count.validate().is_err());

        let invalid_age = RetentionPolicy::new().with_max_age(Duration::seconds(-1));
        assert!(invalid_age.validate().is_err());
    }

    #[test]
    fn test_checkpoint_config_default() {
        let config = CheckpointConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.frequency, CheckpointFrequency::OnStepCompletion);
        matches!(config.storage, CheckpointStorageConfig::Sqlite { .. });
    }

    #[test]
    fn test_checkpoint_config_builder() {
        let config = CheckpointConfig::new()
            .enabled(true)
            .with_frequency(CheckpointFrequency::OnSuccess)
            .with_storage(CheckpointStorageConfig::Redis {
                url: "redis://localhost".to_string(),
            });
        assert!(config.enabled);
        assert_eq!(config.frequency, CheckpointFrequency::OnSuccess);
        matches!(config.storage, CheckpointStorageConfig::Redis { .. });
    }

    #[test]
    fn test_checkpoint_config_validate() {
        let config = CheckpointConfig::new();
        assert!(config.validate().is_ok());

        let invalid_config = CheckpointConfig {
            enabled: true,
            storage: CheckpointStorageConfig::Sqlite {
                namespace: "".to_string(),
                pvc_size: "".to_string(),
            },
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_checkpoint_config_serialization() {
        let config = CheckpointConfig::new().enabled(true);
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: CheckpointConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.enabled, deserialized.enabled);
    }
}
