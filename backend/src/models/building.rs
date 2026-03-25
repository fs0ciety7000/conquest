use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Building {
    pub id:          Uuid,
    pub planet_id:   Uuid,
    pub building_id: String,
    pub level:       i32,
    pub updated_at:  DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BuildingQueue {
    pub id:           Uuid,
    pub planet_id:    Uuid,
    pub building_id:  String,
    pub level:        i32,
    pub started_at:   DateTime<Utc>,
    pub completes_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BuildDto {
    pub building_id: String,
}

#[derive(Debug, Serialize)]
pub struct BuildingStatus {
    pub building_id:  String,
    pub level:        i32,
    pub in_queue:     bool,
    pub completes_at: Option<DateTime<Utc>>,
}
