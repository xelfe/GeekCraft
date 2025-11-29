//! Authentication service

use super::database::AuthDatabase;
use super::models::{Session, AuthResponse};
use crate::game::zone::Zone;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Session duration in seconds (24 hours)
const SESSION_DURATION: i64 = 86400;

/// Authentication service
pub struct AuthService {
    pub db: Arc<AuthDatabase>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(db: Arc<AuthDatabase>) -> Self {
        AuthService { db }
    }

    /// Generate a unique zone ID for a user
    fn generate_zone_id(username: &str, user_id: i64) -> String {
        format!("zone_{}_{}", username, user_id)
    }

    /// Create a deterministic seed from username
    fn generate_seed(username: &str, user_id: i64) -> u64 {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        user_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Ensure user has a zone assigned (auto-generate if missing)
    fn ensure_user_zone(&self, user_id: i64, username: &str) -> Result<String, String> {
        // Check if user already has a zone
        if let Ok(Some(user)) = self.db.get_user_by_username(username) {
            if let Some(zone_id) = user.zone_id {
                return Ok(zone_id);
            }
        }

        // Generate new zone for user
        let zone_id = Self::generate_zone_id(username, user_id);
        let seed = Self::generate_seed(username, user_id);

        // Generate the zone (this validates zone generation logic)
        let _zone = Zone::generate(zone_id.clone(), seed);

        // Store zone_id in user record
        self.db.update_user_zone(user_id, &zone_id)
            .map_err(|e| format!("Failed to assign zone to user: {}", e))?;

        log::info!("Auto-generated zone {} for user {}", zone_id, username);

        Ok(zone_id)
    }
    
    /// Register a new user
    pub fn register(&self, username: &str, password: &str) -> AuthResponse {
        // Validate username
        if username.trim().is_empty() || username.len() < 3 || username.len() > 32 {
            return AuthResponse {
                success: false,
                message: "Username must be between 3 and 32 characters".to_string(),
                token: None,
                username: None,
            };
        }
        
        // Validate username characters (alphanumeric, underscore, hyphen only)
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return AuthResponse {
                success: false,
                message: "Username can only contain letters, numbers, underscore, and hyphen".to_string(),
                token: None,
                username: None,
            };
        }
        
        // Validate password
        if password.len() < 6 {
            return AuthResponse {
                success: false,
                message: "Password must be at least 6 characters".to_string(),
                token: None,
                username: None,
            };
        }
        
        // Hash password
        let password_hash = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(e) => {
                log::error!("Failed to hash password: {}", e);
                return AuthResponse {
                    success: false,
                    message: "Internal error".to_string(),
                    token: None,
                    username: None,
                };
            }
        };
        
