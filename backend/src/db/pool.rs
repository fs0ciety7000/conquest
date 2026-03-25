use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pg_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await?;

    tracing::info!("PostgreSQL connection pool established");
    Ok(pool)
}

pub fn create_redis_client(redis_url: &str) -> Result<redis::Client> {
    let client = redis::Client::open(redis_url)?;
    tracing::info!("Redis client created");
    Ok(client)
}
