use anyhow::Result;
use k8s_openapi::api::core::v1::ServiceAccount;
use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct PolicyRule {
    pub api_groups: Vec<String>,
    pub resources: Vec<String>,
    pub verbs: Vec<String>,
    pub resource_names: Vec<String>,
}

impl PolicyRule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_api_groups(mut self, groups: Vec<String>) -> Self {
        self.api_groups = groups;
        self
    }

    pub fn with_resources(mut self, resources: Vec<String>) -> Self {
        self.resources = resources;
        self
    }

    pub fn with_verbs(mut self, verbs: Vec<String>) -> Self {
        self.verbs = verbs;
        self
    }

    pub fn with_resource_names(mut self, names: Vec<String>) -> Self {
        self.resource_names = names;
        self
    }

    pub fn to_k8s_rule(&self) -> k8s_openapi::api::rbac::v1::PolicyRule {
        k8s_openapi::api::rbac::v1::PolicyRule {
            api_groups: if self.api_groups.is_empty() {
                None
            } else {
                Some(self.api_groups.clone())
            },
            resources: if self.resources.is_empty() {
                None
            } else {
                Some(self.resources.clone())
            },
            verbs: self.verbs.clone(),
            resource_names: if self.resource_names.is_empty() {
                None
            } else {
                Some(self.resource_names.clone())
            },
            ..Default::default()
        }
    }
}

pub struct ServiceAccountBuilder {
    name: String,
    namespace: String,
    annotations: BTreeMap<String, String>,
    labels: BTreeMap<String, String>,
}

