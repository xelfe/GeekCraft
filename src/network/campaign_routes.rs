//! Campaign routes module
//! 
//! HTTP endpoint handlers for campaign operations (start, stop, save, load).

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use lazy_static::lazy_static;

use crate::game::campaign::CampaignManager;
use crate::network::server::AppState;

lazy_static! {
    static ref CAMPAIGN_MANAGER: Arc<RwLock<CampaignManager>> = {
        Arc::new(RwLock::new(CampaignManager::new()))
    };
}

/// Request to start a campaign run
#[derive(Debug, Deserialize)]
pub struct StartRunRequest {
    pub run_id: String,
}

/// Response for start run
#[derive(Debug, Serialize)]
pub struct StartRunResponse {
    pub success: bool,
    pub message: String,
    pub run_id: Option<String>,
}

/// Query parameters for getting run state
#[derive(Debug, Deserialize)]
pub struct RunStateQuery {
    pub run_id: String,
}

/// Response for run state
#[derive(Debug, Serialize)]
pub struct RunStateResponse {
    pub success: bool,
    pub message: String,
    pub run: Option<crate::game::campaign::CampaignRun>,
}

/// Request to stop a run
#[derive(Debug, Deserialize)]
pub struct StopRunRequest {
    pub run_id: String,
}

/// Response for stop run
#[derive(Debug, Serialize)]
pub struct StopRunResponse {
    pub success: bool,
    pub message: String,
}

/// Request to save a run
#[derive(Debug, Deserialize)]
pub struct SaveRunRequest {
    pub run_id: String,
}

/// Response for save run
#[derive(Debug, Serialize)]
pub struct SaveRunResponse {
    pub success: bool,
    pub message: String,
}

/// Response for listing saves
#[derive(Debug, Serialize)]
pub struct ListSavesResponse {
    pub success: bool,
    pub message: String,
    pub saves: Vec<String>,
}

/// Request to load a run
#[derive(Debug, Deserialize)]
pub struct LoadRunRequest {
    pub run_id: String,
}

/// Response for load run
#[derive(Debug, Serialize)]
pub struct LoadRunResponse {
    pub success: bool,
    pub message: String,
    pub run: Option<crate::game::campaign::CampaignRun>,
}

/// Handler to start a campaign run
pub async fn start_run_handler(
    State(_state): State<AppState>,
    Json(payload): Json<StartRunRequest>,
) -> impl IntoResponse {
    let mut manager = CAMPAIGN_MANAGER.write().await;
    
    match manager.start_run(payload.run_id.clone()) {
        Ok(_run) => {
            log::info!("Started campaign run: {}", payload.run_id);
            (
                StatusCode::OK,
                Json(StartRunResponse {
                    success: true,
                    message: format!("Campaign run {} started successfully", payload.run_id),
                    run_id: Some(payload.run_id),
                })
            )
        }
        Err(err) => {
            log::warn!("Failed to start campaign run: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(StartRunResponse {
                    success: false,
                    message: err,
                    run_id: None,
                })
            )
        }
    }
}

/// Handler to get run state
pub async fn get_run_state_handler(
    State(_state): State<AppState>,
    Query(query): Query<RunStateQuery>,
) -> impl IntoResponse {
    let manager = CAMPAIGN_MANAGER.read().await;
    
    match manager.get_run_state(&query.run_id) {
        Some(run) => {
            (
                StatusCode::OK,
                Json(RunStateResponse {
                    success: true,
                    message: format!("Retrieved state for run {}", query.run_id),
                    run: Some(run),
                })
            )
        }
        None => {
            (
                StatusCode::NOT_FOUND,
                Json(RunStateResponse {
                    success: false,
                    message: format!("Run {} not found", query.run_id),
                    run: None,
                })
            )
        }
    }
}

/// Handler to stop a run
pub async fn stop_run_handler(
    State(_state): State<AppState>,
    Json(payload): Json<StopRunRequest>,
) -> impl IntoResponse {
    let mut manager = CAMPAIGN_MANAGER.write().await;
    
    match manager.stop_run(&payload.run_id) {
        Ok(()) => {
            log::info!("Stopped campaign run: {}", payload.run_id);
            (
                StatusCode::OK,
                Json(StopRunResponse {
                    success: true,
                    message: format!("Campaign run {} stopped successfully", payload.run_id),
                })
            )
        }
        Err(err) => {
            log::warn!("Failed to stop campaign run: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(StopRunResponse {
                    success: false,
                    message: err,
                })
            )
        }
    }
}

/// Handler to save a run to disk
pub async fn save_run_handler(
    State(_state): State<AppState>,
    Json(payload): Json<SaveRunRequest>,
) -> impl IntoResponse {
    let manager = CAMPAIGN_MANAGER.read().await;
    
    match manager.save_run(&payload.run_id) {
        Ok(()) => {
            log::info!("Saved campaign run: {}", payload.run_id);
            (
                StatusCode::OK,
                Json(SaveRunResponse {
                    success: true,
                    message: format!("Campaign run {} saved successfully", payload.run_id),
                })
            )
        }
        Err(err) => {
            log::warn!("Failed to save campaign run: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(SaveRunResponse {
                    success: false,
                    message: err,
                })
            )
        }
    }
}

/// Handler to list all saved runs
pub async fn list_saves_handler(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    let manager = CAMPAIGN_MANAGER.read().await;
    
    match manager.list_all_saves() {
        Ok(saves) => {
            (
                StatusCode::OK,
                Json(ListSavesResponse {
                    success: true,
                    message: format!("Found {} saved runs", saves.len()),
                    saves,
                })
            )
        }
        Err(err) => {
            log::warn!("Failed to list saves: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListSavesResponse {
                    success: false,
                    message: err,
                    saves: Vec::new(),
                })
            )
        }
    }
}

/// Handler to load a run from disk
pub async fn load_run_handler(
    State(_state): State<AppState>,
    Json(payload): Json<LoadRunRequest>,
) -> impl IntoResponse {
    let mut manager = CAMPAIGN_MANAGER.write().await;
    
    match manager.load_run(&payload.run_id) {
        Ok(run) => {
            log::info!("Loaded campaign run: {}", payload.run_id);
            (
                StatusCode::OK,
                Json(LoadRunResponse {
                    success: true,
                    message: format!("Campaign run {} loaded successfully", payload.run_id),
                    run: Some(run),
                })
            )
        }
        Err(err) => {
            log::warn!("Failed to load campaign run: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(LoadRunResponse {
                    success: false,
                    message: err,
                    run: None,
                })
            )
        }
    }
}
