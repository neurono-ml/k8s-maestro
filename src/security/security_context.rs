use k8s_openapi::api::core::v1::{
    Capabilities, PodSecurityContext, SeccompProfile, SecurityContext as K8sSecurityContext,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct SecurityContextConfig {
    pub run_as_user: Option<i64>,
    pub run_as_group: Option<i64>,
    pub run_as_non_root: Option<bool>,
    pub read_only_root_filesystem: Option<bool>,
    pub allow_privilege_escalation: Option<bool>,
    pub privileged: Option<bool>,
    pub capabilities_add: Vec<String>,
    pub capabilities_drop: Vec<String>,
    pub seccomp_profile_type: Option<String>,
    pub fs_group: Option<i64>,
    pub fs_group_change_policy: Option<String>,
    pub supplemental_groups: Vec<i64>,
}

impl SecurityContextConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_run_as_user(mut self, user: i64) -> Self {
        self.run_as_user = Some(user);
        self
    }

    pub fn with_run_as_group(mut self, group: i64) -> Self {
        self.run_as_group = Some(group);
        self
    }

    pub fn with_run_as_non_root(mut self, non_root: bool) -> Self {
        self.run_as_non_root = Some(non_root);
        self
    }

    pub fn with_read_only_root_filesystem(mut self, read_only: bool) -> Self {
        self.read_only_root_filesystem = Some(read_only);
        self
    }

    pub fn with_allow_privilege_escalation(mut self, allow: bool) -> Self {
        self.allow_privilege_escalation = Some(allow);
        self
    }

    pub fn with_privileged(mut self, privileged: bool) -> Self {
        self.privileged = Some(privileged);
        self
    }

    pub fn add_capability(mut self, cap: &str) -> Self {
        self.capabilities_add.push(cap.to_string());
        self
    }

    pub fn drop_capability(mut self, cap: &str) -> Self {
        self.capabilities_drop.push(cap.to_string());
        self
    }

    pub fn with_seccomp_profile(mut self, profile_type: &str) -> Self {
        self.seccomp_profile_type = Some(profile_type.to_string());
        self
    }

    pub fn with_fs_group(mut self, group: i64) -> Self {
        self.fs_group = Some(group);
        self
    }

    pub fn with_fs_group_change_policy(mut self, policy: &str) -> Self {
        self.fs_group_change_policy = Some(policy.to_string());
        self
    }

    pub fn with_supplemental_groups(mut self, groups: Vec<i64>) -> Self {
        self.supplemental_groups = groups;
        self
    }

    pub fn restricted() -> Self {
        Self::new()
            .with_run_as_non_root(true)
            .with_allow_privilege_escalation(false)
            .with_read_only_root_filesystem(true)
            .drop_capability("ALL")
            .with_seccomp_profile("RuntimeDefault")
    }

    pub fn baseline() -> Self {
        Self::new()
            .with_run_as_non_root(true)
            .with_allow_privilege_escalation(false)
            .drop_capability("NET_RAW")
    }

    pub fn privileged() -> Self {
        Self::new()
            .with_privileged(true)
            .with_allow_privilege_escalation(true)
    }
}

pub struct PodSecurityContextBuilder {
    fs_group: Option<i64>,
    fs_group_change_policy: Option<String>,
    supplemental_groups: Vec<i64>,
    run_as_user: Option<i64>,
    run_as_group: Option<i64>,
    run_as_non_root: Option<bool>,
    seccomp_profile_type: Option<String>,
}

impl PodSecurityContextBuilder {
    pub fn new() -> Self {
        Self {
            fs_group: None,
            fs_group_change_policy: None,
            supplemental_groups: Vec::new(),
            run_as_user: None,
            run_as_group: None,
            run_as_non_root: None,
            seccomp_profile_type: None,
        }
    }

    pub fn with_fs_group(mut self, group: i64) -> Self {
        self.fs_group = Some(group);
        self
    }

    pub fn with_fs_group_change_policy(mut self, policy: &str) -> Self {
        self.fs_group_change_policy = Some(policy.to_string());
        self
    }

    pub fn with_supplemental_groups(mut self, groups: Vec<i64>) -> Self {
        self.supplemental_groups = groups;
        self
    }

