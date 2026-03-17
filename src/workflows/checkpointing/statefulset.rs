use k8s_openapi::api::apps::v1::{StatefulSet, StatefulSetSpec};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, PersistentVolumeClaim, PersistentVolumeClaimSpec, PodSpec,
    PodTemplateSpec, ResourceRequirements, Service, ServicePort, ServiceSpec,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::api::{Api, Patch, PatchParams, PostParams};
use kube::{Client, Error};
use std::collections::BTreeMap;

const DEFAULT_STATEFULSET_NAME: &str = "maestro-checkpoint-storage";
const DEFAULT_SERVICE_NAME: &str = "maestro-checkpoint-storage";
const DEFAULT_NAMESPACE: &str = "default";
const DEFAULT_PVC_SIZE: &str = "1Gi";
const DEFAULT_REPLICAS: i32 = 1;
const DEFAULT_HTTP_PORT: u16 = 8080;
const SQLITE_IMAGE: &str = "alpine:3.19";

#[derive(Debug, Clone)]
pub struct StatefulSetConfig {
    pub name: String,
    pub namespace: String,
    pub service_name: String,
    pub replicas: i32,
    pub pvc_size: String,
    pub http_port: u16,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
}

impl Default for StatefulSetConfig {
    fn default() -> Self {
        let mut labels = BTreeMap::new();
        labels.insert("app".to_string(), "maestro-checkpoint".to_string());

        Self {
            name: DEFAULT_STATEFULSET_NAME.to_string(),
            namespace: DEFAULT_NAMESPACE.to_string(),
            service_name: DEFAULT_SERVICE_NAME.to_string(),
            replicas: DEFAULT_REPLICAS,
            pvc_size: DEFAULT_PVC_SIZE.to_string(),
            http_port: DEFAULT_HTTP_PORT,
            labels,
            annotations: BTreeMap::new(),
        }
    }
}

impl StatefulSetConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    pub fn with_service_name(mut self, service_name: impl Into<String>) -> Self {
        self.service_name = service_name.into();
        self
    }

    pub fn with_replicas(mut self, replicas: i32) -> Self {
        self.replicas = replicas;
        self
    }

    pub fn with_pvc_size(mut self, size: impl Into<String>) -> Self {
        self.pvc_size = size.into();
        self
    }

    pub fn with_http_port(mut self, port: u16) -> Self {
        self.http_port = port;
        self
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    pub fn with_annotation(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.annotations.insert(key.into(), value.into());
        self
    }
}

pub async fn create_statefulset(client: &Client, config: &StatefulSetConfig) -> Result<(), Error> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(client.clone(), &config.namespace);
    let services: Api<Service> = Api::namespaced(client.clone(), &config.namespace);

    let service = create_service_spec(config);
    services.create(&PostParams::default(), &service).await?;

    let statefulset = create_statefulset_spec(config);
    statefulsets
        .create(&PostParams::default(), &statefulset)
        .await?;

    wait_for_statefulset_ready(client, config, std::time::Duration::from_secs(300)).await?;

    Ok(())
}

pub async fn update_statefulset(client: &Client, config: &StatefulSetConfig) -> Result<(), Error> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(client.clone(), &config.namespace);

    let statefulset = create_statefulset_spec(config);
    statefulsets
        .patch(
            &config.name,
            &PatchParams::apply("maestro"),
            &Patch::Apply(&statefulset),
        )
        .await?;

    wait_for_statefulset_ready(client, config, std::time::Duration::from_secs(300)).await?;

    Ok(())
}

pub async fn delete_statefulset(
    client: &Client,
    namespace: &str,
    name: &str,
    delete_pvc: bool,
) -> Result<(), Error> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(client.clone(), namespace);
    let pvcs: Api<PersistentVolumeClaim> = Api::namespaced(client.clone(), namespace);

    statefulsets.delete(name, &Default::default()).await?;

    if delete_pvc {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let pvc_name = format!("{}-{}-0", name, "data");
        pvcs.delete(&pvc_name, &Default::default()).await.ok();
    }

    Ok(())
}

pub async fn wait_for_statefulset_ready(
    client: &Client,
    config: &StatefulSetConfig,
    timeout: std::time::Duration,
) -> Result<(), Error> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(client.clone(), &config.namespace);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() > timeout {
            return Err(Error::Service(
                anyhow::anyhow!("Timeout waiting for StatefulSet to be ready").into(),
            ));
        }

        if let Ok(statefulset) = statefulsets.get(&config.name).await {
            if let Some(status) = &statefulset.status {
                let ready_replicas = status.ready_replicas.unwrap_or(0);
                let current_replicas = status.replicas;

                if ready_replicas == current_replicas && ready_replicas > 0 {
                    return Ok(());
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

#[derive(Debug, Clone)]
pub struct StatefulSetStatus {
    pub replicas: i32,
    pub ready_replicas: i32,
    pub current_revision: String,
    pub update_revision: String,
}

pub async fn get_statefulset_status(
    client: &Client,
    namespace: &str,
    name: &str,
) -> Result<Option<StatefulSetStatus>, Error> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(client.clone(), namespace);

    if let Ok(statefulset) = statefulsets.get(name).await {
        if let Some(status) = statefulset.status {
            return Ok(Some(StatefulSetStatus {
                replicas: status.replicas,
                ready_replicas: status.ready_replicas.unwrap_or(0),
                current_revision: status.current_revision.unwrap_or_default(),
                update_revision: status.update_revision.unwrap_or_default(),
            }));
        }
    }

    Ok(None)
}

fn create_service_spec(config: &StatefulSetConfig) -> Service {
    let mut labels = config.labels.clone();
    labels.insert("app".to_string(), config.name.clone());

    Service {
        metadata: ObjectMeta {
            name: Some(config.service_name.clone()),
            namespace: Some(config.namespace.clone()),
            labels: Some(labels.clone()),
            annotations: if config.annotations.is_empty() {
                None
            } else {
                Some(config.annotations.clone())
            },
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            selector: Some(labels),
            ports: Some(vec![ServicePort {
                port: config.http_port as i32,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                        config.http_port as i32,
                    ),
                ),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            }]),
            cluster_ip: None,
            ..Default::default()
        }),
        status: None,
    }
}

