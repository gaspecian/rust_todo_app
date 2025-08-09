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