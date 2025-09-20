//! # `User` Service
//!
//! This module contains the bussiness logic for user operations.

use axum::Json;
use email_address::EmailAddress;
use jsonwebtoken::EncodingKey;

use crate::{
    auth::generate_token,
    modules::{
        common::ErrorResponse,
        user::{
            interfaces::{
                FetchUserResponse, LoginUserRequest, LoginUserResponse, NewUserResponse,
                UpdatePasswordRequest, UpdateUserRequest, UpdateUserResponse, UserSignUp,
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
        enconding_key: EncodingKey,
        session_duration: i64,
    ) -> Result<LoginUserResponse, Json<ErrorResponse>> {
        // Validate required fields
        let required_fields = vec!["username", "password"];
        let validated_user: ValidatedLoginUserRequest =
            match validate_required_fields(&user_login, required_fields) {
                Err(missing) => {
                    tracing::warn!("Missing required fields: {0}", &missing);
                    return Err(Json(ErrorResponse::new(format!(
                        "Missing required fields: {missing}"
                    ))));
                }
                Ok(user) => user,
            };

        let user = validated_user.username;

        // Find User login and password in repository
        let Ok(user_info) = self.user_repository.get_user_for_login(&user).await else {
            tracing::warn!("User {0} not found", &user);
            return Err(Json(ErrorResponse::new(
                "Username and Password invalid".to_string(),
            )));
        };

        // Validate password
        let is_password_correct =
            password_validation(&user_info.password, &validated_user.password);
        if !is_password_correct {
            tracing::warn!("Password validation failed for username: {0}", &user);
            return Err(Json(ErrorResponse::new(
                "Username and Password invalid".to_string(),
            )));
        }

        // Generate JWT token
        let token = match generate_token(session_duration, user_info.id, &enconding_key) {
            Ok(token) => token,
            Err(e) => {
                tracing::warn!("Error generating JWT token: {0}", e.message);
                return Err(Json(ErrorResponse::new(
                    "Username and Password invalid".to_string(),
                )));
            }
        };

        Ok(LoginUserResponse {
            token,
            message: "User logged in".to_string(),
        })
    }

    // Fetch User Data
    pub async fn fetch_user(&self, id: i64) -> Result<FetchUserResponse, Json<ErrorResponse>> {
        let user = match self.user_repository.fetch_user(id).await {
            Ok(user) => user,
            Err(e) => {
                tracing::warn!("Error fetching user data: {0}", e);
                return Err(Json(ErrorResponse::new("User not found".to_string())));
            }
        };

        Ok(user)
    }

    // Update User Data
    pub async fn update_user(
        &self,
        id: i64,
        update_request: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, Json<ErrorResponse>> {
        if let Some(ref fone) = update_request.fone {
            if !validate_fone(fone) {
                return Err(Json(ErrorResponse::new("Fone is not valid")));
            }
        }

        match self
            .user_repository
            .update_user(
                id,
                update_request.name,
                update_request.surname,
                update_request.fone,
            )
            .await
        {
            Ok(()) => Ok(UpdateUserResponse {
                message: "User updated successfully".to_string(),
            }),
            Err(e) => {
                tracing::warn!("Error updating user: {}", e);
                Err(Json(ErrorResponse::new("Failed to update user")))
            }
        }
    }

    // Update User Password
    pub async fn update_password(
        &self,
        id: i64,
        password_request: UpdatePasswordRequest,
    ) -> Result<UpdateUserResponse, Json<ErrorResponse>> {
        let required_fields = vec!["current_password", "new_password"];
        let validated_request: UpdatePasswordRequest =
            match validate_required_fields(&password_request, required_fields) {
                Err(missing) => {
                    return Err(Json(ErrorResponse::new(format!(
                        "Missing required fields: {missing}"
                    ))))
                }
                Ok(req) => req,
            };

        // Get current user password - need to get by user ID, not username
        let user_info = match self.user_repository.fetch_user(id).await {
            Ok(_) => {
                // Get user login info by fetching username first, then getting login data
                match self.user_repository.fetch_user(id).await {
                    Ok(user) => {
                        match self
                            .user_repository
                            .get_user_for_login(&user.username)
                            .await
                        {
                            Ok(info) => info,
                            Err(_) => return Err(Json(ErrorResponse::new("User not found"))),
                        }
                    }
                    Err(_) => return Err(Json(ErrorResponse::new("User not found"))),
                }
            }
            Err(_) => return Err(Json(ErrorResponse::new("User not found"))),
        };

        // Validate current password
        let current_password = validated_request.current_password.as_ref()
            .ok_or_else(|| Json(ErrorResponse::new("Current password is required")))?;
        
        if !password_validation(&user_info.password, current_password) {
            return Err(Json(ErrorResponse::new("Current password is incorrect")));
        }

        let new_password = validated_request.new_password.as_ref()
            .ok_or_else(|| Json(ErrorResponse::new("New password is required")))?;
        if !validate_password(new_password) {
            return Err(Json(ErrorResponse::new("New password is not valid")));
        }

        let hashed_password = match hash_password(new_password) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(Json(ErrorResponse::new(format!(
                    "Password hashing error: {e}"
                ))))
            }
        };

        match self
            .user_repository
            .update_password(id, &hashed_password)
            .await
        {
            Ok(()) => Ok(UpdateUserResponse {
                message: "Password updated successfully".to_string(),
            }),
            Err(e) => {
                tracing::warn!("Error updating password: {}", e);
                Err(Json(ErrorResponse::new("Failed to update password")))
            }
        }
    }

    // Delete User
    pub async fn delete_user(&self, id: i64) -> Result<UpdateUserResponse, Json<ErrorResponse>> {
        match self.user_repository.delete_user(id).await {
            Ok(()) => Ok(UpdateUserResponse {
                message: "User deleted successfully".to_string(),
            }),
            Err(e) => {
                tracing::warn!("Error deleting user: {}", e);
                Err(Json(ErrorResponse::new("Failed to delete user")))
            }
        }
    }
}
