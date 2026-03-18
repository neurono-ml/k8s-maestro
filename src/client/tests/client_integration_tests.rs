use anyhow::Result;
use k8s_maestro::clients::MaestroClient;
use k8s_maestro::config::Config;

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_network_policy_client() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()
        .await
        .expect("Failed to build client");

    // Create a network policy using the client
    let mut policy_builder = client.network_policy();

    policy_builder = policy_builder
        .deny_all("test-deny-all", "default")
        .await;

    let policy = policy_builder
        .build()
        .await
        .expect("Failed to build network policy");

    assert_eq!(
        policy.metadata.name.as_ref().unwrap(),
        "test-deny-all".to_string()
    );

    Ok(())
}

#[tokio::test]
async fn integration_test_resource_quota_client() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClientBuilder::new()
        .with_namespace("team-a")
        .build()
        .await
        .expect("Failed to build client");

    // Create a resource quota using the client
    let mut quota_builder = client.resource_quota();

    quota_builder = quota_builder
        .with_scope(k8s_maestro::security::QuotaScope::BestEffort)
        .with_hard_limit("requests.cpu", "2")
        .with_hard_limit("requests.memory", "4Gi")
        .await;

    let quota = quota_builder
        .build()
        .await
        .expect("Failed to build resource quota");

    assert_eq!(
        quota.metadata.name.as_ref().unwrap(),
        "small-quota".to_string()
    );

    Ok(())
}

#[tokio::test]
async fn integration_test_security_context_client() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()
        .await
        .expect("Failed to build client");

    // Create a security context using the client
    let mut ctx_builder = client.security_context();

    ctx_builder = ctx_builder
        .with_run_as_non_root(true)
        .with_read_only_root_filesystem(true)
        .await;

    let security_context = ctx_builder
        .build()
        .await
        .expect("Failed to build security context");

    Ok(())
}

#[tokio::test]
async fn integration_test_rbac_client() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClientBuilder::new()
        .with_namespace("production")
        .build()
        .await
        .expect("Failed to build client");

    // Create RBAC resources using the client
    let mut sa_builder = client.service_account();

    sa_builder = sa_builder
        .with_annotation("eks.amazonaws.com/role-arn", "arn:aws:iam::123456:role/test")
        .await;

    let sa = sa_builder
        .build()
        .await
        .expect("Failed to build service account");

    assert_eq!(
        sa.metadata.name.as_ref().unwrap(),
        "test-sa".to_string()
    );

    let mut role_builder = client.role();

    role_builder = role_builder
        .with_rule(
            client.rbac_policy_rule()
                .with_api_groups(vec!["batch".to_string()])
                .with_resources(vec!["jobs".to_string()])
                .with_verbs(vec!["get".to_string(), "list".to_string()])
        )
        .await;

    let role = role_builder
        .build()
        .await
        .expect("Failed to build role");

    assert_eq!(
        role.metadata.name.as_ref().unwrap(),
        "test-role".to_string()
    );

    Ok(())
}

#[tokio::test]
async fn integration_test_limit_range_client() -> anyhow::Result<()> {
    let ctx = Context::default();
    let client = MaestroClientBuilder::new()
        .with_namespace("default")
        .build()
        .await
        .expect("Failed to build client");

    // Create a limit range using the client
    let mut limit_builder = client.limit_range();

    let container_limit = client.limit_range_item(k8s_maestro::security::LimitRangeType::Container);

    limit_builder = limit_builder
        .with_limit(
            container_limit
                .with_default_value("cpu", "500m")
                .with_default_value("memory", "512Mi")
                .with_max_value("cpu", "2")
                .with_max_value("memory", "4Gi")
                .build()
        )
        .await;

    let limit_range = limit_builder
        .build()
        .await
        .expect("Failed to build limit range");

    assert_eq!(
        limit_range.metadata.name.as_ref().unwrap(),
        "test-limits".to_string()
    );

    Ok(())
}
