use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Universe {
    pub id:               Uuid,
    pub name:             String,
    pub speed_multiplier: i16,
    pub fleet_speed:      i16,
    pub economy_speed:    i16,
    pub research_speed:   i16,
    pub max_players:      i32,
    pub is_active:        bool,
    pub created_at:       DateTime<Utc>,
}
