#[derive(Debug, Clone)]
pub enum K8sObjectVolumeSource {
    ConfigMap(String),
    Secret(String)
}