use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id:             Uuid,
    pub universe_id:    Uuid,
    pub event_type:     String,
    pub payload:        Value,
    pub status:         String,
    pub execution_time: DateTime<Utc>,
    pub processed_at:   Option<DateTime<Utc>>,
    pub error_message:  Option<String>,
    pub created_at:     DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCompletePayload {
    pub planet_id:   Uuid,
    pub empire_id:   Uuid,
    pub building_id: String,
    pub level:       i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchCompletePayload {
    pub empire_id:     Uuid,
    pub technology_id: String,
    pub level:         i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetArrivalPayload {
    pub fleet_id: Uuid,
}
