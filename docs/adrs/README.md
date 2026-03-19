# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for k8s-maestro.

## What are ADRs?

Architecture Decision Records capture significant technical decisions, their context, and their consequences. They serve as historical documentation of the project's architectural evolution.

## ADR Index

| ADR | Title | Status | Date |
| --- | ----- | ------ | ---- |
| (None yet) | - | - | - |

## Creating a New ADR

1. Copy `template.md` to `XXXX-title-with-dashes.md` (use sequential numbering)
2. Fill in all sections of the template
3. Ensure commit tracking is updated after implementation
4. Submit PR for review
5. Update this index after approval

## ADR Status Values

- **Proposed**: Under discussion and review
- **Accepted**: Decision made and being implemented
- **Deprecated**: No longer relevant but kept for reference
- **Superseded**: Replaced by a newer ADR
- **Rejected**: Considered but not adopted

## Commit Tracking Guidelines

When implementing an ADR, update the "Implementation Tracking" section with:

- **Implementation Commit**: The hash of the commit that implements the decision
- **Code Paths**: Specific files and directories modified by the implementation
- **Tests**: Test files added or modified to verify the implementation
- **Related Artifacts**: Links to PRs, issues, or other related documentation

## Best Practices

- Write ADRs early, before implementation starts
- Keep them concise (1-2 pages maximum)
- Be honest about trade-offs and consequences
- Link related ADRs to build decision graph
- Update status when decisions change (create new ADR to supersede)
