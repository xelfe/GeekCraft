// This file manages network communication, allowing clients to connect and interact with the server.

use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, Json,
    http::{Request, StatusCode},
    middleware::{self, Next},
};
use axum::extract::ws::{WebSocket, Message};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use serde::{Deserialize, Serialize};
use futures_util::{SinkExt, StreamExt};

use crate::game::world::World;
use crate::scripting::sandbox::ScriptEngine;
use crate::auth::AuthService;
use crate::auth::models::{RegisterRequest, LoginRequest};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub game_world: Arc<RwLock<World>>,
    pub script_engine: Arc<RwLock<ScriptEngine>>,
    pub auth_service: Arc<AuthService>,
}

/// Request to submit player code
#[derive(Debug, Deserialize)]
pub struct CodeSubmission {
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
pub async fn start_server(
    game_world: Arc<RwLock<World>>, 
    script_engine: Arc<RwLock<ScriptEngine>>,
    auth_service: Arc<AuthService>,
) -> anyhow::Result<()> {
    let app_state = AppState {
        game_world,
        script_engine,
        auth_service,
    };

    // Build the router with all endpoints
    let app = Router::new()
        // Public endpoints (no auth required)
        .route("/", get(root_handler))
        .route("/api/health", get(health_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        // Protected endpoints (auth required)
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/submit", post(submit_code_handler))
        .route("/api/players", get(list_players_handler))
        .route("/api/gamestate", get(game_state_handler))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware))
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
    log::info!("  - POST /api/auth/register");
    log::info!("  - POST /api/auth/login");
    log::info!("  - POST /api/auth/logout (requires auth)");
    log::info!("  - POST /api/submit (requires auth)");
    log::info!("  - GET  /api/players (requires auth)");
    log::info!("  - GET  /api/gamestate (requires auth)");

    // Start the server
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Authentication middleware
async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public endpoints
    let path = request.uri().path();
    if path == "/" 
        || path == "/api/health" 
        || path == "/api/auth/register" 
        || path == "/api/auth/login" 
        || path == "/ws" {
        return Ok(next.run(request).await);
    }
    
    // Get Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());
    
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ")
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Validate token
    match state.auth_service.validate_token(token) {
        Some(session) => {
            // Add user info to request extensions
            request.extensions_mut().insert(session);
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Register handler
async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let response = state.auth_service.register(&payload.username, &payload.password);
    Json(response)
}

/// Login handler
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let response = state.auth_service.login(&payload.username, &payload.password);
    Json(response)
}

/// Logout handler
async fn logout_handler(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
) -> impl IntoResponse {
    // Get token from Authorization header
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    
    let response = state.auth_service.logout(token);
    Json(response)
}

/// Root handler - provides API information
async fn root_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "GeekCraft API Server",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health": "GET /api/health",
            "register": "POST /api/auth/register",
            "login": "POST /api/auth/login",
            "logout": "POST /api/auth/logout (requires auth)",
            "submit_code": "POST /api/submit (requires auth)",
            "list_players": "GET /api/players (requires auth)",
            "game_state": "GET /api/gamestate (requires auth)",
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
    request: Request<axum::body::Body>,
) -> impl IntoResponse {
    // Get session from request extensions
    let session = request.extensions().get::<crate::auth::models::Session>().cloned();
    
    let player_id = match session {
        Some(s) => s.username,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(CodeSubmissionResponse {
                    success: false,
                    message: "Unauthorized".to_string(),
                })
            );
        }
    };
    
    // Parse request body with size limit (1MB for code submissions)
    let bytes = match axum::body::to_bytes(request.into_body(), 1_048_576).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CodeSubmissionResponse {
                    success: false,
                    message: format!("Failed to read request body: {}", e),
                })
            );
        }
    };
    
    let payload: CodeSubmission = match serde_json::from_slice(&bytes) {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CodeSubmissionResponse {
                    success: false,
                    message: format!("Invalid JSON: {}", e),
                })
            );
        }
    };
    
    log::info!("Received code submission from player: {}", player_id);
    
    let mut engine = state.script_engine.write().await;
    
    match engine.submit_code(player_id.clone(), payload.code) {
        Ok(()) => (
            StatusCode::OK,
            Json(CodeSubmissionResponse {
                success: true,
                message: format!("Code submitted successfully for player {}", player_id),
            })
        ),
        Err(err) => {
            log::warn!("Code submission failed: {}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(CodeSubmissionResponse {
                    success: false,
                    message: err,
                })
            )
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

/// WebSocket handler - now supports authentication
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection with authentication support
async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    log::info!("WebSocket client connected");
    
    // Track authenticated session
    let mut authenticated_session: Option<crate::auth::models::Session> = None;
    
    // Send welcome message
    let welcome = serde_json::json!({
        "type": "welcome",
        "message": "Connected to GeekCraft server. Send auth command to authenticate.",
        "version": env!("CARGO_PKG_VERSION"),
        "requiresAuth": true
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
                    let response = handle_websocket_command(
                        command,
                        &state,
                        &mut authenticated_session
                    ).await;
                    
                    if let Ok(response_text) = serde_json::to_string(&response) {
                        let _ = sender.send(Message::Text(response_text)).await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                if let Some(session) = &authenticated_session {
                    log::info!("WebSocket client {} disconnected", session.username);
                } else {
                    log::info!("WebSocket client disconnected");
                }
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

/// Handle WebSocket commands with authentication support
async fn handle_websocket_command(
    command: serde_json::Value, 
    state: &AppState,
    authenticated_session: &mut Option<crate::auth::models::Session>,
) -> serde_json::Value {
    let cmd_type = command.get("type").and_then(|v| v.as_str()).unwrap_or("");
    
    match cmd_type {
        "auth" => {
            // Authenticate via WebSocket
            let token = command.get("token").and_then(|v| v.as_str()).unwrap_or("");
            
            match state.auth_service.validate_token(token) {
                Some(session) => {
                    let username = session.username.clone();
                    *authenticated_session = Some(session);
                    serde_json::json!({
                        "type": "authResponse",
                        "success": true,
                        "username": username
                    })
                }
                None => {
                    serde_json::json!({
                        "type": "authResponse",
                        "success": false,
                        "message": "Invalid or expired token"
                    })
                }
            }
        }
        "getPlayers" => {
            // Require authentication
            if authenticated_session.is_none() {
                return serde_json::json!({
                    "type": "error",
                    "message": "Authentication required. Send auth command first."
                });
            }
            
            let engine = state.script_engine.read().await;
            let players = engine.list_players();
            serde_json::json!({
                "type": "playersResponse",
                "players": players
            })
        }
        "getGameState" => {
            // Require authentication
            if authenticated_session.is_none() {
                return serde_json::json!({
                    "type": "error",
                    "message": "Authentication required. Send auth command first."
                });
            }
            
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