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

/// Handler to get the authenticated user's zone
pub async fn get_my_zone_handler(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> impl IntoResponse {
    // Get session from request extensions (added by auth middleware)
    let session = request.extensions().get::<crate::auth::models::Session>().cloned();

    let username = match session {
        Some(s) => s.username,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(GetZoneResponse {
                    success: false,
                    message: "Authentication required".to_string(),
                    zone: None,
                })
            );
        }
    };

    // Get user's zone_id from database
    let user = match state.auth_service.db.get_user_by_username(&username) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(GetZoneResponse {
                    success: false,
                    message: "User not found".to_string(),
                    zone: None,
                })
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetZoneResponse {
                    success: false,
                    message: format!("Database error: {}", e),
                    zone: None,
                })
            );
        }
    };

    let zone_id = match user.zone_id {
        Some(id) => id,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(GetZoneResponse {
                    success: false,
                    message: "User has no assigned zone".to_string(),
                    zone: None,
                })
            );
        }
    };

    // Fetch the zone from the game world
    let world = state.game_world.read().await;

    match world.get_zone(&zone_id) {
        Some(zone) => {
            (
                StatusCode::OK,
                Json(GetZoneResponse {
                    success: true,
                    message: format!("Your zone {} retrieved successfully", zone_id),
                    zone: Some(zone.clone()),
                })
            )
        }
        None => {
            // Zone exists in user record but not in game world - generate it
            drop(world); // Release read lock
            let mut world = state.game_world.write().await;

            // Use existing zone generation with user's zone_id
            let zone_id_str = zone_id.clone();
            let seed = {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                zone_id.hash(&mut hasher);
                hasher.finish()
            };

            let zone = crate::game::zone::Zone::generate(zone_id.clone(), seed);
            world.add_zone(zone.clone());

            log::info!("Generated zone {} for user {}", zone_id_str, username);

            (
                StatusCode::OK,
                Json(GetZoneResponse {
                    success: true,
                    message: format!("Your zone {} generated and retrieved successfully", zone_id_str),
                    zone: Some(zone),
                })
            )
        }
    }
}
