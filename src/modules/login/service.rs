//! # `Login` Service
//!//! This module contains the business logic for user login operations.

use crate::modules::login::{
    interfaces::login_interfaces::{
        LoginRequest, 
        LoginResponse,
        Claims
    }, 
    repository::login_repository::LoginRepository
};

use crate::modules::common::ErrorResponse;
use crate::AppState;

use argon2::{Argon2, password_hash};

use axum::{
    extract::State, response::IntoResponse, Json
};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};

pub struct LoginService {
    login_repository: LoginRepository,
}

impl LoginService {
    pub fn new(login_repository: LoginRepository) -> Self {
        Self { login_repository }
    }

    pub async fn login(&self, login: LoginRequest, encoding_key: EncodingKey) -> Result<String, sqlx::Error> {
        tracing::info!("Attempting to log in user: {}", login.username);

        let user = self.login_repository.fetch_by_username(&login.username).await?;

        use password_hash::{PasswordHash, PasswordVerifier as _};

        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "password".into(),
                source: Box::new(e),
            })?;

        let password_valid = Argon2::default()
            .verify_password(login.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !password_valid {
            tracing::warn!("Invalid password for user: {}", login.username);
            return Err(sqlx::Error::RowNotFound);
        }

        let now = Utc::now();
        let claims = Claims {
            user_id: user.id as i64,
            exp: (now + Duration::hours(1)).timestamp() as usize, // Token expires in 1 hour
            iat: now.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "token".into(),
                source: Box::new(e),
            })?;

        Ok(token)
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
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    let username = login_request.username.clone();
    let login_repository = LoginRepository::new(app_state.db_pool);
    let login_service = LoginService::new(login_repository);
    
    match login_service.login(login_request, app_state.encoding_key).await {
        Ok(token) => (axum::http::StatusCode::OK, Json(LoginResponse {
            username: username, // Use the cloned username
            token: token.to_string(), // Replace with actual token generation logic
        })).into_response(),
        Err(error) => (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                message: "Login failed".to_string(),
            }),
        ).into_response(),
    }
}