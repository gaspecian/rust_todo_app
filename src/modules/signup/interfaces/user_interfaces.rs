use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

// User structure for the signup module

// This module defines the user interface for the signup process.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserInterface {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct NewUserInterface {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
}
