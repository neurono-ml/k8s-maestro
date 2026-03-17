## 1. README.md Updates

- [ ] 1.1 Add GitHub badges (version, license, build status, docs.rs, crates.io)
- [ ] 1.2 Add project description: "A Kubernetes workflow orchestrator with minimal requirements and full power"
- [ ] 1.3 Add features section with all 10 features (multi-step workflows, conditional execution, step types, services/ingress, sidecars, file observer, checkpointing, multi-tenant security, builder pattern, TDD)
- [ ] 1.4 Add installation instructions
- [ ] 1.5 Add quick start guide with basic example
- [ ] 1.6 Add usage examples (basic and advanced with dependency chains)
- [ ] 1.7 Add API documentation links
- [ ] 1.8 Add contributing guidelines section
- [ ] 1.9 Add license section (MIT OR Apache-2.0)

## 2. Examples README

- [ ] 2.1 Create examples/README.md with descriptions for all examples

## 3. Rename Existing Examples

- [ ] 3.1 Rename use_job_builder.rs to use_workflow_builder.rs (git mv)
- [ ] 3.2 Rename apply_and_watch.rs to apply_and_watch_workflow.rs (git mv)
- [ ] 3.3 Rename delete.rs to delete_workflow.rs (git mv)

## 4. Update Existing Examples

- [ ] 4.1 Update use_workflow_builder.rs with workflow-centric API and comments
- [ ] 4.2 Update apply_and_watch_workflow.rs with workflow patterns and comments
- [ ] 4.3 Update delete_workflow.rs with workflow cleanup patterns and comments
- [ ] 4.4 Update use_volumes.rs to use current API with comments
- [ ] 4.5 Update dependency_system.rs with additional comments

## 5. Create New Examples

- [ ] 5.1 Create use_services.rs demonstrating service exposure
- [ ] 5.2 Create use_sidecar.rs demonstrating sidecar containers
- [ ] 5.3 Create multi_step_workflow.rs demonstrating dependency chains
- [ ] 5.4 Create python_step.rs (aspirational API example)
- [ ] 5.5 Create rust_step.rs (aspirational API example)
- [ ] 5.6 Create wasm_step.rs (aspirational API example)

## 6. Site-Docs Landing Page

- [ ] 6.1 Create site-docs/index.md with project overview and navigation

## 7. Site-Docs Getting Started

- [ ] 7.1 Create site-docs/getting-started/installation.md
- [ ] 7.2 Create site-docs/getting-started/quick-start.md
- [ ] 7.3 Create site-docs/getting-started/concepts.md

## 8. Site-Docs Guides

- [ ] 8.1 Create site-docs/guides/basic-workflow.md
- [ ] 8.2 Create site-docs/guides/dependencies.md
- [ ] 8.3 Create site-docs/guides/services-ingress.md
- [ ] 8.4 Create site-docs/guides/multi-language.md
- [ ] 8.5 Create site-docs/guides/checkpointing.md
- [ ] 8.6 Create site-docs/guides/security.md

## 9. Site-Docs API Reference

- [ ] 9.1 Create site-docs/api/client.md
- [ ] 9.2 Create site-docs/api/workflow.md
- [ ] 9.3 Create site-docs/api/steps.md
- [ ] 9.4 Create site-docs/api/networking.md

## 10. Site-Docs Examples

- [ ] 10.1 Create site-docs/examples/spark-cluster.md
- [ ] 10.2 Create site-docs/examples/ml-pipeline.md
- [ ] 10.3 Create site-docs/examples/data-processing.md

## 11. Site-Docs Reference

- [ ] 11.1 Create site-docs/reference/configuration.md
- [ ] 11.2 Create site-docs/reference/troubleshooting.md

## 12. Verification

- [ ] 12.1 Verify all example files have explanatory comments
- [ ] 12.2 Verify documentation links are correct
- [ ] 12.3 Run cargo clippy on all examples
