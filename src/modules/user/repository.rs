//! # `User` Repository
//! This module defines the user repository for user operations.

use sqlx::{Error, Pool, Postgres};

use crate::modules::user::interfaces::ValidatedUserSignUp;

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    // Constructor
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    // Method that checks if an username is already taken
    pub async fn exists_user_by_username(&self, username: &str) -> Result<Option<bool>, Error> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users where username = $1)",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    // Method that checks if an email is already taken
    pub async fn exists_user_by_email(&self, email: &str) -> Result<Option<bool>, Error> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users where email = $1)", email)
                .fetch_one(&self.pool)
                .await?;

        Ok(exists)
    }

    // Method that creates user in database
    pub async fn create_user(&self, user_signup: ValidatedUserSignUp) -> Result<i32, Error> {
        let created = sqlx::query_scalar!(
            "INSERT INTO users (username, email, password, name, surname, fone, active) VALUES ($1, $2, $3, $4, $5, $6, true) RETURNING id",
            user_signup.username,
            user_signup.email,
            user_signup.password,
            user_signup.name,
            user_signup.surname,
            user_signup.fone
        ).fetch_one(&self.pool).await?;

        Ok(created)
    }

    // Get User password
    pub async fn get_user_password(&self, username: String) -> Result<String, Error> {
        let password =
            sqlx::query_scalar!("SELECT password from users where username = $1", username)
                .fetch_one(&self.pool)
                .await?;

        Ok(password)
    }
}