    pub fn with_run_as_user(mut self, user: i64) -> Self {
        self.run_as_user = Some(user);
        self
    }

    pub fn with_run_as_group(mut self, group: i64) -> Self {
        self.run_as_group = Some(group);
        self
    }

    pub fn with_run_as_non_root(mut self, non_root: bool) -> Self {
        self.run_as_non_root = Some(non_root);
        self
    }

    pub fn with_seccomp_profile(mut self, profile_type: &str) -> Self {
        self.seccomp_profile_type = Some(profile_type.to_string());
        self
    }

    pub fn build(self) -> PodSecurityContext {
        PodSecurityContext {
            fs_group: self.fs_group,
            fs_group_change_policy: self.fs_group_change_policy,
            supplemental_groups: if self.supplemental_groups.is_empty() {
                None
            } else {
                Some(self.supplemental_groups)
            },
            run_as_user: self.run_as_user,
            run_as_group: self.run_as_group,
            run_as_non_root: self.run_as_non_root,
            seccomp_profile: self.seccomp_profile_type.map(|t| SeccompProfile {
                type_: t,
                localhost_profile: None,
            }),
            ..Default::default()
        }
    }

    pub fn from_config(config: &SecurityContextConfig) -> Self {
        Self::new()
            .with_fs_group(config.fs_group.unwrap_or(0))
            .with_fs_group_change_policy(
                config
                    .fs_group_change_policy
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("OnRootMismatch"),
            )
            .with_supplemental_groups(config.supplemental_groups.clone())
            .with_run_as_user(config.run_as_user.unwrap_or(0))
            .with_run_as_group(config.run_as_group.unwrap_or(0))
            .with_run_as_non_root(config.run_as_non_root.unwrap_or(true))
            .with_seccomp_profile(
                config
                    .seccomp_profile_type
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("RuntimeDefault"),
            )
    }
}

pub struct ContainerSecurityContextBuilder {
    run_as_user: Option<i64>,
    run_as_group: Option<i64>,
    run_as_non_root: Option<bool>,
    read_only_root_filesystem: Option<bool>,
    allow_privilege_escalation: Option<bool>,
    privileged: Option<bool>,
    capabilities_add: Vec<String>,
    capabilities_drop: Vec<String>,
    seccomp_profile_type: Option<String>,
}

impl ContainerSecurityContextBuilder {
    pub fn new() -> Self {
        Self {
            run_as_user: None,
            run_as_group: None,
            run_as_non_root: None,
            read_only_root_filesystem: None,
            allow_privilege_escalation: None,
            privileged: None,
            capabilities_add: Vec::new(),
            capabilities_drop: Vec::new(),
            seccomp_profile_type: None,
        }
    }

    pub fn with_run_as_user(mut self, user: i64) -> Self {
        self.run_as_user = Some(user);
        self
    }

    pub fn with_run_as_group(mut self, group: i64) -> Self {
        self.run_as_group = Some(group);
        self
    }

    pub fn with_run_as_non_root(mut self, non_root: bool) -> Self {
        self.run_as_non_root = Some(non_root);
        self
    }

    pub fn with_read_only_root_filesystem(mut self, read_only: bool) -> Self {
        self.read_only_root_filesystem = Some(read_only);
        self
    }

    pub fn with_allow_privilege_escalation(mut self, allow: bool) -> Self {
        self.allow_privilege_escalation = Some(allow);
        self
    }

    pub fn with_privileged(mut self, privileged: bool) -> Self {
        self.privileged = Some(privileged);
        self
    }

    pub fn add_capability(mut self, cap: &str) -> Self {
        self.capabilities_add.push(cap.to_string());
        self
    }

    pub fn drop_capability(mut self, cap: &str) -> Self {
        self.capabilities_drop.push(cap.to_string());
        self
    }

    pub fn with_seccomp_profile(mut self, profile_type: &str) -> Self {
        self.seccomp_profile_type = Some(profile_type.to_string());
        self
    }

