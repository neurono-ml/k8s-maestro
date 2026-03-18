//! Migration utilities for transitioning from v0.3.x to v0.4.0 API.
//!
//! This module provides type aliases and helpers to assist with migrating
//! from the old job-centric API to the new workflow-centric API.
//!
//! # Deprecation Notices
//!
//! All type aliases in this module are deprecated and will be removed in a future version.
//! Use them only during the migration period and update your code to use the new types.
//!
//! # Quick Migration Reference
//!
//! | Old Type (v0.3.x) | New Type (v0.4.0) | Module |
//! |-------------------|-------------------|--------|
//! | `Job` | `Workflow` | `workflows` |
//! | `JobBuilder` | `WorkflowBuilder` | `workflows` |
//! | `MaestroK8sClient` | `MaestroClient` | `client` |
//!
//! # Example: Migrating Type Usage
//!
//! ```rust,no_run
//! use k8s_maestro::migration::*;
//!
//! #[allow(deprecated)]
//! fn legacy_code() {
//!     // Old API - deprecated but still available for migration
//!     let _client: MaestroK8sClient = todo!();
//!     let _builder: JobBuilder = todo!();
//! }
//!
//! // New API - use this instead
//! fn new_code() {
//!     use k8s_maestro::{MaestroClientBuilder, WorkflowBuilder};
//!     
//!     let _client = MaestroClientBuilder::new().build().unwrap();
//!     let _builder = WorkflowBuilder::new();
//! }
//! ```
//!
//! # Migration Path
//!
//! 1. **Phase 1 - Deprecation**: Use type aliases from this module
//! 2. **Phase 2 - Transition**: Mix old and new types in the same codebase
//! 3. **Phase 3 - Completion**: Remove all deprecated type aliases
//!
//! For detailed migration guidance, see the [Migration Guide](../../docs/migration-guide.md).

pub use crate::workflows::WorkflowBuilder;

#[deprecated(
    since = "0.4.0",
    note = "Use `k8s_maestro::workflows::WorkflowBuilder` instead. See the migration guide for details."
)]
/// Deprecated type alias for backward compatibility.
///
/// **Migration Path:** Replace `JobBuilder` with `WorkflowBuilder`
///
/// **Example:**
///
/// ```rust,ignore
/// // Old (deprecated)
/// let job = JobBuilder::new().with_name("my-job").build()?;
///
/// // New
/// let workflow = WorkflowBuilder::new()
///     .with_name("my-workflow")
///     .add_step(JobStep::new("step-1", "image:tag"))
///     .build()?;
/// ```
pub type JobBuilder = WorkflowBuilder;

#[deprecated(
    since = "0.4.0",
    note = "Use `k8s_maestro::workflows::Workflow` instead. See the migration guide for details."
)]
/// Deprecated type alias for backward compatibility.
///
/// **Migration Path:** Replace `Job` with `Workflow`
///
/// **Example:**
///
/// ```rust,ignore
/// // Old (deprecated)
/// let job: Job = JobBuilder::new().build()?;
///
/// // New
/// let workflow: Workflow = WorkflowBuilder::new()
///     .add_step(JobStep::new("step-1", "image:tag"))
///     .build()?;
/// ```
pub type Job = crate::workflows::Workflow;

#[deprecated(
    since = "0.4.0",
    note = "Use `k8s_maestro::MaestroClient` and `MaestroClientBuilder` instead. See the migration guide for details."
)]
/// Deprecated type alias for backward compatibility.
///
/// **Migration Path:** Replace `MaestroK8sClient` with `MaestroClient` created via `MaestroClientBuilder`
///
/// **Example:**
///
/// ```rust,ignore
/// // Old (deprecated)
/// let client = MaestroK8sClient::new().await?;
///
/// // New
/// let client = MaestroClientBuilder::new()
///     .with_namespace("default")
///     .build()?;
/// ```
pub type MaestroK8sClient = crate::clients::MaestroK8sClient;

#[deprecated(
    since = "0.4.0",
    note = "Use `k8s_maestro::workflows::ExecutionMode` instead. See the migration guide for details."
)]
/// Deprecated type alias for backward compatibility.
///
/// **Migration Path:** Replace `JobExecutionMode` with `ExecutionMode`
///
/// **Example:**
///
/// ```rust,ignore
/// // Old (deprecated)
/// let mode = JobExecutionMode::Parallel(4);
///
/// // New
/// let mode = ExecutionMode::Parallel(4);
/// ```
pub type JobExecutionMode = crate::workflows::ExecutionMode;

