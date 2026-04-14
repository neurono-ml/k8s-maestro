use std::sync::Arc;

use kube::Client as KubeClient;
use kube::Config;

pub struct MaestroK8sClient {
    client: Arc<KubeClient>,
}

impl MaestroK8sClient {
    pub async fn new() -> anyhow::Result<Self> {
        let config = Config::infer().await?;
        let client = KubeClient::try_from(config)?;
        
        Ok(Self { client: Arc::new(client) })
    }

    pub fn inner(&self) -> &KubeClient {
        &self.client
    }

    pub fn as_client(&self) -> KubeClient {
        (*self.client).clone()
    }

    pub fn into_inner(self) -> Arc<KubeClient> {
        self.client
    }
}

impl Clone for MaestroK8sClient {
    fn clone(&self) -> Self {
        Self { client: self.client.clone() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires cluster"]
    async fn test_client_creation() {
        let result = MaestroK8sClient::new().await;
        assert!(result.is_ok());
    }
}
