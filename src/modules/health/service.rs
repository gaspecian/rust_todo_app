//! # Health Check Service
//!
//! This module contains the business logic for health check operations.

use axum::{extract::State, response::IntoResponse, Json};
use chrono;

use crate::{
    modules::health::interfaces::health_response::{HealthResponse, PingResponse},
    AppState,
};

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

#[utoipa::path(
    get,
    path = "/ping",
    tag = "Health Check",
    responses(
        (status = 200, description = "Ping successful", body = PingResponse)
    )
)]
pub async fn ping(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query!("SELECT NOW()").fetch_one(&state.db_pool).await {
        Ok(row) => {
            let response = PingResponse {
                message: "Pong".to_string(),
                timestamp: format!("{:?}", row.now),
            };
            (axum::http::StatusCode::OK, Json(response))
        }
        Err(e) => {
            tracing::error!("Database connection failed: {}", e);
            let response = PingResponse {
                message: "Database unavailable".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            (axum::http::StatusCode::SERVICE_UNAVAILABLE, Json(response))
        }
    }
}
