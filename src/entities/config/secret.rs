use anyhow::Result;
use k8s_openapi::{
    api::core::v1::Secret, apimachinery::pkg::apis::meta::v1::ObjectMeta, ByteString,
};
use std::collections::BTreeMap;

/// Type-safe enum for Kubernetes secret types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretType {
    Opaque,
    ServiceAccountToken,
    Dockercfg,
    DockerConfigJson,
    BasicAuth,
    SshAuth,
    Tls,
    BootstrapToken,
}

impl std::fmt::Display for SecretType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretType::Opaque => write!(f, "Opaque"),
            SecretType::ServiceAccountToken => write!(f, "kubernetes.io/service-account-token"),
            SecretType::Dockercfg => write!(f, "kubernetes.io/dockercfg"),
            SecretType::DockerConfigJson => write!(f, "kubernetes.io/dockerconfigjson"),
            SecretType::BasicAuth => write!(f, "kubernetes.io/basic-auth"),
            SecretType::SshAuth => write!(f, "kubernetes.io/ssh-auth"),
            SecretType::Tls => write!(f, "kubernetes.io/tls"),
            SecretType::BootstrapToken => write!(f, "bootstrap.kubernetes.io/token"),
        }
    }
}

/// Builder for creating Kubernetes Secrets.
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::entities::config::{SecretBuilder, SecretType};
///
/// let secret = SecretBuilder::new("my-secret")
///     .with_namespace("production")
///     .with_type(SecretType::Opaque)
///     .with_string_data("username", "admin")
///     .with_string_data("password", "secret123")
///     .build()
///     .unwrap();
/// ```
pub struct SecretBuilder {
    name: String,
    namespace: Option<String>,
    secret_type: SecretType,
    string_data: BTreeMap<String, String>,
    data: BTreeMap<String, Vec<u8>>,
    labels: Option<BTreeMap<String, String>>,
    annotations: Option<BTreeMap<String, String>>,
    immutable: Option<bool>,
}

impl SecretBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: None,
            secret_type: SecretType::Opaque,
            string_data: BTreeMap::new(),
            data: BTreeMap::new(),
            labels: None,
            annotations: None,
            immutable: None,
        }
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_type(mut self, secret_type: SecretType) -> Self {
        self.secret_type = secret_type;
        self
    }

    pub fn with_string_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.string_data.insert(key.into(), value.into());
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, bytes: Vec<u8>) -> Self {
        self.data.insert(key.into(), bytes);
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

    pub fn build(self) -> Result<Secret> {
        let metadata = ObjectMeta {
            name: Some(self.name),
            namespace: self.namespace,
            labels: self.labels,
            annotations: self.annotations,
            ..Default::default()
        };

        let data: Option<BTreeMap<String, ByteString>> = if self.data.is_empty() {
            None
        } else {
            Some(
                self.data
                    .into_iter()
                    .map(|(k, v)| (k, ByteString(v)))
                    .collect(),
            )
        };

        #[allow(clippy::needless_update)]
        Ok(Secret {
            metadata,
            type_: Some(self.secret_type.to_string()),
            string_data: if self.string_data.is_empty() {
                None
            } else {
                Some(self.string_data)
            },
            data,
            immutable: self.immutable,
            ..Default::default()
        })
    }
}
