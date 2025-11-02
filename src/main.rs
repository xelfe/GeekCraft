//! GeekCraft - Entry Point
//! 
//! Application entry point. Initializes the server and starts the game engine.

use geekcraft::{game, network, scripting, auth};
use log::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("🎮 Starting GeekCraft v{}", env!("CARGO_PKG_VERSION"));
    
    // Choose database backend based on environment variable
    // Options: INMEMORY (default), MONGODB
    let db_backend = std::env::var("GEEKCRAFT_DB_BACKEND")
        .unwrap_or_else(|_| "INMEMORY".to_string());
    
    let backend = match db_backend.to_uppercase().as_str() {
        "MONGODB" => {
            let mongodb_url = std::env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017/geekcraft".to_string());
            info!("🍃 Using MongoDB database at {}", mongodb_url);
            auth::DatabaseBackend::MongoDB(mongodb_url)
        }
        _ => {
            info!("📦 Using In-Memory database (data will be lost on restart)");
            info!("💡 For production, use MongoDB: export GEEKCRAFT_DB_BACKEND=MONGODB");
            auth::DatabaseBackend::InMemory
        }
    };
    
    // Initialize authentication database
    let auth_db = Arc::new(auth::AuthDatabase::new(backend)
        .expect("Failed to initialize authentication database"));
    info!("✓ Authentication database initialized");
    
    // Create authentication service
    let auth_service = Arc::new(auth::AuthService::new(auth_db));
    info!("✓ Authentication service initialized");
    
    // Create game world
    let game_world = Arc::new(RwLock::new(game::world::World::new()));
    info!("✓ Game world initialized");
    
    // Create scripting engine
    let script_engine = Arc::new(RwLock::new(scripting::sandbox::ScriptEngine::new()));
    info!("✓ Scripting engine initialized");
    
    // Start network server
    let server_handle = tokio::spawn(async move {
        if let Err(e) = network::server::start_server(
            game_world.clone(), 
            script_engine.clone(),
            auth_service.clone(),
        ).await {
            error!("❌ Server error: {}", e);
        }
    });
    
    info!("✓ Network server started at http://localhost:3030");
    info!("✓ WebSocket available at ws://localhost:3030/ws");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🚀 GeekCraft is ready!");
    info!("📚 Check out the examples in /examples");
    info!("🔐 Authentication enabled - register to start playing");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Wait for server to finish
    server_handle.await?;
    
    Ok(())
}