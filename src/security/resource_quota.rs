use anyhow::Result;
use k8s_openapi::api::core::v1::ResourceQuota;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum QuotaScope {
    Terminating,
    NotTerminating,
    BestEffort,
    NotBestEffort,
}

impl From<QuotaScope> for String {
    fn from(qs: QuotaScope) -> Self {
        match qs {
            QuotaScope::Terminating => "Terminating".to_string(),
            QuotaScope::NotTerminating => "NotTerminating".to_string(),
            QuotaScope::BestEffort => "BestEffort".to_string(),
            QuotaScope::NotBestEffort => "NotBestEffort".to_string(),
        }
    }
}

pub struct ResourceQuotaBuilder {
    name: String,
    namespace: String,
    hard_limits: BTreeMap<String, Quantity>,
    scopes: Vec<QuotaScope>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
    status: Option<String>,
}

impl ResourceQuotaBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            hard_limits: BTreeMap::new(),
            scopes: Vec::new(),
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
            status: None,
        }
    }

    pub fn with_hard_limits(mut self, limits: BTreeMap<String, Quantity>) -> Self {
        self.hard_limits = limits;
        self
    }

    pub fn with_hard_limit(mut self, key: &str, value: &str) -> Self {
        self.hard_limits
            .insert(key.to_string(), Quantity(value.to_string()));
        self
    }

    pub fn with_scopes(mut self, scopes: Vec<QuotaScope>) -> Self {
        self.scopes = scopes;
        self
    }

    pub fn with_scope(mut self, scope: QuotaScope) -> Self {
        self.scopes.push(scope);
        self
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_annotation(mut self, key: &str, value: &str) -> Self {
        self.annotations.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> Result<ResourceQuota> {
        let scopes: Vec<String> = self.scopes.iter().map(|s| s.clone().into()).collect();

        Ok(ResourceQuota {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(self.name),
                namespace: Some(self.namespace),
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                ..Default::default()
            },
            spec: Some(k8s_openapi::api::core::v1::ResourceQuotaSpec {
                hard: if self.hard_limits.is_empty() {
                    None
                } else {
                    Some(self.hard_limits)
                },
                scopes: if scopes.is_empty() {
                    None
                } else {
                    Some(scopes)
                },
                ..Default::default()
            }),
            status: None,
        })
    }

    pub fn small_workload(name: &str, namespace: &str) -> Result<Self> {
        let limits: BTreeMap<String, Quantity> = [
            ("requests.cpu".to_string(), Quantity("2".to_string())),
            ("requests.memory".to_string(), Quantity("4Gi".to_string())),
            ("limits.cpu".to_string(), Quantity("4".to_string())),
            ("limits.memory".to_string(), Quantity("8Gi".to_string())),
            ("count/pods".to_string(), Quantity("10".to_string())),
            (
                "persistentvolumeclaims".to_string(),
                Quantity("5".to_string()),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        Ok(Self::new(name, namespace).with_hard_limits(limits))
    }

    pub fn medium_workload(name: &str, namespace: &str) -> Result<Self> {
        let limits: BTreeMap<String, Quantity> = [
            ("requests.cpu".to_string(), Quantity("10".to_string())),
            ("requests.memory".to_string(), Quantity("20Gi".to_string())),
            ("limits.cpu".to_string(), Quantity("20".to_string())),
            ("limits.memory".to_string(), Quantity("40Gi".to_string())),
            ("count/pods".to_string(), Quantity("50".to_string())),
            (
                "persistentvolumeclaims".to_string(),
                Quantity("20".to_string()),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        Ok(Self::new(name, namespace).with_hard_limits(limits))
    }

    pub fn large_workload(name: &str, namespace: &str) -> Result<Self> {
        let limits: BTreeMap<String, Quantity> = [
            ("requests.cpu".to_string(), Quantity("50".to_string())),
            ("requests.memory".to_string(), Quantity("100Gi".to_string())),
            ("limits.cpu".to_string(), Quantity("100".to_string())),
            ("limits.memory".to_string(), Quantity("200Gi".to_string())),
            ("count/pods".to_string(), Quantity("200".to_string())),
            (
                "persistentvolumeclaims".to_string(),
                Quantity("100".to_string()),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        Ok(Self::new(name, namespace).with_hard_limits(limits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_quota_builder_basic() {
        let quota = ResourceQuotaBuilder::new("test-quota", "default")
            .build()
            .expect("Failed to build quota");

        assert_eq!(quota.metadata.name, Some("test-quota".to_string()));
        assert_eq!(quota.metadata.namespace, Some("default".to_string()));
    }

    #[test]
    fn test_resource_quota_with_hard_limits() {
        let mut limits = BTreeMap::new();
        limits.insert("requests.cpu".to_string(), Quantity("4".to_string()));
        limits.insert("limits.memory".to_string(), Quantity("16Gi".to_string()));

        let quota = ResourceQuotaBuilder::new("test-quota", "default")
            .with_hard_limits(limits)
            .build()
            .expect("Failed to build quota");

        assert!(quota.spec.is_some());
        let spec = quota.spec.unwrap();
        assert!(spec.hard.is_some());
        let hard = spec.hard.unwrap();
        assert_eq!(hard.get("requests.cpu").unwrap().0, "4");
        assert_eq!(hard.get("limits.memory").unwrap().0, "16Gi");
    }

    #[test]
    fn test_resource_quota_with_scopes() {
        let quota = ResourceQuotaBuilder::new("test-quota", "default")
            .with_scope(QuotaScope::Terminating)
            .with_scope(QuotaScope::BestEffort)
            .build()
            .expect("Failed to build quota");

        assert!(quota.spec.is_some());
        let spec = quota.spec.unwrap();
        assert!(spec.scopes.is_some());
        let scopes = spec.scopes.unwrap();
        assert!(scopes.contains(&"Terminating".to_string()));
        assert!(scopes.contains(&"BestEffort".to_string()));
    }

    #[test]
    fn test_small_workload_preset() {
        let quota = ResourceQuotaBuilder::small_workload("small", "team-a")
            .expect("Failed to create small workload preset")
            .build()
            .expect("Failed to build quota");

        assert_eq!(quota.metadata.name, Some("small".to_string()));
        assert!(quota.spec.is_some());
        let spec = quota.spec.unwrap();
        assert!(spec.hard.is_some());
        let hard = spec.hard.unwrap();
        assert_eq!(hard.get("requests.cpu").unwrap().0, "2");
        assert_eq!(hard.get("limits.memory").unwrap().0, "8Gi");
    }

    #[test]
    fn test_medium_workload_preset() {
        let quota = ResourceQuotaBuilder::medium_workload("medium", "team-a")
            .expect("Failed to create medium workload preset")
            .build()
            .expect("Failed to build quota");

        assert_eq!(quota.metadata.name, Some("medium".to_string()));
        assert!(quota.spec.is_some());
        let spec = quota.spec.unwrap();
        assert!(spec.hard.is_some());
        let hard = spec.hard.unwrap();
        assert_eq!(hard.get("requests.cpu").unwrap().0, "10");
        assert_eq!(hard.get("limits.memory").unwrap().0, "40Gi");
    }

    #[test]
    fn test_large_workload_preset() {
        let quota = ResourceQuotaBuilder::large_workload("large", "team-a")
            .expect("Failed to create large workload preset")
            .build()
            .expect("Failed to build quota");

        assert_eq!(quota.metadata.name, Some("large".to_string()));
        assert!(quota.spec.is_some());
        let spec = quota.spec.unwrap();
        assert!(spec.hard.is_some());
        let hard = spec.hard.unwrap();
        assert_eq!(hard.get("requests.cpu").unwrap().0, "50");
        assert_eq!(hard.get("limits.memory").unwrap().0, "200Gi");
    }
}
