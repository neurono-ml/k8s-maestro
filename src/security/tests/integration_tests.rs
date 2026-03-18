use anyhow::Context;
use k8s_openapi::api::core::v1::NetworkPolicy;
use k8s_openapi::api::core::v1::PodSecurityContext;
use k8s_openapi::api::rbac::v1::{ClusterRole, Role};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::api::core::v1::ResourceQuota;
use k8s_openapi::api::rbac::v1::{PolicyRule, ServiceAccount};

use k8s_maestro::clients::MaestroClient;
use k8s_maestro::config::Config;

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_network_policy() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClient::new(&ctx).await;

    // Create a network policy
    let policy_builder = client
        .network_policy()
        .builder("test-integration-policy");

    let policy = policy_builder
        .deny_all("production".to_string(), "default".to_string())
        .await
        .expect("Failed to create network policy");

    assert_eq!(
        policy.metadata.name.as_ref().unwrap(),
        "test-integration-policy".to_string()
    );

    // Apply the network policy to a namespace
    client
        .network_policy()
        .apply(&policy)
        .await
        .expect("Failed to apply network policy");

    // Verify the policy exists
    let fetched_policy = client
        .network_policy()
        .get("test-integration-policy".to_string(), "default".to_string())
        .await
        .expect("Failed to get network policy");

    assert!(fetched_policy.is_some());
    let fetched = fetched_policy.unwrap();
    assert_eq!(fetched.metadata.name.as_ref().unwrap(), "test-integration-policy".to_string());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_resource_quota() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClient::new(&ctx).await;

    // Create a resource quota
    let quota_builder = client
        .resource_quota()
        .builder("test-integration-quota");

    let quota = quota_builder
        .small_workload("team-a", "default")
        .await
        .expect("Failed to create resource quota");

    assert_eq!(
        quota.metadata.name.as_ref().unwrap(),
        "test-integration-quota".to_string()
    );

    // Apply the resource quota to a namespace
    client
        .resource_quota()
        .apply(&quota)
        .await
        .expect("Failed to apply resource quota");

    // Verify the quota exists
    let fetched_quota = client
        .resource_quota()
        .get("test-integration-quota".to_string(), "default".to_string())
        .await
        .expect("Failed to get resource quota");

    assert!(fetched_quota.is_some());
    let fetched = fetched_quota.unwrap();
    assert_eq!(fetched.metadata.name.as_ref().unwrap(), "test-integration-quota".to_string());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_security_context() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClient::new(&ctx).await;

    // Create a security context
    let ctx_builder = client.security_context().builder();

    let restricted_ctx = ctx_builder
        .restricted()
        .build();

    // Apply the security context to a namespace
    client
        .security_context()
        .apply(&restricted_ctx)
        .await
        .expect("Failed to apply security context");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_rbac() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClient::new(&ctx).await;

    // Create RBAC resources
    let sa = client
        .rbac()
        .service_account()
        .builder("test-integration-sa")
        .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/test")
        .build()
        .await
        .expect("Failed to create service account");

    let role = client
        .rbac()
        .role()
        .builder("test-integration-role")
        .add_rule(
            PolicyRule::new()
                .with_api_groups(vec!["batch".to_string()])
                .with_resources(vec!["jobs".to_string()])
                .with_verbs(vec![
                    "get".to_string(),
                    "list".to_string(),
                ]),
        )
        .build()
        .await
        .expect("Failed to create role");

    // Apply RBAC resources
    client
        .rbac()
        .service_account()
        .apply(&sa)
        .await
        .expect("Failed to apply service account");

    client
        .rbac()
        .role()
        .apply(&role)
        .await
        .expect("Failed to apply role");

    // Verify resources exist
    let fetched_sa = client
        .rbac()
        .service_account()
        .get("test-integration-sa".to_string(), "default".to_string())
        .await
        .expect("Failed to get service account");

    assert!(fetched_sa.is_some());
    assert_eq!(
        fetched_sa.unwrap().metadata.name.as_ref().unwrap(),
        "test-integration-sa".to_string()
    );

    let fetched_role = client
        .rbac()
        .role()
        .get("test-integration-role".to_string(), "default".to_string())
        .await
        .expect("Failed to get role");

    assert!(fetched_role.is_some());
    assert_eq!(
        fetched_role.unwrap().metadata.name.as_ref().unwrap(),
        "test-integration-role".to_string()
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_limit_range() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClient::new(&ctx).await;

    // Create a limit range
    let limit_range_builder = client.limit_range().builder("test-integration-limits");

    let limit_range = limit_range_builder
        .with_limit(
            k8s_maestro::security::LimitRangeItemBuilder::new(
                k8s_maestro::security::LimitRangeType::Container
            )
            .with_default_value("cpu", "500m")
            .with_default_value("memory", "512Mi")
            .with_max_value("cpu", "2")
            .with_max_value("memory", "4Gi")
            .build()
        )
        .await
        .expect("Failed to create limit range");

    // Apply the limit range
    client
        .limit_range()
        .apply(&limit_range)
        .await
        .expect("Failed to apply limit range");

    // Verify the limit range exists
    let fetched_limit_range = client
        .limit_range()
        .get("test-integration-limits".to_string(), "default".to_string())
        .await
        .expect("Failed to get limit range");

    assert!(fetched_limit_range.is_some());
    assert_eq!(
        fetched_limit_range.unwrap().metadata.name.as_ref().unwrap(),
        "test-integration-limits".to_string()
    );

    Ok(())
}
