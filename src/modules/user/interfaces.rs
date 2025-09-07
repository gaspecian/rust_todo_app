//! # `Users` Interfaces
//! This module defines the data structures from Users module

use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
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
    pub username: String,
    pub email: String,
    pub message: String,
}