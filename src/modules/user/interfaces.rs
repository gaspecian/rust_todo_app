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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_user_signup_serialization() {
        let signup = UserSignUp {
            username: Some("testuser".to_string()),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            fone: Some("1234567890".to_string()),
            password: Some("password123".to_string()),
        };

        let json = serde_json::to_string(&signup).unwrap();
        assert!(json.contains("testuser"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"username":"testuser","password":"password123"}"#;
        let login: LoginUserRequest = serde_json::from_str(json).unwrap();
        assert_eq!(login.username, Some("testuser".to_string()));
        assert_eq!(login.password, Some("password123".to_string()));
    }

    #[test]
    fn test_new_user_response() {
        let response = NewUserResponse {
            id: 123,
            message: "User created successfully".to_string(),
        };
        assert_eq!(response.id, 123);
        assert_eq!(response.message, "User created successfully");
    }

    #[test]
    fn test_fetch_user_response() {
        let response = FetchUserResponse {
            username: "testuser".to_string(),
            name: Some("Test".to_string()),
            surname: Some("User".to_string()),
            email: "test@example.com".to_string(),
            fone: Some("1234567890".to_string()),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: None,
            active: true,
            activated_at: Some("2023-01-01T00:00:00Z".to_string()),
        };

        assert_eq!(response.username, "testuser");
        assert_eq!(response.email, "test@example.com");
        assert!(response.active);
    }

    #[test]
    fn test_update_user_request() {
        let update = UpdateUserRequest {
            name: Some("Updated Name".to_string()),
            surname: None,
            fone: Some("9876543210".to_string()),
        };

        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert_eq!(update.surname, None);
        assert_eq!(update.fone, Some("9876543210".to_string()));
    }

    #[test]
    fn test_update_password_request() {
        let update = UpdatePasswordRequest {
            current_password: Some("oldpass".to_string()),
            new_password: Some("newpass".to_string()),
        };

        assert_eq!(update.current_password, Some("oldpass".to_string()));
        assert_eq!(update.new_password, Some("newpass".to_string()));
    }

    #[test]
    fn test_validated_user_signup() {
        let validated = ValidatedUserSignUp {
            username: "testuser".to_string(),
            name: "Test".to_string(),
            surname: "User".to_string(),
            email: "test@example.com".to_string(),
            fone: "1234567890".to_string(),
            password: "hashedpassword".to_string(),
        };

        assert_eq!(validated.username, "testuser");
        assert_eq!(validated.email, "test@example.com");
    }
}
