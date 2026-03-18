use kube::Client as KubeClient;
use kube::Config;

pub struct MaestroK8sClient {
    client: KubeClient,
}

impl MaestroK8sClient {
    pub async fn new() -> anyhow::Result<Self> {
        let config = Config::infer().await?;
        let client = KubeClient::try_from(config)?;
        Ok(Self { client })
    }

    pub fn inner(&self) -> &KubeClient {
        &self.client
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
