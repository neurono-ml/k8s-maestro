## ADDED Requirements

### Requirement: Migration guide overview section
The migration guide SHALL include an overview section explaining the motivation for API changes and the benefits of migrating to the new workflow-centric API.

#### Scenario: User understands migration motivation
- **WHEN** a user reads the overview section
- **THEN** they understand why the API changed and what benefits the new API provides

### Requirement: Breaking changes documentation
The migration guide SHALL document all breaking changes between the old API (Job, JobBuilder, MaestroK8sClient) and new API (Workflow, WorkflowBuilder, MaestroClient).

#### Scenario: User identifies breaking changes
- **WHEN** a user reviews the breaking changes section
- **THEN** they see a complete list of all breaking changes with clear descriptions

#### Scenario: Breaking changes are categorized
- **WHEN** a user views breaking changes
- **THEN** changes are categorized by type (renamed types, moved parameters, new concepts)

### Requirement: Migration steps with code examples
The migration guide SHALL provide step-by-step migration instructions with before/after code examples for common use cases.

#### Scenario: User migrates client creation
- **WHEN** a user needs to update client creation code
- **THEN** they find a code example showing old `MaestroK8sClient::new().await?` vs new client builder pattern

#### Scenario: User migrates job creation
- **WHEN** a user needs to update job creation code
- **THEN** they find a code example showing old `JobBuilder` vs new `WorkflowBuilder` usage

#### Scenario: User migrates execution and waiting
- **WHEN** a user needs to update job execution and waiting code
- **THEN** they find a code example showing old `job.wait().await?` vs new `execution.wait().await?`

#### Scenario: User migrates dry_run configuration
- **WHEN** a user needs to update dry_run handling
- **THEN** they find a code example showing per-call dry_run vs client-level configuration

### Requirement: Common pitfalls section
The migration guide SHALL include a common pitfalls section documenting typical migration mistakes and how to avoid them.

#### Scenario: User avoids common migration mistakes
- **WHEN** a user reads the pitfalls section
- **THEN** they are aware of common mistakes and know how to avoid them

#### Scenario: Pitfalls include solutions
- **WHEN** a pitfall is documented
- **THEN** it includes both the problem description and the solution

### Requirement: FAQ section
The migration guide SHALL include a FAQ section addressing common questions about the migration process.

#### Scenario: User finds answers to common questions
- **WHEN** a user has a question about migration
- **THEN** they can find the answer in the FAQ section

#### Scenario: FAQ is searchable
- **WHEN** a user searches for specific terms
- **THEN** relevant FAQ entries are easily discoverable

### Requirement: Module structure mapping
The migration guide SHALL include a clear mapping between old and new module structures (e.g., `entities::job` → `workflows`).

#### Scenario: User finds equivalent types
- **WHEN** a user looks for the new location of a type
- **THEN** they find a mapping table showing old path to new path

### Requirement: Version targeting
The migration guide SHALL clearly indicate which version it targets and when it was last updated.

#### Scenario: User knows guide relevance
- **WHEN** a user opens the migration guide
- **THEN** they see the target version number and last update date
