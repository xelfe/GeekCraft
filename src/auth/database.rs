//! Database abstraction for authentication
//! 
//! Provides a trait-based abstraction that supports multiple database backends:
//! - **In-Memory**: Fast, for testing and development (data lost on restart)
//! - **SQLite**: Simple, file-based, perfect for development and single-server deployments
//! - **Redis**: Fast NoSQL, perfect for production MMO servers (requires Redis server)
//! 
//! **Recommendations:**
//! - **Development**: Use SQLite or In-Memory
//! - **Production (Single Server)**: Use SQLite or Redis
//! - **Production (MMO/Multi-Server)**: Use Redis (horizontal scaling, high concurrency)
//! 
//! Users can easily switch between backends by changing configuration.

use super::models::{User, Session};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Database backend configuration
#[derive(Debug, Clone)]
pub enum DatabaseBackend {
    /// In-memory database (recommended for testing)
    /// - Pros: Fast, no setup required
    /// - Cons: Data lost on restart
    /// - Use case: Testing, development
    InMemory,
    
    /// SQLite database (recommended for development and small deployments)
    /// - Pros: Simple, file-based, no external server needed
    /// - Cons: Limited concurrent writes (~1000 users max)
    /// - Use case: Single server, development, small deployments
    #[cfg(feature = "sqlite")]
    SQLite(String),
    
    /// Redis database (recommended for production MMO servers)
    /// - Pros: Very fast (in-memory), horizontal scaling, high concurrency
    /// - Cons: Requires Redis server, data in RAM
    /// - Use case: Production MMO, multiple servers, >1000 concurrent users
    #[cfg(feature = "redis_backend")]
    Redis(String), // Connection string: "redis://127.0.0.1:6379"
}

/// Authentication database trait
/// Implement this trait to add support for new database backends
pub trait AuthDatabaseTrait: Send + Sync {
    fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String>;
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String>;
    fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String>;
    fn get_session(&self, token: &str) -> Result<Option<Session>, String>;
    fn delete_session(&self, token: &str) -> Result<(), String>;
    fn delete_expired_sessions(&self) -> Result<(), String>;
}

/// Main authentication database wrapper
pub struct AuthDatabase {
    backend: Box<dyn AuthDatabaseTrait>,
}

impl AuthDatabase {
    /// Create a new database with the specified backend
    pub fn new(backend: DatabaseBackend) -> Result<Self, String> {
        let db: Box<dyn AuthDatabaseTrait> = match backend {
            DatabaseBackend::InMemory => Box::new(InMemoryBackend::new()),
            
            #[cfg(feature = "sqlite")]
            DatabaseBackend::SQLite(path) => Box::new(SQLiteBackend::new(&path)?),
            
            #[cfg(feature = "redis_backend")]
            DatabaseBackend::Redis(url) => Box::new(RedisBackend::new(&url)?),
        };
        
        Ok(AuthDatabase { backend: db })
    }
    
    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String> {
        self.backend.create_user(username, password_hash)
    }
    
    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        self.backend.get_user_by_username(username)
    }
    
    pub fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String> {
        self.backend.create_session(token, user_id, expires_at)
    }
    
    pub fn get_session(&self, token: &str) -> Result<Option<Session>, String> {
        self.backend.get_session(token)
    }
    
    pub fn delete_session(&self, token: &str) -> Result<(), String> {
        self.backend.delete_session(token)
    }
    
    pub fn delete_expired_sessions(&self) -> Result<(), String> {
        self.backend.delete_expired_sessions()
    }
}

// ============================================================================
// In-Memory Backend Implementation
// ============================================================================

struct InMemoryBackend {
    users: Arc<Mutex<HashMap<String, User>>>,
    users_by_id: Arc<Mutex<HashMap<i64, User>>>,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    next_user_id: Arc<Mutex<i64>>,
}

impl InMemoryBackend {
    fn new() -> Self {
        InMemoryBackend {
            users: Arc::new(Mutex::new(HashMap::new())),
            users_by_id: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_user_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl AuthDatabaseTrait for InMemoryBackend {
    fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();
        
        if users.contains_key(username) {
            return Err("Username already exists".to_string());
        }
        
        let mut next_id = self.next_user_id.lock().unwrap();
        let user_id = *next_id;
        *next_id += 1;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let user = User {
            id: user_id,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
        };
        
        users.insert(username.to_string(), user.clone());
        self.users_by_id.lock().unwrap().insert(user_id, user.clone());
        
        Ok(user)
    }
    
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.get(username).cloned())
    }
    
    fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String> {
        let users_by_id = self.users_by_id.lock().unwrap();
        let user = users_by_id.get(&user_id)
            .ok_or("User not found")?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let session = Session {
            token: token.to_string(),
            user_id,
            username: user.username.clone(),
            created_at: now,
            expires_at,
        };
        
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(token.to_string(), session);
        
        Ok(())
    }
    
    fn get_session(&self, token: &str) -> Result<Option<Session>, String> {
        let sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get(token) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            
            if session.expires_at < now {
                drop(sessions);
                let _ = self.delete_session(token);
                Ok(None)
            } else {
                Ok(Some(session.clone()))
            }
        } else {
            Ok(None)
        }
    }
    
    fn delete_session(&self, token: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(token);
        Ok(())
    }
    
    fn delete_expired_sessions(&self) -> Result<(), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let mut sessions = self.sessions.lock().unwrap();
        sessions.retain(|_, session| session.expires_at >= now);
        
        Ok(())
    }
}

// ============================================================================
// SQLite Backend Implementation
// ============================================================================

#[cfg(feature = "sqlite")]
use rusqlite::Connection;

#[cfg(feature = "sqlite")]
struct SQLiteBackend {
    conn: Arc<Mutex<Connection>>,
}

#[cfg(feature = "sqlite")]
impl SQLiteBackend {
    fn new(db_path: &str) -> Result<Self, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        
        let backend = SQLiteBackend {
            conn: Arc::new(Mutex::new(conn)),
        };
        
        backend.init_schema()?;
        Ok(backend)
    }
    
    fn init_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        ).map_err(|e| format!("Failed to create users table: {}", e))?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                token TEXT PRIMARY KEY,
                user_id INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
            )",
            [],
        ).map_err(|e| format!("Failed to create sessions table: {}", e))?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)",
            [],
        ).map_err(|e| format!("Failed to create index: {}", e))?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)",
            [],
        ).map_err(|e| format!("Failed to create index: {}", e))?;
        
        Ok(())
    }
}

