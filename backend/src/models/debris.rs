use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DebrisField {
    pub id:          Uuid,
    pub universe_id: Uuid,
    pub galaxy:      i16,
    pub system:      i16,
    pub position:    i16,
    pub metal:       f64,
    pub crystal:     f64,
    pub created_at:  DateTime<Utc>,
    pub expires_at:  DateTime<Utc>,
}
