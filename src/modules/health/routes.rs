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
