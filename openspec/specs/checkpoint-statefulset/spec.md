## ADDED Requirements

### Requirement: StatefulSet lifecycle management
The system SHALL provide functions to create, update, and delete the SQLite StatefulSet for checkpoint storage.

#### Scenario: Create StatefulSet
- **WHEN** create_statefulset() is called
- **THEN** a new StatefulSet named "maestro-checkpoint-storage" shall be created

#### Scenario: Update existing StatefulSet
- **WHEN** update_statefulset() is called on an existing StatefulSet
- **THEN** the StatefulSet shall be updated with the new configuration

#### Scenario: Delete StatefulSet
- **WHEN** delete_statefulset() is called
- **THEN** the StatefulSet shall be deleted from the cluster

### Requirement: StatefulSet configuration
The system SHALL create the StatefulSet with a replica count of 1, stable network identity, and ordered pod startup.

#### Scenario: Configure StatefulSet
- **WHEN** the StatefulSet is created
- **THEN** it shall have serviceName, replicas=1, podManagementPolicy=OrderedReady

### Requirement: PVC creation and management
The system SHALL create a PersistentVolumeClaim template in the StatefulSet with configurable storage size and access mode.

#### Scenario: Create PVC template
- **WHEN** the StatefulSet is created
- **THEN** a PVC template shall be included with specified storage size and ReadWriteOnce access mode

#### Scenario: Configure PVC size
- **WHEN** the PVC size is specified in configuration
- **THEN** the StatefulSet shall create PVCs with the specified size

### Requirement: StatefulSet readiness check
The system SHALL wait for the StatefulSet to be ready before allowing checkpoint operations.

#### Scenario: Wait for readiness
- **WHEN** wait_for_statefulset_ready() is called
- **THEN** the system shall poll the StatefulSet status until all replicas are ready

#### Scenario: Timeout on readiness
- **WHEN** the StatefulSet does not become ready within the timeout
- **THEN** the function shall return an error indicating timeout

### Requirement: SQLite container configuration
The system SHALL configure the SQLite container with appropriate image, resource limits, and HTTP server.

#### Scenario: Configure container image
- **WHEN** the StatefulSet is created
- **THEN** the container shall use a specified SQLite image (e.g., alpine with sqlite3)

#### Scenario: Configure HTTP port
- **WHEN** the StatefulSet is created
- **THEN** the container shall expose an HTTP port (default 8080)

#### Scenario: Configure resource limits
- **WHEN** resource limits are specified in configuration
- **THEN** the container shall have the configured CPU and memory limits

### Requirement: StatefulSet health checks
The system SHALL configure liveness and readiness probes for the SQLite container.

#### Scenario: Configure liveness probe
- **WHEN** the StatefulSet is created
- **THEN** a liveness probe shall be configured to check the HTTP endpoint

#### Scenario: Configure readiness probe
- **WHEN** the StatefulSet is created
- **THEN** a readiness probe shall be configured to check the HTTP endpoint

### Requirement: StatefulSet pod hostname resolution
The system SHALL use the stable hostname provided by the StatefulSet for pod communication.

#### Scenario: Resolve pod hostname
- **WHEN** the HTTP client connects to the SQLite pod
- **THEN** it shall use the stable hostname (maestro-checkpoint-storage-0)

### Requirement: StatefulSet namespace configuration
The system SHALL allow configuration of the namespace where the StatefulSet is deployed.

#### Scenario: Configure namespace
- **WHEN** the namespace is specified in configuration
- **THEN** the StateSet shall be created in the specified namespace

### Requirement: StatefulSet labels and annotations
The system SHALL apply configurable labels and annotations to the StatefulSet for organization and management.

#### Scenario: Apply labels
- **WHEN** labels are specified in configuration
- **THEN** the StatefulSet shall have the specified labels

#### Scenario: Apply annotations
- **WHEN** annotations are specified in configuration
- **THEN** the StatefulSet shall have the specified annotations

### Requirement: StatefulSet cleanup
The system SHALL provide cleanup functions to delete StatefulSet and associated PVCs.

#### Scenario: Clean up StatefulSet
- **WHEN** cleanup_statefulset() is called with delete_pvc=true
- **THEN** the StatefulSet and its PVCs shall be deleted

#### Scenario: Clean up StatefulSet without PVCs
- **WHEN** cleanup_statefulset() is called with delete_pvc=false
- **THEN** the StatefulSet shall be deleted but PVCs retained

### Requirement: StatefulSet status monitoring
The system SHALL provide functions to check the status of the StatefulSet (replicas, ready replicas, current revision).

#### Scenario: Check StatefulSet status
- **WHEN** get_statefulset_status() is called
- **THEN** the function shall return the current status of the StatefulSet
