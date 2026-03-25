use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::gamedata::{schema::Requirement, GameDataRegistry};

/// Returns Ok(()) if all requirements are met.
pub async fn check_requirements(
    pool: &PgPool,
    requirements: &[Requirement],
    planet_id: Uuid,
    empire_id: Uuid,
    registry: &GameDataRegistry,
) -> Result<(), AppError> {
    for req in requirements {
        if let Some(ref bldg_id) = req.building_id {
            let level = get_building_level(pool, planet_id, bldg_id).await?;
            if level < req.level {
                let name = registry
                    .building(bldg_id)
                    .map(|b| b.name.as_str())
                    .unwrap_or(bldg_id);
                return Err(AppError::RequirementsNotMet(format!(
                    "Requires {} level {}",
                    name, req.level
                )));
            }
        }

        if let Some(ref tech_id) = req.technology_id {
            let level = get_technology_level(pool, empire_id, tech_id).await?;
            if level < req.level {
                let name = registry
                    .technology(tech_id)
                    .map(|t| t.name.as_str())
                    .unwrap_or(tech_id);
                return Err(AppError::RequirementsNotMet(format!(
                    "Requires {} level {}",
                    name, req.level
                )));
            }
        }
    }

    Ok(())
}

async fn get_building_level(pool: &PgPool, planet_id: Uuid, building_id: &str) -> Result<i32, AppError> {
    sqlx::query_scalar!(
        "SELECT level FROM buildings WHERE planet_id = $1 AND building_id = $2",
        planet_id,
        building_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
    .map(|v| v.unwrap_or(0))
}

async fn get_technology_level(pool: &PgPool, empire_id: Uuid, technology_id: &str) -> Result<i32, AppError> {
    sqlx::query_scalar!(
        "SELECT level FROM technologies WHERE empire_id = $1 AND technology_id = $2",
        empire_id,
        technology_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
    .map(|v| v.unwrap_or(0))
}
