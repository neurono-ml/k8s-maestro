//! DNS utilities for Kubernetes service discovery.
//!
//! This module provides functions for generating standard Kubernetes DNS names
//! for services, pods, and headless services.

/// Generate the fully qualified domain name for a Kubernetes service.
///
/// # Arguments
///
/// * `service` - The name of the service
/// * `namespace` - The namespace of the service
///
/// # Returns
///
/// The FQDN for the service in the format: `<service>.<namespace>.svc.cluster.local`
pub fn service_dns_name(service: &str, namespace: &str) -> String {
    format!("{}.{}.svc.cluster.local", service, namespace)
}

/// Generate the fully qualified domain name for a Kubernetes pod.
///
/// # Arguments
///
/// * `pod` - The name of the pod
/// * `namespace` - The namespace of the pod
///
/// # Returns
///
/// The FQDN for the pod in the format: `<pod>.<namespace>.pod.cluster.local`
pub fn pod_dns_name(pod: &str, namespace: &str) -> String {
    format!("{}.{}.pod.cluster.local", pod, namespace)
}

/// Generate the wildcard DNS pattern for a headless service.
///
/// # Arguments
///
/// * `service` - The name of the headless service
/// * `namespace` - The namespace of the service
///
/// # Returns
///
/// The wildcard DNS pattern in the format: `*.<service>.<namespace>.svc.cluster.local`
pub fn headless_service_dns_pattern(service: &str, namespace: &str) -> String {
    format!("*.{}.{}.svc.cluster.local", service, namespace)
}

#[cfg(test)]
mod tests {
    include!("dns_test.rs");
}
