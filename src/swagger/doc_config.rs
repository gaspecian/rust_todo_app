//! # Swagger Documentation Configuration
//!
//! This module configures the `OpenAPI` documentation for the application.

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::modules::{
    common::ErrorResponse,
    user::interfaces::{NewUserResponse, UserSignUp},
};
use crate::modules::{
    health::{
        interfaces::health_response::{HealthResponse, PingResponse},
        service,
    },
    user::interfaces::{LoginUserRequest, LoginUserResponse},
};

use crate::modules::user::routes as user_routes;

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
        service::ping,
        service::test_login,
        user_routes::create_user_route,
        user_routes::login_user_route
    ),
    components(
        schemas(HealthResponse, PingResponse, ErrorResponse, NewUserResponse, UserSignUp, LoginUserRequest, LoginUserResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health Check",
        description = "Endpoints related to health checks and basic functionality."),
        (name = "SignUp",
        description = "User registration and signup endpoints."),
        (name = "Login",
        description = "User login endpoints.")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_auth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            );
        } else {
            tracing::warn!("No components registered in OpenAPI spec when adding security scheme.");
        }
    }
}
