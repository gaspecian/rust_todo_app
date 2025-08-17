//! # `Login` Service
//!//! This module contains the business logic for user login operations.

use crate::modules::login::{
    interfaces::login_interfaces::{
        LoginRequest, 
        LoginResponse
    }, 
    repository::login_repository::LoginRepository
};

use crate::modules::common::ErrorResponse;
use crate::AppState;

use axum::{
    extract::State, Json
};

pub struct LoginService {
    login_repository: LoginRepository,
}

impl LoginService {
    pub fn new(login_repository: LoginRepository) -> Self {
        Self { login_repository }
    }

    pub async fn login(&self, login: LoginRequest) -> Result<bool, sqlx::Error> {
        tracing::info!("Attempting to log in user: {}", login.username);

        Ok(true)
    }
}


/// Handler function for the signup route
#[utoipa::path(
    post,
    path = "/login",
    tag = "Login",
    request_body = LoginRequest,
    responses(
        (status = 201, description = "User login successfully", body = LoginResponse),
        (status = 400, description = "Invalid user data", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn login(
    State(app_state): State<AppState>,
    Json(user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    let login_repository = LoginRepository::new(app_state.db_pool);
    let login_service = LoginService::new(login_repository);
    
    match login_service.login(user).await {
        Ok(_) => Ok(Json(LoginResponse {
            id: 1,
            username: "dummy_username".to_string(), // Replace with actual username from the database
            token: "dummy_token".to_string(), // Replace with actual token generation logic
        })),
        Err(e) => Err(Json(ErrorResponse::new(format!("Login failed: {}", e)))),
    }
}