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
