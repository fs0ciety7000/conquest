use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "fleet_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FleetStatus {
    Outbound,
    Returning,
    Orbiting,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "fleet_mission", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FleetMission {
    Attack,
    Transport,
    Colonize,
    Recycle,
    Espionage,
    Deploy,
    Return,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Fleet {
    pub id:              Uuid,
    pub empire_id:       Uuid,
    pub universe_id:     Uuid,
    pub mission:         FleetMission,
    pub status:          FleetStatus,
    pub origin_planet_id: Uuid,
    pub target_galaxy:   i16,
    pub target_system:   i16,
    pub target_position: i16,
    pub metal_cargo:     f64,
    pub crystal_cargo:   f64,
    pub deuterium_cargo: f64,
    pub departure_time:  DateTime<Utc>,
    pub arrival_time:    DateTime<Utc>,
    pub return_time:     Option<DateTime<Utc>>,
    pub created_at:      DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FleetUnit {
    pub id:       Uuid,
    pub fleet_id: Uuid,
    pub ship_id:  String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct SendFleetDto {
    pub mission:         FleetMission,
    pub origin_planet_id: Uuid,
    pub target_galaxy:   i16,
    pub target_system:   i16,
    pub target_position: i16,
    pub ships:           Vec<FleetUnitDto>,
    pub metal_cargo:     Option<f64>,
    pub crystal_cargo:   Option<f64>,
    pub deuterium_cargo: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct FleetUnitDto {
    pub ship_id:  String,
    pub quantity: i32,
}
