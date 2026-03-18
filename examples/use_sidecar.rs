//! This example demonstrates adding sidecar containers to workflow steps.
//!
//! The example shows:
//! - Creating a main container with sidecar support
//! - Adding logging sidecars (e.g., log collectors)
//! - Adding monitoring sidecars (e.g., metrics exporters)
//! - Configuring sidecar resources and lifecycle

use k8s_maestro::{
    entities::{MaestroContainer, SidecarContainer},
    networking::SidecarPlugin,
};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    println!("=== Sidecar Container Examples ===\n");

    println!("Example 1: Logging sidecar with Fluent Bit");
    example_logging_sidecar()?;

    println!("\nExample 2: Monitoring sidecar with Prometheus exporter");
    example_monitoring_sidecar()?;

    println!("\nExample 3: Proxy sidecar with Envoy");
    example_proxy_sidecar()?;

    println!("\nExample 4: Multiple sidecars");
    example_multiple_sidecars()?;

    println!("\nExample 5: Sidecar with custom plugin");
    example_custom_plugin_sidecar()?;

    println!("\n=== All examples completed successfully ===");
    Ok(())
}

fn example_logging_sidecar() -> anyhow::Result<()> {
    println!("Creating a container with Fluent Bit logging sidecar");

    let mut main_container = MaestroContainer::new("nginx:latest", "web-server");
    main_container.set_port(80);
    main_container.set_labels([("app", "frontend")]);

    let logging_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
        .with_shared_volume("/var/log/nginx", "/var/log/nginx")
        .with_environment([("FLUENT_HOST", "logserver".to_string())]);

    println!("Main container: {:?}", main_container);
    println!("Logging sidecar: {:?}", logging_sidecar);
    println!("Logs from /var/log/nginx will be collected by Fluent Bit");

    Ok(())
}

fn example_monitoring_sidecar() -> anyhow::Result<()> {
    println!("Creating a container with Prometheus metrics exporter sidecar");

    let mut main_container = MaestroContainer::new("myapp:latest", "api-server");
    main_container.set_port(8080);
    main_container.set_labels([("app", "api")]);

    let metrics_sidecar =
        SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics-exporter")
            .with_port(9100)
            .with_environment([
                ("COLLECTOR_PROCFS", "/host/proc".to_string()),
                ("COLLECTOR_SYSFS", "/host/sys".to_string()),
            ])
            .with_host_path_mount("/proc", "/host/proc", "ro")
            .with_host_path_mount("/sys", "/host/sys", "ro");

    println!("Main container: {:?}", main_container);
    println!("Metrics sidecar: {:?}", metrics_sidecar);
    println!("Metrics will be available at http://<pod-ip>:9100/metrics");

    Ok(())
}

fn example_proxy_sidecar() -> anyhow::Result<()> {
    println!("Creating a container with Envoy proxy sidecar");

    let mut main_container = MaestroContainer::new("myapp:latest", "application");
    main_container.set_port(8080);
    main_container.set_labels([("app", "backend")]);

    let proxy_sidecar = SidecarContainer::new("envoyproxy/envoy:v1.28", "proxy")
        .with_port(8080)
        .with_port(9901)
        .with_shared_volume("/etc/envoy", "/etc/envoy")
        .with_shared_volume("/var/run/envoy", "/var/run/envoy")
        .with_environment([
            ("ENVOY_UID", "1337".to_string()),
            ("ENVOY_GID", "1337".to_string()),
        ]);

    println!("Main container: {:?}", main_container);
    println!("Proxy sidecar: {:?}", proxy_sidecar);
    println!("Envoy proxy will handle incoming traffic on port 8080");
    println!("Admin interface available at http://<pod-ip>:9901");

    Ok(())
}

fn example_multiple_sidecars() -> anyhow::Result<()> {
    println!("Creating a container with multiple sidecars");

    let mut main_container = MaestroContainer::new("myapp:latest", "main");
    main_container.set_port(8080);
    main_container.set_labels([("app", "multi-sidecar")]);

    let logging_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
        .with_shared_volume("/var/log", "/var/log");

    let metrics_sidecar =
        SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics").with_port(9100);

    let debug_sidecar = SidecarContainer::new("busybox:latest", "debug")
        .with_command(["sh", "-c", "sleep 3600"])
        .with_shared_volume("/app", "/app");

    println!("Main container: {:?}", main_container);
    println!("Sidecars:");
    println!("  - Logging: {:?}", logging_sidecar);
    println!("  - Metrics: {:?}", metrics_sidecar);
    println!("  - Debug: {:?}", debug_sidecar);
    println!("Total containers: 1 main + 3 sidecars = 4 containers in pod");

    Ok(())
}

struct CustomSidecarPlugin;

impl SidecarPlugin for CustomSidecarPlugin {
    fn name(&self) -> &str {
        "custom-sidecar-plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "A custom sidecar plugin for demonstration"
    }

    fn init(&mut self) -> anyhow::Result<()> {
        println!("Initializing custom sidecar plugin...");
        Ok(())
    }

    fn get_sidecar_container(
        &self,
        main_container: &MaestroContainer,
    ) -> anyhow::Result<SidecarContainer> {
        let labels = main_container.labels().unwrap_or(&HashMap::new());
        let app_name = labels.get("app").unwrap_or(&"app".to_string());

        Ok(
            SidecarContainer::new("busybox:latest", format!("{}-sidecar", app_name))
                .with_command(["sh", "-c", "sleep 3600"])
                .with_environment([("MAIN_APP", app_name.clone())]),
        )
    }

    fn cleanup(&mut self) -> anyhow::Result<()> {
        println!("Cleaning up custom sidecar plugin...");
        Ok(())
    }
}

fn example_custom_plugin_sidecar() -> anyhow::Result<()> {
    println!("Creating a sidecar using a custom plugin");

    let mut main_container = MaestroContainer::new("myapp:latest", "app");
    main_container.set_labels([("app", "custom-plugin-example")]);
    main_container.set_port(8080);

    let mut plugin = CustomSidecarPlugin;
    plugin.init()?;

    let sidecar = plugin.get_sidecar_container(&main_container)?;

    println!("Main container: {:?}", main_container);
    println!("Custom plugin sidecar: {:?}", sidecar);

    plugin.cleanup()?;
    Ok(())
}
