use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::building::{BuildDto, BuildingQueue, BuildingStatus};
use crate::routes::AppState;
use crate::services::build_service;

pub async fn list_buildings(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(planet_id): Path<Uuid>,
) -> Result<Json<Vec<BuildingStatus>>, AppError> {
    let empire_id = get_empire_for_planet(&state, auth.user_id, planet_id).await?;

    let rows = sqlx::query!(
        "SELECT building_id, level FROM buildings WHERE planet_id = $1",
        planet_id,
    )
    .fetch_all(&state.db)
    .await
    .map_err(AppError::from)?;

    let queue = sqlx::query!(
        "SELECT building_id, completes_at FROM building_queues WHERE planet_id = $1",
        planet_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::from)?;

    let mut statuses: Vec<BuildingStatus> = rows
        .into_iter()
        .map(|r| BuildingStatus {
            building_id:  r.building_id.clone(),
            level:        r.level,
            in_queue:     queue.as_ref().map(|q| q.building_id == r.building_id).unwrap_or(false),
            completes_at: queue.as_ref().and_then(|q| {
                if q.building_id == r.building_id { Some(q.completes_at) } else { None }
            }),
        })
        .collect();

    // Add entries from game data for buildings not yet constructed
    for (id, _) in &state.game_data.buildings {
        if !statuses.iter().any(|s| &s.building_id == id) {
            statuses.push(BuildingStatus {
                building_id:  id.clone(),
                level:        0,
                in_queue:     queue.as_ref().map(|q| &q.building_id == id).unwrap_or(false),
                completes_at: queue.as_ref().and_then(|q| {
                    if &q.building_id == id { Some(q.completes_at) } else { None }
                }),
            });
        }
    }

    Ok(Json(statuses))
}

pub async fn start_build(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(planet_id): Path<Uuid>,
    Json(dto): Json<BuildDto>,
) -> Result<Json<BuildingQueue>, AppError> {
    let empire_id = get_empire_for_planet(&state, auth.user_id, planet_id).await?;

    let queue = build_service::start_build(
        &state.db,
        planet_id,
        empire_id,
        &dto.building_id,
        &state.game_data,
    )
    .await?;

    Ok(Json(queue))
}

pub async fn cancel_build(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(planet_id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    get_empire_for_planet(&state, auth.user_id, planet_id).await?;
    build_service::cancel_build(&state.db, planet_id).await?;
    Ok(Json(()))
}

async fn get_empire_for_planet(
    state: &AppState,
    user_id: Uuid,
    planet_id: Uuid,
) -> Result<Uuid, AppError> {
    let empire_id: Uuid = sqlx::query_scalar!(
        "SELECT empire_id FROM planets WHERE id = $1",
        planet_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound("Planet not found".into()))?;

    let owns: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM empires WHERE id = $1 AND user_id = $2)",
        empire_id, user_id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)?
    .unwrap_or(false);

    if !owns {
        return Err(AppError::Forbidden("Not your planet".into()));
    }

    Ok(empire_id)
}
