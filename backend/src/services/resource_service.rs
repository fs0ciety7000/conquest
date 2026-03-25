use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::gamedata::GameDataRegistry;
use crate::models::planet::{Planet, PlanetResources};

pub async fn compute_resources(
    pool: &PgPool,
    planet_id: Uuid,
    registry: &GameDataRegistry,
) -> Result<PlanetResources, AppError> {
    let planet = fetch_planet(pool, planet_id).await?;
    let (metal, crystal, deuterium, energy) =
        calculate_current_resources(&planet, pool).await?;

    Ok(PlanetResources {
        planet_id,
        metal,
        crystal,
        deuterium,
        energy,
        updated_at: Utc::now(),
    })
}

pub async fn flush_resources(
    pool: &PgPool,
    planet_id: Uuid,
    _registry: &GameDataRegistry,
) -> Result<PlanetResources, AppError> {
    let planet = sqlx::query_as!(
        Planet,
        r#"
        SELECT id, empire_id, universe_id, name, galaxy, system, position,
               diameter, temperature_min, temperature_max, image_id,
               metal, crystal, deuterium, energy,
               last_resource_update, is_homeworld, created_at
        FROM planets WHERE id = $1 FOR UPDATE
        "#,
        planet_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound("Planet not found".into()))?;

    let (metal, crystal, deuterium, energy) =
        calculate_current_resources(&planet, pool).await?;

    let (metal_cap, crystal_cap, deut_cap) = get_storage_capacity(planet.id, pool).await?;
    let metal = metal.min(metal_cap);
    let crystal = crystal.min(crystal_cap);
    let deuterium = deuterium.min(deut_cap);

    sqlx::query!(
        r#"
        UPDATE planets
        SET metal = $1, crystal = $2, deuterium = $3, energy = $4,
            last_resource_update = NOW()
        WHERE id = $5
        "#,
        metal, crystal, deuterium, energy, planet_id,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    Ok(PlanetResources {
        planet_id,
        metal,
        crystal,
        deuterium,
        energy,
        updated_at: Utc::now(),
    })
}

async fn fetch_planet(pool: &PgPool, planet_id: Uuid) -> Result<Planet, AppError> {
    sqlx::query_as!(
        Planet,
        r#"
        SELECT id, empire_id, universe_id, name, galaxy, system, position,
               diameter, temperature_min, temperature_max, image_id,
               metal, crystal, deuterium, energy,
               last_resource_update, is_homeworld, created_at
        FROM planets WHERE id = $1
        "#,
        planet_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound("Planet not found".into()))
}

async fn calculate_current_resources(
    planet: &Planet,
    pool: &PgPool,
) -> Result<(f64, f64, f64, i32), AppError> {
    let elapsed = (Utc::now() - planet.last_resource_update).num_seconds().max(0) as f64;

    let buildings = get_planet_buildings(pool, planet.id).await?;
    let (economy_speed, _) = get_universe_speeds(pool, planet.universe_id).await?;

    let metal_mine_level   = *buildings.get("bldg_metal_mine").unwrap_or(&0);
    let crystal_mine_level = *buildings.get("bldg_crystal_mine").unwrap_or(&0);
    let deut_synth_level   = *buildings.get("bldg_deuterium_synthesizer").unwrap_or(&0);
    let solar_plant_level  = *buildings.get("bldg_solar_plant").unwrap_or(&0);
    let fusion_level       = *buildings.get("bldg_fusion_reactor").unwrap_or(&0);

    let metal_prod   = metal_production(metal_mine_level) * economy_speed as f64;
    let crystal_prod = crystal_production(crystal_mine_level) * economy_speed as f64;
    let deut_prod    = deuterium_production(deut_synth_level, planet.temperature_max) * economy_speed as f64;

    let solar_energy  = solar_energy_production(solar_plant_level) as i64;
    let fusion_energy = fusion_energy_production(fusion_level) as i64;
    let total_energy  = solar_energy + fusion_energy;

    let metal_energy_need   = metal_energy_consumption(metal_mine_level);
    let crystal_energy_need = crystal_energy_consumption(crystal_mine_level);
    let deut_energy_need    = deuterium_energy_consumption(deut_synth_level);
    let total_energy_need   = metal_energy_need + crystal_energy_need + deut_energy_need;

    let energy_ratio = if total_energy_need > 0 {
        (total_energy as f64 / total_energy_need as f64).min(1.0)
    } else {
        1.0
    };

    let metal_per_sec   = metal_prod   * energy_ratio / 3600.0;
    let crystal_per_sec = crystal_prod * energy_ratio / 3600.0;
    let deut_per_sec    = deut_prod    * energy_ratio / 3600.0;

    let net_energy = (total_energy - total_energy_need) as i32;

    Ok((
        planet.metal    + metal_per_sec   * elapsed,
        planet.crystal  + crystal_per_sec * elapsed,
        planet.deuterium + deut_per_sec   * elapsed,
        net_energy,
    ))
}

fn metal_production(level: i32) -> f64 { 30.0 * level as f64 * 1.1_f64.powi(level) }
fn crystal_production(level: i32) -> f64 { 20.0 * level as f64 * 1.1_f64.powi(level) }
fn deuterium_production(level: i32, max_temp: i16) -> f64 {
    10.0 * level as f64 * 1.1_f64.powi(level) * (1.44 - 0.004 * max_temp as f64)
}
fn solar_energy_production(level: i32) -> i64 { (20.0 * level as f64 * 1.1_f64.powi(level)) as i64 }
fn fusion_energy_production(level: i32) -> i64 { (30.0 * level as f64 * 1.05_f64.powi(level)) as i64 }
fn metal_energy_consumption(level: i32) -> i64 { (10.0 * level as f64 * 1.1_f64.powi(level)) as i64 }
fn crystal_energy_consumption(level: i32) -> i64 { (10.0 * level as f64 * 1.1_f64.powi(level)) as i64 }
fn deuterium_energy_consumption(level: i32) -> i64 { (20.0 * level as f64 * 1.1_f64.powi(level)) as i64 }

async fn get_planet_buildings(pool: &PgPool, planet_id: Uuid) -> Result<std::collections::HashMap<String, i32>, AppError> {
    let rows = sqlx::query!(
        "SELECT building_id, level FROM buildings WHERE planet_id = $1",
        planet_id,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)?;
    Ok(rows.into_iter().map(|r| (r.building_id, r.level)).collect())
}

async fn get_universe_speeds(pool: &PgPool, universe_id: Uuid) -> Result<(i16, i16), AppError> {
    let row = sqlx::query!(
        "SELECT economy_speed, fleet_speed FROM universes WHERE id = $1",
        universe_id,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?;
    Ok((row.economy_speed, row.fleet_speed))
}

async fn get_storage_capacity(planet_id: Uuid, pool: &PgPool) -> Result<(f64, f64, f64), AppError> {
    let buildings = get_planet_buildings(pool, planet_id).await?;
    let metal_storage_level   = *buildings.get("bldg_metal_storage").unwrap_or(&0);
    let crystal_storage_level = *buildings.get("bldg_crystal_storage").unwrap_or(&0);
    let deut_tank_level       = *buildings.get("bldg_deuterium_tank").unwrap_or(&0);
    Ok((
        5000.0 * 2.5_f64.powi(metal_storage_level),
        5000.0 * 2.5_f64.powi(crystal_storage_level),
        5000.0 * 2.5_f64.powi(deut_tank_level),
    ))
}
