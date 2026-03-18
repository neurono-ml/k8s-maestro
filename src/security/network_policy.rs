use anyhow::{Context, Result};
use k8s_openapi::api::networking::v1::{
    NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyPeer,
    NetworkPolicyPort,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum PolicyType {
    Ingress,
    Egress,
    Both,
}

impl From<PolicyType> for String {
    fn from(pt: PolicyType) -> Self {
        match pt {
            PolicyType::Ingress => "Ingress".to_string(),
            PolicyType::Egress => "Egress".to_string(),
            PolicyType::Both => "Both".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NetworkPolicyRule {
    pub namespace_selector: Option<LabelSelector>,
    pub pod_selector: Option<LabelSelector>,
    pub port: Option<NetworkPolicyPort>,
}

impl NetworkPolicyRule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_namespace_selector(mut self, selector: LabelSelector) -> Self {
        self.namespace_selector = Some(selector);
        self
    }

    pub fn with_pod_selector(mut self, selector: LabelSelector) -> Self {
        self.pod_selector = Some(selector);
        self
    }

    pub fn with_port(mut self, port: NetworkPolicyPort) -> Self {
        self.port = Some(port);
        self
    }

    fn to_peers(&self) -> Vec<NetworkPolicyPeer> {
        let mut peers = Vec::new();

        if self.namespace_selector.is_some() || self.pod_selector.is_some() {
            let peer = NetworkPolicyPeer {
                ip_block: None,
                namespace_selector: self.namespace_selector.clone(),
                pod_selector: self.pod_selector.clone(),
            };
            peers.push(peer);
        }

        peers
    }
}

pub struct NetworkPolicyBuilder {
    name: String,
    namespace: String,
    pod_selector: Option<LabelSelector>,
    ingress_rules: Vec<NetworkPolicyIngressRule>,
    egress_rules: Vec<NetworkPolicyEgressRule>,
    policy_types: Vec<PolicyType>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl NetworkPolicyBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            pod_selector: None,
            ingress_rules: Vec::new(),
            egress_rules: Vec::new(),
            policy_types: vec![PolicyType::Both],
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_pod_selector(mut self, selector: LabelSelector) -> Self {
        self.pod_selector = Some(selector);
        self
    }

    pub fn with_ingress_rule(mut self, rule: NetworkPolicyRule) -> Self {
        let peers = rule.to_peers();
        let ingress_rule = NetworkPolicyIngressRule {
            from: if peers.is_empty() { None } else { Some(peers) },
            ports: if rule.port.is_some() {
                Some(vec![rule.port.unwrap()])
            } else {
                None
            },
        };
        self.ingress_rules.push(ingress_rule);
        self
    }

    pub fn with_egress_rule(mut self, rule: NetworkPolicyRule) -> Self {
        let peers = rule.to_peers();
        let egress_rule = NetworkPolicyEgressRule {
            to: if peers.is_empty() { None } else { Some(peers) },
            ports: if rule.port.is_some() {
                Some(vec![rule.port.unwrap()])
            } else {
                None
            },
        };
        self.egress_rules.push(egress_rule);
        self
    }

    pub fn with_policy_types(mut self, types: Vec<PolicyType>) -> Self {
        self.policy_types = types;
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

    pub fn build(self) -> Result<NetworkPolicy> {
        let policy_types: Vec<String> =
            self.policy_types.iter().map(|t| t.clone().into()).collect();

        Ok(NetworkPolicy {
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
            spec: Some(k8s_openapi::api::networking::v1::NetworkPolicySpec {
                pod_selector: self.pod_selector.unwrap_or_default(),
                policy_types: Some(policy_types),
                ingress: if self.ingress_rules.is_empty() {
                    None
                } else {
                    Some(self.ingress_rules)
                },
                egress: if self.egress_rules.is_empty() {
                    None
                } else {
                    Some(self.egress_rules)
                },
            }),
        })
    }

    pub fn deny_all(name: &str, namespace: &str) -> Result<Self> {
        Ok(Self::new(name, namespace).with_pod_selector(LabelSelector {
            match_labels: None,
            match_expressions: None,
        }))
    }

    pub fn allow_all(name: &str, namespace: &str) -> Result<Self> {
        Ok(Self::new(name, namespace))
    }

    pub fn allow_within_namespace(name: &str, namespace: &str) -> Result<Self> {
        let rule = NetworkPolicyRule::new().with_pod_selector(LabelSelector {
            match_labels: Some(
                [(
                    "kubernetes.io/metadata.name".to_string(),
                    namespace.to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
            ),
            match_expressions: None,
        });

        Ok(Self::new(name, namespace)
            .with_pod_selector(LabelSelector {
                match_labels: None,
                match_expressions: None,
            })
            .with_ingress_rule(rule.clone())
            .with_egress_rule(rule))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_policy_builder_basic() {
        let policy = NetworkPolicyBuilder::new("test-policy", "default")
            .build()
            .expect("Failed to build policy");

        assert_eq!(policy.metadata.name, Some("test-policy".to_string()));
        assert_eq!(policy.metadata.namespace, Some("default".to_string()));
    }

    #[test]
    fn test_network_policy_with_pod_selector() {
        let selector = LabelSelector {
            match_labels: Some(
                [("app".to_string(), "web".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            match_expressions: None,
        };

        let policy = NetworkPolicyBuilder::new("test-policy", "default")
            .with_pod_selector(selector)
            .build()
            .expect("Failed to build policy");

        assert!(policy.spec.is_some());
        assert_eq!(
            policy
                .spec
                .unwrap()
                .pod_selector
                .match_labels
                .unwrap()
                .get("app"),
            Some(&"web".to_string())
        );
    }

    #[test]
    fn test_network_policy_with_ingress_rule() {
        let rule = NetworkPolicyRule::new().with_pod_selector(LabelSelector {
            match_labels: Some(
                [("app".to_string(), "api".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            match_expressions: None,
        });

        let policy = NetworkPolicyBuilder::new("test-policy", "default")
            .with_ingress_rule(rule)
            .build()
            .expect("Failed to build policy");

        assert!(policy.spec.is_some());
        let spec = policy.spec.unwrap();
        assert!(spec.ingress.is_some());
        assert_eq!(spec.ingress.unwrap().len(), 1);
    }

    #[test]
    fn test_deny_all_preset() {
        let policy = NetworkPolicyBuilder::deny_all("deny-all", "default")
            .expect("Failed to create deny-all preset")
            .build()
            .expect("Failed to build policy");

        assert_eq!(policy.metadata.name, Some("deny-all".to_string()));
        assert!(policy.spec.is_some());
        let spec = policy.spec.unwrap();
        assert!(spec.ingress.is_none());
        assert!(spec.egress.is_none());
    }

    #[test]
    fn test_allow_all_preset() {
        let policy = NetworkPolicyBuilder::allow_all("allow-all", "default")
            .expect("Failed to create allow-all preset")
            .build()
            .expect("Failed to build policy");

        assert_eq!(policy.metadata.name, Some("allow-all".to_string()));
        assert!(policy.spec.is_some());
        let spec = policy.spec.unwrap();
        assert!(spec.ingress.is_none());
        assert!(spec.egress.is_none());
    }

    #[test]
    fn test_allow_within_namespace_preset() {
        let policy = NetworkPolicyBuilder::allow_within_namespace("allow-ns", "production")
            .expect("Failed to create allow-ns preset")
            .build()
            .expect("Failed to build policy");

        assert_eq!(policy.metadata.name, Some("allow-ns".to_string()));
        assert!(policy.spec.is_some());
        let spec = policy.spec.unwrap();
        assert!(spec.ingress.is_some());
        assert!(spec.egress.is_some());
    }
}
