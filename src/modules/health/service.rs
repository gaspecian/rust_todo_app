//! # Health Check Service
//!
//! This module contains the business logic for health check operations.

use axum::{extract::State, response::IntoResponse, Json};
use chrono;

use crate::{
    auth::Claims, modules::health::interfaces::health_response::{HealthResponse, PingResponse}, AppState
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

/// Ping endpoint handler
/// Returns a simple "Pong" message along with the current server timestamp.
/// This endpoint can be used to verify that the server is responsive and
/// to check the current time on the server.
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

/// Test login endpoint handler
/// This endpoint is protected and requires a valid JWT token.
/// It returns a "Pong" message along with the current server timestamp
/// and logs the user ID extracted from the token.
#[utoipa::path(
    get,
    path = "/test_login",
    tag = "Health Check",
    responses(
        (status = 200, description = "Ping successful", body = PingResponse)
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn test_login(state: State<AppState>, claims: Claims) -> impl IntoResponse {
    tracing::info!("User ID from token: {}", claims.user_id);

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
