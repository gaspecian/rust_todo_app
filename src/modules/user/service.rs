//! # `User` Service
//! 
//! This module contains the bussiness logic for user operations.

use std::any::Any;

use axum::Json;

use crate::modules::{common::ErrorResponse, user::{interfaces::{NewUserResponse, UserSignUp}, repository::UserRepository}};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub const fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, user_signup: UserSignUp) -> Result<NewUserResponse, Json<ErrorResponse>> {
        
        // Check if username is already taken
        match self
            .user_repository
            .exists_user_by_username(&user_signup.username)
            .await 
        {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Username already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        let email = user_signup.email;
        let response = NewUserResponse {
            id: 1,
            username: user_signup.username,
            email: email.clone(),
            message: format!("User created, activation email sent to {0}", email)
        };

        Ok(response)
    }
}