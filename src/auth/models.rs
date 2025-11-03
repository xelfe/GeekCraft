//! Data models for authentication

use serde::{Deserialize, Serialize};

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: i64,
    /// Username
    pub username: String,
    /// Hashed password (not serialized in responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Account creation timestamp (Unix epoch)
    pub created_at: i64,
}

/// Active session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session token
    pub token: String,
    /// User ID associated with this session
    pub user_id: i64,
    /// Username for quick access
    pub username: String,
    /// Session creation timestamp (Unix epoch)
    pub created_at: i64,
    /// Session expiration timestamp (Unix epoch)
    pub expires_at: i64,
}

/// Registration request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Desired username
    pub username: String,
    /// User password (will be hashed)
    pub password: String,
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// User password
    pub password: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// Whether the operation succeeded
    pub success: bool,
    /// Response message
    pub message: String,
    /// Session token (if successful)
    pub token: Option<String>,
    /// Username (if successful)
    pub username: Option<String>,
}