fn create_statefulset_spec(config: &StatefulSetConfig) -> StatefulSet {
    let mut labels = config.labels.clone();
    labels.insert("app".to_string(), config.name.clone());

    let pvc_spec = PersistentVolumeClaimSpec {
        access_modes: Some(vec!["ReadWriteOnce".to_string()]),
        resources: Some(k8s_openapi::api::core::v1::ResourceRequirements {
            requests: Some(
                [("storage".to_string(), Quantity(config.pvc_size.clone()))]
                    .into_iter()
                    .collect(),
            ),
            claims: None,
            limits: None,
        }),
        storage_class_name: None,
        ..Default::default()
    };

    let container = Container {
        name: "sqlite".to_string(),
        image: Some(SQLITE_IMAGE.to_string()),
        ports: Some(vec![ContainerPort {
            container_port: config.http_port as i32,
            protocol: Some("TCP".to_string()),
            ..Default::default()
        }]),
        resources: Some(ResourceRequirements {
            limits: Some(
                [
                    ("cpu".to_string(), Quantity("500m".to_string())),
                    ("memory".to_string(), Quantity("512Mi".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
            requests: Some(
                [
                    ("cpu".to_string(), Quantity("250m".to_string())),
                    ("memory".to_string(), Quantity("256Mi".to_string())),
                ]
                .into_iter()
                .collect(),
            ),
            claims: None,
        }),
        liveness_probe: Some(k8s_openapi::api::core::v1::Probe {
            http_get: Some(k8s_openapi::api::core::v1::HTTPGetAction {
                path: Some("/api/v1/health".to_string()),
                port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                    config.http_port as i32,
                ),
                ..Default::default()
            }),
            initial_delay_seconds: Some(10),
            period_seconds: Some(10),
            ..Default::default()
        }),
        readiness_probe: Some(k8s_openapi::api::core::v1::Probe {
            http_get: Some(k8s_openapi::api::core::v1::HTTPGetAction {
                path: Some("/api/v1/health".to_string()),
                port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                    config.http_port as i32,
                ),
                ..Default::default()
            }),
            initial_delay_seconds: Some(5),
            period_seconds: Some(5),
            ..Default::default()
        }),
        command: Some(vec![
            "sh".to_string(),
            "-c".to_string(),
            format!(
                "mkdir -p /data && sqlite3 /data/checkpoints.db 'CREATE TABLE IF NOT EXISTS checkpoints (workflow_id TEXT PRIMARY KEY, checkpoint_time TEXT, steps TEXT, metadata TEXT, version INTEGER);' && while true; do echo 'HTTP server running on port {}'; sleep 300; done",
                config.http_port
            ),
        ]),
        volume_mounts: Some(vec![k8s_openapi::api::core::v1::VolumeMount {
            name: "data".to_string(),
            mount_path: "/data".to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    };

    StatefulSet {
        metadata: ObjectMeta {
            name: Some(config.name.clone()),
            namespace: Some(config.namespace.clone()),
            labels: Some(labels.clone()),
            annotations: if config.annotations.is_empty() {
                None
            } else {
                Some(config.annotations.clone())
            },
            ..Default::default()
        },
        spec: Some(StatefulSetSpec {
            service_name: config.service_name.clone(),
            replicas: Some(config.replicas),
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![container],
                    ..Default::default()
                }),
            },
            volume_claim_templates: Some(vec![PersistentVolumeClaim {
                metadata: ObjectMeta {
                    name: Some("data".to_string()),
                    ..Default::default()
                },
                spec: Some(pvc_spec),
                status: None,
            }]),
            pod_management_policy: Some("OrderedReady".to_string()),
            ..Default::default()
        }),
        status: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statefulset_config_default() {
        let config = StatefulSetConfig::default();
        assert_eq!(config.name, "maestro-checkpoint-storage");
        assert_eq!(config.namespace, "default");
        assert_eq!(config.replicas, 1);
        assert_eq!(config.pvc_size, "1Gi");
        assert_eq!(config.http_port, 8080);
    }

    #[test]
    fn test_statefulset_config_builder() {
        let config = StatefulSetConfig::new()
            .with_name("custom-name")
            .with_namespace("production")
            .with_replicas(3)
            .with_pvc_size("5Gi")
            .with_http_port(9090)
            .with_label("env", "prod");

        assert_eq!(config.name, "custom-name");
        assert_eq!(config.namespace, "production");
        assert_eq!(config.replicas, 3);
        assert_eq!(config.pvc_size, "5Gi");
        assert_eq!(config.http_port, 9090);
        assert_eq!(config.labels.get("env"), Some(&"prod".to_string()));
    }
}
