//! # Health Check Service
//!
//! This module contains the business logic for health check operations.

use axum::{response::IntoResponse, Json};

use crate::modules::health::interfaces::health_response::HealthResponse;

/// Health check endpoint handler
///
/// Returns the current health status of the application.
/// This endpoint can be used by load balancers and monitoring systems
/// to verify that the application is running correctly.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health Check",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
pub async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "Healthy".to_string(),
    };
    (axum::http::StatusCode::OK, Json(response))
}
