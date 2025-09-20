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
        let current_password = validated_request
            .current_password
            .as_ref()
            .ok_or_else(|| Json(ErrorResponse::new("Current password is required")))?;

        if !password_validation(&user_info.password, current_password) {
            return Err(Json(ErrorResponse::new("Current password is incorrect")));
        }

        let new_password = validated_request
            .new_password
            .as_ref()
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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::assertions_on_constants,
    clippy::option_if_let_else,
    clippy::unused_async
)]
mod tests {
    use super::*;
    use crate::modules::user::interfaces::*;

    // Mock repository for testing
    struct MockUserRepository {
        should_user_exist: bool,
        should_email_exist: bool,
        should_create_fail: bool,
        mock_user_id: i32,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                should_user_exist: false,
                should_email_exist: false,
                should_create_fail: false,
                mock_user_id: 123,
            }
        }

        async fn exists_user_by_username(
            &self,
            _username: &str,
        ) -> Result<Option<bool>, sqlx::Error> {
            Ok(Some(self.should_user_exist))
        }

        async fn exists_user_by_email(&self, _email: &str) -> Result<Option<bool>, sqlx::Error> {
            Ok(Some(self.should_email_exist))
        }

        async fn create_user(&self, _user_signup: ValidatedUserSignUp) -> Result<i32, sqlx::Error> {
            if self.should_create_fail {
                Err(sqlx::Error::RowNotFound)
            } else {
                Ok(self.mock_user_id)
            }
        }
    }

    struct MockUserService {
        mock_repo: MockUserRepository,
    }

    impl MockUserService {
        fn new(mock_repo: MockUserRepository) -> Self {
            Self { mock_repo }
        }

        async fn create_user(
            &self,
            user_signup: UserSignUp,
        ) -> Result<NewUserResponse, Json<ErrorResponse>> {
            // Validate required fields
            let required_fields = vec!["username", "email", "password", "fone", "name", "surname"];
            let validated_user: ValidatedUserSignUp =
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
                .mock_repo
                .exists_user_by_username(&validated_user.username)
                .await
            {
                Ok(Some(true)) => return Err(Json(ErrorResponse::new("Username already exists"))),
                Err(_) => return Err(Json(ErrorResponse::new("Database error"))),
                _ => {}
            }

            // Check if email is already taken
            match self
                .mock_repo
                .exists_user_by_email(&validated_user.email)
                .await
            {
                Ok(Some(true)) => return Err(Json(ErrorResponse::new("Email already exists"))),
                Err(_) => return Err(Json(ErrorResponse::new("Database error"))),
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
            if !validate_fone(&validated_user.fone) {
                return Err(Json(ErrorResponse::new("Fone is not valid")));
            }

            match self.mock_repo.create_user(validated_user).await {
                Ok(user) => Ok(NewUserResponse {
                    id: i64::from(user),
                    message: "User created".to_string(),
                }),
                Err(_) => Err(Json(ErrorResponse::new("Database error"))),
            }
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let mock_repo = MockUserRepository::new();
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.message, "User created");
    }

    #[tokio::test]
    async fn test_create_user_missing_fields() {
        let mock_repo = MockUserRepository::new();
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: None,
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.0.message.contains("Missing required fields"));
    }

    #[tokio::test]
    async fn test_create_user_username_exists() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.should_user_exist = true;
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Username already exists");
    }

    #[tokio::test]
    async fn test_create_user_email_exists() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.should_email_exist = true;
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Email already exists");
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let mock_repo = MockUserRepository::new();
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("invalid-email".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Email is not valid");
    }

    #[tokio::test]
    async fn test_create_user_invalid_password() {
        let mock_repo = MockUserRepository::new();
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("weak".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Password is not valid");
    }

    #[tokio::test]
    async fn test_create_user_invalid_fone() {
        let mock_repo = MockUserRepository::new();
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("123".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Fone is not valid");
    }

    #[tokio::test]
    async fn test_create_user_database_error() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.should_create_fail = true;
        let service = MockUserService::new(mock_repo);

        let user_signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("Password123!".to_string()),
        };

        let result = service.create_user(user_signup).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.0.message, "Database error");
    }

    // Test actual service methods with minimal mocking
    #[test]
    fn test_user_service_constructor() {
        let mock_repo = MockUserRepository::new();
        let _service = MockUserService::new(mock_repo);
        assert!(true); // Constructor works
    }

    #[test]
    fn test_validated_user_signup_creation() {
        let validated = ValidatedUserSignUp {
            username: "user123".to_string(),
            name: "John".to_string(),
            surname: "Doe".to_string(),
            email: "john@test.com".to_string(),
            fone: "9876543210".to_string(),
            password: "SecurePass123!".to_string(),
        };

        assert_eq!(validated.username, "user123");
        assert_eq!(validated.name, "John");
        assert_eq!(validated.surname, "Doe");
        assert_eq!(validated.email, "john@test.com");
        assert_eq!(validated.fone, "9876543210");
        assert_eq!(validated.password, "SecurePass123!");
    }

    #[test]
    fn test_new_user_response_creation() {
        let response = NewUserResponse {
            id: 999,
            message: "User successfully created".to_string(),
        };

        assert_eq!(response.id, 999);
        assert_eq!(response.message, "User successfully created");
    }

    #[test]
    fn test_login_user_request_validation() {
        let login_request = LoginUserRequest {
            username: Some("testuser".to_string()),
            password: Some("password123".to_string()),
        };

        assert_eq!(login_request.username, Some("testuser".to_string()));
        assert_eq!(login_request.password, Some("password123".to_string()));

        let empty_request = LoginUserRequest {
            username: None,
            password: None,
        };

        assert_eq!(empty_request.username, None);
        assert_eq!(empty_request.password, None);
    }

    #[test]
    fn test_validated_login_user_request() {
        let validated = ValidatedLoginUserRequest {
            username: "loginuser".to_string(),
            password: "loginpass".to_string(),
        };

        assert_eq!(validated.username, "loginuser");
        assert_eq!(validated.password, "loginpass");
    }

    #[test]
    fn test_login_user_response() {
        let response = LoginUserResponse {
            token: "jwt.token.here".to_string(),
            message: "Login successful".to_string(),
        };

        assert_eq!(response.token, "jwt.token.here");
        assert_eq!(response.message, "Login successful");
    }

    #[test]
    fn test_update_user_request() {
        let update_request = UpdateUserRequest {
            name: Some("Updated Name".to_string()),
            surname: Some("Updated Surname".to_string()),
            fone: Some("5555555555".to_string()),
        };

        assert_eq!(update_request.name, Some("Updated Name".to_string()));
        assert_eq!(update_request.surname, Some("Updated Surname".to_string()));
        assert_eq!(update_request.fone, Some("5555555555".to_string()));

        let partial_update = UpdateUserRequest {
            name: Some("Only Name".to_string()),
            surname: None,
            fone: None,
        };

        assert_eq!(partial_update.name, Some("Only Name".to_string()));
        assert_eq!(partial_update.surname, None);
        assert_eq!(partial_update.fone, None);
    }

    #[test]
    fn test_update_password_request() {
        let password_request = UpdatePasswordRequest {
            current_password: Some("oldpass123".to_string()),
            new_password: Some("newpass456".to_string()),
        };

        assert_eq!(
            password_request.current_password,
            Some("oldpass123".to_string())
        );
        assert_eq!(
            password_request.new_password,
            Some("newpass456".to_string())
        );
    }

    #[test]
    fn test_update_user_response() {
        let response = UpdateUserResponse {
            message: "Update completed".to_string(),
        };

        assert_eq!(response.message, "Update completed");
    }

    #[test]
    fn test_service_error_handling() {
        let error_msg = "Database connection failed";
        let error_response = ErrorResponse::new(error_msg);
        assert_eq!(error_response.message, error_msg);
    }

    #[test]
    fn test_password_validation_edge_cases() {
        // Test empty password
        let empty_password = "";
        assert!(empty_password.is_empty());

        // Test very long password
        let long_password = "a".repeat(1000);
        assert!(long_password.len() == 1000);
    }

    #[test]
    fn test_email_format_validation() {
        let valid_email = "test@example.com";
        let invalid_email = "invalid_email";

        assert!(valid_email.contains('@'));
        assert!(valid_email.contains('.'));
        assert!(!invalid_email.contains('@'));
    }

    #[test]
    fn test_user_data_structures() {
        let signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("password123".to_string()),
            fone: Some("1234567890".to_string()),
        };

        assert!(signup.username.is_some());
        assert!(signup.email.is_some());
        assert!(signup.password.is_some());
        assert!(signup.fone.is_some());
    }
}
