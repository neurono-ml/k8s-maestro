# ADR-XXXX: [Title]

## Status

Proposed | Accepted | Deprecated | Superseded | Rejected

## Date

YYYY-MM-DD

## Deciders

@username1, @username2, @username3

## Context

[Describe the problem or opportunity that requires a decision.
Include relevant background information and constraints.

Example:
We need to select a primary database for our new e-commerce platform. The system
will handle:
- ~10,000 concurrent users
- Complex product catalog with hierarchical categories
- Transaction processing for orders and payments
- Full-text search for products
- Geospatial queries for store locator

The team has experience with MySQL, PostgreSQL, and MongoDB. We need ACID
compliance for financial transactions.]

## Decision Drivers

- **Driver 1**: [Constraint or requirement]
- **Driver 2**: [Constraint or requirement]
- **Driver 3**: [Constraint or requirement]

## Considered Options

### Option 1: [Option Name]

- **Pros**: [Advantage 1], [Advantage 2]
- **Cons**: [Disadvantage 1], [Disadvantage 2]

### Option 2: [Option Name]

- **Pros**: [Advantage 1], [Advantage 2]
- **Cons**: [Disadvantage 1], [Disadvantage 2]

### Option 3: [Option Name]

- **Pros**: [Advantage 1], [Advantage 2]
- **Cons**: [Disadvantage 1], [Disadvantage 2]

## Decision

We will use **[Selected Option]**.

## Rationale

[Explain why this option was chosen. Reference the decision drivers and
explain how the selected option best addresses them.]

## Consequences

### Positive

- [Consequence 1]
- [Consequence 2]

### Negative

- [Consequence 1]
- [Consequence 2]

### Risks

- [Risk 1]
- [Mitigation strategy]

## Implementation Tracking

### Implementation Commit

- **Commit Hash**: `<commit-hash>` (pending)
- **Commit Message**: Brief description of implementation

### Code Paths

- `crates/k8s-maestro/src/...` (pending)
- `crates/k8s-maestro-k8s/src/...` (pending)

### Tests

- `crates/k8s-maestro/tests/...` (pending)
- `crates/k8s-maestro-k8s/tests/...` (pending)

### Related Artifacts

- PR #: `<pr-number>` (pending)
- Issue #: `<issue-number>` (pending)

## Related Decisions

- [ADR-XXXX](XXXX-title.md) - [Relationship to this ADR]
- [ADR-XXXX](XXXX-title.md) - [Relationship to this ADR]

## References

- [Link to documentation](URL)
- [Link to specification](URL)
- Internal: [Path to internal documentation]
