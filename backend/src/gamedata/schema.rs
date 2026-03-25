use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    pub metal:     f64,
    pub crystal:   f64,
    pub deuterium: f64,
    pub energy:    Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostFormula {
    pub base:     ResourceCost,
    pub factor:   f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeFormula {
    pub base_seconds:  u64,
    pub factor:        f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub building_id:    Option<String>,
    pub technology_id:  Option<String>,
    pub level:          i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionFormula {
    pub base:           f64,
    pub factor:         f64,
    pub energy_factor:  Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingConfig {
    pub id:           String,
    pub name:         String,
    pub description:  String,
    pub max_level:    Option<i32>,
    pub cost:         CostFormula,
    pub build_time:   TimeFormula,
    pub requirements: Vec<Requirement>,
    pub production:   Option<HashMap<String, ProductionFormula>>,
    pub storage:      Option<HashMap<String, f64>>,
    pub effects:      Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStats {
    pub structural_integrity: u64,
    pub shield_power:         u64,
    pub attack_power:         u64,
    pub speed:                u64,
    pub cargo_capacity:       u64,
    pub fuel_consumption:     u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipConfig {
    pub id:           String,
    pub name:         String,
    pub description:  String,
    pub cost:         ResourceCost,
    pub build_time:   u64,
    pub requirements: Vec<Requirement>,
    pub stats:        ShipStats,
    pub rapid_fire:   Option<HashMap<String, u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechConfig {
    pub id:           String,
    pub name:         String,
    pub description:  String,
    pub max_level:    Option<i32>,
    pub cost:         CostFormula,
    pub research_time: TimeFormula,
    pub requirements: Vec<Requirement>,
    pub effects:      Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseStats {
    pub structural_integrity: u64,
    pub shield_power:         u64,
    pub attack_power:         u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseConfig {
    pub id:           String,
    pub name:         String,
    pub description:  String,
    pub cost:         ResourceCost,
    pub build_time:   u64,
    pub requirements: Vec<Requirement>,
    pub stats:        DefenseStats,
    pub rapid_fire:   Option<HashMap<String, u32>>,
}
