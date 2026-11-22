//! Guild creation and membership.

use sqlx::PgPool;
use uuid::Uuid;

use common::constants::{GUILD_FOUNDER_MIN_LEVEL, GUILD_MAX_MEMBERS};
use common::errors::AppError;

/// Guild lifecycle helper over the `guilds` / `guild_members` tables.
pub struct GuildService {
    pool: PgPool,
}

impl GuildService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a guild and seat the founder as its first member.
    pub async fn create(
        &self,
        founder_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<Uuid, AppError> {
        let level = self.founder_level(founder_id).await?;
        if level < GUILD_FOUNDER_MIN_LEVEL as i32 {
            return Err(AppError::Validation(format!(
                "Reach level {GUILD_FOUNDER_MIN_LEVEL} before founding a guild (you're at level {level})"
            )));
        }

        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO guilds (name, description, founder_id)
             VALUES ($1, $2, $3)
             RETURNING id",
        )
        .bind(name)
        .bind(description)
        .bind(founder_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        sqlx::query(
            "INSERT INTO guild_members (guild_id, user_id, role)
             VALUES ($1, $2, 'founder')",
        )
        .bind(row.0)
        .bind(founder_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.0)
    }

    /// Join an open guild (idempotent).
    pub async fn join(&self, guild_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !self.has_capacity(guild_id).await? {
            return Err(AppError::Validation(format!(
                "This guild is full (limit {GUILD_MAX_MEMBERS} members)"
            )));
        }

        sqlx::query(
            "INSERT INTO guild_members (guild_id, user_id, role)
             VALUES ($1, $2, 'member')
             ON CONFLICT (guild_id, user_id) DO NOTHING",
        )
        .bind(guild_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    /// Current member count (used to enforce the guild cap).
    pub async fn member_count(&self, guild_id: Uuid) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM guild_members WHERE guild_id = $1",
        )
        .bind(guild_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0)
    }

    /// Whether a guild has room for another member.
    pub async fn has_capacity(&self, guild_id: Uuid) -> Result<bool, AppError> {
        Ok(self.member_count(guild_id).await? < GUILD_MAX_MEMBERS as i64)
    }

    /// The founder's highest attained level (used to gate guild creation).
    async fn founder_level(&self, user_id: Uuid) -> Result<i32, AppError> {
        let row: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(current_level), 0) FROM learner_skills WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0)
    }
}