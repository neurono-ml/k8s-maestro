## Why

The project lacks comprehensive documentation, making it difficult for new users to understand capabilities, get started quickly, and use the library effectively. The current README.md is minimal (19 lines), there's no GitHub Pages documentation, and examples need to be updated to reflect the new workflow-centric API. Professional documentation with badges, examples, and guides is essential for adoption and usability.

## What Changes

- **README.md**: Complete rewrite with badges, features, installation, usage examples, API links, and contributing guidelines
- **site-docs/**: New GitHub Pages documentation structure with landing page, guides, API docs, and examples
- **examples/**: Rename and update existing examples to use workflow-centric API, add new examples for multi-step workflows, services, sidecars, and multi-language steps
- **examples/README.md**: New file with descriptions for all examples
- Project description: "A Kubernetes workflow orchestrator with minimal requirements and full power"

### Files to Create
- `README.md` (rewrite)
- `examples/README.md` (new)
- `examples/use_workflow_builder.rs` (renamed from use_job_builder.rs)
- `examples/apply_and_watch_workflow.rs` (renamed from apply_and_watch.rs)
- `examples/delete_workflow.rs` (renamed from delete.rs)
- `examples/use_services.rs` (new)
- `examples/use_sidecar.rs` (new)
- `examples/multi_step_workflow.rs` (new)
- `examples/python_step.rs` (new)
- `examples/rust_step.rs` (new)
- `examples/wasm_step.rs` (new)
- `site-docs/index.md` (new)
- `site-docs/getting-started/installation.md` (new)
- `site-docs/getting-started/quick-start.md` (new)
- `site-docs/getting-started/concepts.md` (new)
- `site-docs/guides/basic-workflow.md` (new)
- `site-docs/guides/dependencies.md` (new)
- `site-docs/guides/services-ingress.md` (new)
- `site-docs/guides/multi-language.md` (new)
- `site-docs/guides/checkpointing.md` (new)
- `site-docs/guides/security.md` (new)
- `site-docs/api/client.md` (new)
- `site-docs/api/workflow.md` (new)
- `site-docs/api/steps.md` (new)
- `site-docs/api/networking.md` (new)
- `site-docs/examples/spark-cluster.md` (new)
- `site-docs/examples/ml-pipeline.md` (new)
- `site-docs/examples/data-processing.md` (new)
- `site-docs/reference/configuration.md` (new)
- `site-docs/reference/troubleshooting.md` (new)

### Files to Update
- `examples/use_volumes.rs` (update to new API)

### Files to Remove
- `examples/use_job_builder.rs` (renamed)
- `examples/apply_and_watch.rs` (renamed)
- `examples/delete.rs` (renamed)

## Capabilities

### New Capabilities
- `comprehensive-readme`: Professional README with badges, features, quick start, installation, examples, and contributing guidelines
- `github-pages-docs`: Complete site-docs structure for GitHub Pages with getting-started, guides, API, examples, and reference sections
- `workflow-examples`: Updated and new examples demonstrating workflow builder, multi-step workflows, services, sidecars, and multi-language steps

### Modified Capabilities
None - this is purely a documentation update

## Impact

- **Documentation**: Major improvement in user onboarding and API discoverability
- **Examples**: Better demonstration of library capabilities with workflow-centric API
- **No code changes**: All changes are documentation and example files only
- **GitHub Pages**: New documentation site structure ready for deployment