    pub fn build(self) -> K8sSecurityContext {
        let capabilities =
            if !self.capabilities_add.is_empty() || !self.capabilities_drop.is_empty() {
                Some(Capabilities {
                    add: if self.capabilities_add.is_empty() {
                        None
                    } else {
                        Some(self.capabilities_add)
                    },
                    drop: if self.capabilities_drop.is_empty() {
                        None
                    } else {
                        Some(self.capabilities_drop)
                    },
                })
            } else {
                None
            };

        K8sSecurityContext {
            run_as_user: self.run_as_user,
            run_as_group: self.run_as_group,
            run_as_non_root: self.run_as_non_root,
            read_only_root_filesystem: self.read_only_root_filesystem,
            allow_privilege_escalation: self.allow_privilege_escalation,
            privileged: self.privileged,
            capabilities,
            seccomp_profile: self.seccomp_profile_type.map(|t| SeccompProfile {
                type_: t,
                localhost_profile: None,
            }),
            ..Default::default()
        }
    }

    pub fn from_config(config: &SecurityContextConfig) -> Self {
        let mut builder = Self::new();

        if let Some(user) = config.run_as_user {
            builder = builder.with_run_as_user(user);
        }

        if let Some(group) = config.run_as_group {
            builder = builder.with_run_as_group(group);
        }

        if let Some(non_root) = config.run_as_non_root {
            builder = builder.with_run_as_non_root(non_root);
        }

        if let Some(read_only) = config.read_only_root_filesystem {
            builder = builder.with_read_only_root_filesystem(read_only);
        }

        if let Some(allow) = config.allow_privilege_escalation {
            builder = builder.with_allow_privilege_escalation(allow);
        }

        if let Some(privileged) = config.privileged {
            builder = builder.with_privileged(privileged);
        }

        for cap in &config.capabilities_add {
            builder = builder.add_capability(cap);
        }

        for cap in &config.capabilities_drop {
            builder = builder.drop_capability(cap);
        }

        if let Some(profile) = &config.seccomp_profile_type {
            builder = builder.with_seccomp_profile(profile);
        }

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_context_config_restricted() {
        let config = SecurityContextConfig::restricted();
        assert_eq!(config.run_as_non_root, Some(true));
        assert_eq!(config.allow_privilege_escalation, Some(false));
        assert_eq!(config.read_only_root_filesystem, Some(true));
        assert!(config.capabilities_drop.contains(&"ALL".to_string()));
    }

    #[test]
    fn test_security_context_config_baseline() {
        let config = SecurityContextConfig::baseline();
        assert_eq!(config.run_as_non_root, Some(true));
        assert_eq!(config.allow_privilege_escalation, Some(false));
        assert!(config.capabilities_drop.contains(&"NET_RAW".to_string()));
    }

    #[test]
    fn test_security_context_config_privileged() {
        let config = SecurityContextConfig::privileged();
        assert_eq!(config.privileged, Some(true));
        assert_eq!(config.allow_privilege_escalation, Some(true));
    }

    #[test]
    fn test_pod_security_context_builder() {
        let pod_ctx = PodSecurityContextBuilder::new()
            .with_fs_group(2000)
            .with_supplemental_groups(vec![1000, 2000])
            .with_run_as_non_root(true)
            .build();

        assert_eq!(pod_ctx.fs_group, Some(2000));
        assert_eq!(pod_ctx.supplemental_groups, Some(vec![1000, 2000]));
        assert_eq!(pod_ctx.run_as_non_root, Some(true));
    }

    #[test]
    fn test_container_security_context_builder() {
        let container_ctx = ContainerSecurityContextBuilder::new()
            .with_run_as_user(1000)
            .with_run_as_non_root(true)
            .add_capability("NET_ADMIN")
            .drop_capability("KILL")
            .build();

        assert_eq!(container_ctx.run_as_user, Some(1000));
        assert_eq!(container_ctx.run_as_non_root, Some(true));
        assert!(container_ctx.capabilities.is_some());
        let caps = container_ctx.capabilities.unwrap();
        assert!(caps.add.unwrap().contains(&"NET_ADMIN".to_string()));
        assert!(caps.drop.unwrap().contains(&"KILL".to_string()));
    }

    #[test]
    fn test_container_from_restricted_config() {
        let config = SecurityContextConfig::restricted();
        let container_ctx = ContainerSecurityContextBuilder::from_config(&config).build();

        assert_eq!(container_ctx.run_as_non_root, Some(true));
        assert_eq!(container_ctx.allow_privilege_escalation, Some(false));
        assert_eq!(container_ctx.read_only_root_filesystem, Some(true));
    }
}
