use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

// SignUp structure for the signup module

// This module defines the signup interface for the signup process.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SignUpInterface {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct NewSignUpInterface {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SignUpResponse {
    pub id: String,
    pub username: String,
}
