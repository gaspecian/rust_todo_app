//! # Common Module
//!
//! This module contains shared types and utilities used across the application.

use axum::{response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

/// Standard error response structure
#[derive(Serialize, ToSchema)]
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