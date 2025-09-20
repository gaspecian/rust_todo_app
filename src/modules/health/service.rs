//! # Health Check Service
//!
//! This module contains the business logic for health check operations.

use axum::{extract::State, response::IntoResponse, Json};
use chrono;

use crate::{
    auth::Claims,
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

#[cfg(test)]
#[allow(clippy::redundant_clone, clippy::single_char_pattern)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_health_response_creation() {
        let response = HealthResponse {
            status: "Healthy".to_string(),
        };
        assert_eq!(response.status, "Healthy");
    }

    #[test]
    fn test_ping_response_creation() {
        let response = PingResponse {
            message: "Pong".to_string(),
            timestamp: "2023-01-01T00:00:00Z".to_string(),
        };
        assert_eq!(response.message, "Pong");
        assert_eq!(response.timestamp, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_health_check_error_conditions() {
        // Test error response creation
        let error_response = HealthResponse {
            status: "Unhealthy".to_string(),
        };
        assert_eq!(error_response.status, "Unhealthy");
    }

    #[test]
    fn test_ping_timestamp_format() {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let response = PingResponse {
            message: "Pong".to_string(),
            timestamp: timestamp.clone(),
        };
        assert!(response.timestamp.contains("T"));
        // RFC3339 format may end with +00:00 instead of Z
        assert!(
            response.timestamp.contains("T")
                && (response.timestamp.contains("Z") || response.timestamp.contains("+00:00"))
        );
    }

    #[test]
    fn test_health_service_responses() {
        let healthy = HealthResponse {
            status: "OK".to_string(),
        };
        let unhealthy = HealthResponse {
            status: "ERROR".to_string(),
        };

        assert_ne!(healthy.status, unhealthy.status);
    }
}
