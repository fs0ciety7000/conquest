use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Technology {
    pub id:            Uuid,
    pub empire_id:     Uuid,
    pub technology_id: String,
    pub level:         i32,
    pub updated_at:    DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResearchQueue {
    pub id:            Uuid,
    pub empire_id:     Uuid,
    pub planet_id:     Uuid,
    pub technology_id: String,
    pub level:         i32,
    pub started_at:    DateTime<Utc>,
    pub completes_at:  DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ResearchDto {
    pub technology_id: String,
    pub planet_id:     Uuid,
}
