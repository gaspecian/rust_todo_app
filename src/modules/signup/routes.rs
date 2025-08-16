//! #SignUp Routes
//! This module defines the HTTP routes for user signup functionality.

use axum::{routing::post, Router};

use crate::modules::signup::service::signup;
use crate::AppState;

/// Creates and returns the signup routes
/// This function sets up all signup-related HTTP routes and returns
/// a configured Router that can be merged
pub fn signup_routes() -> Router<AppState> {
    Router::new().route("/signup", post(signup))
}
