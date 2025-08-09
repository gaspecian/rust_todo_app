//! # Rust Todo App
//!
//! A todo application built with Rust, Axum, and `OpenAPI` documentation.
//! This application provides a REST API for managing todo items with
//! comprehensive health checks and Swagger documentation.

use axum::{routing::get, Router};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod swagger {
    pub mod doc_config;
}

mod modules;

use modules::health::health_routes;
use swagger::doc_config::ApiDoc;

/// Main application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("{address}:{port}");

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .merge(health_routes())
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
