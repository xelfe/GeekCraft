//! World module - CLEAN VERSION
//!
//! World is the SINGLE SOURCE OF TRUTH for zone management.
//! Handles zone generation, storage, and retrieval.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::game::zone::Zone;
use crate::auth::database::AuthDatabase;
use std::sync::Arc;

/// Game world containing zones and game state
pub struct World {
    tick: u64,
    /// Map of zone_id to Zone - single source of truth
    zones: HashMap<String, Zone>,
    /// Reference to auth database for zone_id updates
    auth_db: Arc<AuthDatabase>,
}

impl World {
    /// Create a new game world with auth database reference
    pub fn new(auth_db: Arc<AuthDatabase>) -> Self {
        World {
            tick: 0,
            zones: HashMap::new(),
            auth_db,
        }
    }

    /// Get the current game tick
    pub fn get_tick(&self) -> u64 {
        self.tick
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

    /// Generate zone ID for a user (deterministic)
    fn generate_zone_id(username: &str, user_id: i64) -> String {
        format!("zone_{}_{}", username, user_id)
    }

    /// Create a deterministic seed from username and user_id
    fn generate_seed(username: &str, user_id: i64) -> u64 {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        user_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Ensure a user has a zone assigned (auto-generate if missing)
    ///
    /// This is the ONLY place where zones are created for users.
    /// Call this when a player connects/spawns.
    pub fn ensure_user_zone(&mut self, user_id: i64, username: &str) -> Result<String, String> {
        // Check if user already has a zone assigned in database
        if let Ok(Some(user)) = self.auth_db.get_user_by_username(username) {
            if let Some(zone_id) = &user.zone_id {
                // User has zone_id in database - check if zone exists in world
                if self.zones.contains_key(zone_id) {
                    log::debug!("User {} already has loaded zone: {}", username, zone_id);
                    return Ok(zone_id.clone());
                } else {
                    // Zone ID exists in DB but not loaded - regenerate it
                    log::info!("Regenerating zone {} for user {}", zone_id, username);
                    let seed = Self::generate_seed(username, user_id);
                    let zone = Zone::generate(zone_id.clone(), seed);
                    self.zones.insert(zone_id.clone(), zone);
                    return Ok(zone_id.clone());
                }
            }
        }

        // No zone assigned - generate new zone
        let zone_id = Self::generate_zone_id(username, user_id);
        let seed = Self::generate_seed(username, user_id);

        log::info!("Generating new zone {} for user {}", zone_id, username);

        // Generate the zone
        let zone = Zone::generate(zone_id.clone(), seed);
        self.zones.insert(zone_id.clone(), zone);

        // Store zone_id in user record
        self.auth_db.update_user_zone(user_id, &zone_id)
            .map_err(|e| format!("Failed to assign zone to user: {}", e))?;

        Ok(zone_id)
    }

    /// Get or create a user's zone (convenience method)
    pub fn get_or_create_user_zone(&mut self, user_id: i64, username: &str) -> Result<&Zone, String> {
        let zone_id = self.ensure_user_zone(user_id, username)?;
        self.zones.get(&zone_id)
            .ok_or_else(|| "Zone was created but not found".to_string())
    }

    /// Remove a zone from the world (for cleanup/unloading)
    pub fn unload_zone(&mut self, zone_id: &str) -> Option<Zone> {
        log::info!("Unloading zone: {}", zone_id);
        self.zones.remove(zone_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::database::{AuthDatabase, DatabaseBackend};

    fn create_test_world() -> World {
        let db = Arc::new(
            AuthDatabase::new(DatabaseBackend::InMemory)
                .expect("Failed to create test database")
        );
        World::new(db)
    }

    #[test]
    fn test_ensure_user_zone_creates_zone() {
        let mut world = create_test_world();

        // Create user in database first
        let password_hash = bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap();
        let user = world.auth_db.create_user("testuser", &password_hash).unwrap();

        // Ensure zone
        let zone_id = world.ensure_user_zone(user.id, "testuser").unwrap();

        // Verify zone exists
        assert!(world.get_zone(&zone_id).is_some());

        // Verify zone_id stored in database
        let updated_user = world.auth_db.get_user_by_username("testuser").unwrap().unwrap();
        assert_eq!(updated_user.zone_id, Some(zone_id));
    }

    #[test]
    fn test_ensure_user_zone_is_idempotent() {
        let mut world = create_test_world();

        let password_hash = bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap();
        let user = world.auth_db.create_user("testuser", &password_hash).unwrap();

        // Call ensure_user_zone twice
        let zone_id_1 = world.ensure_user_zone(user.id, "testuser").unwrap();
        let zone_id_2 = world.ensure_user_zone(user.id, "testuser").unwrap();

        // Should return same zone_id
        assert_eq!(zone_id_1, zone_id_2);

        // Should only have one zone in world
        assert_eq!(world.get_zone_ids().len(), 1);
    }

    #[test]
    fn test_zone_generation_is_deterministic() {
        let zone_id_1 = World::generate_zone_id("testuser", 123);
        let zone_id_2 = World::generate_zone_id("testuser", 123);

        assert_eq!(zone_id_1, zone_id_2);

        let seed_1 = World::generate_seed("testuser", 123);
        let seed_2 = World::generate_seed("testuser", 123);

        assert_eq!(seed_1, seed_2);
    }

    #[test]
    fn test_different_users_get_different_zones() {
        let mut world = create_test_world();

        let password_hash = bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap();
        let user1 = world.auth_db.create_user("user1", &password_hash).unwrap();
        let user2 = world.auth_db.create_user("user2", &password_hash).unwrap();

        let zone_id_1 = world.ensure_user_zone(user1.id, "user1").unwrap();
        let zone_id_2 = world.ensure_user_zone(user2.id, "user2").unwrap();

        assert_ne!(zone_id_1, zone_id_2);
    }
}