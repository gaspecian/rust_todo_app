//! # Swagger Documentation Configuration
//!
//! This module configures the `OpenAPI` documentation for the application.

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::modules::{
    common::ErrorResponse,
    user::interfaces::{FetchUserResponse, NewUserResponse, UserSignUp},
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
        user_routes::login_user_route,
        user_routes::fetch_user_route,
        user_routes::update_user_route,
        user_routes::delete_user_route,
        user_routes::update_password_route,
    ),
    components(
        schemas(HealthResponse, PingResponse, ErrorResponse, NewUserResponse, UserSignUp, LoginUserRequest, LoginUserResponse, FetchUserResponse)
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
        description = "User login endpoints."),
        (name = "User Management",
        description = "User management endpoints.")
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use utoipa::OpenApi;

    #[test]
    fn test_api_doc_creation() {
        let doc = ApiDoc::openapi();
        assert_eq!(doc.info.title, "Todo App API");
        assert_eq!(doc.info.version, "1.0.0");
        assert!(doc.info.description.is_some());
    }

    #[test]
    fn test_security_addon() {
        let addon = SecurityAddon;
        let mut openapi = utoipa::openapi::OpenApi::new(
            utoipa::openapi::Info::new("Test", "1.0.0"),
            utoipa::openapi::Paths::new(),
        );

        // Add components to test the security addon
        openapi.components = Some(utoipa::openapi::Components::new());
        addon.modify(&mut openapi);

        assert!(openapi.components.is_some());
        let components = openapi.components.unwrap();
        assert!(components.security_schemes.contains_key("jwt_auth"));
    }

    #[test]
    fn test_security_addon_no_components() {
        let addon = SecurityAddon;
        let mut openapi = utoipa::openapi::OpenApi::new(
            utoipa::openapi::Info::new("Test", "1.0.0"),
            utoipa::openapi::Paths::new(),
        );

        // Test with no components
        addon.modify(&mut openapi);
        assert!(openapi.components.is_none());
    }
}
