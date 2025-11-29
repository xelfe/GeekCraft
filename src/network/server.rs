//! Network server module
//!
//! Manages HTTP/WebSocket communication, REST API endpoints, and client connections.

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
use crate::game::map_generator::{GameMap, ClusteringConfig};
use crate::scripting::sandbox::ScriptEngine;
use crate::auth::AuthService;
use crate::auth::models::{RegisterRequest, LoginRequest};
use crate::network::campaign_routes::{
    start_run_handler,
    get_run_state_handler,
    stop_run_handler,
    save_run_handler,
    list_saves_handler,
    load_run_handler,
};
use crate::network::zone_routes::{
    generate_zone_handler,
    get_zone_handler,
    list_zones_handler,
};

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

// ============================================================================
// Map Generation API Types
// ============================================================================

/// Request structure for map generation
#[derive(Debug, Deserialize)]
pub struct MapGenerationRequest {
    /// Width of the map (10-200)
    pub width: usize,
    /// Height of the map (10-200)
    pub height: usize,
    /// Game mode: "default", "solo", or "online"
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "default".to_string()
}

/// Response structure for map generation
#[derive(Debug, Serialize)]
pub struct MapGenerationResponse {
    pub success: bool,
    pub map: Option<serde_json::Value>,
    pub error: Option<String>,
}

// ============================================================================
// Start Server
// ============================================================================

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
        // Campaign endpoints (no auth required for now)
        .route("/api/campaign/start", post(start_run_handler))
        .route("/api/campaign/state", get(get_run_state_handler))
        .route("/api/campaign/stop", post(stop_run_handler))
        .route("/api/campaign/save", post(save_run_handler))
        .route("/api/campaign/saves", get(list_saves_handler))
        .route("/api/campaign/load", post(load_run_handler))
        // Zone endpoints (no auth required for now)
        .route("/api/zone/generate", post(generate_zone_handler))
        .route("/api/zone/:zone_id", get(get_zone_handler))
        .route("/api/zones", get(list_zones_handler))
        // Map generation endpoints (public for now)
        .route("/api/map/generate", post(generate_map_handler))
        .route("/api/map/test", get(test_map_handler))
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
    log::info!("  - POST /api/campaign/start");
    log::info!("  - GET  /api/campaign/state");
    log::info!("  - POST /api/campaign/stop");
    log::info!("  - POST /api/campaign/save");
    log::info!("  - GET  /api/campaign/saves");
    log::info!("  - POST /api/campaign/load");
    log::info!("  - POST /api/zone/generate");
    log::info!("  - GET  /api/zone/:zone_id");
    log::info!("  - GET  /api/zones");
    log::info!("  - POST /api/map/generate");
    log::info!("  - GET  /api/map/test");

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// Map Generation Handlers
// ============================================================================

/// Generate a new map with improved clustering
/// POST /api/map/generate
///
/// Request body:
/// ```json
/// {
///   "width": 80,
///   "height": 60,
///   "mode": "solo"
/// }
/// ```
async fn generate_map_handler(
    State(_state): State<AppState>,
    Json(payload): Json<MapGenerationRequest>,
) -> impl IntoResponse {
    // Validate input dimensions
    if payload.width < 10 || payload.width > 200 {
        return (
            StatusCode::BAD_REQUEST,
            Json(MapGenerationResponse {
                success: false,
                map: None,
                error: Some("Width must be between 10 and 200".to_string()),
            })
        );
    }

    if payload.height < 10 || payload.height > 200 {
        return (
            StatusCode::BAD_REQUEST,
            Json(MapGenerationResponse {
                success: false,
                map: None,
                error: Some("Height must be between 10 and 200".to_string()),
            })
        );
    }

    // Select clustering configuration based on mode
    let config = match payload.mode.as_str() {
        "solo" => ClusteringConfig::solo_campaign(),
        "online" => ClusteringConfig::online_mode(),
        _ => ClusteringConfig::default(),
    };

    log::info!(
        "Generating {}x{} map in {} mode",
        payload.width,
        payload.height,
        payload.mode
    );

    // Generate the map
    let map = GameMap::generate(payload.width, payload.height, config);

    (
        StatusCode::OK,
        Json(MapGenerationResponse {
            success: true,
            map: Some(map.to_json()),
            error: None,
        })
    )
}

/// Test endpoint for map generation
/// GET /api/map/test
async fn test_map_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "message": "GeekCraft Map Generation API is running",
        "version": env!("CARGO_PKG_VERSION"),
        "modes": ["default", "solo", "online"],
        "dimensions": {
            "min": 10,
            "max": 200
        }
    }))
}

// ============================================================================
// Authentication Middleware
// ============================================================================

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
        || path == "/ws"
        || path.starts_with("/api/campaign/")
        || path.starts_with("/api/zone")
        || path.starts_with("/api/map") {
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

// ============================================================================
// Authentication Handlers
// ============================================================================

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

// ============================================================================
// Basic Handlers
// ============================================================================

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
            "websocket": "WS /ws",
            "campaign_start": "POST /api/campaign/start",
            "campaign_state": "GET /api/campaign/state",
            "campaign_stop": "POST /api/campaign/stop",
            "campaign_save": "POST /api/campaign/save",
            "campaign_saves": "GET /api/campaign/saves",
            "campaign_load": "POST /api/campaign/load",
            "map_generate": "POST /api/map/generate",
            "map_test": "GET /api/map/test"
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

// ============================================================================
// Game Handlers
// ============================================================================

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

// ============================================================================
// WebSocket Handlers
// ============================================================================

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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    // Remove dependency on tower::util::ServiceExt
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::Service;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use axum::{Router, routing::post};
    use crate::game::world::World;
    use crate::scripting::sandbox::ScriptEngine;

    #[tokio::test]
    async fn test_map_generation_valid() {
        let app_state = create_test_app_state();
        let app = Router::new()
            .route("/api/map/generate", post(generate_map_handler))
            .with_state(app_state);
        let request_body = serde_json::json!({
            "width": 50,
            "height": 50,
            "mode": "default"
        });
        let mut app = app.into_service();
        let request = Request::builder()
            .method("POST")
            .uri("/api/map/generate")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();
        let response = app.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_map_generation_invalid_dimensions() {
        let app_state = create_test_app_state();
        let app = Router::new()
            .route("/api/map/generate", post(generate_map_handler))
            .with_state(app_state);
        let request_body = serde_json::json!({
            "width": 5,
            "height": 50,
            "mode": "default"
        });
        let mut app = app.into_service();
        let request = Request::builder()
            .method("POST")
            .uri("/api/map/generate")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();
        let response = app.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    fn create_test_app_state() -> AppState {
        use crate::auth::DatabaseBackend;
        let game_world = Arc::new(tokio::sync::RwLock::new(World::new()));
        let script_engine = Arc::new(tokio::sync::RwLock::new(ScriptEngine::new()));
        let auth_db = Arc::new(
            crate::auth::AuthDatabase::new(DatabaseBackend::InMemory)
                .expect("Failed to create test auth database")
        );
        let auth_service = Arc::new(crate::auth::AuthService::new(auth_db));
        AppState {
            game_world,
            script_engine,
            auth_service,
        }
    }
}