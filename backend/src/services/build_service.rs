use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::gamedata::GameDataRegistry;
use crate::models::building::BuildingQueue;
use crate::services::{requirements_service, resource_service};

fn building_cost(base_metal: f64, base_crystal: f64, base_deut: f64, factor: f64, next_level: i32)
    -> (f64, f64, f64)
{
    (
        base_metal    * factor.powi(next_level - 1),
        base_crystal  * factor.powi(next_level - 1),
        base_deut     * factor.powi(next_level - 1),
    )
}

fn build_duration_secs(base_secs: u64, factor: f64, level: i32, robotics: i32, nanite: i32) -> i64 {
    let raw = base_secs as f64 * factor.powi(level - 1);
    let div = (1 + robotics) as f64 * 2_f64.powi(nanite);
    (raw / div).max(1.0) as i64
}

pub async fn start_build(
    pool: &PgPool,
    planet_id: Uuid,
    empire_id: Uuid,
    building_id: &str,
    registry: &GameDataRegistry,
) -> Result<BuildingQueue, AppError> {
    let config = registry
        .building(building_id)
        .ok_or_else(|| AppError::NotFound(format!("Unknown building: {building_id}")))?;

    let mut tx = pool.begin().await.map_err(AppError::from)?;

    // Ensure no queue slot is occupied
    let busy: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM building_queues WHERE planet_id = $1)",
        planet_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::from)?
    .unwrap_or(false);

    if busy {
        return Err(AppError::Conflict("Construction queue is full".into()));
    }

    let current_level: i32 = sqlx::query_scalar!(
        "SELECT level FROM buildings WHERE planet_id = $1 AND building_id = $2",
        planet_id,
        building_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(AppError::from)?
    .unwrap_or(0);

    if let Some(max) = config.max_level {
        if current_level >= max {
            return Err(AppError::BadRequest(format!(
                "Building already at max level ({max})"
            )));
        }
    }

    let next_level = current_level + 1;

    requirements_service::check_requirements(
        pool,
        &config.requirements,
        planet_id,
        empire_id,
        registry,
    )
    .await?;

    let resources = resource_service::flush_resources(pool, planet_id, registry).await?;
    let (cost_m, cost_c, cost_d) = building_cost(
        config.cost.base.metal,
        config.cost.base.crystal,
        config.cost.base.deuterium,
        config.cost.factor,
        next_level,
    );

    if resources.metal < cost_m || resources.crystal < cost_c || resources.deuterium < cost_d {
        return Err(AppError::InsufficientResources("Not enough resources".into()));
    }

    sqlx::query!(
        "UPDATE planets SET metal = metal - $1, crystal = crystal - $2, deuterium = deuterium - $3 WHERE id = $4",
        cost_m, cost_c, cost_d, planet_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::from)?;

    let robotics: i32 = sqlx::query_scalar!(
        "SELECT level FROM buildings WHERE planet_id = $1 AND building_id = 'bldg_robotics_factory'",
        planet_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(AppError::from)?
    .unwrap_or(0);

    let nanite: i32 = sqlx::query_scalar!(
        "SELECT level FROM buildings WHERE planet_id = $1 AND building_id = 'bldg_nanite_factory'",
        planet_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(AppError::from)?
    .unwrap_or(0);

    let duration = build_duration_secs(
        config.build_time.base_seconds,
        config.build_time.factor,
        next_level,
        robotics,
        nanite,
    );

    let now = Utc::now();
    let completes_at = now + chrono::Duration::seconds(duration);

    let queue_entry = sqlx::query_as!(
        BuildingQueue,
        r#"
        INSERT INTO building_queues (planet_id, building_id, level, started_at, completes_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, planet_id, building_id, level, started_at, completes_at
        "#,
        planet_id, building_id, next_level, now, completes_at,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::from)?;

    let universe_id: Uuid = sqlx::query_scalar!(
        "SELECT universe_id FROM planets WHERE id = $1",
        planet_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::from)?;

    let payload = serde_json::json!({
        "planet_id":   planet_id,
        "empire_id":   empire_id,
        "building_id": building_id,
        "level":       next_level,
    });

    sqlx::query!(
        r#"
        INSERT INTO event_queue (universe_id, event_type, payload, execution_time)
        VALUES ($1, 'BUILD_COMPLETE', $2, $3)
        "#,
        universe_id, payload, completes_at,
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::from)?;

    tx.commit().await.map_err(AppError::from)?;

    Ok(queue_entry)
}

pub async fn cancel_build(pool: &PgPool, planet_id: Uuid) -> Result<(), AppError> {
    let deleted = sqlx::query!(
        "DELETE FROM building_queues WHERE planet_id = $1 RETURNING id",
        planet_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?;

    if deleted.is_none() {
        return Err(AppError::NotFound("No active construction".into()));
    }

    Ok(())
}
