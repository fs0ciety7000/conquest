use std::path::Path;
use anyhow::{Context, Result};

use super::schema::{BuildingConfig, DefenseConfig, ShipConfig, TechConfig};

pub fn load_buildings(data_dir: &Path) -> Result<Vec<BuildingConfig>> {
    load_json_dir(data_dir.join("buildings"))
}

pub fn load_ships(data_dir: &Path) -> Result<Vec<ShipConfig>> {
    load_json_dir(data_dir.join("ships"))
}

pub fn load_technologies(data_dir: &Path) -> Result<Vec<TechConfig>> {
    load_json_dir(data_dir.join("technologies"))
}

pub fn load_defenses(data_dir: &Path) -> Result<Vec<DefenseConfig>> {
    load_json_dir(data_dir.join("defenses"))
}

fn load_json_dir<T: serde::de::DeserializeOwned>(dir: impl AsRef<Path>) -> Result<Vec<T>> {
    let dir = dir.as_ref();
    let mut items = Vec::new();

    if !dir.exists() {
        tracing::warn!("Game data directory not found: {}", dir.display());
        return Ok(items);
    }

    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read: {}", path.display()))?;

        let item: T = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse: {}", path.display()))?;

        items.push(item);
    }

    Ok(items)
}
