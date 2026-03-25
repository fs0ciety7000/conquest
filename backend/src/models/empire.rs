use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Empire {
    pub id:           Uuid,
    pub user_id:      Uuid,
    pub universe_id:  Uuid,
    pub name:         String,
    pub points:       i64,
    pub rank:         Option<i32>,
    pub is_protected: bool,
    pub created_at:   DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmpireDto {
    pub name:       String,
    pub home_name:  Option<String>,
}
