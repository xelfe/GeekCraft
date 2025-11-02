//! Authentication module
//! 
//! Provides user authentication and session management.
//! Supports multiple database backends (SQLite for development, PostgreSQL/MySQL for production).

pub mod models;
pub mod service;
pub mod database;

pub use models::{User, Session};
pub use service::AuthService;
pub use database::{AuthDatabase, DatabaseBackend};
