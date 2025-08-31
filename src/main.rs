//! # Rust Todo App
//!
//! A todo application built with Rust, Axum, and `OpenAPI` documentation.
//! This application provides a REST API for managing todo items with
//! comprehensive health checks and Swagger documentation.

use axum::Router;
use dotenvy::dotenv;
use jsonwebtoken::{ DecodingKey, EncodingKey };
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod swagger {
    pub mod doc_config;
}

mod modules;
mod auth;

use modules::health::health_routes;
use modules::signup::signup_routes;
use modules::login::login_routes;
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
        .merge(signup_routes())
        .merge(login_routes())
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
