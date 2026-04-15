# Service and Ingress

Maestro provides builders for Kubernetes networking resources: Services and Ingresses. These builders enable you to expose applications within the cluster and route external traffic to your workloads.

## Overview

The networking module provides three main components:

| Component | Description |
|-----------|-------------|
| `ServiceBuilder` | Create Kubernetes Service resources for internal and external service exposure |
| `IngressBuilder` | Create Kubernetes Ingress resources for HTTP/HTTPS routing |
| DNS Utilities | Helper functions for generating Kubernetes DNS names |

Import the networking module:

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType, ServicePort};
use k8s_maestro::networking::{IngressBuilder, PathType, IngressPath, TLSConfig};
use k8s_maestro::networking::dns::{service_dns_name, pod_dns_name, headless_service_dns_pattern};
```

---

## ServiceBuilder

`ServiceBuilder` creates Kubernetes Service resources that provide stable network endpoints for pods. Services enable communication between components within the cluster and expose applications externally.

### Service Types

Kubernetes supports four Service types:

| Type | Description | Use Case |
|------|-------------|----------|
| `ClusterIP` | Internal cluster IP (default) | Internal microservice communication |
| `Headless` | No cluster IP, DNS-based discovery | Stateful workloads, custom service discovery |
| `NodePort` | Exposes on each node's static port | Development, testing |
| `LoadBalancer` | External load balancer | Production cloud deployments |

### Builder API

#### Constructor

```rust
ServiceBuilder::new()
```

Creates a new ServiceBuilder with default values (ClusterIP type, no ports).

#### Configuration Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `with_name(name)` | Sets the service name | `Self` |
| `with_namespace(namespace)` | Sets the namespace | `Self` |
| `with_port(port, target_port, protocol)` | Adds a single port | `Self` |
| `with_ports(ports)` | Sets all ports as a `Vec<ServicePort>` | `Self` |
| `with_type(service_type)` | Sets the ServiceType | `Self` |
| `with_selector(labels)` | Sets pod selector as BTreeMap | `Self` |
| `with_cluster_ip(ip)` | Sets the cluster IP | `Self` |
| `with_session_affinity(affinity)` | Sets session affinity (e.g., "ClientIP") | `Self` |
| `with_external_traffic_policy(policy)` | Sets external traffic policy ("Cluster" or "Local") | `Self` |
| `build()` | Builds the Service resource | `Result<Service>` |

### ServicePort Structure

```rust
pub struct ServicePort {
    pub port: i32,           // Service port (external)
    pub target_port: i32,    // Container port (internal)
    pub protocol: String,    // "TCP" or "UDP"
    pub name: Option<String>, // Port name (optional)
}
```

Default ServicePort: port 80, target port 8080, TCP protocol.

### Examples

#### Basic ClusterIP Service

A simple ClusterIP service for internal communication:

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType};
use k8s_openapi::api::core::v1::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service: Service = ServiceBuilder::new()
        .with_name("my-service")
        .with_namespace("default")
        .with_type(ServiceType::ClusterIP)
        .with_port(80, 8080, "TCP")
        .with_selector(std::collections::BTreeMap::from([
            "app".to_string(), "my-app".to_string(),
        ]))
        .build()?;

    println!("Created service: {}", service.metadata.name.unwrap());
    println!("Cluster IP: {:?}", service.spec.unwrap().cluster_ip);

    Ok(())
}
```

#### NodePort Service

Exposes the service on a static port on each node:

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = ServiceBuilder::new()
        .with_name("web-service")
        .with_namespace("production")
        .with_type(ServiceType::NodePort)
        .with_port(80, 8080, "TCP")
        .with_selector(std::collections::BTreeMap::from([
            "app".to_string(), "web".to_string(),
        ]))
        .build()?;

    println!("NodePort service created: {}", service.metadata.name.unwrap());

    Ok(())
}
```

#### Headless Service

Headless services return pod endpoints directly via DNS without a virtual IP:

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let selector = BTreeMap::from([
        "app".to_string(), "stateful".to_string(),
    ]);

    let service = ServiceBuilder::new()
        .with_name("stateful-service")
        .with_namespace("default")
        .with_type(ServiceType::Headless)
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .build()?;

    let spec = service.spec.unwrap();
    assert_eq!(spec.cluster_ip, Some("None".to_string()));

    println!("Headless service created: {}", service.metadata.name.unwrap());

    Ok(())
}
```

