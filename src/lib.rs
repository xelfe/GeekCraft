//! GeekCraft - Main Library
//! 
//! Programming game inspired by Screeps and Starcraft.
//! Players program JavaScript bots to control units.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Game management module (world, campaign, zones)
pub mod game;

/// Network module (server, WebSocket, REST API)
pub mod network;

/// Scripting module (JavaScript sandbox)
pub mod scripting;

/// Authentication module (user management, sessions)
pub mod auth;

/// Game version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default server configuration
pub mod config {
    /// Default server port
    pub const DEFAULT_PORT: u16 = 3030;
    
    /// Default server address
    pub const DEFAULT_HOST: &str = "127.0.0.1";
    
    /// Number of ticks per second
    pub const TICKS_PER_SECOND: u32 = 60;
    
    /// Maximum timeout for script execution (ms)
    pub const SCRIPT_TIMEOUT_MS: u64 = 100;
    
    /// Maximum memory for a script (MB)
    pub const SCRIPT_MAX_MEMORY_MB: usize = 128;
}