#[cfg(feature = "sqlite")]
impl AuthDatabaseTrait for SQLiteBackend {
    fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        conn.execute(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
            [username, password_hash, &now.to_string()],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                "Username already exists".to_string()
            } else {
                format!("Failed to create user: {}", e)
            }
        })?;
        
        let user_id = conn.last_insert_rowid();
        
        Ok(User {
            id: user_id,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
        })
    }
    
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, created_at FROM users WHERE username = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let user = stmt.query_row([username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3)?,
            })
        });
        
        match user {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    
    fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        conn.execute(
            "INSERT INTO sessions (token, user_id, created_at, expires_at) VALUES (?1, ?2, ?3, ?4)",
            [token, &user_id.to_string(), &now.to_string(), &expires_at.to_string()],
        ).map_err(|e| format!("Failed to create session: {}", e))?;
        
        Ok(())
    }
    
    fn get_session(&self, token: &str) -> Result<Option<Session>, String> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT s.token, s.user_id, u.username, s.created_at, s.expires_at 
             FROM sessions s 
             JOIN users u ON s.user_id = u.id 
             WHERE s.token = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let session = stmt.query_row([token], |row| {
            Ok(Session {
                token: row.get(0)?,
                user_id: row.get(1)?,
                username: row.get(2)?,
                created_at: row.get(3)?,
                expires_at: row.get(4)?,
            })
        });
        
        match session {
            Ok(session) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                
                if session.expires_at < now {
                    drop(stmt);
                    drop(conn);
                    let _ = self.delete_session(token);
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    
    fn delete_session(&self, token: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute("DELETE FROM sessions WHERE token = ?1", [token])
            .map_err(|e| format!("Failed to delete session: {}", e))?;
        
        Ok(())
    }
    
    fn delete_expired_sessions(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        conn.execute("DELETE FROM sessions WHERE expires_at < ?1", [now])
            .map_err(|e| format!("Failed to delete expired sessions: {}", e))?;
        
        Ok(())
    }
}

// ============================================================================
// Redis Backend Implementation (NoSQL for Production MMO)
// ============================================================================

#[cfg(feature = "redis_backend")]
use redis::{Client, Commands, Connection as RedisConnection};

#[cfg(feature = "redis_backend")]
struct RedisBackend {
    client: Client,
}

#[cfg(feature = "redis_backend")]
impl RedisBackend {
    fn new(redis_url: &str) -> Result<Self, String> {
        let client = Client::open(redis_url)
            .map_err(|e| format!("Failed to connect to Redis: {}", e))?;
        
        // Test connection
        let mut conn = client.get_connection()
            .map_err(|e| format!("Failed to get Redis connection: {}", e))?;
        
        let _: () = redis::cmd("PING")
            .query(&mut conn)
            .map_err(|e| format!("Redis PING failed: {}", e))?;
        
        Ok(RedisBackend { client })
    }
    
    fn get_conn(&self) -> Result<RedisConnection, String> {
        self.client.get_connection()
            .map_err(|e| format!("Failed to get Redis connection: {}", e))
    }
}

#[cfg(feature = "redis_backend")]
impl AuthDatabaseTrait for RedisBackend {
    fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String> {
        let mut conn = self.get_conn()?;
        
        // Check if user exists
        let exists: bool = conn.hexists("users", username)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        if exists {
            return Err("Username already exists".to_string());
        }
        
        // Get next user ID
        let user_id: i64 = conn.incr("next_user_id", 1)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let user = User {
            id: user_id,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
        };
        
        // Store user as JSON
        let user_json = serde_json::to_string(&user)
            .map_err(|e| format!("Failed to serialize user: {}", e))?;
        
        let _: () = conn.hset("users", username, &user_json)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        let _: () = conn.hset("users_by_id", user_id, &user_json)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        Ok(user)
    }
    
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut conn = self.get_conn()?;
        
        let user_json: Option<String> = conn.hget("users", username)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        match user_json {
            Some(json) => {
                let user: User = serde_json::from_str(&json)
                    .map_err(|e| format!("Failed to deserialize user: {}", e))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
    
    fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String> {
        let mut conn = self.get_conn()?;
        
        // Get user to retrieve username
        let user_json: Option<String> = conn.hget("users_by_id", user_id)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        let user: User = match user_json {
            Some(json) => serde_json::from_str(&json)
                .map_err(|e| format!("Failed to deserialize user: {}", e))?,
            None => return Err("User not found".to_string()),
        };
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let session = Session {
            token: token.to_string(),
            user_id,
            username: user.username,
            created_at: now,
            expires_at,
        };
        
        let session_json = serde_json::to_string(&session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;
        
        // Calculate TTL (time to live) in seconds
        let ttl = (expires_at - now) as usize;
        
        // Store session with automatic expiration (Redis TTL)
        let _: () = conn.set_ex(format!("session:{}", token), session_json, ttl)
            .map_err(|e| format!("Redis error: {}", e))?;
        
        Ok(())
    }
    
    fn get_session(&self, token: &str) -> Result<Option<Session>, String> {
        let mut conn = self.get_conn()?;
        
        let session_json: Option<String> = conn.get(format!("session:{}", token))
            .map_err(|e| format!("Redis error: {}", e))?;
        
        match session_json {
            Some(json) => {
                let session: Session = serde_json::from_str(&json)
                    .map_err(|e| format!("Failed to deserialize session: {}", e))?;
                
                // Redis TTL handles expiration automatically, but double-check
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                
                if session.expires_at < now {
                    let _ = self.delete_session(token);
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            None => Ok(None),
        }
    }
    
    fn delete_session(&self, token: &str) -> Result<(), String> {
        let mut conn = self.get_conn()?;
        
        let _: () = conn.del(format!("session:{}", token))
            .map_err(|e| format!("Redis error: {}", e))?;
        
        Ok(())
    }
    
    fn delete_expired_sessions(&self) -> Result<(), String> {
        // Redis automatically deletes expired keys with TTL, so this is a no-op
        Ok(())
    }
}
