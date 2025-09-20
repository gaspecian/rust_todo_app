#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use once_cell::sync::Lazy;
use regex::Regex;

static LOWERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static UPPERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static DIGIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
static SPECIAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[@$!%*?&]").unwrap());
static VALID_CHARS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap());

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
        && LOWERCASE_REGEX.is_match(password)
        && UPPERCASE_REGEX.is_match(password)
        && DIGIT_REGEX.is_match(password)
        && SPECIAL_REGEX.is_match(password)
        && VALID_CHARS_REGEX.is_match(password)
}

pub fn hash_password(password: &str) -> Result<String, String> {
    // Salt generation
    let salt = SaltString::generate(&mut OsRng);

    // Argon2::default() provides a default configuration for Argon2
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hashed_password) => Ok(hashed_password.to_string()),
        Err(e) => {
            tracing::error!("Failed to hash password: {e}");
            Err(format!("Failed to hash password: {e}"))
        }
    }
}

// Function that validates if password input is valid
pub fn password_validation(stored_password_hash: &str, password_input: &str) -> bool {
    let hash = match PasswordHash::new(stored_password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Error getting password hash: {e}");
            return false;
        }
    };

    Argon2::default()
        .verify_password(password_input.as_bytes(), &hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_valid() {
        assert!(validate_password("Password123!"));
        assert!(validate_password("MySecure@Pass1"));
        assert!(validate_password("Test123$Password"));
    }

    #[test]
    fn test_validate_password_too_short() {
        assert!(!validate_password("Pass1!"));
        assert!(!validate_password(""));
    }

    #[test]
    fn test_validate_password_missing_lowercase() {
        assert!(!validate_password("PASSWORD123!"));
    }

    #[test]
    fn test_validate_password_missing_uppercase() {
        assert!(!validate_password("password123!"));
    }

    #[test]
    fn test_validate_password_missing_digit() {
        assert!(!validate_password("Password!"));
    }

    #[test]
    fn test_validate_password_missing_special() {
        assert!(!validate_password("Password123"));
    }

    #[test]
    fn test_validate_password_invalid_chars() {
        assert!(!validate_password("Password123!#"));
        assert!(!validate_password("Password123! "));
    }

    #[test]
    fn test_hash_password_success() {
        let password = "TestPassword123!";
        let result = hash_password(password);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_password_validation_success() {
        let password = "TestPassword123!";
        let hash = hash_password(password).unwrap();
        assert!(password_validation(&hash, password));
    }

    #[test]
    fn test_password_validation_wrong_password() {
        let password = "TestPassword123!";
        let wrong_password = "WrongPassword123!";
        let hash = hash_password(password).unwrap();
        assert!(!password_validation(&hash, wrong_password));
    }

    #[test]
    fn test_password_validation_invalid_hash() {
        assert!(!password_validation("invalid_hash", "TestPassword123!"));
        assert!(!password_validation("", "TestPassword123!"));
    }
}
