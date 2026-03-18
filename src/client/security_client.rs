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
//! let policy = security.network_policy()
//!     .deny_all("production", "default")
//!     .await?;
//!
//! // Create a resource quota
//! let quota = security.resource_quota()
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
//! let sa = security.service_account()
//!     .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/test")
//!     .build()
//!     .await?;
//! ```
//!
//! # Client Methods
//!
//! The `SecurityClient` trait provides the following methods for building security resources:
//!
//! ## Network Policy
//!
//! ```ignore
//! fn network_policy(&self) -> NetworkPolicyBuilder
//! ```
//!
//! Returns a builder for creating network policies.
//!
//! ## Resource Quota
//!
//! ```ignore
//! fn resource_quota(&self) -> ResourceQuotaBuilder
//! ```
//!
//! Returns a builder for creating resource quotas.
//!
//! ## Security Context
//!
//! ```ignore
//! fn security_context(&self) -> PodSecurityContextBuilder
//! ```
//!
//! Returns a builder for creating pod-level security contexts.
//!
//! ## Security Context Config
//!
//! ```ignore
//! fn security_context_config(&self) -> SecurityContextConfig
//! ```
//!
//! Returns a builder for creating security context configurations.
//!
//! ## Service Account
//!
//! ```ignore
//! fn service_account(&self) -> ServiceAccountBuilder
//! ```
//!
//! Returns a builder for creating service accounts.
//!
//! ## Role
//!
//! ```ignore
//! fn role(&self) -> RoleBuilder
//! ```
//!
//! Returns a builder for creating namespace-scoped roles.
//!
//! ## Cluster Role
//!
//! ```ignore
//! fn cluster_role(&self) -> ClusterRoleBuilder
//! ```
//!
//! Returns a builder for creating cluster-wide roles.
//!
//! ## Role Binding
//!
//! ```ignore
//! fn role_binding(&self) -> RoleBindingBuilder
//! ```
//!
//! Returns a builder for creating role bindings.
//!
//! ## Cluster Role Binding
//!
//! ```ignore
//! fn cluster_role_binding(&self) -> ClusterRoleBindingBuilder
//! ```
//!
//! Returns a builder for creating cluster role bindings.
//!
//! ## RBAC Policy Rule
//!
//! ```ignore
//! fn rbac_policy_rule(&self) -> RbacPolicyRule
//! ```
//!
//! Returns a builder for creating RBAC policy rules.
//!
//! ## Limit Range
//!
//! ```ignore
//! fn limit_range(&self) -> LimitRangeBuilder
//! ```
//!
//! Returns a builder for creating limit ranges.
//!
    /// Returns a network policy builder.
    pub fn network_policy(&self) -> NetworkPolicyBuilder {
        NetworkPolicyBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a resource quota builder.
    pub fn resource_quota(&self) -> ResourceQuotaBuilder {
        ResourceQuotaBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a security context builder.
    pub fn security_context(&self) -> PodSecurityContextBuilder {
        PodSecurityContextBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a security context config builder.
    pub fn security_context_config(&self) -> SecurityContextConfig {
        SecurityContextConfig::new()
    }

    /// Returns a service account builder.
    pub fn service_account(&self) -> ServiceAccountBuilder {
        ServiceAccountBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a role builder.
    pub fn role(&self) -> RoleBuilder {
        RoleBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a cluster role builder.
    pub fn cluster_role(&self) -> ClusterRoleBuilder {
        ClusterRoleBuilder::new()
    }

    /// Returns a role binding builder.
    pub fn role_binding(&self) -> RoleBindingBuilder {
        RoleBindingBuilder::new(self.client.namespace().to_string())
    }

    /// Returns a cluster role binding builder.
    pub fn cluster_role_binding(&self) -> ClusterRoleBindingBuilder {
        ClusterRoleBindingBuilder::new()
    }

    /// Returns a policy rule builder.
    pub fn policy_rule(&self) -> RbacPolicyRule {
        RbacPolicyRule::new()
    }

    /// Returns a limit range builder.
    pub fn limit_range(&self) -> LimitRangeBuilder {
        LimitRangeBuilder::new(self.client.namespace().to_string())
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
    fn test_security_client_namespace() {
        use crate::client::{MaestroClient, MaestroClientBuilder};

        let ctx = crate::Context::default();
        let client = MaestroClientBuilder::new()
            .with_namespace("test-ns")
            .build()
            .unwrap();

        let security_client = SecurityClient { client: &client };

        assert_eq!(security_client.network_policy().name, "test-ns".to_string());
        assert_eq!(
            security_client.resource_quota().namespace,
            "test-ns".to_string()
        );
        assert_eq!(
            security_client.security_context().namespace,
            "test-ns".to_string()
        );
        assert_eq!(
            security_client.service_account().namespace,
            "test-ns".to_string()
        );
        assert_eq!(security_client.role().namespace, "test-ns".to_string());
    }
}
