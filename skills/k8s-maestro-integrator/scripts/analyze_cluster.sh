#!/bin/bash
# 
# analyze_cluster.sh - Analyze Kubernetes cluster and provide recommendations
#
# This script analyzes the Kubernetes cluster configuration and provides
# recommendations for deploying k8s-maestro workflows.
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
print_header() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

print_section() {
    echo -e "${YELLOW}$1${NC}"
    echo -e "${YELLOW}--------------------------------${NC}"
}

print_info() {
    echo -e "  ${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "  ${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "  ${RED}✗${NC} $1"
}

# Check if kubectl is available
check_kubectl() {
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl not found. Please install kubectl."
        exit 1
    fi
}

# Check cluster connectivity
check_connectivity() {
    print_section "Cluster Connectivity"
    
    if kubectl cluster-info &> /dev/null; then
        print_info "Connected to Kubernetes cluster"
        
        # Get current context
        CONTEXT=$(kubectl config current-context 2>/dev/null || echo "unknown")
        print_info "Current context: $CONTEXT"
        
        # Get cluster version
        VERSION=$(kubectl version --short 2>/dev/null | grep "Server Version" | awk '{print $3}' || echo "unknown")
        print_info "Cluster version: $VERSION"
    else
        print_error "Cannot connect to Kubernetes cluster"
        print_error "Please check your kubeconfig"
        exit 1
    fi
}

# Analyze nodes
analyze_nodes() {
    print_section "Node Analysis"
    
    # Get node count
    NODE_COUNT=$(kubectl get nodes --no-headers 2>/dev/null | wc -l)
    print_info "Number of nodes: $NODE_COUNT"
    
    # Get node status
    READY_NODES=$(kubectl get nodes --no-headers 2>/dev/null | grep -c " Ready" || echo "0")
    NOT_READY_NODES=$((NODE_COUNT - READY_NODES))
    
    if [ $NOT_READY_NODES -eq 0 ]; then
        print_info "All nodes are ready"
    else
        print_warning "$NOT_READY_NODES node(s) not ready"
    fi
    
    # Get node resources
    print_info "Node resources:"
    kubectl top nodes 2>/dev/null | grep -v "NAME" | while read line; do
        echo -e "    $line"
    done
}

# Analyze namespaces
analyze_namespaces() {
    print_section "Namespace Analysis"
    
    # Get all namespaces
    NAMESPACES=$(kubectl get namespaces --no-headers 2>/dev/null | awk '{print $1}')
    
    print_info "Available namespaces:"
    for ns in $NAMESPACES; do
        echo -e "    - $ns"
    done
    
    # Get current namespace
    CURRENT_NS=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null || echo "default")
    print_info "Current namespace: $CURRENT_NS"
}

