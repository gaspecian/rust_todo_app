//! # `SignUp` Repository
//! This module defines the signup repository interface for the signup process.

use crate::modules::signup::interfaces::signup_interfaces::{NewSignUpInterface, SignUpResponse};
use sqlx::{Pool, Postgres};

pub struct SignUpRepository {
    pool: Pool<Postgres>,
}

impl SignUpRepository {
    // Contructor to create a new SignUpRepository instance
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Method that checks if a user exists by username
    pub async fn exists_by_username(&self, username: &str) -> Result<Option<bool>, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    // Method that checks if a user exists by email
    pub async fn exists_by_email(&self, email: &str) -> Result<Option<bool>, sqlx::Error> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
                .fetch_one(&self.pool)
                .await?;

        Ok(exists)
    }

    // Method that creates a new user
    pub async fn create_user(
        &self,
        user: &NewSignUpInterface,
    ) -> Result<SignUpResponse, sqlx::Error> {
        let user_id = sqlx::query_scalar!(
            "INSERT INTO users (username, email, password, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW()) RETURNING id",
            user.username,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SignUpResponse {
            id: user_id.to_string(),
            username: user.username.clone(),
        })
    }
}
