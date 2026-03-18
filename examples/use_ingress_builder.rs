use k8s_maestro::{IngressBuilder, IngressPath, PathType, TLSConfig};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    println!("Ingress Builder Examples\n=========================\n");

    println!("Example 1: Basic Ingress");
    let basic_ingress = IngressBuilder::new()
        .with_name("basic-ingress")
        .with_namespace("default")
        .with_host("example.com")
        .with_path("/", "web-service", 80)
        .build()?;

    println!("Created ingress: {:?}", basic_ingress.metadata.name);
    let spec = basic_ingress.spec.as_ref().unwrap();
    let rules = spec.rules.as_ref().unwrap();
    let host = rules[0].host.clone();

    println!("Host: {:?}", host);

    println!("\nExample 2: Ingress with TLS");
    let tls_ingress = IngressBuilder::new()
        .with_name("secure-ingress")
        .with_namespace("default")
        .with_host("secure.example.com")
        .with_path("/", "secure-service", 443)
        .with_tls_secret("tls-secret")
        .build()?;

    println!("Created TLS ingress: {:?}", tls_ingress.metadata.name);
    let spec = tls_ingress.spec.as_ref().unwrap();
    let tls = spec.tls.as_ref().unwrap();
    let secret_name = tls[0].secret_name.clone();

    println!("TLS secret: {:?}", secret_name);

    println!("\nExample 3: Ingress with Multiple Paths");
    let paths = vec![
        IngressPath {
            path: "/api".to_string(),
            path_type: PathType::Prefix,
            service_name: "api-service".to_string(),
            service_port: 8080,
        },
        IngressPath {
            path: "/static".to_string(),
            path_type: PathType::Prefix,
            service_name: "static-service".to_string(),
            service_port: 80,
        },
        IngressPath {
            path: "/metrics".to_string(),
            path_type: PathType::Exact,
            service_name: "metrics-service".to_string(),
            service_port: 9090,
        },
    ];

    let multi_path_ingress = IngressBuilder::new()
        .with_name("multi-path-ingress")
        .with_namespace("default")
        .with_host("app.example.com")
        .with_paths(paths)
        .build()?;

    println!(
        "Created multi-path ingress: {:?}",
        multi_path_ingress.metadata.name
    );
    let spec = multi_path_ingress.spec.as_ref().unwrap();
    let rules = spec.rules.as_ref().unwrap();
    let http = rules[0].http.as_ref().unwrap();

    println!("Number of paths: {:?}", http.paths.len());

    println!("\nExample 4: Ingress with Annotations and Class");
    let mut annotations = BTreeMap::new();
    annotations.insert(
        "nginx.ingress.kubernetes.io/rewrite-target".to_string(),
        "/$2".to_string(),
    );
    annotations.insert(
        "nginx.ingress.kubernetes.io/use-regex".to_string(),
        "true".to_string(),
    );
    annotations.insert(
        "cert-manager.io/cluster-issuer".to_string(),
        "letsencrypt-prod".to_string(),
    );

    let annotated_ingress = IngressBuilder::new()
        .with_name("annotated-ingress")
        .with_namespace("default")
        .with_host("api.example.com")
        .with_path("/api(/|$)(.*)", "api-service", 8080)
        .with_annotations(annotations)
        .with_ingress_class("nginx")
        .build()?;

    println!(
        "Created annotated ingress: {:?}",
        annotated_ingress.metadata.name
    );
    println!(
        "Ingress class: {:?}",
        annotated_ingress.spec.unwrap().ingress_class_name.unwrap()
    );
    println!(
        "Annotations: {:?}",
        annotated_ingress.metadata.annotations.unwrap()
    );

    println!("\nExample 5: Ingress with Full TLS Configuration");
    let tls_config = TLSConfig {
        hosts: vec![
            "app1.example.com".to_string(),
            "app2.example.com".to_string(),
        ],
        secret_name: "multi-cert-secret".to_string(),
    };

    let multi_tls_ingress = IngressBuilder::new()
        .with_name("multi-tls-ingress")
        .with_namespace("default")
        .with_host("app1.example.com")
        .with_path("/", "app1-service", 80)
        .with_tls_config(tls_config)
        .build()?;

    println!(
        "Created multi-TLS ingress: {:?}",
        multi_tls_ingress.metadata.name
    );
    let spec = multi_tls_ingress.spec.as_ref().unwrap();
    let tls = spec.tls.as_ref().unwrap();
    let hosts = tls[0].hosts.as_ref().unwrap();

    println!("TLS hosts: {:?}", hosts);

    Ok(())
}
