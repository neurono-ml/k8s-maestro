//! Security module for Kubernetes security configurations.
//!
//! This module provides builders and utilities for managing Kubernetes security resources
//! including network policies, resource quotas, security contexts, RBAC, and limit ranges.

pub mod limits;
pub mod network_policy;
pub mod rbac;
pub mod resource_quota;
pub mod security_context;

pub use limits::{LimitRangeBuilder, LimitRangeItemBuilder, LimitRangeType};
pub use network_policy::{NetworkPolicyBuilder, NetworkPolicyRule, PolicyType};
pub use rbac::{
    ClusterRoleBindingBuilder, ClusterRoleBuilder, PolicyRule, RoleBindingBuilder, RoleBuilder,
    ServiceAccountBuilder,
};
pub use resource_quota::{QuotaScope, ResourceQuotaBuilder};
pub use security_context::{
    ContainerSecurityContextBuilder, PodSecurityContextBuilder, SecurityContextConfig,
};
