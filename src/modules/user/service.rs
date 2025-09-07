//! # `User` Service
//! 
//! This module contains the bussiness logic for user operations.

use axum::Json;
use email_address::EmailAddress;

use crate::{modules::{common::ErrorResponse, user::{interfaces::{NewUserResponse, UserSignUp}, repository::UserRepository}}, utils::password::validate_password};

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

        // Check if email is already taken
        match self
            .user_repository
            .exists_user_by_email(&user_signup.email)
            .await
        {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Email already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        let email_validation = EmailAddress::is_valid(&user_signup.email);
        if !email_validation {
            return Err(Json(ErrorResponse::new("Email is not valid")));
        }

        if !validate_password(&user_signup.password) {
            return Err(Json(ErrorResponse::new("Password is not valid")));
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