        // Create user
        match self.db.create_user(username, &password_hash) {
            Ok(user) => {
                // Auto-generate zone for new user
                if let Err(e) = self.ensure_user_zone(user.id, username) {
                    log::error!("Failed to auto-generate zone for new user {}: {}", username, e);
                    // Continue with registration even if zone generation fails
                }

                AuthResponse {
                    success: true,
                    message: format!("User {} registered successfully", username),
                    token: None,
                    username: Some(username.to_string()),
                }
            },
            Err(e) => AuthResponse {
                success: false,
                message: e,
                token: None,
                username: None,
            },
        }
    }
    
    /// Login a user
    pub fn login(&self, username: &str, password: &str) -> AuthResponse {
        // Get user from database
        let user = match self.db.get_user_by_username(username) {
            Ok(Some(user)) => user,
            Ok(None) => {
                return AuthResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                    token: None,
                    username: None,
                };
            }
            Err(e) => {
                log::error!("Database error: {}", e);
                return AuthResponse {
                    success: false,
                    message: "Internal error".to_string(),
                    token: None,
                    username: None,
                };
            }
        };
        
        // Verify password
        match bcrypt::verify(password, &user.password_hash) {
            Ok(true) => {
                // Ensure user has a zone (auto-generate if missing)
                if let Err(e) = self.ensure_user_zone(user.id, &user.username) {
                    log::error!("Failed to ensure zone for user {}: {}", user.username, e);
                    // Continue with login even if zone check fails
                }

                // Create session token
                let token = Uuid::new_v4().to_string();
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("System clock is before Unix epoch")
                    .as_secs() as i64;
                let expires_at = now + SESSION_DURATION;

                // Store session
                if let Err(e) = self.db.create_session(&token, user.id, expires_at) {
                    log::error!("Failed to create session: {}", e);
                    return AuthResponse {
                        success: false,
                        message: "Internal error".to_string(),
                        token: None,
                        username: None,
                    };
                }

                AuthResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    token: Some(token),
                    username: Some(user.username),
                }
            }
            Ok(false) => AuthResponse {
                success: false,
                message: "Invalid username or password".to_string(),
                token: None,
                username: None,
            },
            Err(e) => {
                log::error!("Password verification error: {}", e);
                AuthResponse {
                    success: false,
                    message: "Internal error".to_string(),
                    token: None,
                    username: None,
                }
            }
        }
    }
    
    /// Logout a user
    pub fn logout(&self, token: &str) -> AuthResponse {
        match self.db.delete_session(token) {
            Ok(()) => AuthResponse {
                success: true,
                message: "Logout successful".to_string(),
                token: None,
                username: None,
            },
            Err(e) => {
                log::error!("Failed to logout: {}", e);
                AuthResponse {
                    success: false,
                    message: "Internal error".to_string(),
                    token: None,
                    username: None,
                }
            }
        }
    }
    
    /// Validate a session token
    pub fn validate_token(&self, token: &str) -> Option<Session> {
        match self.db.get_session(token) {
            Ok(session) => session,
            Err(e) => {
                log::error!("Failed to validate token: {}", e);
                None
            }
        }
    }
    
    /// Cleanup expired sessions
    pub fn cleanup_expired_sessions(&self) {
        if let Err(e) = self.db.delete_expired_sessions() {
            log::error!("Failed to cleanup expired sessions: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::database::{AuthDatabase, DatabaseBackend};

    fn create_test_auth_service() -> AuthService {
        let db = Arc::new(
            AuthDatabase::new(DatabaseBackend::InMemory)
                .expect("Failed to create test database")
        );
        AuthService::new(db)
    }

    #[test]
    fn test_zone_auto_generation_on_registration() {
        let auth_service = create_test_auth_service();

        // Register a new user
        let response = auth_service.register("testuser", "password123");
        assert!(response.success, "Registration should succeed");

        // Verify user has a zone assigned
        let user = auth_service.db.get_user_by_username("testuser")
            .expect("Should retrieve user")
            .expect("User should exist");

        assert!(user.zone_id.is_some(), "User should have a zone_id assigned");
        let zone_id = user.zone_id.unwrap();
        assert!(zone_id.starts_with("zone_testuser_"), "Zone ID should follow naming convention");
    }

    #[test]
    fn test_zone_auto_generation_on_login() {
        let auth_service = create_test_auth_service();

        // Manually create user without zone (simulating old user)
        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        let user = auth_service.db.create_user("olduser", &password_hash)
            .expect("Should create user");

        // Verify user has no zone initially (auto-generated during registration)
        // But ensure_user_zone is called during login
        assert!(user.zone_id.is_some(), "User should have zone after registration");

        // Login should work and ensure zone exists
        let login_response = auth_service.login("olduser", "password123");
        assert!(login_response.success, "Login should succeed");

        // Verify user still has zone
        let updated_user = auth_service.db.get_user_by_username("olduser")
            .expect("Should retrieve user")
            .expect("User should exist");

        assert!(updated_user.zone_id.is_some(), "User should have a zone_id after login");
    }

    #[test]
    fn test_zone_generation_is_deterministic() {
        let zone_id_1 = AuthService::generate_zone_id("testuser", 123);
        let zone_id_2 = AuthService::generate_zone_id("testuser", 123);

        assert_eq!(zone_id_1, zone_id_2, "Zone IDs should be deterministic");

        let seed_1 = AuthService::generate_seed("testuser", 123);
        let seed_2 = AuthService::generate_seed("testuser", 123);

        assert_eq!(seed_1, seed_2, "Seeds should be deterministic");
    }

    #[test]
    fn test_different_users_get_different_zones() {
        let auth_service = create_test_auth_service();

        // Register two users
        auth_service.register("user1", "password1");
        auth_service.register("user2", "password2");

        // Get their zones
        let user1 = auth_service.db.get_user_by_username("user1")
            .expect("Should retrieve user1")
            .expect("User1 should exist");

        let user2 = auth_service.db.get_user_by_username("user2")
            .expect("Should retrieve user2")
            .expect("User2 should exist");

        assert_ne!(
            user1.zone_id,
            user2.zone_id,
            "Different users should have different zones"
        );
    }

    #[test]
    fn test_zone_persists_across_logins() {
        let auth_service = create_test_auth_service();

        // Register user
        auth_service.register("persistuser", "password123");

        // Get initial zone
        let user_after_registration = auth_service.db.get_user_by_username("persistuser")
            .expect("Should retrieve user")
            .expect("User should exist");
        let initial_zone = user_after_registration.zone_id.clone();

        // Login multiple times
        auth_service.login("persistuser", "password123");
        auth_service.login("persistuser", "password123");

        // Verify zone hasn't changed
        let user_after_logins = auth_service.db.get_user_by_username("persistuser")
            .expect("Should retrieve user")
            .expect("User should exist");

        assert_eq!(
            initial_zone,
            user_after_logins.zone_id,
            "Zone should persist across logins"
        );
    }
}