impl ServiceAccountBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            annotations: BTreeMap::new(),
            labels: BTreeMap::new(),
        }
    }

    pub fn with_annotation(mut self, key: &str, value: &str) -> Self {
        self.annotations.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> Result<ServiceAccount> {
        Ok(ServiceAccount {
            metadata: ObjectMeta {
                name: Some(self.name),
                namespace: Some(self.namespace),
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

pub struct RoleBuilder {
    name: String,
    namespace: String,
    rules: Vec<PolicyRule>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl RoleBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            rules: Vec::new(),
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_rules(mut self, rules: Vec<PolicyRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn add_rule(mut self, rule: PolicyRule) -> Self {
        self.rules.push(rule);
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

    pub fn build(self) -> Result<Role> {
        let k8s_rules: Vec<k8s_openapi::api::rbac::v1::PolicyRule> =
            self.rules.iter().map(|r| r.to_k8s_rule()).collect();

        Ok(Role {
            metadata: ObjectMeta {
                name: Some(self.name),
                namespace: Some(self.namespace),
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                ..Default::default()
            },
            rules: if k8s_rules.is_empty() {
                None
            } else {
                Some(k8s_rules)
            },
        })
    }

    pub fn workflow_executor(name: &str, namespace: &str) -> Result<Self> {
        let rules = vec![
            PolicyRule::new()
                .with_api_groups(vec!["batch".to_string(), "".to_string()])
                .with_resources(vec!["jobs".to_string(), "pods".to_string()])
                .with_verbs(vec![
                    "get".to_string(),
                    "list".to_string(),
                    "watch".to_string(),
                    "create".to_string(),
                    "update".to_string(),
                    "patch".to_string(),
                    "delete".to_string(),
                ]),
            PolicyRule::new()
                .with_api_groups(vec!["".to_string()])
                .with_resources(vec!["pods/log".to_string()])
                .with_verbs(vec!["get".to_string(), "list".to_string()]),
        ];

        Ok(Self::new(name, namespace).with_rules(rules))
    }

    pub fn workflow_viewer(name: &str, namespace: &str) -> Result<Self> {
        let rules = vec![
            PolicyRule::new()
                .with_api_groups(vec![
                    "batch".to_string(),
                    "apps".to_string(),
                    "extensions".to_string(),
                    "".to_string(),
                ])
                .with_resources(vec![
                    "jobs".to_string(),
                    "pods".to_string(),
                    "deployments".to_string(),
                    "replicasets".to_string(),
                ])
                .with_verbs(vec![
                    "get".to_string(),
                    "list".to_string(),
                    "watch".to_string(),
                ]),
            PolicyRule::new()
                .with_api_groups(vec!["".to_string()])
                .with_resources(vec!["pods/log".to_string(), "pods/status".to_string()])
                .with_verbs(vec!["get".to_string(), "list".to_string()]),
        ];

        Ok(Self::new(name, namespace).with_rules(rules))
    }

    pub fn admin(name: &str, namespace: &str) -> Result<Self> {
        let rules = vec![PolicyRule::new()
            .with_api_groups(vec!["*".to_string()])
            .with_resources(vec!["*".to_string()])
            .with_verbs(vec!["*".to_string()])];

        Ok(Self::new(name, namespace).with_rules(rules))
    }
}

pub struct RoleBindingBuilder {
    name: String,
    namespace: String,
    subjects: Vec<k8s_openapi::api::rbac::v1::Subject>,
    role_ref: Option<k8s_openapi::api::rbac::v1::RoleRef>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl RoleBindingBuilder {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            subjects: Vec::new(),
            role_ref: None,
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_subject(mut self, subject: k8s_openapi::api::rbac::v1::Subject) -> Self {
        self.subjects.push(subject);
        self
    }

    pub fn with_subject_service_account(mut self, name: &str, namespace: &str) -> Self {
        let subject = k8s_openapi::api::rbac::v1::Subject {
            kind: "ServiceAccount".to_string(),
            name: name.to_string(),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        };
        self.subjects.push(subject);
        self
    }

    pub fn with_subject_user(mut self, name: &str) -> Self {
        let subject = k8s_openapi::api::rbac::v1::Subject {
            kind: "User".to_string(),
            name: name.to_string(),
            ..Default::default()
        };
        self.subjects.push(subject);
        self
    }

    pub fn with_role_ref(mut self, role_ref: k8s_openapi::api::rbac::v1::RoleRef) -> Self {
        self.role_ref = Some(role_ref);
        self
    }

    pub fn with_role_ref_role(mut self, name: &str) -> Self {
        self.role_ref = Some(k8s_openapi::api::rbac::v1::RoleRef {
            kind: "Role".to_string(),
            name: name.to_string(),
            api_group: "rbac.authorization.k8s.io".to_string(),
        });
        self
    }

    pub fn with_role_ref_cluster_role(mut self, name: &str) -> Self {
        self.role_ref = Some(k8s_openapi::api::rbac::v1::RoleRef {
            kind: "ClusterRole".to_string(),
            name: name.to_string(),
            api_group: "rbac.authorization.k8s.io".to_string(),
        });
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

    pub fn build(self) -> Result<RoleBinding> {
        if self.role_ref.is_none() {
            anyhow::bail!("role_ref must be specified");
        }

        Ok(RoleBinding {
            metadata: ObjectMeta {
                name: Some(self.name),
                namespace: Some(self.namespace),
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                ..Default::default()
            },
            subjects: if self.subjects.is_empty() {
                None
            } else {
                Some(self.subjects)
            },
            role_ref: self.role_ref.unwrap(),
        })
    }
}

pub struct ClusterRoleBuilder {
    name: String,
    aggregation_rule: Option<k8s_openapi::api::rbac::v1::AggregationRule>,
    rules: Vec<PolicyRule>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl ClusterRoleBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            aggregation_rule: None,
            rules: Vec::new(),
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_rules(mut self, rules: Vec<PolicyRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn add_rule(mut self, rule: PolicyRule) -> Self {
        self.rules.push(rule);
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

    pub fn build(self) -> Result<ClusterRole> {
        let k8s_rules: Vec<k8s_openapi::api::rbac::v1::PolicyRule> =
            self.rules.iter().map(|r| r.to_k8s_rule()).collect();

        Ok(ClusterRole {
            metadata: ObjectMeta {
                name: Some(self.name),
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                ..Default::default()
            },
            aggregation_rule: self.aggregation_rule,
            rules: if k8s_rules.is_empty() {
                None
            } else {
                Some(k8s_rules)
            },
        })
    }
}

pub struct ClusterRoleBindingBuilder {
    name: String,
    subjects: Vec<k8s_openapi::api::rbac::v1::Subject>,
    role_ref: Option<k8s_openapi::api::rbac::v1::RoleRef>,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl ClusterRoleBindingBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subjects: Vec::new(),
            role_ref: None,
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
        }
    }

    pub fn with_subject(mut self, subject: k8s_openapi::api::rbac::v1::Subject) -> Self {
        self.subjects.push(subject);
        self
    }

    pub fn with_subject_service_account(mut self, name: &str, namespace: &str) -> Self {
        let subject = k8s_openapi::api::rbac::v1::Subject {
            kind: "ServiceAccount".to_string(),
            name: name.to_string(),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        };
        self.subjects.push(subject);
        self
    }

    pub fn with_subject_user(mut self, name: &str) -> Self {
        let subject = k8s_openapi::api::rbac::v1::Subject {
            kind: "User".to_string(),
            name: name.to_string(),
            ..Default::default()
        };
        self.subjects.push(subject);
        self
    }

    pub fn with_role_ref(mut self, role_ref: k8s_openapi::api::rbac::v1::RoleRef) -> Self {
        self.role_ref = Some(role_ref);
        self
    }

    pub fn with_role_ref_cluster_role(mut self, name: &str) -> Self {
        self.role_ref = Some(k8s_openapi::api::rbac::v1::RoleRef {
            kind: "ClusterRole".to_string(),
            name: name.to_string(),
            api_group: "rbac.authorization.k8s.io".to_string(),
        });
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

    pub fn build(self) -> Result<ClusterRoleBinding> {
        if self.role_ref.is_none() {
            anyhow::bail!("role_ref must be specified");
        }

        Ok(ClusterRoleBinding {
            metadata: ObjectMeta {
                name: Some(self.name),
                annotations: if self.annotations.is_empty() {
                    None
                } else {
                    Some(self.annotations)
                },
                labels: if self.labels.is_empty() {
                    None
                } else {
                    Some(self.labels)
                },
                ..Default::default()
            },
            subjects: if self.subjects.is_empty() {
                None
            } else {
                Some(self.subjects)
            },
            role_ref: self.role_ref.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_account_builder() {
        let sa = ServiceAccountBuilder::new("test-sa", "default")
            .with_annotation(
                "eks.amazonaws.com/role-arn",
                "arn:aws:iam::123456:role/test",
            )
            .build()
            .expect("Failed to build service account");

        assert_eq!(sa.metadata.name, Some("test-sa".to_string()));
        assert_eq!(sa.metadata.namespace, Some("default".to_string()));
        assert!(sa.metadata.annotations.is_some());
    }

    #[test]
    fn test_role_builder() {
        let rule = PolicyRule::new()
            .with_api_groups(vec!["".to_string()])
            .with_resources(vec!["pods".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string()]);

        let role = RoleBuilder::new("pod-reader", "default")
            .add_rule(rule)
            .build()
            .expect("Failed to build role");

        assert_eq!(role.metadata.name, Some("pod-reader".to_string()));
        assert!(role.rules.is_some());
        assert_eq!(role.rules.unwrap().len(), 1);
    }

    #[test]
    fn test_workflow_executor_preset() {
        let role = RoleBuilder::workflow_executor("executor", "default")
            .expect("Failed to create executor preset")
            .build()
            .expect("Failed to build role");

        assert_eq!(role.metadata.name, Some("executor".to_string()));
        assert!(role.rules.is_some());
        let rules = role.rules.unwrap();
        assert!(rules.len() > 0);
        assert!(rules[0].verbs.contains(&"create".to_string()));
    }

    #[test]
    fn test_workflow_viewer_preset() {
        let role = RoleBuilder::workflow_viewer("viewer", "default")
            .expect("Failed to create viewer preset")
            .build()
            .expect("Failed to build role");

        assert_eq!(role.metadata.name, Some("viewer".to_string()));
        assert!(role.rules.is_some());
        let rules = role.rules.unwrap();
        assert!(rules.len() > 0);
        assert!(rules[0].verbs.contains(&"get".to_string()));
    }

    #[test]
    fn test_admin_preset() {
        let role = RoleBuilder::admin("admin", "default")
            .expect("Failed to create admin preset")
            .build()
            .expect("Failed to build role");

        assert_eq!(role.metadata.name, Some("admin".to_string()));
        assert!(role.rules.is_some());
        let rules = role.rules.unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].api_groups, Some(vec!["*".to_string()]));
    }

    #[test]
    fn test_role_binding_builder() {
        let binding = RoleBindingBuilder::new("test-binding", "default")
            .with_subject_service_account("test-sa", "default")
            .with_role_ref_role("pod-reader")
            .build()
            .expect("Failed to build role binding");

        assert_eq!(binding.metadata.name, Some("test-binding".to_string()));
        assert!(binding.subjects.is_some());
        assert_eq!(binding.role_ref.kind, "Role");
    }

    #[test]
    fn test_cluster_role_builder() {
        let rule = PolicyRule::new()
            .with_api_groups(vec!["".to_string()])
            .with_resources(vec!["nodes".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string()]);

        let cluster_role = ClusterRoleBuilder::new("node-reader")
            .add_rule(rule)
            .build()
            .expect("Failed to build cluster role");

        assert_eq!(cluster_role.metadata.name, Some("node-reader".to_string()));
        assert!(cluster_role.rules.is_some());
        assert_eq!(cluster_role.rules.unwrap().len(), 1);
    }

    #[test]
    fn test_cluster_role_binding_builder() {
        let binding = ClusterRoleBindingBuilder::new("test-cluster-binding")
            .with_subject_service_account("test-sa", "default")
            .with_role_ref_cluster_role("cluster-admin")
            .build()
            .expect("Failed to build cluster role binding");

        assert_eq!(
            binding.metadata.name,
            Some("test-cluster-binding".to_string())
        );
        assert!(binding.subjects.is_some());
        assert_eq!(binding.role_ref.kind, "ClusterRole");
    }

    #[test]
    fn test_policy_rule() {
        let rule = PolicyRule::new()
            .with_api_groups(vec!["batch".to_string()])
            .with_resources(vec!["jobs".to_string()])
            .with_verbs(vec!["get".to_string(), "list".to_string()])
            .with_resource_names(vec!["specific-job".to_string()]);

        let k8s_rule = rule.to_k8s_rule();
        assert!(k8s_rule.api_groups.is_some());
        assert!(k8s_rule.resources.is_some());
        assert!(k8s_rule.resource_names.is_some());
    }
}
