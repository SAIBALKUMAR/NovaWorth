//! User database queries.
//! Uses SQLX_OFFLINE=true compatible form with query_as! / query_scalar!
//! Requires .sqlx/ cache or a live DB; for compile-only use offline mode.

use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use common::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRow {
    pub id:            Uuid,
    pub email:         String,
    pub username:      String,
    pub password_hash: Option<String>,
    pub role:          String,
    pub created_at:    chrono::DateTime<chrono::Utc>,
    pub updated_at:    chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserParams {
    pub email:         String,
    pub username:      String,
    pub password_hash: Option<String>,
}

pub async fn create_user(pool: &PgPool, params: CreateUserParams) -> Result<UserRow, AppError> {
    sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, password_hash, role, created_at, updated_at
        "#,
    )
    .bind(&params.email)
    .bind(&params.username)
    .bind(&params.password_hash)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username, password_hash, role, created_at, updated_at
        FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, username, password_hash, role, created_at, updated_at
        FROM users WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
    let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(row.0)
}

pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool, AppError> {
    let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(row.0)
}
