use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginResponse {
    pub username: String,
    pub token: String, // JWT or session token
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Claims {
    // Standard claims
    pub exp: usize, // Expiration time (as a Unix timestamp)
    pub iat: usize, // Issued at time (as a Unix timestamp)

    // Custom claims
    pub user_id: i64,
}