#### LoadBalancer Service with Session Affinity

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = ServiceBuilder::new()
        .with_name("lb-service")
        .with_namespace("production")
        .with_type(ServiceType::LoadBalancer)
        .with_port(443, 8443, "TCP")
        .with_selector(std::collections::BTreeMap::from([
            "app".to_string(), "secure-app".to_string(),
        ]))
        .with_session_affinity("ClientIP")
        .with_external_traffic_policy("Local")
        .build()?;

    println!("LoadBalancer service created: {}", service.metadata.name.unwrap());

    Ok(())
}
```

#### Multiple Ports

```rust
use k8s_maestro::networking::{ServiceBuilder, ServiceType, ServicePort};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ports = vec![
        ServicePort {
            port: 80,
            target_port: 8080,
            protocol: "TCP".to_string(),
            name: Some("http".to_string()),
        },
        ServicePort {
            port: 443,
            target_port: 8443,
            protocol: "TCP".to_string(),
            name: Some("https".to_string()),
        },
    ];

    let service = ServiceBuilder::new()
        .with_name("multi-port-service")
        .with_namespace("default")
        .with_type(ServiceType::ClusterIP)
        .with_ports(ports)
        .with_selector(std::collections::BTreeMap::from([
            "app".to_string(), "web".to_string(),
        ]))
        .build()?;

    let spec = service.spec.unwrap();
    println!("Ports: {}", spec.ports.unwrap().len());

    Ok(())
}
```

---

## IngressBuilder

`IngressBuilder` creates Kubernetes Ingress resources that manage external access to services, typically HTTP/HTTPS traffic. Ingresses provide path-based routing, host-based routing, and TLS termination.

### PathType

Ingress paths can match requests in different ways:

| Type | Description | Example |
|------|-------------|---------|
| `Exact` | Exact string match | `/api` matches only `/api` |
| `Prefix` | URL prefix match | `/api` matches `/api`, `/api/v1` |
| `ImplementationSpecific` | Provider-specific behavior | Default, depends on ingress controller |

### Builder API

#### Constructor

```rust
IngressBuilder::new()
```

Creates a new IngressBuilder with default values (no paths, no TLS).

#### Configuration Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `with_name(name)` | Sets the ingress name | `Self` |
| `with_namespace(namespace)` | Sets the namespace | `Self` |
| `with_host(host)` | Sets the hostname | `Self` |
| `with_path(path, service_name, service_port)` | Adds a single path | `Self` |
| `with_paths(paths)` | Sets all paths as a `Vec<IngressPath>` | `Self` |
| `with_tls_secret(secret_name)` | Adds TLS with secret name | `Self` |
| `with_tls_config(tls)` | Adds TLS with full TLSConfig | `Self` |
| `with_annotations(annotations)` | Sets annotations as BTreeMap | `Self` |
| `with_ingress_class(class_name)` | Sets the ingress class | `Self` |
| `build()` | Builds the Ingress resource | `Result<Ingress>` |

### Supporting Structures

#### IngressPath

```rust
pub struct IngressPath {
    pub path: String,           // URL path
    pub path_type: PathType,    // Match type
    pub service_name: String,  // Backend service name
    pub service_port: i32,      // Backend service port
}
```

Default: path "/", Prefix type, service_port 80.

#### TLSConfig

```rust
pub struct TLSConfig {
    pub hosts: Vec<String>,   // Hostnames for TLS
    pub secret_name: String, // Secret containing TLS certificate
}
```

### Examples

#### Basic Ingress

A simple ingress routing all requests to a backend service:

```rust
use k8s_maestro::networking::IngressBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ingress = IngressBuilder::new()
        .with_name("web-ingress")
        .with_namespace("default")
        .with_host("example.com")
        .with_path("/", "web-service", 80)
        .with_ingress_class("nginx")
        .build()?;

    println!("Created ingress: {}", ingress.metadata.name.unwrap());

    Ok(())
}
```

#### Ingress with TLS

Secure the ingress with TLS using a Kubernetes secret:

```rust
use k8s_maestro::networking::{IngressBuilder, TLSConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tls_config = TLSConfig {
        hosts: vec!["example.com".to_string()],
        secret_name: "example-tls-secret".to_string(),
    };

    let ingress = IngressBuilder::new()
        .with_name("secure-ingress")
        .with_namespace("production")
        .with_host("example.com")
        .with_path("/", "web-service", 80)
        .with_tls_config(tls_config)
        .with_ingress_class("nginx")
        .build()?;

    let spec = ingress.spec.unwrap();
    let tls = spec.tls.unwrap().first().unwrap();
    println!("TLS secret: {}", tls.secret_name.as_ref().unwrap());

    Ok(())
}
```

#### Multi-Path Ingress

Route different paths to different services:

```rust
use k8s_maestro::networking::{IngressBuilder, IngressPath, PathType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let paths = vec![
        IngressPath {
            path: "/".to_string(),
            path_type: PathType::Prefix,
            service_name: "web-frontend".to_string(),
            service_port: 80,
        },
        IngressPath {
            path: "/api".to_string(),
            path_type: PathType::Prefix,
            service_name: "api-backend".to_string(),
            service_port: 8080,
        },
        IngressPath {
            path: "/admin".to_string(),
            path_type: PathType::Exact,
            service_name: "admin-frontend".to_string(),
            service_port: 80,
        },
    ];

    let ingress = IngressBuilder::new()
        .with_name("multi-path-ingress")
        .with_namespace("default")
        .with_host("example.com")
        .with_paths(paths)
        .with_ingress_class("nginx")
        .build()?;

    println!("Created multi-path ingress: {}", ingress.metadata.name.unwrap());

    Ok(())
}
```

#### Ingress with Annotations

Add annotations for ingress controller-specific configuration:

```rust
use k8s_maestro::networking::IngressBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let annotations = BTreeMap::from([
        ("nginx.ingress.kubernetes.io/rewrite-target".to_string(), "/".to_string()),
        ("nginx.ingress.kubernetes.io/ssl-redirect".to_string(), "true".to_string()),
    ]);

    let ingress = IngressBuilder::new()
        .with_name("annotated-ingress")
        .with_namespace("default")
        .with_host("example.com")
        .with_path("/path".to_string(), "backend-service".to_string(), 80)
        .with_annotations(annotations)
        .with_ingress_class("nginx")
        .build()?;

    let metadata = ingress.metadata;
    println!("Annotations: {:?}", metadata.annotations);

    Ok(())
}
```

#### Ingress with Multiple Hosts

```rust
use k8s_maestro::networking::{IngressBuilder, TLSConfig, PathType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tls_config = TLSConfig {
        hosts: vec![
            "example.com".to_string(),
            "www.example.com".to_string(),
        ],
        secret_name: "multi-host-tls".to_string(),
    };

    let ingress = IngressBuilder::new()
        .with_name("multi-host-ingress")
        .with_namespace("production")
        .with_host("example.com")
        .with_path("/", "default-service", 80)
        .with_path("/api".to_string(), "api-service".to_string(), 8080)
        .with_tls_config(tls_config)
        .with_ingress_class("nginx")
        .build()?;

    println!("Created multi-host ingress: {}", ingress.metadata.name.unwrap());

    Ok(())
}
```

---

## DNS Utilities

Maestro provides utility functions for generating Kubernetes DNS names. These follow the standard Kubernetes DNS naming convention.

### Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `service_dns_name(service, namespace)` | FQDN for a service | `"<service>.<namespace>.svc.cluster.local"` |
| `pod_dns_name(pod, namespace)` | FQDN for a pod | `"<pod>.<namespace>.pod.cluster.local"` |
| `headless_service_dns_pattern(service, namespace)` | Wildcard pattern for headless service | `"*.<service>.<namespace>.svc.cluster.local"` |

### Examples

#### Service DNS Name

```rust
use k8s_maestro::networking::dns::service_dns_name;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dns = service_dns_name("web-service", "production");
    println!("Service DNS: {}", dns);
    // Output: web-service.production.svc.cluster.local

    Ok(())
}
```

#### Pod DNS Name

```rust
use k8s_maestro::networking::dns::pod_dns_name;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dns = pod_dns_name("my-pod-xyz123", "default");
    println!("Pod DNS: {}", dns);
    // Output: my-pod-xyz123.default.pod.cluster.local

    Ok(())
}
```

#### Headless Service DNS Pattern

```rust
use k8s_maestro::networking::dns::headless_service_dns_pattern;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pattern = headless_service_dns_pattern("headless-service", "default");
    println!("Headless DNS pattern: {}", pattern);
    // Output: *.headless-service.default.svc.cluster.local

    Ok(())
}
```

---

## Combined Examples

The following examples demonstrate how to combine networking resources with `KubeJobStep` for complete application deployment.

### Job + Service (expose_service)

Create a job and expose it with a ClusterIP service:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::networking::{ServiceBuilder, ServiceType};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "default";

    let selector = BTreeMap::from([
        "job-name".to_string(), "web-job".to_string(),
    ]);

    let container = MaestroContainer::new("nginx:latest", "main")
        .set_arguments(&["nginx".to_string(), "-g".to_string(), "daemon off;".to_string()]);

    let job = KubeJobStepBuilder::new()
        .with_name("web-job")
        .with_namespace(namespace)
        .add_container(Box::new(container))
        .with_client(client.clone())
        .build()?;

    let service = ServiceBuilder::new()
        .with_name("web-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(80, 80, "TCP")
        .with_selector(selector)
        .build()?;

    println!("Created job: {}", job.resource_name());
    println!("Created service: {}", service.metadata.name.unwrap());
    println!("Service DNS: {}.{}.svc.cluster.local", 
             service.metadata.name.unwrap(), 
             namespace);

    Ok(())
}
```

