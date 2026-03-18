//! Security client for managing Kubernetes security resources.
//!
//! This module provides builders for creating and managing Kubernetes security resources
//! including network policies, resource quotas, security contexts, RBAC, and limit ranges.
//!
//! # Example
//!
//! ```no_run
//! use k8s_maestro::{MaestroClientBuilder, MaestroClient};
//! use k8s_maestro::client::SecurityClient;
//!
//! let client = MaestroClientBuilder::new()
//!     .with_namespace("production")
//!     .build()
//!     .unwrap();
//!
//! let security = SecurityClient { client: &client };
//!
//! // Create a network policy
//! let policy = security.network_policy("deny-all")
//!     .deny_all("deny-all", "default")
//!     .await?;
//!
//! // Create a resource quota
//! let quota = security.resource_quota("small-quota")
//!     .with_scope(QuotaScope::Terminating)
//!     .build()
//!     .await?;
//!
//! // Create a security context
//! let ctx = security.security_context()
//!     .restricted()
//!     .build();
//!
//! // Create a service account
//! let sa = security.service_account("test-sa")
//!     .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/test")
//!     .build()
//!     .await?;
//! ```

use super::MaestroClient;
use crate::security::{
    ClusterRoleBindingBuilder, ClusterRoleBuilder, LimitRangeBuilder, LimitRangeItemBuilder,
    LimitRangeType, NetworkPolicyBuilder, PodSecurityContextBuilder, PolicyRule as RbacPolicyRule,
    ResourceQuotaBuilder, RoleBindingBuilder, RoleBuilder, SecurityContextConfig,
    ServiceAccountBuilder,
};

/// Client for managing Kubernetes security resources.
pub struct SecurityClient<'a> {
    pub(crate) client: &'a MaestroClient,
}

impl<'a> SecurityClient<'a> {
    /// Returns a network policy builder.
    pub fn network_policy(&self, name: &str) -> NetworkPolicyBuilder {
        NetworkPolicyBuilder::new(name, self.client.namespace())
    }

    /// Returns a resource quota builder.
    pub fn resource_quota(&self, name: &str) -> ResourceQuotaBuilder {
        ResourceQuotaBuilder::new(name, self.client.namespace())
    }

    /// Returns a security context builder.
    pub fn security_context(&self) -> PodSecurityContextBuilder {
        PodSecurityContextBuilder::new()
    }

    /// Returns a security context config builder.
    pub fn security_context_config(&self) -> SecurityContextConfig {
        SecurityContextConfig::new()
    }

    /// Returns a service account builder.
    pub fn service_account(&self, name: &str) -> ServiceAccountBuilder {
        ServiceAccountBuilder::new(name, self.client.namespace())
    }

    /// Returns a role builder.
    pub fn role(&self, name: &str) -> RoleBuilder {
        RoleBuilder::new(name, self.client.namespace())
    }

    /// Returns a cluster role builder.
    pub fn cluster_role(&self, name: &str) -> ClusterRoleBuilder {
        ClusterRoleBuilder::new(name)
    }

    /// Returns a role binding builder.
    pub fn role_binding(&self, name: &str) -> RoleBindingBuilder {
        RoleBindingBuilder::new(name, self.client.namespace())
    }

    /// Returns a cluster role binding builder.
    pub fn cluster_role_binding(&self, name: &str) -> ClusterRoleBindingBuilder {
        ClusterRoleBindingBuilder::new(name)
    }

    /// Returns a policy rule builder.
    pub fn policy_rule(&self) -> RbacPolicyRule {
        RbacPolicyRule::new()
    }

    /// Returns a limit range builder.
    pub fn limit_range(&self, name: &str) -> LimitRangeBuilder {
        LimitRangeBuilder::new(name, self.client.namespace())
    }

    /// Returns a limit range item builder.
    pub fn limit_range_item(&self, limit_type: LimitRangeType) -> LimitRangeItemBuilder {
        LimitRangeItemBuilder::new(limit_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_client_creation() {
        use crate::client::{MaestroClient, MaestroClientBuilder};

        let client = MaestroClientBuilder::new()
            .with_namespace("test-ns")
            .build()
            .unwrap();

        let security_client = SecurityClient { client: &client };

        // Test that all builder methods work without panicking
        let _ = security_client.network_policy("test-policy");
        let _ = security_client.resource_quota("test-quota");
        let _ = security_client.security_context();
        let _ = security_client.security_context_config();
        let _ = security_client.service_account("test-sa");
        let _ = security_client.role("test-role");
        let _ = security_client.cluster_role("test-cluster-role");
        let _ = security_client.role_binding("test-binding");
        let _ = security_client.cluster_role_binding("test-cluster-binding");
        let _ = security_client.policy_rule();
        let _ = security_client.limit_range("test-limits");
        let _ = security_client.limit_range_item(crate::security::LimitRangeType::Container);
    }
}
