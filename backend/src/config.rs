use std::path::PathBuf;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url:        String,
    pub redis_url:           String,
    pub jwt_secret:          String,
    pub jwt_expiry_minutes:  i64,
    pub game_data_path:      PathBuf,
    pub cors_origins:        Vec<String>,
    pub default_universe_id: String,
    pub env:                 String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL not set"))?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET not set"))?,
            jwt_expiry_minutes: std::env::var("JWT_EXPIRY_MINUTES")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            game_data_path: PathBuf::from(
                std::env::var("GAME_DATA_PATH").unwrap_or_else(|_| "./game_data".to_string())
            ),
            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
                .split(',')
                .map(str::trim)
                .map(String::from)
                .collect(),
            default_universe_id: std::env::var("DEFAULT_UNIVERSE_ID")
                .unwrap_or_else(|_| "00000000-0000-0000-0000-000000000001".to_string()),
            env: std::env::var("ENV").unwrap_or_else(|_| "development".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.env == "production"
    }
}