### Job + Ingress (expose_ingress)

Create a job and expose it via Ingress:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::networking::{IngressBuilder, ServiceBuilder, ServiceType};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "production";

    let selector = BTreeMap::from([
        "app".to_string(), "web".to_string(),
    ]);

    let container = MaestroContainer::new("my-web-app:latest", "main")
        .set_arguments(&["npm".to_string(), "start".to_string()]);

    let job = KubeJobStepBuilder::new()
        .with_name("web-app")
        .with_namespace(namespace)
        .add_container(Box::new(container))
        .with_client(client.clone())
        .build()?;

    let service = ServiceBuilder::new()
        .with_name("web-app-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(80, 3000, "TCP")
        .with_selector(selector)
        .build()?;

    let ingress = IngressBuilder::new()
        .with_name("web-app-ingress")
        .with_namespace(namespace)
        .with_host("web.example.com")
        .with_path("/", "web-app-service", 80)
        .with_ingress_class("nginx")
        .build()?;

    println!("Created job: {}", job.resource_name());
    println!("Created service: {}", service.metadata.name.unwrap());
    println!("Created ingress: {}", ingress.metadata.name.unwrap());
    println!("Access at: https://web.example.com");

    Ok(())
}
```

### Ingress + TLS + NetworkPolicy

Create a complete secure deployment with Ingress, TLS, and NetworkPolicy:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::networking::{IngressBuilder, ServiceBuilder, ServiceType, TLSConfig};
use k8s_maestro::networking::dns::service_dns_name;
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "secure-app";

    let selector = BTreeMap::from([
        "app".to_string(), "secure-app".to_string(),
    ]);

    let container = MaestroContainer::new("my-secure-app:latest", "main")
        .set_arguments(&["node".to_string(), "server.js".to_string()])
        .set_environment_variables(BTreeMap::from([
            "PORT".to_string(), "8080".to_string(),
            "NODE_ENV".to_string(), "production".to_string(),
        ]));

    let job = KubeJobStepBuilder::new()
        .with_name("secure-app")
        .with_namespace(namespace)
        .add_container(Box::new(container))
        .with_client(client.clone())
        .build()?;

    let service = ServiceBuilder::new()
        .with_name("secure-app-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(443, 8080, "TCP")
        .with_selector(selector)
        .build()?;

    let tls_config = TLSConfig {
        hosts: vec!["secure.example.com".to_string()],
        secret_name: "app-tls-secret".to_string(),
    };

    let annotations = BTreeMap::from([
        ("nginx.ingress.kubernetes.io/ssl-redirect".to_string(), "true".to_string()),
        ("nginx.ingress.kubernetes.io/proxy-body-size".to_string(), "50m".to_string()),
    ]);

    let ingress = IngressBuilder::new()
        .with_name("secure-app-ingress")
        .with_namespace(namespace)
        .with_host("secure.example.com")
        .with_path("/", "secure-app-service", 443)
        .with_tls_config(tls_config)
        .with_annotations(annotations)
        .with_ingress_class("nginx")
        .build()?;

    let service_dns = service_dns_name("secure-app-service", namespace);

    println!("=== Secure App Deployment ===");
    println!("Job: {}", job.resource_name());
    println!("Service: {}", service.metadata.name.unwrap());
    println!("Service DNS: {}", service_dns);
    println!("Ingress: {}", ingress.metadata.name.unwrap());
    println!("URL: https://secure.example.com");

    Ok(())
}
```

