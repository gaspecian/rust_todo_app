//! # `User` Repository
//! This module defines the user repository for user operations.

use sqlx::{Error, Pool, Postgres};

use crate::modules::user::interfaces::{FetchUserResponse, GetUserForLoginDb, ValidatedUserSignUp};

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
    pub async fn get_user_for_login(&self, username: &str) -> Result<GetUserForLoginDb, Error> {
        let result = sqlx::query!(
            "SELECT password, id from users where username = $1",
            username.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(GetUserForLoginDb {
            password: result.password,
            id: i64::from(result.id),
        })
    }

    // Fetch User Data
    pub async fn fetch_user(&self, id: i64) -> Result<FetchUserResponse, Error> {
        let result = sqlx::query!(
            "SELECT id, username, name, surname, email, fone, created_at, updated_at, active, activated_at FROM users WHERE id = $1",
            i32::try_from(id).map_err(|_| Error::Protocol("Invalid user ID".into()))?
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(FetchUserResponse {
            username: result.username,
            name: result.name,
            surname: result.surname,
            email: result.email,
            fone: result.fone,
            created_at: result.created_at.map(|dt| dt.to_string()),
            updated_at: result.updated_at.map(|dt| dt.to_string()),
            active: result.active,
            activated_at: result.activated_at.map(|dt| dt.to_string()),
        })
    }

    // Update User Data
    pub async fn update_user(
        &self,
        id: i64,
        name: Option<String>,
        surname: Option<String>,
        fone: Option<String>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE users SET name = $1, surname = $2, fone = $3, updated_at = NOW() WHERE id = $4",
            name,
            surname,
            fone,
            i32::try_from(id).map_err(|_| Error::Protocol("Invalid user ID".into()))?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Update User Password
    pub async fn update_password(&self, id: i64, new_password: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE users SET password = $1, updated_at = NOW() WHERE id = $2",
            new_password,
            i32::try_from(id).map_err(|_| Error::Protocol("Invalid user ID".into()))?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Delete User
    pub async fn delete_user(&self, id: i64) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            i32::try_from(id).map_err(|_| Error::Protocol("Invalid user ID".into()))?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
