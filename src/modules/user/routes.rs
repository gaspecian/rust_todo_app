//! #`User` Routes
//! This module defines the HTTP routes for users funciionality.

use axum::routing::{delete, get, post, put};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::auth::Claims;
use crate::modules::common::ErrorResponse;
use crate::modules::user::interfaces::{
    FetchUserResponse, LoginUserRequest, LoginUserResponse, UpdatePasswordRequest,
    UpdateUserRequest, UpdateUserResponse, UserSignUp,
};
use crate::modules::user::repository::UserRepository;
use crate::modules::user::service::UserService;
use crate::AppState;

#[derive(Serialize, ToSchema)]
struct Response {
    message: String,
}

// Creates and returns the signup routes
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user/signup", post(create_user_route))
        .route("/user/login", post(login_user_route))
        .route("/user", get(fetch_user_route))
        .route("/user", put(update_user_route))
        .route("/user/password", post(update_password_route))
        .route("/user", delete(delete_user_route))
}

// Create User Route
/// Handler function for the signup route
#[utoipa::path(
    post,
    path = "/user/signup",
    tag = "SignUp",
    request_body = UserSignUp,
    responses(
        (status = 201, description = "User signed up successfully", body = Response),
        (status = 400, description = "Invalid user data", body = ErrorResponse)
    )
)]
pub async fn create_user_route(
    State(app_state): State<AppState>,
    Json(user_signup): Json<UserSignUp>,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    match user_service.create_user(user_signup).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}

/// Handler function for login route
#[utoipa::path(
    post,
    path = "/user/login",
    tag = "Login",
    //request_body = UserSignUp,
    responses(
        (status = 201, description = "User logged successfully", body = LoginUserResponse),
        (status = 401, description = "Not Authorized", body = ErrorResponse),
        (status = 400, description = "Invalid user data", body = ErrorResponse)
    )
)]
pub async fn login_user_route(
    State(app_state): State<AppState>,
    Json(user_login): Json<LoginUserRequest>,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    tracing::info!("Login attempt");

    match user_service
        .login_user(
            user_login,
            app_state.encoding_key,
            app_state.session_duration_minutes,
        )
        .await
    {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}

// Fetch User Route
#[utoipa::path(
    get,
    path = "/user",
    tag = "User Management",
    responses(
        (status = 200, description = "User fetched successfully", body = FetchUserResponse),
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn fetch_user_route(
    State(app_state): State<AppState>,
    claims: Claims,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    let user_id = claims.user_id;

    match user_service.fetch_user(user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}

// Update User Route
#[utoipa::path(
    put,
    path = "/user",
    tag = "User Management",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UpdateUserResponse),
        (status = 400, description = "Invalid data", body = ErrorResponse)
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn update_user_route(
    State(app_state): State<AppState>,
    claims: Claims,
    Json(update_request): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    match user_service
        .update_user(claims.user_id, update_request)
        .await
    {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}

// Update Password Route
#[utoipa::path(
    post,
    path = "/user/password",
    tag = "User Management",
    request_body = UpdatePasswordRequest,
    responses(
        (status = 200, description = "Password updated successfully", body = UpdateUserResponse),
        (status = 400, description = "Invalid password data", body = ErrorResponse)
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn update_password_route(
    State(app_state): State<AppState>,
    claims: Claims,
    Json(password_request): Json<UpdatePasswordRequest>,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    match user_service
        .update_password(claims.user_id, password_request)
        .await
    {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}

// Delete User Route
#[utoipa::path(
    delete,
    path = "/user",
    tag = "User Management",
    responses(
        (status = 200, description = "User deleted successfully", body = UpdateUserResponse),
        (status = 500, description = "Failed to delete user", body = ErrorResponse)
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn delete_user_route(
    State(app_state): State<AppState>,
    claims: Claims,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(app_state.db_pool.clone());
    let user_service = UserService::new(user_repository);

    match user_service.delete_user(claims.user_id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: error.0.message,
            }),
        )
            .into_response(),
    }
}
