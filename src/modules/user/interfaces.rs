//! # `Users` Interfaces
//! This module defines the data structures from Users module

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ValidatedUserSignUp {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub fone: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
    // User Id in Database
    pub id: i64,
    // Username for application login
    pub username: String,
    // User name
    pub name: String,
    // User surname
    pub surname: String,
    // User email
    pub email: String,
    // User Fone
    pub fone: String,
    // User password hashed
    pub password: String,
    // User creation date
    pub created_at: Option<OffsetDateTime>,
    // User update date
    pub updated_at: Option<OffsetDateTime>,
    // Check if user is active
    pub active: bool,
    // User activation date
    pub activated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UserSignUp {
    // Username for application login
    pub username: Option<String>,
    // User name
    pub name: Option<String>,
    // User surname
    pub surname: Option<String>,
    // User email
    pub email: Option<String>,
    // User Fone
    pub fone: Option<String>,
    // User password
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct NewUserResponse {
    pub id: i64,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginUserRequest {
    // Username for application login
    pub username: Option<String>,
    // User password
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ValidatedLoginUserRequest {
    // Username for application login
    pub username: String,
    // User password
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginUserResponse {
    // Token for authentications
    pub token: String,
    // Message for authentication
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetUserForLoginDb {
    // Token for authentications
    pub password: String,
    // Message for authentication
    pub id: i64,
}

// Fetch User Data
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct FetchUserResponse {
    pub username: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: String,
    pub fone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    // Check if user is active
    pub active: bool,
    // User activation date
    pub activated_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub fone: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdatePasswordRequest {
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateUserResponse {
    pub message: String,
}
