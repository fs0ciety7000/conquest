use std::{env, sync::Arc};
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod db;
mod error;
mod gamedata;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod websocket;
mod workers;

#[tokio::main]
async fn main() -> Result<()> {
    // Init tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load env
    dotenvy::dotenv().ok();
    let cfg = config::Config::from_env()?;

    // Parse mode from --mode=api or --mode=worker
    let mode = env::args()
        .skip(1)
        .find(|a| a.starts_with("--mode="))
        .and_then(|a| a.strip_prefix("--mode=").map(str::to_string))
        .unwrap_or_else(|| "api".to_string());

    tracing::info!(mode = %mode, version = env!("CARGO_PKG_VERSION"), "Starting Space Conquest");

    // Init DB pool
    let db_pool = db::pool::create_pg_pool(&cfg.database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Database migrations applied");

    // Init Redis
    let redis = db::pool::create_redis_client(&cfg.redis_url)?;

    // Load game data
    let game_data = Arc::new(gamedata::GameDataRegistry::load(&cfg.game_data_path)?);

    let ws_hub = websocket::WsHub::new();

    match mode.as_str() {
        "api" => {
            let state = routes::AppState {
                db:        db_pool,
                config:    Arc::new(cfg),
                game_data,
                ws_hub,
            };

            let app = routes::build_router(state);
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
            tracing::info!(address = "0.0.0.0:8080", "API server started");
            axum::serve(listener, app).await?;
        }
        "worker" => {
            // Discover all active universes and spawn one worker per universe
            let universes: Vec<uuid::Uuid> = sqlx::query_scalar!(
                "SELECT id FROM universes WHERE is_active = true"
            )
            .fetch_all(&db_pool)
            .await?;

            tracing::info!(count = universes.len(), "Starting workers for universes");

            let mut handles = Vec::new();
            for universe_id in universes {
                let pool = db_pool.clone();
                let redis = redis.clone();
                let registry = (*game_data).clone();
                let hub = ws_hub.clone();

                let handle = tokio::spawn(async move {
                    workers::event_processor::run(pool, redis, registry, hub, universe_id).await;
                });
                handles.push(handle);
            }

            futures::future::join_all(handles).await;
        }
        _ => {
            anyhow::bail!("Unknown mode '{}'. Use --mode=api or --mode=worker", mode);
        }
    }

    Ok(())
}
