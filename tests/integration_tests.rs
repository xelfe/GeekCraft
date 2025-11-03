// Integration tests for GeekCraft.
// Note: Integration tests are compiled as a separate crate,
// so we must use the crate name as the path root.

use geekcraft::game::world::World;
use geekcraft::game::zone::{Zone, SurfaceType, ZONE_SIZE};
use geekcraft::auth::{AuthDatabase, DatabaseBackend};

#[test]
fn test_game_world_initialization() {
    // Create a new world and verify initial state
    let world = World::new();
    assert_eq!(world.get_tick(), 0, "Newly created world should start at tick 0");
}

#[test]
fn test_auth_database_inmemory() {
    // Test In-Memory database backend
    let db = AuthDatabase::new(DatabaseBackend::InMemory)
        .expect("Failed to create In-Memory database");
    
    // Test user creation
    let user = db.create_user("testuser", "hashed_password")
        .expect("Failed to create user");
    
    assert_eq!(user.username, "testuser");
    assert_eq!(user.password_hash, "hashed_password");
    assert!(user.id > 0);
    
    // Test user retrieval
    let retrieved = db.get_user_by_username("testuser")
        .expect("Failed to get user")
        .expect("User not found");
    
    assert_eq!(retrieved.username, user.username);
    assert_eq!(retrieved.id, user.id);
    
    // Test duplicate user prevention
    let duplicate_result = db.create_user("testuser", "other_hash");
    assert!(duplicate_result.is_err(), "Should not allow duplicate usernames");
    
    // Test session creation
    let token = "test-token-123";
    let expires_at = user.created_at + 3600; // 1 hour from creation
    
    db.create_session(token, user.id, expires_at)
        .expect("Failed to create session");
    
    // Test session retrieval
    let session = db.get_session(token)
        .expect("Failed to get session")
        .expect("Session not found");
    
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.username, user.username);
    
    // Test session deletion
    db.delete_session(token)
        .expect("Failed to delete session");
    
    let deleted_session = db.get_session(token)
        .expect("Failed to check deleted session");
    
    assert!(deleted_session.is_none(), "Session should be deleted");
}

#[test]
fn test_zone_generation_and_world_integration() {
    let mut world = World::new();
    
    // Generate a zone for a player
    let zone_id = world.generate_player_zone("player1");
    
    // Verify zone was created
    assert_eq!(zone_id, "player_player1_zone");
    
    // Retrieve the zone
    let zone = world.get_zone(&zone_id)
        .expect("Zone should exist in world");
    
    // Verify zone properties
    assert_eq!(zone.id, zone_id);
    assert_eq!(zone.tiles.len(), ZONE_SIZE);
    assert_eq!(zone.tiles[0].len(), ZONE_SIZE);
    assert!(zone.exits.len() >= 2 && zone.exits.len() <= 4);
    
    // Verify zone has all surface types
    let has_plain = zone.count_surface_type(SurfaceType::Plain) > 0;
    let has_swamp = zone.count_surface_type(SurfaceType::Swamp) > 0;
    let has_obstacle = zone.count_surface_type(SurfaceType::Obstacle) > 0;
    
    assert!(has_plain, "Zone should have Plain tiles");
    assert!(has_swamp, "Zone should have Swamp tiles");
    assert!(has_obstacle, "Zone should have Obstacle tiles");
}

#[test]
fn test_multiple_zones_in_world() {
    let mut world = World::new();
    
    // Generate zones for multiple players
    let zone1_id = world.generate_player_zone("player1");
    let zone2_id = world.generate_player_zone("player2");
    let zone3_id = world.generate_player_zone("player3");
    
    // Verify all zones exist
    assert!(world.get_zone(&zone1_id).is_some());
    assert!(world.get_zone(&zone2_id).is_some());
    assert!(world.get_zone(&zone3_id).is_some());
    
    // Verify zone IDs are listed
    let zone_ids = world.get_zone_ids();
    assert_eq!(zone_ids.len(), 3);
    assert!(zone_ids.contains(&zone1_id));
    assert!(zone_ids.contains(&zone2_id));
    assert!(zone_ids.contains(&zone3_id));
}

#[test]
fn test_zone_deterministic_for_same_player() {
    let mut world1 = World::new();
    let mut world2 = World::new();
    
    // Generate zone for same player in different worlds
    let zone1_id = world1.generate_player_zone("player1");
    let zone2_id = world2.generate_player_zone("player1");
    
    let zone1 = world1.get_zone(&zone1_id).unwrap();
    let zone2 = world2.get_zone(&zone2_id).unwrap();
    
    // Zones should be identical for same player ID
    assert_eq!(zone1.tiles[0][0].surface_type, zone2.tiles[0][0].surface_type);
    assert_eq!(zone1.tiles[15][15].surface_type, zone2.tiles[15][15].surface_type);
    assert_eq!(zone1.exits.len(), zone2.exits.len());
}