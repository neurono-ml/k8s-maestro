## Context

The current `MaestroK8sClient` requires passing `dry_run` parameters to each method call (`create_job`, `wait`, `delete_job`, etc.). This creates API verbosity and potential for inconsistencies across operations. The client configuration is minimal, relying mostly on Kubernetes client defaults. The codebase uses a pattern of builder structures for containers and jobs, suggesting this approach for the client as well.

The project is a Kubernetes job orchestrator library in Rust, with a goal of providing a high-level, type-safe API. Users need a clean, intuitive way to configure and interact with the system.

## Goals / Non-Goals

**Goals:**
- Provide fluent builder API for client construction
- Centralize configuration including `dry_run`, namespace, timeouts, logging, and resource limits
- Reduce API surface by removing redundant parameters from methods
- Maintain type safety and async patterns consistent with the codebase
- Support both testing (dry_run) and production workflows

**Non-Goals:**
- Backward compatibility with `MaestroK8sClient` (this is a breaking change)
- Auto-discovery of Kubernetes configuration (users must explicitly provide config)
- Connection pooling or advanced client lifecycle management (defer to kube-rs)
- Multi-cluster or multi-namespace operations within single client instance

## Decisions

### Use Builder Pattern with Fluent API
**Decision:** Implement `MaestroClientBuilder` with methods that return `Self` for chaining (`with_namespace()`, `with_dry_run()`, etc.)

**Rationale:**
- Consistent with existing patterns (`MaestroContainer`, `JobBuilder`)
- Provides clear API with compile-time validation
- Allows optional configuration with sensible defaults
- Industry standard for complex object construction

**Alternatives considered:**
- Direct struct initialization: Too verbose, less clear which fields are required
- Config struct passed to `new()`: Less discoverable, harder to add optional fields later
- Functional options pattern: More complex, less idiomatic in Rust

### Store Configuration in Immutable Struct
**Decision:** `MaestroClient` stores configuration in immutable fields after construction

**Rationale:**
- Thread-safe without interior mutability
- Prevents accidental runtime configuration changes
- Aligns with functional programming principles in async Rust

### Use Option<T> for Optional Fields
**Decision:** Optional configuration (kube config path, namespace, timeout) stored as `Option<T>`

**Rationale:**
- Clear semantic meaning of "not configured"
- Allows kube-rs to use its own defaults when None
- Avoids sentinel values that could conflict with real values

### Return impl Trait from API Methods
**Decision:** Return `impl WorkFlow` instead of concrete types from `create_workflow`, `get_workflow`, `list_workflows`

**Rationale:**
- Allows implementation flexibility
- Reduces coupling to concrete types
- Enables future optimization or type changes without API breakage
- Consistent with Rust idioms for trait objects

### Dry Run as Runtime Configuration
**Decision:** Store `dry_run: bool` in client configuration, apply at execution time

**Rationale:**
- Separates test mode from production mode cleanly
- Eliminates parameter passing complexity
- Makes test setup explicit and intentional
- Cannot be accidentally changed mid-operation (immutable)

## Risks / Trade-offs

**Risk:** Breaking change requires all users to migrate code
- **Mitigation:** Comprehensive migration guide in documentation, clear compile-time errors point to needed changes

**Risk:** Users may not configure required options leading to runtime errors
- **Mitigation:** Provide clear defaults, validate configuration in `build()`, document required vs optional

**Risk:** Builder complexity may obscure which configuration is actually applied
- **Mitigation:** Clear documentation, logical grouping of methods, sensible defaults for all options

**Trade-off:** Immutability means new client instance needed for different configurations
- **Acceptable:** Configuration should not change frequently; clients are lightweight to create

**Risk:** Workflow trait not fully specified yet
- **Mitigation:** Design API generically, define WorkFlow trait bounds clearly, allow flexibility for future implementation

## Migration Plan

### Phase 1: Implementation
1. Create `src/client/` directory structure
2. Implement `MaestroClient` and `MaestroClientBuilder`
3. Add WorkFlow trait definition (if not exists)
4. Migrate core functionality from `MaestroK8sClient`
5. Add unit and integration tests

### Phase 2: Internal Migration
1. Update existing code to use new `MaestroClient`
2. Update all integration tests
3. Remove old `MaestroK8sClient` after migration complete
4. Rename `src/clients/` to `src/client/`

### Phase 3: Documentation
1. Update README with builder examples
2. Add migration guide for existing users
3. Update API documentation

### Rollback Strategy
- Keep both clients temporarily during migration
- Feature flag can control which client is used
- Git history allows reverting to `MaestroK8sClient` if issues arise

## Open Questions

- What are the exact methods and trait bounds for `WorkFlow`? (Needs exploration of existing workflow code)
- Should `build()` validate kube config accessibility or defer to first operation?
- What are sensible defaults for timeout and resource limits?
- Should the client support re-authentication or token refresh automatically?
