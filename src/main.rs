//! GeekCraft - Entry Point
//! 
//! Application entry point. Initializes the server and starts the game engine.

use geekcraft::{game, network, scripting};
use log::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("🎮 Starting GeekCraft v{}", env!("CARGO_PKG_VERSION"));
    
    // Create game world
    let game_world = Arc::new(RwLock::new(game::world::World::new()));
    info!("✓ Game world initialized");
    
    // Create scripting engine
    let script_engine = Arc::new(RwLock::new(scripting::sandbox::ScriptEngine::new()));
    info!("✓ Scripting engine initialized");
    
    // Start network server
    let server_handle = tokio::spawn(async move {
        if let Err(e) = network::server::start_server(game_world.clone(), script_engine.clone()).await {
            error!("❌ Server error: {}", e);
        }
    });
    
    info!("✓ Network server started at http://localhost:3030");
    info!("✓ WebSocket available at ws://localhost:3030/ws");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🚀 GeekCraft is ready!");
    info!("📚 Check out the examples in /examples");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Wait for server to finish
    server_handle.await?;
    
    Ok(())
}