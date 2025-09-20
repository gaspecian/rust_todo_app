//! # Health Response Data Structure
//!
//! This module defines the response structure for health check endpoints.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response structure for health check endpoints
///
/// This structure represents the JSON response returned by health check endpoints.
/// It includes the current status of the application.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HealthResponse {
    /// Current health status of the application
    pub status: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct PingResponse {
    pub message: String,
    pub timestamp: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_creation() {
        let response = HealthResponse {
            status: "healthy".to_string(),
        };
        assert_eq!(response.status, "healthy");
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
    }

    #[test]
    fn test_health_response_deserialization() {
        let json = r#"{"status":"healthy"}"#;
        let response: HealthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "healthy");
    }

    #[test]
    fn test_ping_response_creation() {
        let response = PingResponse {
            message: "pong".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        };
        assert_eq!(response.message, "pong");
        assert_eq!(response.timestamp, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_ping_response_serialization() {
        let response = PingResponse {
            message: "pong".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("pong"));
        assert!(json.contains("2023-01-01T00:00:00Z"));
    }

    #[test]
    fn test_ping_response_deserialization() {
        let json = r#"{"message":"pong","timestamp":"2023-01-01T00:00:00Z"}"#;
        let response: PingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.message, "pong");
        assert_eq!(response.timestamp, "2023-01-01T00:00:00Z");
    }
}
