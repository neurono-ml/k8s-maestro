use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use k8s_openapi::{
    api::core::v1::Secret, apimachinery::pkg::apis::meta::v1::ObjectMeta, ByteString,
};
use serde_json::json;
use std::collections::BTreeMap;

/// Builder for creating Kubernetes docker-registry secrets (ImagePullSecrets).
///
/// This builder creates secrets of type `kubernetes.io/dockerconfigjson` for
/// authenticating with container image registries.
///
/// # Example
///
/// ```no_run
/// use k8s_maestro::entities::config::ImagePullSecretBuilder;
///
/// let secret = ImagePullSecretBuilder::new("my-registry-secret")
///     .with_registry("https://index.docker.io/v1/")
///     .with_username("myuser")
///     .with_password("mypassword")
///     .with_email("user@example.com")
///     .build()
///     .unwrap();
/// ```
pub struct ImagePullSecretBuilder {
    name: String,
    namespace: Option<String>,
    registry: Option<String>,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
}

impl ImagePullSecretBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: None,
            registry: None,
            username: None,
            password: None,
            email: None,
        }
    }

    pub fn with_registry(mut self, registry: impl Into<String>) -> Self {
        self.registry = Some(registry.into());
        self
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn build(self) -> Result<Secret> {
        let registry = self
            .registry
            .ok_or_else(|| anyhow::anyhow!("registry is required"))?;
        let username = self
            .username
            .ok_or_else(|| anyhow::anyhow!("username is required"))?;
        let password = self
            .password
            .ok_or_else(|| anyhow::anyhow!("password is required"))?;

        let auth = format!("{}:{}", username, password);
        let auth_encoded = STANDARD.encode(auth);

        let docker_config = json!({
            "auths": {
                registry: {
                    "username": username,
                    "password": password,
                    "email": self.email.unwrap_or_default(),
                    "auth": auth_encoded
                }
            }
        });

        let docker_config_json = docker_config.to_string();
        let docker_config_encoded = STANDARD.encode(docker_config_json);

        let metadata = ObjectMeta {
            name: Some(self.name),
            namespace: self.namespace,
            ..Default::default()
        };

        let mut data = BTreeMap::new();
        data.insert(
            ".dockerconfigjson".to_string(),
            ByteString(docker_config_encoded.into_bytes()),
        );

        Ok(Secret {
            metadata,
            type_: Some("kubernetes.io/dockerconfigjson".to_string()),
            data: Some(data),
            ..Default::default()
        })
    }
}
