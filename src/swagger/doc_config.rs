//! # Swagger Documentation Configuration
//!
//! This module configures the `OpenAPI` documentation for the application.

use utoipa::OpenApi;

use crate::modules::health::{interfaces::health_response::HealthResponse, service};

/// `OpenAPI` documentation configuration
///
/// This struct defines the `OpenAPI` specification for the entire application,
/// including all endpoints, schemas, and metadata.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Todo App API",
        version = "1.0.0",
        description = "API documentation for the Todo App built with Axum and Utoipa.",
    ),
    paths(
        service::health_check,
    ),
    components(
        schemas(HealthResponse)
    ),
    tags(
        (name = "Health Check",
        description = "Endpoints related to health checks and basic functionality.")
    )
)]
pub struct ApiDoc;
