//! Database connection pool management.

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Create a Postgres connection pool.
/// Should be called once at service startup.
pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to Postgres: {e}"))
}

/// Run all pending migrations from the /migrations directory.
pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("../../migrations")
        .run(pool)
        .await
        .unwrap_or_else(|e| panic!("Migration failed: {e}"));
}