### Complete Multi-Service Architecture

Example of a complete architecture with frontend, backend, and API services:

```rust
use k8s_maestro::clients::MaestroK8sClient;
use k8s_maestro::entities::{MaestroContainer, ContainerLike};
use k8s_maestro::networking::{IngressBuilder, IngressPath, ServiceBuilder, ServiceType, PathType};
use k8s_maestro::steps::kubernetes::KubeJobStepBuilder;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MaestroK8sClient::new().await?;
    let namespace = "app-stack";

    // Frontend Job
    let frontend_selector = BTreeMap::from([("component".to_string(), "frontend".to_string())]);
    let frontend_container = MaestroContainer::new("nginx:alpine", "frontend");
    let frontend_job = KubeJobStepBuilder::new()
        .with_name("frontend")
        .with_namespace(namespace)
        .add_container(Box::new(frontend_container))
        .with_client(client.clone())
        .build()?;

    let frontend_service = ServiceBuilder::new()
        .with_name("frontend-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(80, 80, "TCP")
        .with_selector(frontend_selector)
        .build()?;

    // Backend Job
    let backend_selector = BTreeMap::from([("component".to_string(), "backend".to_string())]);
    let backend_container = MaestroContainer::new("my-backend:latest", "backend");
    let backend_job = KubeJobStepBuilder::new()
        .with_name("backend")
        .with_namespace(namespace)
        .add_container(Box::new(backend_container))
        .with_client(client.clone())
        .build()?;

    let backend_service = ServiceBuilder::new()
        .with_name("backend-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(8080, 8080, "TCP")
        .with_selector(backend_selector)
        .build()?;

    // API Job
    let api_selector = BTreeMap::from([("component".to_string(), "api".to_string())]);
    let api_container = MaestroContainer::new("my-api:latest", "api");
    let api_job = KubeJobStepBuilder::new()
        .with_name("api")
        .with_namespace(namespace)
        .add_container(Box::new(api_container))
        .with_client(client.clone())
        .build()?;

    let api_service = ServiceBuilder::new()
        .with_name("api-service")
        .with_namespace(namespace)
        .with_type(ServiceType::ClusterIP)
        .with_port(3000, 3000, "TCP")
        .with_selector(api_selector)
        .build()?;

    // Ingress with multiple paths
    let paths = vec![
        IngressPath {
            path: "/".to_string(),
            path_type: PathType::Prefix,
            service_name: "frontend-service".to_string(),
            service_port: 80,
        },
        IngressPath {
            path: "/api".to_string(),
            path_type: PathType::Prefix,
            service_name: "api-service".to_string(),
            service_port: 3000,
        },
        IngressPath {
            path: "/backend".to_string(),
            path_type: PathType::Prefix,
            service_name: "backend-service".to_string(),
            service_port: 8080,
        },
    ];

    let ingress = IngressBuilder::new()
        .with_name("app-ingress")
        .with_namespace(namespace)
        .with_host("app.example.com")
        .with_paths(paths)
        .with_ingress_class("nginx")
        .build()?;

    println!("=== Application Stack Deployment ===");
    println!("Jobs: frontend, backend, api");
    println!("Services: frontend-service, backend-service, api-service");
    println!("Ingress: app.example.com");

    Ok(())
}
```

