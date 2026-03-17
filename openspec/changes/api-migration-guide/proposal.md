## Why

Users adopting k8s-maestro need clear guidance to migrate from the job-centric API (Job, JobBuilder, MaestroK8sClient) to the new workflow-centric API (Workflow, WorkflowBuilder). Without migration documentation and compatibility helpers, users face confusion and potential breakage when upgrading. This change ensures a smooth transition path for existing users while introducing the new, more powerful workflow system.

## What Changes

- **BREAKING**: `Job` concept renamed to `Workflow` (conceptual shift from single jobs to multi-step workflows)
- **BREAKING**: `JobBuilder` → `WorkflowBuilder` with new fluent API
- **BREAKING**: `MaestroK8sClient` → `MaestroClient` with builder pattern configuration
- **BREAKING**: `create_job()` → `create_workflow()` method rename
- **BREAKING**: `dry_run` parameter moved from individual function calls to client builder configuration
- **BREAKING**: `entities::job` module → `entities::workflows` module
- **BREAKING**: New traits system (`WorkFlowStep`, `ResourceLimitedStep`) replaces direct container manipulation
- New `ExecutionMode` enum (Sequential, Parallel) for workflow execution control
- New `CheckpointConfig` for workflow state persistence
- New dependency chain system for conditional step execution
- Deprecation warnings on old types (if kept for backward compatibility)
- Type aliases for backward compatibility where possible
- Migration utilities module with helper functions

## Capabilities

### New Capabilities
- `migration-guide`: Comprehensive documentation covering overview, breaking changes, migration steps, code comparisons, common pitfalls, and FAQ
- `migration-utilities`: Helper module with deprecation warnings, type aliases, and optional migration helpers for backward compatibility

### Modified Capabilities
- None (this is a documentation and compatibility addition)

## Impact

- **Documentation**: New `docs/migration-guide.md` file
- **Source Code**: New `src/migration/mod.rs` module with utilities
- **API Surface**: Deprecation attributes on old types, type aliases for compatibility
- **Users**: Clear migration path with code examples and troubleshooting
- **Dependencies**: No new external dependencies required
