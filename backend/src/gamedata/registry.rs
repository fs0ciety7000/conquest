use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

use super::loader;
use super::schema::{BuildingConfig, DefenseConfig, ShipConfig, TechConfig};

#[derive(Debug, Clone)]
pub struct GameDataRegistry {
    pub buildings:    HashMap<String, BuildingConfig>,
    pub ships:        HashMap<String, ShipConfig>,
    pub technologies: HashMap<String, TechConfig>,
    pub defenses:     HashMap<String, DefenseConfig>,
}

impl GameDataRegistry {
    pub fn load(data_dir: &Path) -> Result<Self> {
        let buildings: HashMap<String, BuildingConfig> = loader::load_buildings(data_dir)?
            .into_iter()
            .map(|b| (b.id.clone(), b))
            .collect();

        let ships: HashMap<String, ShipConfig> = loader::load_ships(data_dir)?
            .into_iter()
            .map(|s| (s.id.clone(), s))
            .collect();

        let technologies: HashMap<String, TechConfig> = loader::load_technologies(data_dir)?
            .into_iter()
            .map(|t| (t.id.clone(), t))
            .collect();

        let defenses: HashMap<String, DefenseConfig> = loader::load_defenses(data_dir)?
            .into_iter()
            .map(|d| (d.id.clone(), d))
            .collect();

        tracing::info!(
            buildings = buildings.len(),
            ships = ships.len(),
            technologies = technologies.len(),
            defenses = defenses.len(),
            "Game data loaded"
        );

        Ok(Self { buildings, ships, technologies, defenses })
    }

    pub fn building(&self, id: &str) -> Option<&BuildingConfig> {
        self.buildings.get(id)
    }

    pub fn ship(&self, id: &str) -> Option<&ShipConfig> {
        self.ships.get(id)
    }

    pub fn technology(&self, id: &str) -> Option<&TechConfig> {
        self.technologies.get(id)
    }

    pub fn defense(&self, id: &str) -> Option<&DefenseConfig> {
        self.defenses.get(id)
    }
}
