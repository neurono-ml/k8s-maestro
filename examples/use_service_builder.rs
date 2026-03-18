use k8s_maestro::{ServiceBuilder, ServicePort, ServiceType};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    println!("Service Builder Examples\n=========================\n");

    println!("Example 1: Basic ClusterIP Service");
    let mut selector = BTreeMap::new();
    selector.insert("app".to_string(), "webapp".to_string());

    let clusterip_service = ServiceBuilder::new()
        .with_name("web-service")
        .with_namespace("default")
        .with_port(80, 8080, "TCP")
        .with_selector(selector)
        .with_type(ServiceType::ClusterIP)
        .build()?;

    println!("Created service: {:?}", clusterip_service.metadata.name);
    println!("Service type: {:?}", clusterip_service.spec.unwrap().type_);

    println!("\nExample 2: Headless Service for StatefulSet");
    let mut stateful_selector = BTreeMap::new();
    stateful_selector.insert("app".to_string(), "database".to_string());

    let headless_service = ServiceBuilder::new()
        .with_name("stateful-db")
        .with_namespace("default")
        .with_port(5432, 5432, "TCP")
        .with_selector(stateful_selector)
        .with_type(ServiceType::Headless)
        .build()?;

    println!(
        "Created headless service: {:?}",
        headless_service.metadata.name
    );
    println!("ClusterIP: {:?}", headless_service.spec.unwrap().cluster_ip);

    println!("\nExample 3: LoadBalancer Service");
    let mut lb_selector = BTreeMap::new();
    lb_selector.insert("app".to_string(), "frontend".to_string());

    let lb_service = ServiceBuilder::new()
        .with_name("frontend-lb")
        .with_namespace("default")
        .with_port(80, 80, "TCP")
        .with_selector(lb_selector)
        .with_type(ServiceType::LoadBalancer)
        .with_session_affinity("ClientIP")
        .with_external_traffic_policy("Local")
        .build()?;

    println!(
        "Created LoadBalancer service: {:?}",
        lb_service.metadata.name
    );
    println!(
        "Session affinity: {:?}",
        lb_service.spec.unwrap().session_affinity.unwrap()
    );

    println!("\nExample 4: Service with Multiple Ports");
    let mut multi_selector = BTreeMap::new();
    multi_selector.insert("app".to_string(), "api".to_string());

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

    let multi_port_service = ServiceBuilder::new()
        .with_name("api-service")
        .with_namespace("default")
        .with_ports(ports)
        .with_selector(multi_selector)
        .build()?;

    println!(
        "Created multi-port service: {:?}",
        multi_port_service.metadata.name
    );
    println!(
        "Number of ports: {:?}",
        multi_port_service.spec.unwrap().ports.unwrap().len()
    );

    Ok(())
}
