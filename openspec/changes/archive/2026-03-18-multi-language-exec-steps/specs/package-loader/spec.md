## ADDED Requirements

### Requirement: Package source enumeration
The system SHALL provide a `PackageSource` enum with variants for Git, RemotePath, LocalPath, and Registry sources.

#### Scenario: Git source with branch and path
- **WHEN** a PackageSource::Git is created with url "https://github.com/org/repo", branch "main", and path "packages/core"
- **THEN** the source SHALL store all three values for later resolution

#### Scenario: Remote URL source
- **WHEN** a PackageSource::RemotePath is created with url "https://example.com/package.tar.gz"
- **THEN** the source SHALL store the URL for HTTP download

#### Scenario: Local filesystem source
- **WHEN** a PackageSource::LocalPath is created with path "/local/packages/core"
- **THEN** the source SHALL store the path for filesystem access

#### Scenario: Registry source
- **WHEN** a PackageSource::Registry is created with registry "https://registry.io", package_name "my-package", and version "1.0.0"
- **THEN** the source SHALL store all three values for registry resolution

### Requirement: Package loader interface
The system SHALL provide a `PackageLoader` struct with methods to load packages from any PackageSource.

#### Scenario: Load from any source
- **WHEN** PackageLoader::load is called with a valid PackageSource
- **THEN** the loader SHALL resolve the source and return a PathBuf to the local package location

#### Scenario: Fetch from Git repository
- **WHEN** PackageLoader::fetch_git is called with a valid Git URL, branch, and path
- **THEN** the loader SHALL clone the repository, checkout the branch, and return the path to the specified subdirectory

#### Scenario: Fetch from remote URL
- **WHEN** PackageLoader::fetch_remote is called with a valid HTTP URL
- **THEN** the loader SHALL download the file and return the path to the downloaded file

#### Scenario: Validate local path
- **WHEN** PackageLoader::validate_local is called with a valid local path
- **THEN** the loader SHALL verify the path exists and is accessible, returning the canonicalized path

### Requirement: Package caching
The system SHALL provide a `PackageCache` struct to cache downloaded packages locally.

#### Scenario: Cache hit
- **WHEN** a package is requested that already exists in the cache
- **THEN** the system SHALL return the cached path without re-downloading

#### Scenario: Cache miss
- **WHEN** a package is requested that does not exist in the cache
- **THEN** the system SHALL download the package, store it in the cache, and return the cached path

#### Scenario: Cache key generation
- **WHEN** caching a package from any source
- **THEN** the system SHALL generate a unique cache key using SHA-256 hash of the source identifier

### Requirement: Error handling
The system SHALL return descriptive errors for package loading failures.

#### Scenario: Git clone failure
- **WHEN** a Git clone operation fails due to invalid URL or authentication
- **THEN** the system SHALL return an error with the Git error message

#### Scenario: Network failure
- **WHEN** a remote download fails due to network issues
- **THEN** the system SHALL return an error with the HTTP status or connection error

#### Scenario: Invalid local path
- **WHEN** a local path does not exist or is not accessible
- **THEN** the system SHALL return an error with the filesystem error message
