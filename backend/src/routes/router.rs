use std::sync::Arc;

use axum::{
    extract::{Extension, State, WebSocketUpgrade},
    middleware,
    response::Response,
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::config::Config;
use crate::gamedata::GameDataRegistry;
use crate::handlers::{auth, building, empire, gamedata, health, planet};
use crate::middleware::{auth_layer, AuthUser};
use crate::websocket::WsHub;

#[derive(Clone)]
pub struct AppState {
    pub db:        PgPool,
    pub config:    Arc<Config>,
    pub game_data: Arc<GameDataRegistry>,
    pub ws_hub:    WsHub,
}

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Public
        .route("/api/health",          get(health::health))
        .route("/api/health/db",       get(health::health_db))
        .route("/api/auth/register",   post(auth::register))
        .route("/api/auth/login",      post(auth::login))
        .route("/api/auth/refresh",    post(auth::refresh))
        .route("/api/game-data",       get(gamedata::get_game_data))
        // Protected (auth middleware applied per-route via from_fn)
        .route("/api/universes/:universe_id/empire",  get(empire::get_empire))
        .route("/api/universes/:universe_id/empire",  post(empire::create_empire))
        .route("/api/planets/:planet_id/resources",   get(planet::get_resources))
        .route("/api/planets/:planet_id/buildings",   get(building::list_buildings))
        .route("/api/planets/:planet_id/buildings",   post(building::start_build))
        .route("/api/planets/:planet_id/buildings",   delete(building::cancel_build))
        .route("/api/ws",                             get(ws_handler))
        .layer(middleware::from_fn(auth_layer))
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .with_state(state)
}

async fn ws_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    ws: WebSocketUpgrade,
) -> Response {
    let empire_id = sqlx::query_scalar!(
        "SELECT id FROM empires WHERE user_id = $1 LIMIT 1",
        auth.user_id,
    )
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten()
    .unwrap_or_else(Uuid::new_v4);

    ws.on_upgrade(move |socket| async move {
        state.ws_hub.register(empire_id, socket).await;
    })
}
