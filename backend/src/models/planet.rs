use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Planet {
    pub id:                   Uuid,
    pub empire_id:            Uuid,
    pub universe_id:          Uuid,
    pub name:                 String,
    pub galaxy:               i16,
    pub system:               i16,
    pub position:             i16,
    pub diameter:             i32,
    pub temperature_min:      i16,
    pub temperature_max:      i16,
    pub image_id:             i16,
    pub metal:                f64,
    pub crystal:              f64,
    pub deuterium:            f64,
    pub energy:               i32,
    pub last_resource_update: DateTime<Utc>,
    pub is_homeworld:         bool,
    pub created_at:           DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetResources {
    pub planet_id:  Uuid,
    pub metal:      f64,
    pub crystal:    f64,
    pub deuterium:  f64,
    pub energy:     i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RenamePlanetDto {
    pub name: String,
}
