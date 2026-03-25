use axum::{extract::State, Json};
use serde::Serialize;

use crate::gamedata::schema::{BuildingConfig, DefenseConfig, ShipConfig, TechConfig};
use crate::routes::AppState;

#[derive(Serialize)]
pub struct GameDataResponse {
    pub buildings:    Vec<BuildingConfig>,
    pub ships:        Vec<ShipConfig>,
    pub technologies: Vec<TechConfig>,
    pub defenses:     Vec<DefenseConfig>,
}

pub async fn get_game_data(State(state): State<AppState>) -> Json<GameDataResponse> {
    Json(GameDataResponse {
        buildings:    state.game_data.buildings.values().cloned().collect(),
        ships:        state.game_data.ships.values().cloned().collect(),
        technologies: state.game_data.technologies.values().cloned().collect(),
        defenses:     state.game_data.defenses.values().cloned().collect(),
    })
}
