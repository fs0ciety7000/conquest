use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::empire::{CreateEmpireDto, Empire};
use crate::models::planet::Planet;
use crate::routes::AppState;
use crate::services::empire_service;

#[derive(Serialize)]
pub struct EmpireWithPlanets {
    pub empire:  Empire,
    pub planets: Vec<Planet>,
}

pub async fn get_empire(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(universe_id): Path<Uuid>,
) -> Result<Json<EmpireWithPlanets>, AppError> {
    let empire = empire_service::get_empire_by_user(&state.db, auth.user_id, universe_id).await?;

    let planets = sqlx::query_as!(
        Planet,
        r#"
        SELECT id, empire_id, universe_id, name, galaxy, system, position,
               diameter, temperature_min, temperature_max, image_id,
               metal, crystal, deuterium, energy,
               last_resource_update, is_homeworld, created_at
        FROM planets WHERE empire_id = $1
        ORDER BY is_homeworld DESC, created_at ASC
        "#,
        empire.id,
    )
    .fetch_all(&state.db)
    .await
    .map_err(AppError::from)?;

    Ok(Json(EmpireWithPlanets { empire, planets }))
}

pub async fn create_empire(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(universe_id): Path<Uuid>,
    Json(dto): Json<CreateEmpireDto>,
) -> Result<Json<EmpireWithPlanets>, AppError> {
    let (empire, planet) =
        empire_service::create_empire(&state.db, auth.user_id, universe_id, dto).await?;

    Ok(Json(EmpireWithPlanets {
        empire,
        planets: vec![planet],
    }))
}
