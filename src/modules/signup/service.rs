//! # `SignUp` Service
//!
//! This module contains the business logic for user signup operations.
use time::OffsetDateTime;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use axum::{extract::State, response::IntoResponse, Json};

use crate::modules::common::ErrorResponse;
use crate::modules::signup::interfaces::signup_interfaces::{
    NewSignUpInterface, SignUpRequest, SignUpResponse,
};
use crate::modules::signup::repository::signup_repository::SignUpRepository;
use crate::AppState;

pub struct SignUpService {
    signup_repository: SignUpRepository,
}

impl SignUpService {
    // Constructor to create a new SignUpService instance
    pub const fn new(signup_repository: SignUpRepository) -> Self {
        Self { signup_repository }
    }

    pub async fn signup_user(
        &self,
        user: SignUpRequest,
    ) -> Result<SignUpResponse, Json<ErrorResponse>> {
        // Validate user data
        if user.password != user.confirm_password {
            return Err(Json(ErrorResponse::new("Passwords do not match")));
        }

        match self
            .signup_repository
            .exists_by_username(&user.username)
            .await
        {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Username already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        match self.signup_repository.exists_by_email(&user.email).await {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Email already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        // Hash the password
        let hashed_password =
            Self::hash_password(&user.password).map_err(|e| Json(ErrorResponse::new(e)))?;

        // Save user to database
        let created_user = self
            .signup_repository
            .create_user(&NewSignUpInterface {
                username: user.username,
                email: user.email,
                password: hashed_password,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })
            .await
            .map_err(|e| Json(ErrorResponse::new(format!("Failed to create user: {e}"))))?;

        Ok(created_user)
    }

    fn hash_password(password: &str) -> Result<String, String> {
        // Salt generation
        let salt = SaltString::generate(&mut OsRng);

        // Argon2::default() provides a default configuration for Argon2
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password: {e}"))?
            .to_string();

        Ok(password_hash)
    }
}

/// Handler function for the signup route
#[utoipa::path(
    post,
    path = "/signup",
    tag = "SignUp",
    request_body = SignUpRequest,
    responses(
        (status = 201, description = "User signed up successfully", body = SignUpResponse),
        (status = 400, description = "Invalid user data", body = ErrorResponse)
    )
)]
pub async fn signup(
    State(app_state): State<AppState>,
    Json(user_request): Json<SignUpRequest>,
) -> impl IntoResponse {
    let signup_repository = SignUpRepository::new(app_state.db_pool);
    let signup_service = SignUpService::new(signup_repository);

    match signup_service.signup_user(user_request).await {
        Ok(user) => (axum::http::StatusCode::CREATED, Json(user)).into_response(),
        Err(error) => (axum::http::StatusCode::BAD_REQUEST, error).into_response(),
    }
}
