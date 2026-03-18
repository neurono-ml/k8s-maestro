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

    let main_container = MaestroContainer::new("nginx:latest", "web-server");

    let mut logging_env = std::collections::BTreeMap::new();
    logging_env.insert("FLUENT_HOST".to_string(), "logserver".to_string());

    let logging_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector")
        .set_environment_variables(logging_env);

    println!("Main container: {:?}", main_container);
    println!("Logging sidecar: {:?}", logging_sidecar);
    println!("Logs from /var/log/nginx will be collected by Fluent Bit");

    Ok(())
}

fn example_monitoring_sidecar() -> anyhow::Result<()> {
    println!("Creating a container with Prometheus metrics exporter sidecar");

    let main_container = MaestroContainer::new("myapp:latest", "api-server");

    let mut metrics_env = std::collections::BTreeMap::new();
    metrics_env.insert("COLLECTOR_PROCFS".to_string(), "/host/proc".to_string());
    metrics_env.insert("COLLECTOR_SYSFS".to_string(), "/host/sys".to_string());

    let metrics_sidecar =
        SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics-exporter")
            .set_environment_variables(metrics_env);

    println!("Main container: {:?}", main_container);
    println!("Metrics sidecar: {:?}", metrics_sidecar);
    println!("Metrics will be available at http://<pod-ip>:9100/metrics");

    Ok(())
}

fn example_proxy_sidecar() -> anyhow::Result<()> {
    println!("Creating a container with Envoy proxy sidecar");

    let main_container = MaestroContainer::new("myapp:latest", "application");

    let mut proxy_env = std::collections::BTreeMap::new();
    proxy_env.insert("ENVOY_UID".to_string(), "1337".to_string());
    proxy_env.insert("ENVOY_GID".to_string(), "1337".to_string());

    let proxy_sidecar = SidecarContainer::new("envoyproxy/envoy:v1.28", "proxy")
        .set_environment_variables(proxy_env);

    println!("Main container: {:?}", main_container);
    println!("Proxy sidecar: {:?}", proxy_sidecar);
    println!("Envoy proxy will handle incoming traffic on port 8080");
    println!("Admin interface available at http://<pod-ip>:9901");

    Ok(())
}

fn example_multiple_sidecars() -> anyhow::Result<()> {
    println!("Creating a container with multiple sidecars");

    let main_container = MaestroContainer::new("myapp:latest", "main");

    let logging_sidecar = SidecarContainer::new("fluent/fluent-bit:2.2", "log-collector");

    let metrics_sidecar = SidecarContainer::new("prom/prometheus-node-exporter:latest", "metrics");

    let debug_sidecar = SidecarContainer::new("busybox:latest", "debug").set_arguments(&[
        "sh".to_owned(),
        "-c".to_owned(),
        "sleep 3600".to_owned(),
    ]);

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

    fn image(&self) -> &str {
        "busybox:latest"
    }

    fn create_sidecar(&self) -> anyhow::Result<SidecarContainer> {
        Ok(
            SidecarContainer::new(self.image(), self.name()).set_arguments(&[
                "sh".to_owned(),
                "-c".to_owned(),
                "sleep 3600".to_owned(),
            ]),
        )
    }

    fn default_config(&self) -> std::collections::BTreeMap<String, serde_json::Value> {
        let mut config = std::collections::BTreeMap::new();
        config.insert(
            "version".to_string(),
            serde_json::Value::String("1.0.0".to_string()),
        );
        config
    }
}

fn example_custom_plugin_sidecar() -> anyhow::Result<()> {
    println!("Creating a sidecar using a custom plugin");

    let main_container = MaestroContainer::new("myapp:latest", "app");

    let plugin = CustomSidecarPlugin;
    let sidecar = plugin.create_sidecar()?;

    println!("Main container: {:?}", main_container);
    println!("Custom plugin sidecar: {:?}", sidecar);

    Ok(())
}
