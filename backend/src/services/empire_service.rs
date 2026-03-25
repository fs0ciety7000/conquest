use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::empire::{CreateEmpireDto, Empire};
use crate::models::planet::Planet;

pub async fn create_empire(
    pool: &PgPool,
    user_id: Uuid,
    universe_id: Uuid,
    dto: CreateEmpireDto,
) -> Result<(Empire, Planet), AppError> {
    let exists: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM empires WHERE user_id = $1 AND universe_id = $2)",
        user_id, universe_id,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?
    .unwrap_or(false);

    if exists {
        return Err(AppError::Conflict("Empire already exists in this universe".into()));
    }

    let max_players: i32 = sqlx::query_scalar!(
        "SELECT max_players FROM universes WHERE id = $1",
        universe_id,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?;

    let current_count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM empires WHERE universe_id = $1",
        universe_id,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?
    .unwrap_or(0);

    if current_count >= max_players as i64 {
        return Err(AppError::Conflict("Universe is full".into()));
    }

    let mut tx = pool.begin().await.map_err(AppError::from)?;

    let empire = sqlx::query_as!(
        Empire,
        r#"
        INSERT INTO empires (user_id, universe_id, name)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, universe_id, name, points, rank, is_protected, created_at
        "#,
        user_id, universe_id, dto.name,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::from)?;

    let (galaxy, system, position) = find_empty_slot(&mut tx, universe_id).await?;

    let planet_name = dto.home_name.unwrap_or_else(|| "Homeworld".to_string());
    let diameter: i32      = rand::thread_rng().gen_range(8_000..=12_000);
    let temp_min: i16      = rand::thread_rng().gen_range(-10_i16..=30_i16);
    let temp_max: i16      = temp_min + rand::thread_rng().gen_range(40_i16..=60_i16);
    let image_id: i16      = rand::thread_rng().gen_range(1_i16..=10_i16);

    let planet = sqlx::query_as!(
        Planet,
        r#"
        INSERT INTO planets
            (empire_id, universe_id, name, galaxy, system, position,
             diameter, temperature_min, temperature_max, image_id, is_homeworld)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, true)
        RETURNING
            id, empire_id, universe_id, name, galaxy, system, position,
            diameter, temperature_min, temperature_max, image_id,
            metal, crystal, deuterium, energy,
            last_resource_update, is_homeworld, created_at
        "#,
        empire.id, universe_id, planet_name,
        galaxy, system, position,
        diameter, temp_min, temp_max, image_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::from)?;

    tx.commit().await.map_err(AppError::from)?;

    Ok((empire, planet))
}

pub async fn get_empire_by_user(
    pool: &PgPool,
    user_id: Uuid,
    universe_id: Uuid,
) -> Result<Empire, AppError> {
    sqlx::query_as!(
        Empire,
        r#"
        SELECT id, user_id, universe_id, name, points, rank, is_protected, created_at
        FROM empires WHERE user_id = $1 AND universe_id = $2
        "#,
        user_id, universe_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound("Empire not found".into()))
}

async fn find_empty_slot(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    universe_id: Uuid,
) -> Result<(i16, i16, i16), AppError> {
    for _ in 0..100 {
        let (galaxy, system, position) = {
            let mut rng = rand::thread_rng();
            (rng.gen_range(1_i16..=9_i16), rng.gen_range(1_i16..=499_i16), rng.gen_range(1_i16..=15_i16))
        };

        let occupied: bool = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM planets
                WHERE universe_id = $1 AND galaxy = $2 AND system = $3 AND position = $4
            )
            "#,
            universe_id, galaxy, system, position,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(AppError::from)?
        .unwrap_or(false);

        if !occupied {
            return Ok((galaxy, system, position));
        }
    }
    Err(AppError::Internal("Could not find an empty planet slot".into()))
}
