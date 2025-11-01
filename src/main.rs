//! GeekCraft - Entry Point
//! 
//! Point d'entrée de l'application. Initialise le serveur et démarre le moteur de jeu.

use geekcraft::{game, network, scripting};
use log::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialiser le logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("🎮 Démarrage de GeekCraft v{}", env!("CARGO_PKG_VERSION"));
    
    // Créer le monde de jeu
    let game_world = Arc::new(RwLock::new(game::world::World::new()));
    info!("✓ Monde de jeu initialisé");
    
    // Créer le moteur de scripting
    let script_engine = Arc::new(RwLock::new(scripting::sandbox::ScriptEngine::new()));
    info!("✓ Moteur de scripting initialisé");
    
    // Démarrer le serveur réseau
    let server_handle = tokio::spawn(async move {
        if let Err(e) = network::server::start_server(game_world.clone(), script_engine.clone()).await {
            error!("❌ Erreur du serveur : {}", e);
        }
    });
    
    info!("✓ Serveur réseau démarré sur http://localhost:3030");
    info!("✓ WebSocket disponible sur ws://localhost:3030/ws");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🚀 GeekCraft est prêt !");
    info!("📚 Consultez les exemples dans /examples");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Attendre que le serveur se termine
    server_handle.await?;
    
    Ok(())
}