use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub token: String, // JWT or session token
}
