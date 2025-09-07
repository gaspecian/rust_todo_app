//! #`User` Routes
//! This module defines the HTTP routes for users funciionality.

use axum::routing::post;
use axum::{extract::State, http::StatusCode, Router, response::IntoResponse, Json, routing::get};
use serde::Serialize;
use utoipa::ToSchema;

use crate::modules::common::ErrorResponse;
use crate::modules::user::interfaces::UserSignUp;
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
        .route("/user", get(fetch_user_route))
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
        Err(error) => (StatusCode::BAD_REQUEST, Json(ErrorResponse{
            message: error.0.message
        })).into_response(), 
    }
}

// Fetch User Route
#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "User fetched successfully"),
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn fetch_user_route(
    State(app_state): State<AppState>
) -> impl IntoResponse {
    let response = Response {
        message: "User fetched successfully".to_string()
    };

    (StatusCode::OK, Json(response))
}

