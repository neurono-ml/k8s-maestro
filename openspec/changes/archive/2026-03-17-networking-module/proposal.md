## Why

Workflow steps currently cannot expose network services for inter-step communication. This prevents patterns where one step needs to receive requests from another step or from external systems. Users must manually create Kubernetes Service and Ingress resources, which breaks the fluent API experience and requires deep Kubernetes knowledge.

## What Changes

- Add `src/networking/` module with service and ingress builders
- Create `ServiceBuilder` for creating Kubernetes Services with fluent API
- Create `IngressBuilder` for creating Kubernetes Ingress resources with fluent API
- Add DNS utilities for generating service discovery names
- Support all service types: ClusterIP, Headless, NodePort, LoadBalancer
- Support TLS configuration for Ingress resources
- Add comprehensive unit and integration tests

## Capabilities

### New Capabilities

- `service-builder`: Builder pattern for creating Kubernetes Services with support for all service types (ClusterIP, Headless, NodePort, LoadBalancer), port configuration, selectors, and advanced options like session affinity and external traffic policy
- `ingress-builder`: Builder pattern for creating Kubernetes Ingress resources with host/path routing, TLS configuration, annotations, and ingress class support
- `dns-utilities`: DNS name generation utilities for service discovery, including service DNS names, pod DNS names, and headless service DNS patterns

### Modified Capabilities

None - this is a new module addition

## Impact

- New module `k8s-maestro::networking` added to the public API
- Dependencies: `k8s-openapi` Service and Ingress types already available
- No breaking changes to existing code
- Enables new workflow patterns for inter-step communication
