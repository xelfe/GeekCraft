//! Zone routes module
//!
//! HTTP endpoint handlers for zone generation and retrieval.
//! FIXED: Updated to use World.ensure_user_zone() API

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
    pub player_id: String,  // This is the username
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
/// FIXED: Now uses ensure_user_zone instead of generate_player_zone
pub async fn generate_zone_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateZoneRequest>,
) -> impl IntoResponse {
    // Get user from database first (we need user_id)
    let user = match state.auth_service.db.get_user_by_username(&payload.player_id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(GenerateZoneResponse {
                    success: false,
                    message: format!("User {} not found", payload.player_id),
                    zone_id: None,
                })
            );
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenerateZoneResponse {
                    success: false,
                    message: "Database error".to_string(),
                    zone_id: None,
                })
            );
        }
    };

    // FIXED: Use ensure_user_zone instead of generate_player_zone
    let mut world = state.game_world.write().await;
    let zone_id = match world.ensure_user_zone(user.id, &user.username) {
        Ok(zone_id) => zone_id,
        Err(e) => {
            log::error!("Failed to ensure zone for user {}: {}", payload.player_id, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenerateZoneResponse {
                    success: false,
                    message: format!("Failed to generate zone: {}", e),
                    zone_id: None,
                })
            );
        }
    };

    log::info!("Generated/ensured zone {} for player {}", zone_id, payload.player_id);

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
/// This handler automatically generates a zone if the user doesn't have one yet
pub async fn get_my_zone_handler(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> impl IntoResponse {
    // Get session from request extensions (added by auth middleware)
    let session = request.extensions().get::<crate::auth::models::Session>().cloned();

    let session = match session {
        Some(s) => s,
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

    // FIXED: Use ensure_user_zone which handles everything
    let mut world = state.game_world.write().await;
    let zone_id = match world.ensure_user_zone(session.user_id, &session.username) {
        Ok(zone_id) => zone_id,
        Err(e) => {
            log::error!("Failed to ensure zone for user {}: {}", session.username, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetZoneResponse {
                    success: false,
                    message: format!("Failed to get/generate zone: {}", e),
                    zone: None,
                })
            );
        }
    };

    // Get the zone
    let zone = match world.get_zone(&zone_id) {
        Some(zone) => zone.clone(),
        None => {
            // This shouldn't happen since ensure_user_zone creates the zone
            log::error!("Zone {} exists in DB but not in world for user {}", zone_id, session.username);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetZoneResponse {
                    success: false,
                    message: "Internal error: zone not found after creation".to_string(),
                    zone: None,
                })
            );
        }
    };

    log::info!("User {} retrieved their zone {}", session.username, zone_id);

    (
        StatusCode::OK,
        Json(GetZoneResponse {
            success: true,
            message: format!("Your zone {} retrieved successfully", zone_id),
            zone: Some(zone),
        })
    )
}