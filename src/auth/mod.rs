//! Authentication module
//! 
//! Provides user authentication and session management.
//! Supports multiple database backends (In-Memory for development, MongoDB for production).

pub mod models;
pub mod service;
pub mod database;

pub use models::{User, Session};
pub use service::AuthService;
pub use database::{AuthDatabase, DatabaseBackend};
