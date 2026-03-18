use anyhow::Result;
use k8s_openapi::api::core::v1::{LimitRange, LimitRangeItem, LimitRangeSpec};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LimitRangeType {
    Container,
    Pod,
    PersistentVolumeClaim,
}

impl From<LimitRangeType> for String {
    fn from(lrt: LimitRangeType) -> Self {
        match lrt {
            LimitRangeType::Container => "Container".to_string(),
            LimitRangeType::Pod => "Pod".to_string(),
            LimitRangeType::PersistentVolumeClaim => "PersistentVolumeClaim".to_string(),
        }
    }
}

pub struct LimitRangeBuilder {
    name: String,
    namespace: String,
    limits: Vec<LimitRangeItem>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl LimitRangeBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            limits: Vec::new(),
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_limit(mut self, limit: LimitRangeItem) -> Self {
        self.limits.push(limit);
        self
    }

    pub fn with_limits(mut self, limits: Vec<LimitRangeItem>) -> Self {
        self.limits = limits;
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

    pub fn build(self) -> Result<LimitRange> {
        Ok(LimitRange {
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
            spec: if self.limits.is_empty() {
                None
            } else {
                Some(LimitRangeSpec {
                    limits: self.limits,
                })
            },
        })
    }
}

pub struct LimitRangeItemBuilder {
    limit_type: LimitRangeType,
    max: Option<BTreeMap<String, Quantity>>,
    min: Option<BTreeMap<String, Quantity>>,
    default: Option<BTreeMap<String, Quantity>>,
    default_request: Option<BTreeMap<String, Quantity>>,
    max_limit_request_ratio: Option<BTreeMap<String, String>>,
}

impl LimitRangeItemBuilder {
    pub fn new(limit_type: LimitRangeType) -> Self {
        Self {
            limit_type,
            max: None,
            min: None,
            default: None,
            default_request: None,
            max_limit_request_ratio: None,
        }
    }

    pub fn with_max(mut self, max: BTreeMap<String, Quantity>) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_max_value(mut self, key: &str, value: &str) -> Self {
        if self.max.is_none() {
            self.max = Some(BTreeMap::new());
        }
        self.max
            .as_mut()
            .unwrap()
            .insert(key.to_string(), Quantity(value.to_string()));
        self
    }

    pub fn with_min(mut self, min: BTreeMap<String, Quantity>) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_min_value(mut self, key: &str, value: &str) -> Self {
        if self.min.is_none() {
            self.min = Some(BTreeMap::new());
        }
        self.min
            .as_mut()
            .unwrap()
            .insert(key.to_string(), Quantity(value.to_string()));
        self
    }

    pub fn with_default(mut self, default: BTreeMap<String, Quantity>) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_default_value(mut self, key: &str, value: &str) -> Self {
        if self.default.is_none() {
            self.default = Some(BTreeMap::new());
        }
        self.default
            .as_mut()
            .unwrap()
            .insert(key.to_string(), Quantity(value.to_string()));
        self
    }

    pub fn with_default_request(mut self, default_request: BTreeMap<String, Quantity>) -> Self {
        self.default_request = Some(default_request);
        self
    }

    pub fn with_default_request_value(mut self, key: &str, value: &str) -> Self {
        if self.default_request.is_none() {
            self.default_request = Some(BTreeMap::new());
        }
        self.default_request
            .as_mut()
            .unwrap()
            .insert(key.to_string(), Quantity(value.to_string()));
        self
    }

    pub fn with_max_limit_request_ratio(mut self, ratio: BTreeMap<String, String>) -> Self {
        self.max_limit_request_ratio = Some(ratio);
        self
    }

    pub fn build(self) -> LimitRangeItem {
        LimitRangeItem {
            type_: self.limit_type.into(),
            max: self.max,
            min: self.min,
            default: self.default,
            default_request: self.default_request,
            max_limit_request_ratio: self.max_limit_request_ratio.map(|m| {
                m.iter()
                    .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                    .collect()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_range_builder_basic() {
        let limit_range = LimitRangeBuilder::new("test-limits", "default")
            .build()
            .expect("Failed to build limit range");

        assert_eq!(limit_range.metadata.name, Some("test-limits".to_string()));
        assert_eq!(limit_range.metadata.namespace, Some("default".to_string()));
    }

    #[test]
    fn test_limit_range_item_builder_container() {
        let item = LimitRangeItemBuilder::new(LimitRangeType::Container)
            .with_default_value("cpu", "500m")
            .with_default_value("memory", "512Mi")
            .with_max_value("cpu", "2")
            .with_max_value("memory", "4Gi")
            .with_min_value("cpu", "100m")
            .build();

        assert_eq!(item.type_, "Container");
        assert!(item.default.is_some());
        assert!(item.max.is_some());
        assert!(item.min.is_some());
    }

    #[test]
    fn test_limit_range_with_item() {
        let item = LimitRangeItemBuilder::new(LimitRangeType::Container)
            .with_default_value("cpu", "500m")
            .with_max_value("memory", "2Gi")
            .build();

        let limit_range = LimitRangeBuilder::new("container-limits", "default")
            .with_limit(item)
            .build()
            .expect("Failed to build limit range");

        assert!(limit_range.spec.is_some());
        let spec = limit_range.spec.unwrap();
        assert_eq!(spec.limits.len(), 1);
        assert_eq!(spec.limits[0].type_, "Container");
    }

    #[test]
    fn test_limit_range_type_persistent_volume_claim() {
        let item = LimitRangeItemBuilder::new(LimitRangeType::PersistentVolumeClaim)
            .with_min_value("storage", "1Gi")
            .with_max_value("storage", "100Gi")
            .build();

        assert_eq!(item.type_, "PersistentVolumeClaim");
        assert!(item.min.is_some());
        assert!(item.max.is_some());
    }

    #[test]
    fn test_limit_range_type_pod() {
        let item = LimitRangeItemBuilder::new(LimitRangeType::Pod)
            .with_max_value("cpu", "4")
            .with_max_value("memory", "8Gi")
            .build();

        assert_eq!(item.type_, "Pod");
    }

    #[test]
    fn test_multiple_limit_items() {
        let container_item = LimitRangeItemBuilder::new(LimitRangeType::Container)
            .with_default_value("cpu", "500m")
            .with_default_value("memory", "512Mi")
            .build();

        let pvc_item = LimitRangeItemBuilder::new(LimitRangeType::PersistentVolumeClaim)
            .with_min_value("storage", "1Gi")
            .with_max_value("storage", "50Gi")
            .build();

        let limit_range = LimitRangeBuilder::new("multi-limits", "default")
            .with_limit(container_item)
            .with_limit(pvc_item)
            .build()
            .expect("Failed to build limit range");

        assert!(limit_range.spec.is_some());
        let spec = limit_range.spec.unwrap();
        assert_eq!(spec.limits.len(), 2);
    }

    #[test]
    fn test_limit_range_with_default_request() {
        let item = LimitRangeItemBuilder::new(LimitRangeType::Container)
            .with_default_value("cpu", "500m")
            .with_default_value("memory", "512Mi")
            .with_default_request_value("cpu", "100m")
            .with_default_request_value("memory", "256Mi")
            .build();

        assert!(item.default_request.is_some());
        let default_request = item.default_request.unwrap();
        assert_eq!(default_request.get("cpu").unwrap().0, "100m");
        assert_eq!(default_request.get("memory").unwrap().0, "256Mi");
    }
}
