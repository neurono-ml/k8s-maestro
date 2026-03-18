//! This example demonstrates creating and managing Kubernetes Services.
//!
//! The example shows:
//! - Creating a Service with ServiceBuilder
//! - Configuring service types (ClusterIP, NodePort, LoadBalancer)
//! - Setting service selectors and ports
//! - Exposing services with different configurations

use k8s_maestro::{ServiceBuilder, ServiceType};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    println!("=== Kubernetes Service Examples ===\n");

    println!("Example 1: ClusterIP Service (default)");
    example_clusterip_service()?;

    println!("\nExample 2: NodePort Service");
    example_nodeport_service()?;

    println!("\nExample 3: LoadBalancer Service");
    example_loadbalancer_service()?;

    println!("\nExample 4: Service with multiple ports");
    example_multiport_service()?;

    println!("\nExample 5: Headless Service (ClusterIP with None)");
    example_headless_service()?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

fn example_clusterip_service() -> anyhow::Result<()> {
    println!("Creating a ClusterIP service (internal only)");

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "my-app".to_string());
    selector.insert("tier".to_string(), "frontend".to_string());

    let service = ServiceBuilder::new()
        .with_name("my-service")
        .with_namespace("default")
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    println!("Service created: {:?}", service);
    println!("Service will be accessible at: my-service.default.svc.cluster.local");
    Ok(())
}

fn example_nodeport_service() -> anyhow::Result<()> {
    println!("Creating a NodePort service (exposed on each node)");

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "web-app".to_string());

    let service = ServiceBuilder::new()
        .with_name("web-service")
        .with_namespace("default")
        .with_port(80, 80, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::NodePort)
        .build()?;

    println!("Service created: {:?}", service);
    println!("Service will be accessible at: <node-ip>:<node-port>");
    Ok(())
}

fn example_loadbalancer_service() -> anyhow::Result<()> {
    println!("Creating a LoadBalancer service (external load balancer)");

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "api-server".to_string());

    let service = ServiceBuilder::new()
        .with_name("api-service")
        .with_namespace("default")
        .with_port(443, 8443, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::LoadBalancer)
        .build()?;

    println!("Service created: {:?}", service);
    println!("Service will be accessible at: <external-ip>");
    Ok(())
}

fn example_multiport_service() -> anyhow::Result<()> {
    println!("Creating a service with multiple ports");

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "multiport-app".to_string());

    let mut service_builder = ServiceBuilder::new()
        .with_name("multiport-service")
        .with_namespace("default")
        .with_selector(selector);

    service_builder = service_builder
        .with_port(80, 8080, "TCP")
        .with_port(443, 8443, "TCP")
        .with_port(9090, 9090, "TCP");

    let service = service_builder.build()?;

    println!("Service created with multiple ports: {:?}", service);
    println!("Ports: 80->8080, 443->8443, 9090->9090");
    Ok(())
}

fn example_headless_service() -> anyhow::Result<()> {
    println!("Creating a headless service (for StatefulSets)");

    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "database".to_string());

    let service = ServiceBuilder::new()
        .with_name("headless-service")
        .with_namespace("default")
        .with_port(5432, 5432, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    println!("Service created: {:?}", service);
    println!("Headless service: no cluster IP, DNS returns pod IPs");
    println!("DNS pattern: *.headless-service.default.svc.cluster.local");
    Ok(())
}
