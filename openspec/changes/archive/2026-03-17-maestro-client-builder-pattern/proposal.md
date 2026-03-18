## Why

The current MaestroK8sClient scatters the `dry_run` parameter across function calls, making configuration management cumbersome and error-prone. A builder pattern provides cleaner API design, better configuration encapsulation, and more intuitive client setup while maintaining backward compatibility through migration path.

## What Changes

- **BREAKING**: Replace `MaestroK8sClient` with new `MaestroClient` using builder pattern
- Create `MaestroClientBuilder` with fluent configuration methods
- Move `dry_run` from individual function parameters to client configuration
- **BREAKING**: Rename directory `clients/` to `client/`
- Remove `dry_run` parameter from all client methods
- Add configuration options: namespace, kube config path, default timeout, log level, default resource limits
- Provide workflow management API: `create_workflow`, `get_workflow`, `list_workflows`

## Capabilities

### New Capabilities
- `maestro-client-builder`: Builder pattern for constructing MaestroClient with fluent API and centralized configuration
- `maestro-client-api`: Core client API providing workflow management operations (create, get, list)

### Modified Capabilities
- (None - new API replaces existing MaestroK8sClient)

## Impact

- **Breaking Change**: Existing code using `MaestroK8sClient` must migrate to `MaestroClient` with builder pattern
- API surface area reduced - dry_run parameters removed from all methods
- Directory structure change: `src/clients/` → `src/client/`
- Integration tests must be updated to use new client API
- Documentation needs updates to reflect new client creation pattern
