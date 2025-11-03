//! Zone routes module
//! 
//! HTTP endpoint handlers for zone generation and retrieval.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::game::zone::Zone;
use crate::network::server::AppState;

/// Request to generate a new zone
#[derive(Debug, Deserialize)]
pub struct GenerateZoneRequest {
    pub player_id: String,
}

/// Response for zone generation
#[derive(Debug, Serialize)]
pub struct GenerateZoneResponse {
    pub success: bool,
    pub message: String,
    pub zone_id: Option<String>,
}

/// Response for getting a zone
#[derive(Debug, Serialize)]
pub struct GetZoneResponse {
    pub success: bool,
    pub message: String,
    pub zone: Option<Zone>,
}

/// Response for listing all zones
#[derive(Debug, Serialize)]
pub struct ListZonesResponse {
    pub success: bool,
    pub message: String,
    pub zone_ids: Vec<String>,
}

/// Handler to generate a new zone for a player
pub async fn generate_zone_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateZoneRequest>,
) -> impl IntoResponse {
    let mut world = state.game_world.write().await;
    
    let zone_id = world.generate_player_zone(&payload.player_id);
    
    log::info!("Generated zone {} for player {}", zone_id, payload.player_id);
    
    (
        StatusCode::OK,
        Json(GenerateZoneResponse {
            success: true,
            message: format!("Zone generated successfully for player {}", payload.player_id),
            zone_id: Some(zone_id),
        })
    )
}

/// Handler to get a specific zone by ID
pub async fn get_zone_handler(
    State(state): State<AppState>,
    Path(zone_id): Path<String>,
) -> impl IntoResponse {
    let world = state.game_world.read().await;
    
    match world.get_zone(&zone_id) {
        Some(zone) => {
            (
                StatusCode::OK,
                Json(GetZoneResponse {
                    success: true,
                    message: format!("Zone {} retrieved successfully", zone_id),
                    zone: Some(zone.clone()),
                })
            )
        }
        None => {
            (
                StatusCode::NOT_FOUND,
                Json(GetZoneResponse {
                    success: false,
                    message: format!("Zone {} not found", zone_id),
                    zone: None,
                })
            )
        }
    }
}

/// Handler to list all zones
pub async fn list_zones_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let world = state.game_world.read().await;
    
    let zone_ids = world.get_zone_ids();
    
    (
        StatusCode::OK,
        Json(ListZonesResponse {
            success: true,
            message: format!("Found {} zones", zone_ids.len()),
            zone_ids,
        })
    )
}
