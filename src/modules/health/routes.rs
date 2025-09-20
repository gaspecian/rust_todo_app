//! # Health Check Routes
//!
//! This module defines the HTTP routes for health check endpoints.

use axum::{routing::get, Router};

use crate::modules::health::service::{health_check, ping, test_login};
use crate::AppState;

/// Creates and returns the health check routes
///
/// This function sets up all health-related HTTP routes and returns
/// a configured Router that can be merged with the main application router.
pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/ping", get(ping))
        .route("/test_login", get(test_login))
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_health_routes_creation() {
        let _routes = health_routes();
        // If we can create the routes without panic, the test passes
        assert!(true);
    }

    #[tokio::test]
    async fn test_health_check_function() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[test]
    fn test_health_routes_structure() {
        // Test that we can build the router structure
        let _router = Router::new()
            .route("/health", get(health_check))
            .route("/ping", get(ping))
            .route("/test_login", get(test_login));

        // If we reach here, the routes were created successfully
        assert!(true);
    }
}
