use axum::{extract::State, Json};
use serde::Serialize;

use crate::routes::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status:  &'static str,
    pub version: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status:  "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn health_db(State(state): State<AppState>) -> Json<HealthResponse> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .expect("DB health check failed");

    Json(HealthResponse {
        status:  "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}