# Analyze storage classes
analyze_storage_classes() {
    print_section "Storage Classes"
    
    # Get storage classes
    STORAGE_CLASSES=$(kubectl get storageclasses --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$STORAGE_CLASSES" ]; then
        print_info "Available storage classes:"
        for sc in $STORAGE_CLASSES; do
            # Get storage class details
            PROVISIONER=$(kubectl get storageclass $sc -o jsonpath='{.provisioner}' 2>/dev/null || echo "unknown")
            TYPE=$(kubectl get storageclass $sc -o jsonpath='{.metadata.annotations.storageclass\.kubernetes\.io/is-default-class}' 2>/dev/null || echo "")
            
            if [ "$TYPE" = "true" ]; then
                echo -e "    - $sc (default) - $PROVISIONER"
            else
                echo -e "    - $sc - $PROVISIONER"
            fi
        done
    else
        print_warning "No storage classes found"
    fi
}

# Analyze ingress classes
analyze_ingress_classes() {
    print_section "Ingress Classes"
    
    # Get ingress classes
    INGRESS_CLASSES=$(kubectl get ingressclass --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$INGRESS_CLASSES" ]; then
        print_info "Available ingress classes:"
        for ic in $INGRESS_CLASSES; do
            # Get ingress class details
            CONTROLLER=$(kubectl get ingressclass $ic -o jsonpath='{.spec.controller}' 2>/dev/null || echo "unknown")
            TYPE=$(kubectl get ingressclass $ic -o jsonpath='{.metadata.annotations.ingressclass\.kubernetes\.io/is-default-class}' 2>/dev/null || echo "")
            
            if [ "$TYPE" = "true" ]; then
                echo -e "    - $ic (default) - $CONTROLLER"
            else
                echo -e "    - $ic - $CONTROLLER"
            fi
        done
    else
        print_warning "No ingress classes found"
    fi
}

# Analyze resource quotas
analyze_resource_quotas() {
    print_section "Resource Quotas"
    
    # Get current namespace
    CURRENT_NS=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null || echo "default")
    
    # Get resource quotas
    QUOTAS=$(kubectl get resourcequota -n $CURRENT_NS --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$QUOTAS" ]; then
        print_info "Resource quotas in namespace '$CURRENT_NS':"
        for quota in $QUOTAS; do
            echo -e "    - $quota"
            kubectl describe resourcequota $quota -n $CURRENT_NS 2>/dev/null | grep -A 20 "Resource Quotas" | grep -E "^\s+(cpu|memory|pods)" | sed 's/^/      /' || true
        done
    else
        print_info "No resource quotas found in namespace '$CURRENT_NS'"
    fi
}

# Analyze limit ranges
analyze_limit_ranges() {
    print_section "Limit Ranges"
    
    # Get current namespace
    CURRENT_NS=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null || echo "default")
    
    # Get limit ranges
    LIMIT_RANGES=$(kubectl get limitrange -n $CURRENT_NS --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$LIMIT_RANGES" ]; then
        print_info "Limit ranges in namespace '$CURRENT_NS':"
        for lr in $LIMIT_RANGES; do
            echo -e "    - $lr"
        done
    else
        print_info "No limit ranges found in namespace '$CURRENT_NS'"
    fi
}

# Analyze secrets
analyze_secrets() {
    print_section "Secrets"
    
    # Get current namespace
    CURRENT_NS=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null || echo "default")
    
    # Get secrets
    SECRETS=$(kubectl get secrets -n $CURRENT_NS --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$SECRETS" ]; then
        SECRET_COUNT=$(echo "$SECRETS" | wc -l)
        print_info "$SECRET_COUNT secret(s) found in namespace '$CURRENT_NS':"
        for secret in $SECRETS; do
            TYPE=$(kubectl get secret $secret -n $CURRENT_NS -o jsonpath='{.type}' 2>/dev/null || echo "unknown")
            echo -e "    - $secret ($TYPE)"
        done
    else
        print_info "No secrets found in namespace '$CURRENT_NS'"
    fi
}

# Analyze configmaps
analyze_configmaps() {
    print_section "ConfigMaps"
    
    # Get current namespace
    CURRENT_NS=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null || echo "default")
    
    # Get configmaps
    CONFIGMAPS=$(kubectl get configmap -n $CURRENT_NS --no-headers 2>/dev/null | awk '{print $1}')
    
    if [ -n "$CONFIGMAPS" ]; then
        CM_COUNT=$(echo "$CONFIGMAPS" | wc -l)
        print_info "$CM_COUNT configmap(s) found in namespace '$CURRENT_NS':"
        for cm in $CONFIGMAPS; do
            echo -e "    - $cm"
        done
    else
        print_info "No configmaps found in namespace '$CURRENT_NS'"
    fi
}

# Provide recommendations
provide_recommendations() {
    print_section "Recommendations"
    
    # Get cluster version
    VERSION=$(kubectl version --short 2>/dev/null | grep "Server Version" | awk '{print $3}' || echo "unknown")
    
    print_info "Based on cluster analysis:"
    echo ""
    
    # Version recommendation
    if [ "$VERSION" != "unknown" ]; then
        MAJOR_MINOR=$(echo $VERSION | cut -d. -f1,2)
        print_info "Cluster version: $VERSION"
        print_info "Recommended k8s-maestro feature: k8s_v${MAJOR_MINOR}"
        print_info "Include exec-steps feature for Python/Rust script support"
        echo ""
    fi
    
    # Storage recommendation
    STORAGE_CLASSES=$(kubectl get storageclasses --no-headers 2>/dev/null | awk '{print $1}' | head -1)
    if [ -n "$STORAGE_CLASSES" ]; then
        print_info "For persistent storage, use storage class: $STORAGE_CLASSES"
        echo ""
    fi
    
    # Ingress recommendation
    INGRESS_CLASSES=$(kubectl get ingressclass --no-headers 2>/dev/null | awk '{print $1}' | head -1)
    if [ -n "$INGRESS_CLASSES" ]; then
        print_info "For external access, use ingress class: $INGRESS_CLASSES"
        echo ""
    fi
    
    # Resource limits recommendation
    print_info "Set appropriate resource limits for your workflows:"
    echo "    - CPU: 500m-2000m depending on workload"
    echo "    - Memory: 512Mi-4Gi depending on workload"
    echo "    - Storage: Use appropriate PVC size for your data"
    echo ""
    
    # Security recommendation
    print_info "Security best practices:"
    echo "    - Use secrets for sensitive data"
    echo "    - Implement network policies"
    echo "    - Set up RBAC with least privilege"
    echo "    - Use pod security contexts"
    echo ""
    
    # Monitoring recommendation
    print_info "Monitoring and observability:"
    echo "    - Add logging sidecars (e.g., Fluent Bit)"
    echo "    - Add metrics sidecars (e.g., Prometheus)"
    echo "    - Implement health checks (readiness, liveness)"
}

# Main execution
main() {
    print_header "Kubernetes Cluster Analysis"
    
    check_kubectl
    check_connectivity
    analyze_nodes
    analyze_namespaces
    analyze_storage_classes
    analyze_ingress_classes
    analyze_resource_quotas
    analyze_limit_ranges
    analyze_secrets
    analyze_configmaps
    provide_recommendations
    
    print_header "Analysis Complete"
}

# Run main function
main
