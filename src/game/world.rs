//! World module
//! 
//! Manages the game world state, including zones and tick counter.

use std::collections::HashMap;
use crate::game::zone::Zone;

/// Game world containing zones and game state
pub struct World {
    tick: u64,
    /// Map of zone_id to Zone for multi-zone world support
    zones: HashMap<String, Zone>,
}

impl World {
    /// Create a new game world
    pub fn new() -> Self {
        World {
            tick: 0,
            zones: HashMap::new(),
        }
    }

    /// Get the current game tick
    pub fn get_tick(&self) -> u64 {
        self.tick
    }

    /// Add a zone to the world
    pub fn add_zone(&mut self, zone: Zone) {
        self.zones.insert(zone.id.clone(), zone);
    }

    /// Get a zone by ID
    pub fn get_zone(&self, zone_id: &str) -> Option<&Zone> {
        self.zones.get(zone_id)
    }

    /// Get a mutable reference to a zone by ID
    pub fn get_zone_mut(&mut self, zone_id: &str) -> Option<&mut Zone> {
        self.zones.get_mut(zone_id)
    }

    /// Get all zone IDs
    pub fn get_zone_ids(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }

    /// Generate and add a new zone for a player
    pub fn generate_player_zone(&mut self, player_id: &str) -> String {
        let zone_id = format!("player_{}_zone", player_id);
        
        // Use player_id hash as seed for deterministic generation
        let seed = Self::hash_string(&zone_id);
        
        let zone = Zone::generate(zone_id.clone(), seed);
        self.add_zone(zone);
        
        zone_id
    }

    /// Simple string hash function for seed generation
    fn hash_string(s: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}