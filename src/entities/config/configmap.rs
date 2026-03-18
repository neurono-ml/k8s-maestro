use anyhow::Result;
use k8s_openapi::{
    api::core::v1::ConfigMap, apimachinery::pkg::apis::meta::v1::ObjectMeta, ByteString,
};
use std::collections::BTreeMap;

/// Builder for creating Kubernetes ConfigMaps.
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::entities::config::ConfigMapBuilder;
/// use std::collections::BTreeMap;
///
/// let configmap = ConfigMapBuilder::new("my-config")
///     .with_namespace("production")
///     .with_data("config.yaml", "key: value")
///     .with_immutable(true)
///     .build()
///     .unwrap();
/// ```
pub struct ConfigMapBuilder {
    name: String,
    namespace: Option<String>,
    data: BTreeMap<String, String>,
    binary_data: BTreeMap<String, Vec<u8>>,
    labels: Option<BTreeMap<String, String>>,
    annotations: Option<BTreeMap<String, String>>,
    immutable: Option<bool>,
}

impl ConfigMapBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: None,
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
            labels: None,
            annotations: None,
            immutable: None,
        }
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    pub fn with_binary_data(mut self, key: impl Into<String>, bytes: Vec<u8>) -> Self {
        self.binary_data.insert(key.into(), bytes);
        self
    }

    pub fn with_labels(mut self, labels: BTreeMap<String, String>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn with_annotations(mut self, annotations: BTreeMap<String, String>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn with_immutable(mut self, immutable: bool) -> Self {
        self.immutable = Some(immutable);
        self
    }

    pub fn build(self) -> Result<ConfigMap> {
        let metadata = ObjectMeta {
            name: Some(self.name),
            namespace: self.namespace,
            labels: self.labels,
            annotations: self.annotations,
            ..Default::default()
        };

        let binary_data: Option<BTreeMap<String, ByteString>> = if self.binary_data.is_empty() {
            None
        } else {
            Some(
                self.binary_data
                    .into_iter()
                    .map(|(k, v)| (k, ByteString(v)))
                    .collect(),
            )
        };

        #[allow(clippy::needless_update)]
        Ok(ConfigMap {
            metadata,
            data: if self.data.is_empty() {
                None
            } else {
                Some(self.data)
            },
            binary_data,
            immutable: self.immutable,
            ..Default::default()
        })
    }
}
