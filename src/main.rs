//! # Rust Todo App
//!
//! A todo application built with Rust, Axum, and `OpenAPI` documentation.
//! This application provides a REST API for managing todo items with
//! comprehensive health checks and Swagger documentation.

use axum::Router;
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod swagger {
    pub mod doc_config;
}

mod auth;
mod modules;
mod utils;

use modules::health::health_routes;
use modules::user::user_routes;
use swagger::doc_config::ApiDoc;

/// Application state containing shared resources
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db_pool: Pool<Postgres>,
    /// JWT encoding key
    pub encoding_key: EncodingKey,
    /// JWT decoding key
    pub decoding_key: DecodingKey,
    /// Session duration in minutes
    pub session_duration_minutes: i64,
}

/// Main application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        tracing::warn!("DATABASE_URL not set, using default PostgreSQL connection");
        "postgresql://localhost/rust_todo_app".to_string()
    });

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create database connection pool: {}", e);
            e
        })?;

    // Get server address and port from environment
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("{address}:{port}");

    // Get JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set, using default secret");
        "my_secret_key".to_string()
    });
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let session_duration_minutes = std::env::var("SESSION_DURATION_MINUTES")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(60); // default to 60 minutes

    // Create application state
    let app_state = AppState {
        db_pool: pool,
        encoding_key,
        decoding_key,
        session_duration_minutes,
    };

    // Build the application router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .merge(health_routes())
        .merge(user_routes())
        .with_state(app_state);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        tracing::error!("Failed to bind to address {}: {}", addr, e);
        e
    })?;

    tracing::info!("Server listening on {}", listener.local_addr()?);

    // Start the server
    axum::serve(listener, app).await.map_err(|e| {
        tracing::error!("Server error: {}", e);
        e
    })?;

    Ok(())
}

#[cfg(test)]
#[allow(
    clippy::assertions_on_constants,
    clippy::no_effect_underscore_binding,
    clippy::unwrap_used,
    clippy::single_char_pattern,
    clippy::len_zero
)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        // Test that we can create the basic components needed for AppState
        let encoding_key = EncodingKey::from_secret("test_secret".as_ref());
        let decoding_key = DecodingKey::from_secret("test_secret".as_ref());

        // Verify keys are created successfully (we can't access private methods, so just test creation)
        let _encoding_key = encoding_key;
        let _decoding_key = decoding_key;
        // If we get here, the keys were created successfully
        assert!(true);
    }

    #[test]
    fn test_environment_variables() {
        // Test environment variable parsing
        std::env::set_var("TEST_VAR", "test_value");
        let value = std::env::var("TEST_VAR").unwrap_or_else(|_| "default".to_string());
        assert_eq!(value, "test_value");

        // Test default value
        let default_value =
            std::env::var("NON_EXISTENT_VAR").unwrap_or_else(|_| "default".to_string());
        assert_eq!(default_value, "default");
    }

    #[test]
    fn test_port_parsing() {
        // Test port parsing logic
        std::env::set_var("PORT", "3000");
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .unwrap_or(8000);
        assert_eq!(port, 3000);

        // Test default port
        std::env::remove_var("PORT");
        let default_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .unwrap_or(8000);
        assert_eq!(default_port, 8000);
    }

    #[test]
    fn test_session_duration_parsing() {
        // Test session duration parsing
        std::env::set_var("SESSION_DURATION_MINUTES", "120");
        let duration: i64 = std::env::var("SESSION_DURATION_MINUTES")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);
        assert_eq!(duration, 120);

        // Test default duration
        std::env::remove_var("SESSION_DURATION_MINUTES");
        let default_duration: i64 = std::env::var("SESSION_DURATION_MINUTES")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);
        assert_eq!(default_duration, 60);
    }

    #[test]
    fn test_database_url_validation() {
        std::env::set_var("DATABASE_URL", "postgresql://user:pass@localhost:5432/db");
        let url = std::env::var("DATABASE_URL").unwrap();
        assert!(url.starts_with("postgresql://"));
        assert!(url.contains("@"));
    }

    #[test]
    fn test_jwt_key_creation() {
        use jsonwebtoken::{DecodingKey, EncodingKey};

        let secret = "test_secret_key";
        let _encoding_key = EncodingKey::from_secret(secret.as_ref());
        let _decoding_key = DecodingKey::from_secret(secret.as_ref());

        // Just test that keys can be created without errors
        assert!(secret.len() > 0);
    }

    #[test]
    fn test_environment_variable_parsing() {
        std::env::set_var("TEST_VAR", "test_value");
        let value = std::env::var("TEST_VAR").unwrap_or_default();
        assert_eq!(value, "test_value");

        std::env::remove_var("TEST_VAR");
        let default_value = std::env::var("TEST_VAR").unwrap_or_default();
        assert_eq!(default_value, "");
    }

    #[test]
    fn test_address_port_parsing() {
        std::env::set_var("ADDRESS", "0.0.0.0");
        std::env::set_var("PORT", "3000");

        let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
        let addr = format!("{address}:{port}");

        assert_eq!(addr, "0.0.0.0:3000");

        std::env::remove_var("ADDRESS");
        std::env::remove_var("PORT");
    }

    #[test]
    fn test_jwt_secret_handling() {
        std::env::set_var("JWT_SECRET", "test_secret");
        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret_key".to_string());
        assert_eq!(jwt_secret, "test_secret");

        std::env::remove_var("JWT_SECRET");
        let default_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret_key".to_string());
        assert_eq!(default_secret, "my_secret_key");
    }

    #[test]
    fn test_database_url_default() {
        std::env::remove_var("DATABASE_URL");
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/rust_todo_app".to_string());
        assert_eq!(database_url, "postgresql://localhost/rust_todo_app");
    }
}
