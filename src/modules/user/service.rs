//! # `User` Service
//!
//! This module contains the bussiness logic for user operations.

use axum::Json;
use email_address::EmailAddress;

use crate::{
    modules::{
        common::ErrorResponse,
        user::{
            interfaces::{
                LoginUserRequest, LoginUserResponse, NewUserResponse, UserSignUp,
                ValidatedLoginUserRequest, ValidatedUserSignUp,
            },
            repository::UserRepository,
        },
    },
    utils::{
        fone_validation::validate_fone,
        password::{hash_password, password_validation, validate_password},
        required_fields::validate_required_fields,
    },
};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub const fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    // Function that creates an user in the application
    pub async fn create_user(
        &self,
        user_signup: UserSignUp,
    ) -> Result<NewUserResponse, Json<ErrorResponse>> {
        // Validate required fields
        let required_fields = vec!["username", "email", "password", "fone", "name", "surname"];
        let mut validated_user: ValidatedUserSignUp =
            match validate_required_fields(&user_signup, required_fields) {
                Err(missing) => {
                    return Err(Json(ErrorResponse::new(format!(
                        "Missing required fields: {missing}"
                    ))))
                }
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

        // Check if email is valid
        let email_validation = EmailAddress::is_valid(&validated_user.email);
        if !email_validation {
            return Err(Json(ErrorResponse::new("Email is not valid")));
        }

        // Check if password is valid
        if !validate_password(&validated_user.password) {
            return Err(Json(ErrorResponse::new("Password is not valid")));
        }

        // Check if Fone is Valid
        if !validate_fone(&validated_user.fone.to_string()) {
            return Err(Json(ErrorResponse::new("Fone is not valid")));
        }

        let hashed_password = match hash_password(&validated_user.password) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(Json(ErrorResponse::new(format!(
                    "Password hashing error: {e}"
                ))))
            }
        };
        validated_user.password = hashed_password;

        match self.user_repository.create_user(validated_user).await {
            Ok(user) => Ok(NewUserResponse {
                id: i64::from(user),
                message: "User created".to_string(),
            }),
            Err(e) => Err(Json(ErrorResponse::new(format!("Database error: {e}")))),
        }
    }

    // Function that handles user login
    pub async fn login_user(
        &self,
        user_login: LoginUserRequest,
    ) -> Result<LoginUserResponse, Json<ErrorResponse>> {
        tracing::debug!("Login attempt started");

        // Validate required fields
        let required_fields = vec!["username", "password"];
        let validated_user: ValidatedLoginUserRequest =
            match validate_required_fields(&user_login, required_fields) {
                Err(missing) => {
                    tracing::debug!("Missing required fields: {0}", &missing);
                    return Err(Json(ErrorResponse::new(format!(
                        "Missing required fields: {missing}"
                    ))));
                }
                Ok(user) => user,
            };

        let user = validated_user.username;

        // Find User login and password in repository
        let stored_password = match self.user_repository.get_user_password(&user).await {
            Ok(password) => password,
            Err(e) => {
                tracing::debug!("User {0} not found", &user);

                return Err(Json(ErrorResponse::new(format!(
                    "Username and Password invalid"
                ))));
            }
        };

        let is_password_correct = password_validation(&stored_password, &validated_user.password);
        if !is_password_correct {
            tracing::debug!("Password validation failed for username: {0}", &user);
            return Err(Json(ErrorResponse::new(format!(
                "Username and Password invalid"
            ))));
        }

        tracing::debug!("Login attempt successfully");

        return Ok(LoginUserResponse {
            token: "123443".to_string(),
            message: "User logged in".to_string(),
        });
    }
}
