use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CombatReport {
    pub id:              Uuid,
    pub universe_id:     Uuid,
    pub attacker_empire_id: Uuid,
    pub defender_empire_id: Uuid,
    pub target_planet_id:   Uuid,
    pub attacker_won:    bool,
    pub metal_stolen:    f64,
    pub crystal_stolen:  f64,
    pub deuterium_stolen: f64,
    pub report_data:     Value,
    pub fought_at:       DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DebrisField {
    pub id:         Uuid,
    pub galaxy:     i16,
    pub system:     i16,
    pub position:   i16,
    pub universe_id: Uuid,
    pub metal:      f64,
    pub crystal:    f64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
