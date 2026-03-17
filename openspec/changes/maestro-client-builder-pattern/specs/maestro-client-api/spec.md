## ADDED Requirements

### Requirement: Client creates workflow
The `MaestroClient` SHALL provide a `create_workflow(workflow: impl WorkFlow)` method that creates and executes a Kubernetes workflow using the client's configuration.

#### Scenario: Create workflow with dry_run enabled
- **WHEN** client has dry_run=true
- **AND** user calls `client.create_workflow(workflow)`
- **THEN** client validates the workflow
- **AND** client does not execute actual Kubernetes operations
- **AND** method returns Ok(()) on successful validation

#### Scenario: Create workflow with dry_run disabled
- **WHEN** client has dry_run=false
- **AND** user calls `client.create_workflow(workflow)`
- **THEN** client validates the workflow
- **AND** client executes actual Kubernetes operations
- **AND** method returns Ok(()) on successful creation

#### Scenario: Create workflow returns error on validation failure
- **WHEN** user calls `client.create_workflow(workflow)` with invalid workflow
- **THEN** method returns Err(anyhow::Error) with validation error details

#### Scenario: Create workflow returns error on Kubernetes failure
- **WHEN** client has dry_run=false
- **AND** Kubernetes API rejects the workflow
- **THEN** method returns Err(anyhow::Error) with Kubernetes error details

### Requirement: Client retrieves workflow by identifier
The `MaestroClient` SHALL provide a `get_workflow(id: impl AsRef<str>)` method that retrieves an existing workflow or returns None if not found.

#### Scenario: Get existing workflow
- **WHEN** user calls `client.get_workflow("workflow-id")`
- **AND** workflow exists in Kubernetes
- **THEN** method returns Some(workflow) with the workflow implementation
- **AND** workflow contains current Kubernetes state

#### Scenario: Get non-existent workflow
- **WHEN** user calls `client.get_workflow("non-existent-id")`
- **AND** workflow does not exist in Kubernetes
- **THEN** method returns None

#### Scenario: Get workflow respects client namespace
- **WHEN** client is configured with namespace="production"
- **AND** user calls `client.get_workflow("workflow-id")`
- **THEN** client searches only in the "production" namespace
- **AND** returns workflow if found in that namespace

#### Scenario: Get workflow returns error on API failure
- **WHEN** Kubernetes API call fails
- **THEN** method returns Err(anyhow::Error) with API error details

### Requirement: Client lists all workflows
The `MaestroClient` SHALL provide a `list_workflows()` method that returns all workflows accessible to the client.

#### Scenario: List workflows in default namespace
- **WHEN** client is configured with namespace="default"
- **AND** user calls `client.list_workflows()`
- **THEN** method returns Vec containing all workflows in "default" namespace
- **AND** each workflow implements WorkFlow trait

#### Scenario: List workflows returns empty vector when no workflows exist
- **WHEN** namespace has no workflows
- **AND** user calls `client.list_workflows()`
- **THEN** method returns empty Vec

#### Scenario: List workflows returns error on API failure
- **WHEN** Kubernetes API call fails
- **THEN** method returns Err(anyhow::Error) with API error details

### Requirement: Client methods use client configuration
All client methods SHALL use the client's configured namespace, timeout, and resource limits when executing operations.

#### Scenario: Create workflow uses configured namespace
- **WHEN** client has namespace="custom-ns"
- **AND** user calls `client.create_workflow(workflow)`
- **THEN** workflow is created in "custom-ns" namespace

#### Scenario: List workflows uses configured namespace
- **WHEN** client has namespace="staging"
- **AND** user calls `client.list_workflows()`
- **THEN** only workflows in "staging" namespace are returned

### Requirement: Client methods are async
All client methods SHALL be async functions returning `anyhow::Result`.

#### Scenario: Create workflow is async
- **WHEN** user calls `client.create_workflow(workflow).await`
- **THEN** method executes asynchronously
- **AND** returns anyhow::Result

#### Scenario: Get workflow is async
- **WHEN** user calls `client.get_workflow("id").await`
- **THEN** method executes asynchronously
- **AND** returns anyhow::Result<Option<impl WorkFlow>>

#### Scenario: List workflows is async
- **WHEN** user calls `client.list_workflows().await`
- **THEN** method executes asynchronously
- **AND** returns anyhow::Result<Vec<impl WorkFlow>>

### Requirement: Workflow methods do not accept dry_run parameter
The client methods SHALL NOT accept a dry_run parameter, as it is configured in the client.

#### Scenario: Create workflow without dry_run parameter
- **WHEN** user inspects `create_workflow` signature
- **THEN** method does not have dry_run parameter
- **AND** dry_run behavior is controlled by client configuration

#### Scenario: Get workflow without dry_run parameter
- **WHEN** user inspects `get_workflow` signature
- **THEN** method does not have dry_run parameter

#### Scenario: List workflows without dry_run parameter
- **WHEN** user inspects `list_workflows` signature
- **THEN** method does not have dry_run parameter
