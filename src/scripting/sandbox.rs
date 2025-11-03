//! Secure sandbox environment for executing player scripts
//! 
//! Provides isolation for player code from the rest of the system.

use std::collections::HashMap;

/// Script execution sandbox
pub struct Sandbox {
    /// Variables accessible in the sandbox
    variables: HashMap<String, f64>,
    /// Player code submissions (player_id -> code)
    codes: HashMap<String, String>,
}

/// Type alias for ScriptEngine
pub type ScriptEngine = Sandbox;

impl Sandbox {
    /// Create a new sandbox
    pub fn new() -> Self {
        Sandbox {
            variables: HashMap::new(),
            codes: HashMap::new(),
        }
    }

    /// Set a variable in the sandbox
    pub fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    /// Get a variable from the sandbox
    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }

    /// Submit player code
    pub fn submit_code(&mut self, player_id: String, code: String) -> Result<(), String> {
        const MAX_CODE_LENGTH: usize = 1_000_000; // 1MB limit
        
        if code.len() > MAX_CODE_LENGTH {
            return Err(format!("Code too large: {} bytes (max: {} bytes)", code.len(), MAX_CODE_LENGTH));
        }
        
        if player_id.trim().is_empty() {
            return Err("Player ID cannot be empty".to_string());
        }
        
        self.codes.insert(player_id, code);
        Ok(())
    }

    /// Get player code
    pub fn get_code(&self, player_id: &str) -> Option<&String> {
        self.codes.get(player_id)
    }

    /// List all players with submitted code
    pub fn list_players(&self) -> Vec<String> {
        self.codes.keys().cloned().collect()
    }

    /// Execute a script in the sandbox
    pub fn execute_script(&self, _script: &str) -> Result<(), String> {
        // Placeholder for future script execution logic
        Ok(())
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}