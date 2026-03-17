## Context

k8s-maestro is evolving from a simple job orchestrator to a comprehensive workflow orchestration system. The original API focused on individual Kubernetes Jobs with `JobBuilder` and `MaestroK8sClient`. The new API introduces `Workflow`, `WorkflowBuilder`, and a planned `MaestroClient` with builder pattern configuration, supporting multi-step workflows with dependencies, parallel execution, and checkpointing.

Users need clear migration guidance to adopt the new API without breaking existing implementations. This design covers the migration documentation structure and backward compatibility utilities.

## Goals / Non-Goals

**Goals:**
- Provide comprehensive migration documentation with before/after code examples
- Create optional migration utilities module with deprecation warnings and type aliases
- Document all breaking changes with clear migration paths
- Include common pitfalls and FAQ for troubleshooting
- Ensure migration guide examples are tested and accurate

**Non-Goals:**
- Automatically migrate user code
- Maintain full backward compatibility (old API may be deprecated)
- Create migration scripts or tools
- Change the new API design to accommodate old patterns

## Decisions

### D1: Migration Guide Structure
**Decision**: Single comprehensive document at `docs/migration-guide.md` with sections for overview, breaking changes, migration steps, code comparisons, pitfalls, and FAQ.

**Rationale**: A single document is easier to maintain and provides a complete reference. Users can search within one file rather than navigating multiple documents.

**Alternatives considered**:
- Multiple migration docs per component → Rejected: harder to maintain, users may miss relevant sections
- Inline documentation only → Rejected: not discoverable enough for migration planning

### D2: Backward Compatibility Approach
**Decision**: Provide type aliases and deprecation warnings in `src/migration/mod.rs` for gradual migration, not full backward compatibility.

**Rationale**: Full compatibility would require maintaining two parallel APIs indefinitely. Type aliases and deprecation warnings guide users without locking in the old design.

**Alternatives considered**:
- Full backward compatibility layer → Rejected: maintenance burden, delays migration
- No compatibility helpers → Rejected: abrupt breaking changes harm user experience

### D3: dry_run Parameter Migration
**Decision**: Document the shift from per-call `dry_run` parameter to client-level configuration in the new `MaestroClient` builder.

**Rationale**: Client-level dry_run is more consistent and reduces parameter repetition. The migration guide will show how to configure this once at client creation.

**Alternatives considered**:
- Keep dry_run on both levels → Rejected: confusing, two ways to do the same thing
- Remove dry_run entirely → Rejected: essential for testing and validation

### D4: Module Structure Changes
**Decision**: Document the transition from `entities::job` to `workflows` module, with clear mapping of types.

**Rationale**: The new module structure reflects the conceptual shift from jobs to workflows. Clear mapping helps users find equivalent types.

**Alternatives considered**:
- Keep old module structure with new types → Rejected: confusing naming
- Create compatibility re-exports → Considered: may add as type aliases

### D5: Migration Utilities Scope
**Decision**: Create a `migration` module with deprecation macros and type aliases only. No automatic conversion functions.

**Rationale**: Automatic conversion is complex and error-prone for trait-based systems. Simple aliases and warnings provide enough guidance.

**Alternatives considered**:
- Full conversion utilities → Rejected: too complex, may introduce bugs
- No utilities module → Rejected: loses opportunity for deprecation warnings

## Risks / Trade-offs

### Risk: Examples May Become Outdated
**Mitigation**: Include version numbers in examples, test examples during CI, add "last verified" dates.

### Risk: Users May Miss Migration Guide
**Mitigation**: Add prominent notice in README, CHANGELOG, and lib.rs documentation. Consider compile-time deprecation warnings.

### Risk: Type Aliases May Confuse IDEs
**Mitigation**: Document that aliases are for migration only, use clear naming (`LegacyJobBuilder` vs `JobBuilder`).

### Risk: New API May Still Evolve
**Mitigation**: Mark migration guide with the version it targets (0.4.0), update as API stabilizes.

## Migration Plan

### Phase 1: Documentation (This Change)
1. Create `docs/migration-guide.md` with comprehensive migration information
2. Create `src/migration/mod.rs` with deprecation utilities
3. Add deprecation warnings to existing examples
4. Update CHANGELOG.md with migration notes

### Phase 2: Rollout
1. Merge migration guide with version 0.4.0
2. Announce in release notes
3. Monitor GitHub issues for migration problems
4. Update guide based on user feedback

### Rollback Strategy
If critical issues arise:
1. Migration guide is documentation-only, can be updated without code changes
2. Type aliases can be extended for additional compatibility
3. Deprecation warnings can be adjusted based on user feedback

## Open Questions

1. Should we provide a `cargo fix` compatible lint for automatic migration suggestions?
2. Should the migration guide include performance comparisons between old and new APIs?
3. Should we create a migration example in the `examples/` directory?
4. How long should deprecation warnings exist before removal?