---

## Error Handling

### Common Service Errors

**Missing Required Fields**

```rust
// Error: Service name is required
let service = ServiceBuilder::new()
    .with_namespace("default")
    .with_port(80, 8080, "TCP")
    .build();
// Returns: Err("Service name is required")
```

**No Ports Specified**

```rust
// Error: Build succeeds but service has no ports
let service = ServiceBuilder::new()
    .with_name("empty-service")
    .with_namespace("default")
    .build()?;
// Service created but not useful without ports
```

### Common Ingress Errors

**No Host Specified**

```rust
// Error: Host is required
let ingress = IngressBuilder::new()
    .with_name("ingress")
    .with_namespace("default")
    .with_path("/", "service", 80)
    .build();
// Returns: Err("Host is required")
```

**No Paths Specified**

```rust
// Error: At least one path is required
let ingress = IngressBuilder::new()
    .with_name("ingress")
    .with_namespace("default")
    .with_host("example.com")
    .build();
// Returns: Err("At least one path is required")
```

---

## Related Resources

- [KubeJobStep](./k8s-job-step.md) - Creating Kubernetes Jobs
- [MaestroContainer](./containers.md) - Container configuration
- [Basic Workflows](./basic-workflow.md) - Workflow patterns
- [Configuration Reference](../reference/configuration.md) - Configuration options
- [Troubleshooting](../reference/troubleshooting.md) - Common issues and solutions

## API Links

- **Source Code**: [`src/networking/service.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/networking/service.rs)
- **Source Code**: [`src/networking/ingress.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/networking/ingress.rs)
- **Source Code**: [`src/networking/dns.rs`](https://github.com/k8s-maestro/k8s-maestro/blob/main/src/networking/dns.rs)
- **docs.rs**: [ServiceBuilder documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/networking/struct.ServiceBuilder.html)
- **docs.rs**: [IngressBuilder documentation](https://docs.rs/k8s-maestro/latest/k8s_maestro/networking/struct.IngressBuilder.html)