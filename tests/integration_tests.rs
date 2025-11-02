// Integration tests for GeekCraft.
// Note: Integration tests are compiled as a separate crate,
// so we must use the crate name as the path root.

use geekcraft::game::world::World;
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