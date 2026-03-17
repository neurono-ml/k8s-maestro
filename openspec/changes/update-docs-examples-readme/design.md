## Context

K8s-Maestro is a Kubernetes workflow orchestrator library written in Rust. The current documentation consists of a minimal README (19 lines) and no structured documentation site. The codebase has evolved to include workflow builders, dependency chains, conditional execution, and multi-step workflows, but this is not reflected in the documentation.

The library provides:
- `WorkflowBuilder` for creating multi-step workflows
- `DependencyChain` and `DependencyGraph` for managing step dependencies
- `ConditionBuilder` for conditional execution with closures
- Various step traits: `WorkFlowStep`, `KubeWorkFlowStep`, `ExecutableWorkFlowStep`, etc.
- Resource management with `ResourceLimits`
- Checkpointing support with `CheckpointConfig`

## Goals / Non-Goals

**Goals:**
- Create professional README with badges, features, and usage examples
- Establish GitHub Pages documentation structure in `site-docs/`
- Update examples to demonstrate workflow-centric API
- Add new examples for services, sidecars, and multi-language steps
- Ensure all examples compile and demonstrate real use cases

**Non-Goals:**
- Deploy GitHub Pages (just create the structure)
- Change any library code
- Create actual implementations for Python/Rust/WASM steps (examples show intended API usage)
- Add CI/CD for documentation deployment

## Decisions

### 1. README Structure
**Decision**: Use standard Rust crate README format with badges at top
**Rationale**: Follows conventions from popular crates like tokio, serde, kube-rs
**Alternatives considered**: 
- Minimal README → Rejected: insufficient for adoption
- Wiki-only → Rejected: README is first touchpoint

### 2. Site-Docs Organization
**Decision**: Use hierarchical structure: getting-started, guides, api, examples, reference
**Rationale**: Follows documentation patterns from Kubernetes, Argo Workflows, and Temporal
**Alternatives considered**:
- Flat structure → Rejected: harder to navigate
- Single-page → Rejected: too long for complex docs

### 3. Example Naming Convention
**Decision**: Rename to `use_*.rs` pattern for consistency, `*_workflow.rs` for workflow examples
**Rationale**: Clear intent from filename, groups related examples
**Alternatives considered**:
- Keep existing names → Rejected: inconsistent with new API focus
- Numbered examples → Rejected: less descriptive

### 4. Multi-Language Step Examples
**Decision**: Create placeholder examples showing intended API, not full implementations
**Rationale**: Library doesn't yet have Python/Rust/WASM step implementations; examples serve as API design
**Alternatives considered**:
- Skip these examples → Rejected: users need to see planned capabilities
- Full implementations → Rejected: out of scope for documentation task

### 5. Badge Selection
**Decision**: Include crates.io, docs.rs, license, build status, and version badges
**Rationale**: Standard professional Rust crate presentation
**Alternatives considered**:
- More badges (downloads, Discord) → Can be added later
- No badges → Rejected: looks unprofessional

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Examples may not compile if API changes | Use actual current API where possible; mark planned examples clearly |
| Documentation may become stale | Include version numbers and "last updated" references |
| Multi-language examples are aspirational | Add clear comments that these show planned API |
| Large documentation set takes time to maintain | Focus on core use cases first |

## Migration Plan

1. Create new documentation files (no migration needed - additive change)
2. Rename example files with git mv to preserve history
3. Update examples with new API patterns
4. Verify all examples compile
5. No rollback needed - pure documentation addition

## Open Questions

None - scope is well-defined documentation update.
