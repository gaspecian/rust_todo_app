//! # `User` Repository
//! This module defines the user repository for user operations.

use sqlx::{Pool, Postgres, Error};

pub struct UserRepository {
    pool: Pool<Postgres>
}

impl UserRepository {
    // Constructor
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Method that checks if a username is already taken
    pub async fn exists_user_by_username(&self, username: &str) -> Result<Option<bool>, Error> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users where username = $1)",
            username
        ).fetch_one(&self.pool).await?;

        Ok(exists)
    }
}
