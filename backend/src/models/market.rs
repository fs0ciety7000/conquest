use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "resource_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    Metal,
    Crystal,
    Deuterium,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "offer_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferStatus {
    Active,
    Fulfilled,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketOffer {
    pub id:             Uuid,
    pub universe_id:    Uuid,
    pub seller_empire_id: Uuid,
    pub sell_resource:  ResourceType,
    pub sell_amount:    f64,
    pub buy_resource:   ResourceType,
    pub buy_amount:     f64,
    pub status:         OfferStatus,
    pub created_at:     DateTime<Utc>,
    pub expires_at:     DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOfferDto {
    pub sell_resource: ResourceType,
    pub sell_amount:   f64,
    pub buy_resource:  ResourceType,
    pub buy_amount:    f64,
    pub planet_id:     Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AcceptOfferDto {
    pub planet_id: Uuid,
}
