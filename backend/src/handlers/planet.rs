use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::AuthUser;
use crate::models::planet::PlanetResources;
use crate::routes::AppState;
use crate::services::resource_service;

pub async fn get_resources(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(planet_id): Path<Uuid>,
) -> Result<Json<PlanetResources>, AppError> {
    // Verify ownership
    let empire_id: Uuid = sqlx::query_scalar!(
        "SELECT empire_id FROM planets WHERE id = $1",
        planet_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound("Planet not found".into()))?;

    let owner: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM empires WHERE id = $1 AND user_id = $2)",
        empire_id, auth.user_id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)?
    .unwrap_or(false);

    if !owner {
        return Err(AppError::Forbidden("Not your planet".into()));
    }

    let resources = resource_service::compute_resources(&state.db, planet_id, &state.game_data).await?;
    Ok(Json(resources))
}
