use serde::Serialize;
use uuid::Uuid;

/// Events pushed over WebSocket to the client.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WsEvent {
    BuildComplete {
        planet_id:   Uuid,
        building_id: String,
        level:       i32,
    },
    ResearchComplete {
        technology_id: String,
        level:         i32,
    },
    ResourceUpdate {
        planet_id:  Uuid,
        metal:      f64,
        crystal:    f64,
        deuterium:  f64,
        energy:     i32,
    },
    FleetArrived {
        fleet_id: Uuid,
    },
    CombatReport {
        report_id: Uuid,
    },
}
