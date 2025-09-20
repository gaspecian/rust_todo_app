//! # Common Module
//!
//! This module contains shared types and utilities used across the application.

use axum::{response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

/// Standard error response structure
#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_new_string() {
        let error = ErrorResponse::new("Test error message");
        assert_eq!(error.message, "Test error message");
    }

    #[test]
    fn test_error_response_new_str() {
        let message = "Another error";
        let error = ErrorResponse::new(message);
        assert_eq!(error.message, "Another error");
    }

    #[test]
    fn test_error_response_new_string_owned() {
        let message = String::from("Owned string error");
        let error = ErrorResponse::new(message);
        assert_eq!(error.message, "Owned string error");
    }

    #[test]
    fn test_error_response_into_response() {
        let error = ErrorResponse::new("Test error");
        let response = error.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
