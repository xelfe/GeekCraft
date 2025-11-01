//! GeekCraft - Bibliothèque Principale
//! 
//! Jeu de programmation inspiré de Screeps et Starcraft.
//! Les joueurs programment des bots en JavaScript pour contrôler des unités.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Module de gestion du jeu (monde, entités, simulation)
pub mod game;

/// Module API pour l'interface de scripting
pub mod api;

/// Module réseau (serveur, WebSocket, API REST)
pub mod network;

/// Module de scripting (sandbox JavaScript)
pub mod scripting;

/// Version du jeu
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuration par défaut du serveur
pub mod config {
    /// Port par défaut du serveur
    pub const DEFAULT_PORT: u16 = 3030;
    
    /// Adresse par défaut du serveur
    pub const DEFAULT_HOST: &str = "127.0.0.1";
    
    /// Nombre de ticks par seconde
    pub const TICKS_PER_SECOND: u32 = 60;
    
    /// Timeout maximum pour l'exécution d'un script (ms)
    pub const SCRIPT_TIMEOUT_MS: u64 = 100;
    
    /// Mémoire maximale pour un script (MB)
    pub const SCRIPT_MAX_MEMORY_MB: usize = 128;
}