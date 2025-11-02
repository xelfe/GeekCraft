// This file manages network communication, allowing clients to connect and interact with the server.

use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, Json,
};
use axum::extract::ws::{WebSocket, Message};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use serde::{Deserialize, Serialize};
use futures_util::{SinkExt, StreamExt};

use crate::game::world::World;
use crate::scripting::sandbox::ScriptEngine;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub game_world: Arc<RwLock<World>>,
    pub script_engine: Arc<RwLock<ScriptEngine>>,
}

/// Request to submit player code
#[derive(Debug, Deserialize)]
pub struct CodeSubmission {
    pub player_id: String,
    pub code: String,
}

/// Response after code submission
#[derive(Debug, Serialize)]
pub struct CodeSubmissionResponse {
    pub success: bool,
    pub message: String,
}

/// Response for getting player code
#[derive(Debug, Serialize)]
pub struct PlayerCodeResponse {
    pub player_id: String,
    pub code: Option<String>,
}

/// Response for listing players
#[derive(Debug, Serialize)]
pub struct PlayersListResponse {
    pub players: Vec<String>,
}

/// Game state response
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub tick: u64,
    pub players: Vec<String>,
}

/// Start the Axum HTTP and WebSocket server
pub async fn start_server(game_world: Arc<RwLock<World>>, script_engine: Arc<RwLock<ScriptEngine>>) -> anyhow::Result<()> {
    let app_state = AppState {
        game_world,
        script_engine,
    };

    // Build the router with all endpoints
    let app = Router::new()
        // REST API endpoints
        .route("/", get(root_handler))
        .route("/api/health", get(health_handler))
        .route("/api/submit", post(submit_code_handler))
        .route("/api/players", get(list_players_handler))
        .route("/api/gamestate", get(game_state_handler))
        // WebSocket endpoint
        .route("/ws", get(websocket_handler))
        // Add state
        .with_state(app_state)
        // Add CORS middleware
        // NOTE: CORS is configured to allow all origins for development.
        // For production deployment, restrict allowed origins to your specific domains:
        // .layer(CorsLayer::new().allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        // Add tracing middleware
        .layer(TraceLayer::new_for_http());

    // Bind to address
    let addr = "0.0.0.0:3030";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    log::info!("✓ Axum server listening on http://{}", addr);
    log::info!("✓ WebSocket endpoint: ws://{}/ws", addr);
    log::info!("✓ API endpoints:");
    log::info!("  - GET  /");
    log::info!("  - GET  /api/health");
    log::info!("  - POST /api/submit");
    log::info!("  - GET  /api/players");
    log::info!("  - GET  /api/gamestate");

    // Start the server
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Root handler - provides API information
async fn root_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "GeekCraft API Server",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health": "GET /api/health",
            "submit_code": "POST /api/submit",
            "list_players": "GET /api/players",
            "game_state": "GET /api/gamestate",
            "websocket": "WS /ws"
        }
    }))
}

/// Health check handler
async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "geekcraft"
    }))
}

/// Handler to submit player code
async fn submit_code_handler(
    State(state): State<AppState>,
    Json(payload): Json<CodeSubmission>,
) -> impl IntoResponse {
    log::info!("Received code submission from player: {}", payload.player_id);
    
    let mut engine = state.script_engine.write().await;
    
    match engine.submit_code(payload.player_id.clone(), payload.code) {
        Ok(()) => Json(CodeSubmissionResponse {
            success: true,
            message: format!("Code submitted successfully for player {}", payload.player_id),
        }),
        Err(err) => {
            log::warn!("Code submission failed: {}", err);
            Json(CodeSubmissionResponse {
                success: false,
                message: err,
            })
        }
    }
}

/// Handler to list all players
async fn list_players_handler(State(state): State<AppState>) -> impl IntoResponse {
    let engine = state.script_engine.read().await;
    let players = engine.list_players();
    
    Json(PlayersListResponse { players })
}

/// Handler to get current game state
async fn game_state_handler(State(state): State<AppState>) -> impl IntoResponse {
    let world = state.game_world.read().await;
    let engine = state.script_engine.read().await;
    let players = engine.list_players();
    
    Json(GameStateResponse {
        tick: world.get_tick(),
        players,
    })
}

/// WebSocket handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    log::info!("WebSocket client connected");
    
    // Send welcome message
    let welcome = serde_json::json!({
        "type": "welcome",
        "message": "Connected to GeekCraft server",
        "version": env!("CARGO_PKG_VERSION")
    });
    
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                log::debug!("Received WebSocket message: {}", text);
                
                // Try to parse as JSON command
                if let Ok(command) = serde_json::from_str::<serde_json::Value>(&text) {
                    let response = handle_websocket_command(command, &state).await;
                    
                    if let Ok(response_text) = serde_json::to_string(&response) {
                        let _ = sender.send(Message::Text(response_text)).await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                log::info!("WebSocket client disconnected");
                break;
            }
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Handle WebSocket commands
async fn handle_websocket_command(command: serde_json::Value, state: &AppState) -> serde_json::Value {
    let cmd_type = command.get("type").and_then(|v| v.as_str()).unwrap_or("");
    
    match cmd_type {
        "getPlayers" => {
            let engine = state.script_engine.read().await;
            let players = engine.list_players();
            serde_json::json!({
                "type": "playersResponse",
                "players": players
            })
        }
        "getGameState" => {
            let world = state.game_world.read().await;
            let engine = state.script_engine.read().await;
            let players = engine.list_players();
            serde_json::json!({
                "type": "gameStateResponse",
                "tick": world.get_tick(),
                "players": players
            })
        }
        _ => {
            serde_json::json!({
                "type": "error",
                "message": format!("Unknown command type: {}", cmd_type)
            })
        }
    }
}