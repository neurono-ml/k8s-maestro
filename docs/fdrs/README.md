# Feature Description Records

This directory contains Feature Description Records (FDRs) for k8s-maestro.

## What are FDRs?

Feature Description Records document planned features and improvements, their requirements, design, and implementation tracking. They serve as blueprints for feature development and provide visibility into the project roadmap.

## FDR Index

| FDR | Title | Status | Date |
| --- | ----- | ------ | ---- |
| (None yet) | - | - | - |

## Creating a New FDR

1. Copy `template.md` to `XXXX-title-with-dashes.md` (use sequential numbering)
2. Fill in all sections of the template
3. Track commits, code paths, and tests during implementation
4. Submit PR for review
5. Update this index after approval

## FDR Status Values

- **Proposed**: Feature proposed and awaiting approval
- **In Progress**: Currently being implemented
- **Completed**: Feature fully implemented and tested
- **On Hold**: Temporarily paused
- **Cancelled**: Feature will not be implemented

## Implementation Tracking Guidelines

When implementing an FDR, update each phase with:

- **Commits**: List of commit hashes with brief descriptions
- **Code Paths**: Specific files and directories modified in each phase
- **Tests**: Test files added or modified to verify each phase
- **Status**: Current progress of each phase

## Best Practices

- Create FDRs early in the planning process
- Break down large features into manageable phases
- Update tracking information as implementation progresses
- Link related ADRs that influence the feature design
- Document decisions made during implementation
