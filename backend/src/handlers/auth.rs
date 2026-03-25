use axum::{extract::State, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::models::user::{LoginDto, RegisterDto};
use crate::routes::AppState;
use crate::services::auth_service::{self, AuthResponse};

pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = auth_service::register(&state.db, dto, &state.config.jwt_secret).await?;
    Ok(Json(res))
}

pub async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = auth_service::login(&state.db, dto, &state.config.jwt_secret).await?;
    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct RefreshDto {
    pub refresh_token: String,
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(dto): Json<RefreshDto>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = auth_service::refresh(&state.db, &dto.refresh_token, &state.config.jwt_secret).await?;
    Ok(Json(res))
}
