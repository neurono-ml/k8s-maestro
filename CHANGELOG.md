# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Test Infrastructure
- Comprehensive test infrastructure with three-tier organization (unit, integration, E2E)
- Kind cluster lifecycle management using testcontainers
- YAML fixture loading and parsing utilities
- Resource creation helpers for ConfigMaps, Secrets, PVCs, and Namespaces
- Resource cleanup helpers with timeout and retry logic
- Resource validation helpers with configurable timeouts
- Selective mocking utilities for unit tests without cluster dependencies
- Sample integration and E2E test files demonstrating test patterns

#### Project Structure
- Module organization with `client`, `workflows`, `steps`, `entities`, `networking`, `security`, and `images` modules
- Basic crate documentation describing k8s-maestro as a Kubernetes job orchestrator

### Documentation
- Test infrastructure documentation in `tests/common/README.md`
- Fixture creation patterns in `tests/common/fixtures/README.md`
- Testing guide in `site-docs/testing.md`
- Updated `AGENTS.md` with test infrastructure usage instructions

### Dependencies
- Added testcontainers for Docker-based test isolation
- Added mockall for mocking in unit tests
- Added base64 for kubeconfig encoding

## [0.3.0] - Previous Release

### Added
- Initial workflow and step management
- Workflow builder pattern
- Step result and status types
- Dependency management with DAG support
