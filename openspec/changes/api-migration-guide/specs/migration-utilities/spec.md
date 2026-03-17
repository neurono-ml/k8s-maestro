## ADDED Requirements

### Requirement: Deprecation warnings on old types
The migration utilities module SHALL provide deprecation warnings using Rust's `#[deprecated]` attribute on old types when they are kept for backward compatibility.

#### Scenario: User sees deprecation warning
- **WHEN** a user compiles code using deprecated types
- **THEN** they see a clear deprecation warning with migration guidance

#### Scenario: Deprecation includes version and alternative
- **WHEN** a deprecation warning is displayed
- **THEN** it includes the deprecation version and suggests the replacement type

### Requirement: Type aliases for backward compatibility
The migration utilities module SHALL provide type aliases mapping old type names to new type names where semantically equivalent.

#### Scenario: Type alias compiles old code
- **WHEN** a user uses an old type name via alias
- **THEN** the code compiles with a deprecation warning instead of an error

#### Scenario: Type alias points to correct new type
- **WHEN** a type alias is defined
- **THEN** it maps to the semantically equivalent new type

### Requirement: Migration helper module structure
The migration utilities SHALL be organized in a dedicated `src/migration/mod.rs` module that can be optionally imported.

#### Scenario: Module is optional
- **WHEN** a user does not need migration utilities
- **THEN** they are not required to import the migration module

#### Scenario: Module is documented
- **WHEN** a user views the migration module documentation
- **THEN** they see clear usage instructions and examples

### Requirement: Code example annotations
The migration utilities module documentation SHALL include code examples showing proper migration patterns.

#### Scenario: User finds migration examples in docs
- **WHEN** a user views migration module documentation
- **THEN** they find runnable code examples for common migrations

### Requirement: No runtime overhead
The migration utilities SHALL NOT introduce runtime overhead when used, relying only on compile-time constructs.

#### Scenario: Aliases have no runtime cost
- **WHEN** type aliases are used
- **THEN** there is zero runtime performance impact

#### Scenario: Deprecation checks are compile-time only
- **WHEN** deprecation attributes are processed
- **THEN** they occur only at compile time with no runtime cost
