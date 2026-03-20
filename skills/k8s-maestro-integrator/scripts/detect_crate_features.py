#!/usr/bin/env python3
"""
Detect required crate features for k8s-maestro based on cluster configuration.

This script analyzes the Kubernetes cluster and recommends appropriate
feature flags for the k8s-maestro dependency in Cargo.toml.
"""

import subprocess
import sys
from typing import Optional, Tuple


def run_command(cmd: list, capture: bool = True) -> Tuple[bool, str]:
    """Run a shell command and return success status and output."""
    try:
        result = subprocess.run(cmd, capture_output=capture, text=True, check=False)
        return result.returncode == 0, result.stdout.strip() if capture else ""
    except Exception as e:
        return False, str(e)


def check_cluster_connectivity() -> bool:
    """Check if we can connect to the Kubernetes cluster."""
    success, _ = run_command(["kubectl", "cluster-info"])
    return success


def get_cluster_version() -> Optional[str]:
    """Get the Kubernetes cluster version."""
    success, output = run_command(["kubectl", "version", "--short"])
    if success:
        for line in output.split("\n"):
            if "Server Version" in line or "server" in line.lower():
                # Parse version like "v1.29.2"
                parts = line.split()
                for part in parts:
                    if part.startswith("v1."):
                        version = part[1:]  # Remove 'v' prefix
                        # Get major.minor only
                        major_minor = ".".join(version.split(".")[:2])
                        return major_minor
    return None


def get_available_storage_classes() -> list:
    """Get available storage classes in the cluster."""
    success, output = run_command(["kubectl", "get", "storageclasses"])
    if success:
        lines = output.split("\n")
        # Skip header and empty lines
        classes = []
        for line in lines[1:]:
            if line.strip():
                # First column is the name
                parts = line.split()
                if parts:
                    classes.append(parts[0])
        return classes
    return []


def get_available_ingress_classes() -> list:
    """Get available ingress classes in the cluster."""
    success, output = run_command(["kubectl", "get", "ingressclass"])
    if success:
        lines = output.split("\n")
        classes = []
        for line in lines[1:]:
            if line.strip():
                parts = line.split()
                if parts:
                    classes.append(parts[0])
        return classes
    return []


def get_namespaces() -> list:
    """Get all namespaces in the cluster."""
    success, output = run_command(["kubectl", "get", "namespaces"])
    if success:
        lines = output.split("\n")
        namespaces = []
        for line in lines[1:]:
            if line.strip():
                parts = line.split()
                if parts:
                    namespaces.append(parts[0])
        return namespaces
    return []


def get_current_context() -> Optional[str]:
    """Get the current Kubernetes context."""
    success, output = run_command(["kubectl", "config", "current-context"])
    if success:
        return output
    return None


def recommend_features(cluster_version: Optional[str]) -> dict:
    """Recommend crate features based on cluster version."""
    recommendations = {
        "k8s_version": cluster_version,
        "feature": None,
        "exec_steps": False,
        "reason": "",
        "alternatives": [],
    }

    if not cluster_version:
        recommendations["reason"] = "Could not detect cluster version"
        recommendations["feature"] = "k8s_v1_28"  # Default
        return recommendations

    # Map cluster versions to feature flags
    version_map = {
        "1.28": "k8s_v1_28",
        "1.29": "k8s_v1_29",
        "1.30": "k8s_v1_30",
        "1.31": "k8s_v1_31",
        "1.32": "k8s_v1_32",
    }

    # Get major.minor
    major_minor = ".".join(cluster_version.split(".")[:2])

    if major_minor in version_map:
        recommended = version_map[major_minor]
        recommendations["feature"] = recommended
        recommendations["reason"] = (
            f"Cluster version {cluster_version} matches feature {recommended}"
        )

        # Suggest exec-steps based on use cases
        recommendations["exec_steps"] = True
    else:
        # Find closest match
        versions = sorted(version_map.keys(), reverse=True)
        for v in versions:
            if float(v) < float(major_minor):
                recommended = version_map[v]
                recommendations["feature"] = recommended
                recommendations["reason"] = (
                    f"Cluster version {cluster_version} is newer than available features. "
                    f"Using {recommended} as closest match. Consider upgrading k8s-maestro."
                )
                break
        else:
            # Use latest
            recommended = versions[-1]
            recommendations["feature"] = recommended
            recommendations["reason"] = (
                f"Cluster version {cluster_version} is older than available features. "
                f"Using {recommended} (oldest available). "
                f"Consider upgrading your cluster."
            )

    return recommendations


def print_cargo_toml_line(recommendations: dict) -> None:
    """Print the Cargo.toml dependency line."""
    feature = recommendations["feature"]
    exec_steps = "exec-steps" if recommendations["exec_steps"] else ""

    if exec_steps:
        print(
            f'\nk8s-maestro = {{ version = "1.0", features = ["{feature}", "{exec_steps}"] }}'
        )
    else:
        print(f'\nk8s-maestro = {{ version = "1.0", features = ["{feature}"] }}')


def print_recommendations(
    recommendations: dict,
    storage_classes: list,
    ingress_classes: list,
    namespaces: list,
) -> None:
    """Print detailed recommendations."""
    print("=" * 70)
    print("k8s-maestro Crate Feature Detection")
    print("=" * 70)
    print()

    # Cluster info
    print("Cluster Information:")
    print("-" * 70)
    print(f"  Context: {get_current_context()}")
    print(f"  Version: {recommendations['k8s_version']}")
    print(
        f"  Status: {'Connected' if check_cluster_connectivity() else 'Not connected'}"
    )
    print()

    # Feature recommendations
    print("Feature Recommendations:")
    print("-" * 70)
    print(f"  Recommended feature: {recommendations['feature']}")
    print(f"  Include exec-steps: {recommendations['exec_steps']}")
    print(f"  Reason: {recommendations['reason']}")
    print()

    # Cargo.toml line
    print("Cargo.toml Dependency:")
    print("-" * 70)
    print_cargo_toml_line(recommendations)
    print()

    # Storage classes
    if storage_classes:
        print("Available Storage Classes:")
        print("-" * 70)
        for sc in storage_classes:
            print(f"  - {sc}")
        print()

    # Ingress classes
    if ingress_classes:
        print("Available Ingress Classes:")
        print("-" * 70)
        for ic in ingress_classes:
            print(f"  - {ic}")
        print()

    # Namespaces
    if namespaces:
        print("Available Namespaces:")
        print("-" * 70)
        for ns in namespaces:
            print(f"  - {ns}")
        print()

    print("=" * 70)


def main():
    """Main function."""
    # Check if kubectl is available
    success, _ = run_command(["which", "kubectl"])
    if not success:
        print("Error: kubectl not found in PATH")
        print("Please install kubectl and configure it to access your cluster")
        sys.exit(1)

    # Check cluster connectivity
    if not check_cluster_connectivity():
        print("Warning: Cannot connect to Kubernetes cluster")
        print("Using default recommendations")
        print()

    # Get cluster information
    cluster_version = get_cluster_version()
    storage_classes = get_available_storage_classes()
    ingress_classes = get_available_ingress_classes()
    namespaces = get_namespaces()

    # Get recommendations
    recommendations = recommend_features(cluster_version)

    # Print recommendations
    print_recommendations(recommendations, storage_classes, ingress_classes, namespaces)

    # Exit with appropriate code
    sys.exit(0 if recommendations["feature"] else 1)


if __name__ == "__main__":
    main()
