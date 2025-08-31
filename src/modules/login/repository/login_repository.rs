//! # `Login` Repository
//! This module defines the login repository interface for the login process.

use sqlx::{Error, Pool, Postgres};

use crate::modules::signup::interfaces::signup_interfaces::SignUpInterface;

pub struct LoginRepository {
    pool: Pool<Postgres>,
}

impl LoginRepository {
    // Constructor to create a new LoginRepository instance
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Method that fetches a user by username
    pub async fn fetch_by_username(&self, username: &str) -> Result<SignUpInterface, Error> {
        let user = sqlx::query_as!(
            SignUpInterface,
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
