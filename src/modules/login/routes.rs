//! # `Login` Routes
//! This module defines the HTTP routes for user login functionality.

use axum::{routing::post, Router};

use crate::modules::login::service::login;
use crate::AppState;

pub fn login_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}
