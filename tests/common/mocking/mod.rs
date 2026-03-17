//! Selective mocking utilities for unit tests.
//!
//! This module provides utilities for mocking Kubernetes API calls in unit tests,
//! allowing tests to validate logic without requiring a real cluster.

use std::collections::VecDeque;

/// A trait for Kubernetes client operations that can be mocked.
///
/// This trait defines the common operations that tests may need to mock
/// for unit testing without a real Kubernetes cluster.
pub trait K8sClientOperations: Send + Sync {
    /// Creates a resource in the cluster.
    fn create_resource(
        &self,
        resource_type: &str,
        name: &str,
        namespace: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, MockError>;

    /// Gets a resource from the cluster.
    fn get_resource(
        &self,
        resource_type: &str,
        name: &str,
        namespace: &str,
    ) -> Result<serde_json::Value, MockError>;

    /// Deletes a resource from the cluster.
    fn delete_resource(
        &self,
        resource_type: &str,
        name: &str,
        namespace: &str,
    ) -> Result<(), MockError>;

    /// Lists resources in a namespace.
    fn list_resources(
        &self,
        resource_type: &str,
        namespace: &str,
        label_selector: Option<String>,
    ) -> Result<Vec<serde_json::Value>, MockError>;

    /// Updates a resource in the cluster.
    fn update_resource(
        &self,
        resource_type: &str,
        name: &str,
        namespace: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, MockError>;
}

/// Error type for mock operations.
#[derive(Debug, Clone)]
pub enum MockError {
    /// Resource not found.
    NotFound(String),
    /// Resource already exists.
    AlreadyExists(String),
    /// Invalid input.
    InvalidInput(String),
    /// Server error.
    ServerError(String),
    /// Unauthorized.
    Unauthorized,
    /// Generic error.
    Generic(String),
}

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            MockError::AlreadyExists(msg) => write!(f, "AlreadyExists: {}", msg),
            MockError::InvalidInput(msg) => write!(f, "InvalidInput: {}", msg),
            MockError::ServerError(msg) => write!(f, "ServerError: {}", msg),
            MockError::Unauthorized => write!(f, "Unauthorized"),
            MockError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for MockError {}

/// A mock K8s client with controllable responses.
///
/// This struct allows tests to configure what responses the mock client
/// should return for various operations.
pub struct MockK8sClient {
    /// Queue of responses to return for create operations.
    create_responses: VecDeque<Result<serde_json::Value, MockError>>,
    /// Queue of responses to return for get operations.
    get_responses: VecDeque<Result<serde_json::Value, MockError>>,
    /// Queue of responses to return for delete operations.
    delete_responses: VecDeque<Result<(), MockError>>,
    /// Queue of responses to return for list operations.
    list_responses: VecDeque<Result<Vec<serde_json::Value>, MockError>>,
    /// Queue of responses to return for update operations.
    update_responses: VecDeque<Result<serde_json::Value, MockError>>,
}

impl Default for MockK8sClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockK8sClient {
    /// Creates a new mock client with empty response queues.
    pub fn new() -> Self {
        Self {
            create_responses: VecDeque::new(),
            get_responses: VecDeque::new(),
            delete_responses: VecDeque::new(),
            list_responses: VecDeque::new(),
            update_responses: VecDeque::new(),
        }
    }

    /// Adds a response to the create response queue.
    pub fn add_create_response(mut self, response: Result<serde_json::Value, MockError>) -> Self {
        self.create_responses.push_back(response);
        self
    }

    /// Adds a response to the get response queue.
    pub fn add_get_response(mut self, response: Result<serde_json::Value, MockError>) -> Self {
        self.get_responses.push_back(response);
        self
    }

    /// Adds a response to the delete response queue.
    pub fn add_delete_response(mut self, response: Result<(), MockError>) -> Self {
        self.delete_responses.push_back(response);
        self
    }

    /// Adds a response to the list response queue.
    pub fn add_list_response(
        mut self,
        response: Result<Vec<serde_json::Value>, MockError>,
    ) -> Self {
        self.list_responses.push_back(response);
        self
    }

    /// Adds a response to the update response queue.
    pub fn add_update_response(mut self, response: Result<serde_json::Value, MockError>) -> Self {
        self.update_responses.push_back(response);
        self
    }

    /// Returns the next create response from the queue.
    pub fn next_create_response(&mut self) -> Result<serde_json::Value, MockError> {
        self.create_responses
            .pop_front()
            .unwrap_or_else(|| Err(MockError::Generic("No more create responses".to_string())))
    }

    /// Returns the next get response from the queue.
    pub fn next_get_response(&mut self) -> Result<serde_json::Value, MockError> {
        self.get_responses
            .pop_front()
            .unwrap_or_else(|| Err(MockError::Generic("No more get responses".to_string())))
    }

    /// Returns the next delete response from the queue.
    pub fn next_delete_response(&mut self) -> Result<(), MockError> {
        self.delete_responses
            .pop_front()
            .unwrap_or_else(|| Err(MockError::Generic("No more delete responses".to_string())))
    }

    /// Returns the next list response from the queue.
    pub fn next_list_response(&mut self) -> Result<Vec<serde_json::Value>, MockError> {
        self.list_responses
            .pop_front()
            .unwrap_or_else(|| Err(MockError::Generic("No more list responses".to_string())))
    }

    /// Returns the next update response from the queue.
    pub fn next_update_response(&mut self) -> Result<serde_json::Value, MockError> {
        self.update_responses
            .pop_front()
            .unwrap_or_else(|| Err(MockError::Generic("No more update responses".to_string())))
    }
}

/// Generates a mock error response.
///
/// # Arguments
///
/// * `error_type` - The type of error to generate
/// * `message` - The error message
pub fn mock_error(error_type: &str, message: &str) -> MockError {
    match error_type {
        "NotFound" => MockError::NotFound(message.to_string()),
        "AlreadyExists" => MockError::AlreadyExists(message.to_string()),
        "InvalidInput" => MockError::InvalidInput(message.to_string()),
        "ServerError" => MockError::ServerError(message.to_string()),
        "Unauthorized" => MockError::Unauthorized,
        _ => MockError::Generic(message.to_string()),
    }
}

/// Creates a successful mock response with a simple resource.
///
/// # Arguments
///
/// * `resource_type` - The type of resource (e.g., "ConfigMap")
/// * `name` - The name of the resource
/// * `namespace` - The namespace of the resource
pub fn mock_resource_response(
    resource_type: &str,
    name: &str,
    namespace: &str,
) -> serde_json::Value {
    serde_json::json!({
        "apiVersion": "v1",
        "kind": resource_type,
        "metadata": {
            "name": name,
            "namespace": namespace,
        },
    })
}

/// Creates a mock response sequence for multi-step tests.
///
/// This function creates a sequence of responses that can be used
/// to simulate a multi-step operation in tests.
///
/// # Arguments
///
/// * `responses` - A vector of responses to queue up
///
/// # Returns
///
/// A `MockK8sClient` configured with the response sequence.
pub fn mock_response_sequence(responses: Vec<MockResponse>) -> MockK8sClient {
    let mut client = MockK8sClient::new();

    for response in responses {
        client = match response {
            MockResponse::Create(r) => client.add_create_response(r),
            MockResponse::Get(r) => client.add_get_response(r),
            MockResponse::Delete(r) => client.add_delete_response(r),
            MockResponse::List(r) => client.add_list_response(r),
            MockResponse::Update(r) => client.add_update_response(r),
        };
    }

    client
}

/// Types of mock responses for response sequences.
pub enum MockResponse {
    /// Create operation response.
    Create(Result<serde_json::Value, MockError>),
    /// Get operation response.
    Get(Result<serde_json::Value, MockError>),
    /// Delete operation response.
    Delete(Result<(), MockError>),
    /// List operation response.
    List(Result<Vec<serde_json::Value>, MockError>),
    /// Update operation response.
    Update(Result<serde_json::Value, MockError>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_error_display() {
        let error = MockError::NotFound("test resource".to_string());
        assert_eq!(format!("{}", error), "NotFound: test resource");

        let error = MockError::AlreadyExists("existing resource".to_string());
        assert_eq!(format!("{}", error), "AlreadyExists: existing resource");
    }

    #[test]
    fn test_mock_k8s_client_create_response() {
        let mut client =
            MockK8sClient::new().add_create_response(Ok(serde_json::json!({"name": "test"})));

        let response = client.next_create_response();
        assert!(response.is_ok());
        assert_eq!(response.unwrap()["name"], "test");
    }

    #[test]
    fn test_mock_k8s_client_error_response() {
        let mut client = MockK8sClient::new()
            .add_get_response(Err(MockError::NotFound("not found".to_string())));

        let response = client.next_get_response();
        assert!(response.is_err());
    }

    #[test]
    fn test_mock_error_helper() {
        let error = mock_error("NotFound", "resource missing");
        match error {
            MockError::NotFound(msg) => assert_eq!(msg, "resource missing"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_mock_resource_response() {
        let response = mock_resource_response("ConfigMap", "test-cm", "default");
        assert_eq!(response["kind"], "ConfigMap");
        assert_eq!(response["metadata"]["name"], "test-cm");
        assert_eq!(response["metadata"]["namespace"], "default");
    }

    #[test]
    fn test_mock_response_sequence() {
        let sequence = vec![
            MockResponse::Get(Ok(serde_json::json!({"name": "test"}))),
            MockResponse::Delete(Ok(())),
        ];

        let mut client = mock_response_sequence(sequence);

        let get_response = client.next_get_response();
        assert!(get_response.is_ok());

        let delete_response = client.next_delete_response();
        assert!(delete_response.is_ok());
    }
}
