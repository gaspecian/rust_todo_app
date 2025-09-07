//! # `User` Service
//! 
//! This module contains the bussiness logic for user operations.

use axum::Json;
use email_address::EmailAddress;

use crate::{modules::{common::ErrorResponse, user::{interfaces::{NewUserResponse, UserSignUp, ValidatedUserSignUp}, repository::UserRepository}}, utils::{password::validate_password, required_fields::validate_required_fields}};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub const fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, user_signup: UserSignUp) -> Result<NewUserResponse, Json<ErrorResponse>> {

        // Validate required fields
        let required_fields = vec!["username", "email", "password", "fone", "name", "surname"];
        let validated_user: ValidatedUserSignUp = match validate_required_fields(&user_signup, required_fields) {
            Err(missing) => return Err(Json(ErrorResponse::new(format!("Missing required fields: {missing}")))),
            Ok(user) => user,
        };
        
        // Check if username is already taken
        match self
            .user_repository
            .exists_user_by_username(&validated_user.username)
            .await 
        {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Username already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        // Check if email is already taken
        match self
            .user_repository
            .exists_user_by_email(&validated_user.email)
            .await
        {
            Ok(Some(true)) => return Err(Json(ErrorResponse::new("Email already exists"))),
            Err(e) => return Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
            _ => {}
        }

        let email_validation = EmailAddress::is_valid(&validated_user.email);
        if !email_validation {
            return Err(Json(ErrorResponse::new("Email is not valid")));
        }

        if !validate_password(&validated_user.password) {
            return Err(Json(ErrorResponse::new("Password is not valid")));
        }

        let response = NewUserResponse {
            id: 1,
            username: validated_user.username,
            email: validated_user.email.clone(),
            message: format!("User created, activation email sent to {0}", validated_user.email)
        };

        Ok(response)
    }
}