#[deprecated(
    since = "0.4.0",
    note = "Use `k8s_maestro::workflows::WorkflowMetadata` instead. See the migration guide for details."
)]
/// Deprecated type alias for backward compatibility.
///
/// **Migration Path:** Replace `JobConfig` with `WorkflowMetadata`
///
/// **Example:**
///
/// ```rust,ignore
/// // Old (deprecated)
/// let config = JobConfig {
///     labels: labels.clone(),
///     annotations: annotations.clone(),
/// };
///
/// // New
/// let metadata = WorkflowMetadata {
///     labels,
///     annotations,
///     ..Default::default()
/// };
/// ```
pub type JobConfig = crate::workflows::WorkflowMetadata;

/// Migration helper for converting old client initialization to new builder pattern.
///
/// This helper function provides a bridge between the old `MaestroK8sClient::new().await`
/// pattern and the new `MaestroClientBuilder` pattern.
///
/// **Deprecated:** This function is only for temporary migration purposes.
/// Use `MaestroClientBuilder::new().build()` directly in new code.
///
/// # Example
///
/// ```rust,no_run
/// use k8s_maestro::migration::create_legacy_client;
/// use k8s_maestro::MaestroClientBuilder;
///
/// #[allow(deprecated)]
/// async fn migrate() -> anyhow::Result<()> {
///     // Old pattern (via migration helper)
///     let client = create_legacy_client().await?;
///
///     // New pattern (use this in new code)
///     let client = MaestroClientBuilder::new()
///         .with_namespace("default")
///         .build()?;
///
///     Ok(())
/// }
/// ```
#[deprecated(
    since = "0.4.0",
    note = "Use `MaestroClientBuilder::new().build()` directly instead. This helper will be removed."
)]
#[allow(deprecated)]
pub async fn create_legacy_client() -> anyhow::Result<MaestroK8sClient> {
    MaestroK8sClient::new().await
}

/// Migration helper for simulating old dry_run behavior with new client.
///
/// In the old API, `dry_run` was passed per-call to methods like `create_job(&job, namespace, dry_run)`.
/// In the new API, `dry_run` is configured once at client creation.
///
/// This helper allows you to create clients with different dry_run settings to simulate
/// the old per-call behavior.
///
/// **Deprecated:** This function is only for temporary migration purposes.
/// Configure dry_run at client creation time in new code.
///
/// # Example
///
/// ```rust,no_run
/// use k8s_maestro::migration::{create_dry_run_client, create_production_client};
///
/// #[allow(deprecated)]
/// async fn migrate() -> anyhow::Result<()> {
///     // Simulate old per-call dry_run pattern
///     let prod_client = create_production_client()?;
///     let dry_run_client = create_dry_run_client()?;
///
///     // Instead of: client.create_job(&job, ns, false)
///     prod_client.create_workflow(workflow)?;
///
///     // Instead of: client.create_job(&job, ns, true)
///     dry_run_client.create_workflow(workflow)?;
///
///     Ok(())
/// }
/// ```
#[deprecated(
    since = "0.4.0",
    note = "Configure dry_run at client creation: `MaestroClientBuilder::new().with_dry_run(true).build()`"
)]
pub fn create_dry_run_client() -> anyhow::Result<crate::MaestroClient> {
    crate::MaestroClientBuilder::new()
        .with_dry_run(true)
        .build()
}

/// Migration helper for creating a production client (dry_run = false).
///
/// **Deprecated:** Use `MaestroClientBuilder::new().build()` directly.
#[deprecated(
    since = "0.4.0",
    note = "Use `MaestroClientBuilder::new().build()` directly instead."
)]
pub fn create_production_client() -> anyhow::Result<crate::MaestroClient> {
    crate::MaestroClientBuilder::new().build()
}

/// Migration helper for converting namespace parameter from old API calls.
///
/// In the old API, namespace was passed per-call: `client.create_job(&job, namespace, dry_run)`.
/// In the new API, namespace is configured once at client creation.
///
/// This helper creates clients with different namespaces to simulate the old per-call behavior.
///
/// **Deprecated:** Configure namespace at client creation time in new code.
///
/// # Example
///
/// ```rust,no_run
/// use k8s_maestro::migration::create_client_for_namespace;
///
/// #[allow(deprecated)]
/// async fn migrate() -> anyhow::Result<()> {
///     // Old pattern: client.create_job(&job, "default", false)
///     let default_client = create_client_for_namespace("default")?;
///
///     // Old pattern: client.create_job(&job, "production", false)
///     let prod_client = create_client_for_namespace("production")?;
///
///     Ok(())
/// }
/// ```
#[deprecated(
    since = "0.4.0",
    note = "Configure namespace at client creation: `MaestroClientBuilder::new().with_namespace(ns).build()`"
)]
pub fn create_client_for_namespace(namespace: &str) -> anyhow::Result<crate::MaestroClient> {
    crate::MaestroClientBuilder::new()
        .with_namespace(namespace)
        .build()
}

