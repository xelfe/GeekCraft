//! Authentication service

use super::database::AuthDatabase;
use super::models::{Session, AuthResponse};
use uuid::Uuid;
use std::sync::Arc;

/// Session duration in seconds (24 hours)
const SESSION_DURATION: i64 = 86400;

/// Authentication service
pub struct AuthService {
    db: Arc<AuthDatabase>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(db: Arc<AuthDatabase>) -> Self {
        AuthService { db }
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
            Ok(_) => AuthResponse {
                success: true,
                message: format!("User {} registered successfully", username),
                token: None,
                username: Some(username.to_string()),
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
