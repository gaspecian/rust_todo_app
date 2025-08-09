//! # Health Check Service
//!
//! This module contains the business logic for health check operations.

use axum::{extract::State, response::IntoResponse, Json};

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
    let now = match sqlx::query!("SELECT NOW()").fetch_one(&state.db_pool).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("Failed to fetch current time from database: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(PingResponse {
                    message: "Database error".to_string(),
                    timestamp: String::new(),
                }),
            );
        }
    };

    let response = PingResponse {
        message: "Pong".to_string(),
        timestamp: format!("{:?}", now.now),
    };
    (axum::http::StatusCode::OK, Json(response))
}