/// Migration macro for suppressing deprecation warnings during migration.
///
/// Use this macro to temporarily suppress deprecation warnings while migrating
/// code from the old API to the new API.
///
/// # Example
///
/// ```rust,ignore
/// use k8s_maestro::migration::allow_deprecated;
///
/// allow_deprecated! {
///     let client = MaestroK8sClient::new().await?;
///     let job = JobBuilder::new().build()?;
/// }
///
/// // Later, update to new API:
/// let client = MaestroClientBuilder::new().build()?;
/// let workflow = WorkflowBuilder::new()
///     .add_step(JobStep::new("step-1", "image:tag"))
///     .build()?;
/// ```
#[macro_export]
macro_rules! allow_deprecated {
    ($($tt:tt)*) => {
        #[allow(deprecated)]
        {
            $($tt)*
        }
    };
}

/// Migration checklist trait for tracking migration progress.
///
/// Implement this trait for your migration tests to ensure all migration
/// steps have been completed.
///
/// # Example
///
/// ```rust,ignore
/// use k8s_maestro::migration::MigrationChecklist;
///
/// struct MyMigrationTests;
///
/// impl MigrationChecklist for MyMigrationTests {
///     fn completed_steps() -> Vec<&'static str> {
///         vec![
///             "Updated imports",
///             "Migrated client creation",
///             "Replaced Job with Workflow",
///             "Moved dry_run to client builder",
///         ]
///     }
/// }
/// ```
pub trait MigrationChecklist {
    /// Returns a list of completed migration steps.
    fn completed_steps() -> Vec<&'static str>;

    /// Returns the total number of migration steps.
    fn total_steps() -> usize {
        6
    }

    /// Calculates migration progress as a percentage.
    fn migration_progress() -> f64 {
        let completed = Self::completed_steps().len() as f64;
        let total = Self::total_steps() as f64;
        (completed / total) * 100.0
    }

    /// Returns true if migration is complete.
    fn is_migration_complete() -> bool {
        Self::completed_steps().len() == Self::total_steps()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_alias_job_builder_compiles() {
        #[allow(deprecated)]
        let _builder: JobBuilder = WorkflowBuilder::new();
    }

    #[test]
    fn test_type_alias_job_compiles() {
        #[allow(deprecated)]
        let _type_check: std::marker::PhantomData<Job> = std::marker::PhantomData;
    }

    #[test]
    fn test_type_alias_execution_mode_compiles() {
        #[allow(deprecated)]
        let _type_check: std::marker::PhantomData<JobExecutionMode> = std::marker::PhantomData;
    }

    #[test]
    fn test_create_dry_run_client() {
        #[allow(deprecated)]
        let result = create_dry_run_client();
        assert!(result.is_ok());
        let client = result.unwrap();
        assert!(client.dry_run());
    }

    #[test]
    fn test_create_production_client() {
        #[allow(deprecated)]
        let result = create_production_client();
        assert!(result.is_ok());
        let client = result.unwrap();
        assert!(!client.dry_run());
    }

    #[test]
    fn test_create_client_for_namespace() {
        #[allow(deprecated)]
        let result = create_client_for_namespace("test-namespace");
        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.namespace(), "test-namespace");
    }

    #[test]
    fn test_allow_deprecated_macro() {
        allow_deprecated! {
            #[allow(deprecated)]
            let _builder: JobBuilder = WorkflowBuilder::new();
        }
    }

    #[test]
    fn test_migration_checklist_progress() {
        struct TestMigration;

        impl MigrationChecklist for TestMigration {
            fn completed_steps() -> Vec<&'static str> {
                vec!["Step 1", "Step 2", "Step 3"]
            }

            fn total_steps() -> usize {
                6
            }
        }

        assert_eq!(TestMigration::migration_progress(), 50.0);
        assert!(!TestMigration::is_migration_complete());
    }

    #[test]
    fn test_migration_checklist_complete() {
        struct CompleteMigration;

        impl MigrationChecklist for CompleteMigration {
            fn completed_steps() -> Vec<&'static str> {
                vec!["Step 1", "Step 2", "Step 3", "Step 4", "Step 5", "Step 6"]
            }
        }

        assert_eq!(CompleteMigration::migration_progress(), 100.0);
        assert!(CompleteMigration::is_migration_complete());
    }
}
