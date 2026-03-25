use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "alliance_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllianceRole {
    Leader,
    Officer,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Alliance {
    pub id:           Uuid,
    pub universe_id:  Uuid,
    pub name:         String,
    pub tag:          String,
    pub description:  Option<String>,
    pub leader_empire_id: Uuid,
    pub created_at:   DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AllianceMember {
    pub id:          Uuid,
    pub alliance_id: Uuid,
    pub empire_id:   Uuid,
    pub role:        AllianceRole,
    pub joined_at:   DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAllianceDto {
    pub name: String,
    pub tag:  String,
}
