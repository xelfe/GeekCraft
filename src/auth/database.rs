//! Database abstraction for authentication and game data
//! 
//! Provides a trait-based abstraction that supports multiple database backends:
//! - **In-Memory**: Fast, for testing and development (data lost on restart)
//! - **MongoDB**: Persistent NoSQL, perfect for production MMO servers (requires MongoDB server)
//! 
//! **Recommendations:**
//! - **Development/Testing**: Use In-Memory
//! - **Production MMO**: Use MongoDB (persistent storage, horizontal scaling, high concurrency)
//! 
//! Users can easily switch between backends by changing configuration.

use super::models::{User, Session};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Helper function to get current Unix timestamp safely
fn get_unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock is before Unix epoch")
        .as_secs() as i64
}

/// Database backend configuration
#[derive(Debug, Clone)]
pub enum DatabaseBackend {
    /// In-memory database (recommended for development and testing)
    /// - Pros: Fast, no setup required
    /// - Cons: Data lost on restart
    /// - Use case: Development, testing
    InMemory,
    
    /// MongoDB database (recommended for production MMO servers)
    /// - Pros: Persistent storage, horizontal scaling, high concurrency, flexible schema
    /// - Cons: Requires MongoDB server
    /// - Use case: Production MMO, multiple servers, persistent game data
    MongoDB(String), // Connection string: "mongodb://localhost:27017"
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
            DatabaseBackend::MongoDB(url) => {
                // MongoDB backend needs async initialization, we'll use a blocking approach here
                Box::new(MongoBackend::new(&url)?)
            }
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
        
        let now = get_unix_timestamp();
        
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
        
        let now = get_unix_timestamp();
        
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
            let now = get_unix_timestamp();
            
            if session.expires_at < now {
                // Session expired - delete it
                let token_clone = token.to_string();
                drop(sessions); // Release lock before delete
                let _ = self.delete_session(&token_clone);
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
        let now = get_unix_timestamp();
        
        let mut sessions = self.sessions.lock().unwrap();
        sessions.retain(|_, session| session.expires_at >= now);
        
        Ok(())
    }
}

// ============================================================================
// MongoDB Backend Implementation (NoSQL for Production MMO)
// ============================================================================

use mongodb::{
    Client, 
    bson::{doc, Document, to_document, from_document},
    options::{ClientOptions, IndexOptions},
    IndexModel,
};
use std::time::Duration;

struct MongoBackend {
    client: Client,
    db_name: String,
}

// Note: This implementation creates a new runtime for each operation to maintain
// compatibility with the synchronous AuthDatabaseTrait. For production use with
// high throughput, consider refactoring the trait to be async or using a shared
// runtime via tokio::runtime::Handle::current().block_on().

impl MongoBackend {
    fn new(mongodb_url: &str) -> Result<Self, String> {
        // Extract database name from URL or use default
        let db_name = mongodb_url
            .split('/')
            .last()
            .and_then(|s| s.split('?').next())
            .filter(|s| !s.is_empty())
            .unwrap_or("geekcraft")
            .to_string();
        
        // Create a runtime and initialize MongoDB client
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        let (client, db_name_final) = rt.block_on(async {
            // Parse MongoDB connection string
            let client_options = ClientOptions::parse(mongodb_url)
                .await
                .map_err(|e| format!("Failed to parse MongoDB URL: {}", e))?;
            
            // Create MongoDB client
            let client = Client::with_options(client_options)
                .map_err(|e| format!("Failed to create MongoDB client: {}", e))?;
            
            // Test connection
            client
                .database(&db_name)
                .run_command(doc! { "ping": 1 }, None)
                .await
                .map_err(|e| format!("MongoDB connection test failed: {}", e))?;
            
            let db = client.database(&db_name);
            let sessions_collection = db.collection::<Document>("sessions");
            
            // Create TTL index for sessions (auto-expire after expires_at)
            let index_options = IndexOptions::builder()
                .expire_after(Duration::from_secs(0))
                .build();
            
            let index_model = IndexModel::builder()
                .keys(doc! { "expires_at": 1 })
                .options(index_options)
                .build();
            
            sessions_collection
                .create_index(index_model, None)
                .await
                .map_err(|e| format!("Failed to create TTL index: {}", e))?;
            
            // Create unique index on username
            let users_collection = db.collection::<Document>("users");
            let username_index = IndexModel::builder()
                .keys(doc! { "username": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .build()
                )
                .build();
            
            users_collection
                .create_index(username_index, None)
                .await
                .map_err(|e| format!("Failed to create username index: {}", e))?;
            
            Ok::<(Client, String), String>((client, db_name))
        })?;
        
        Ok(MongoBackend { client, db_name: db_name_final })
    }
    
    fn get_database(&self) -> mongodb::Database {
        self.client.database(&self.db_name)
    }
}

impl AuthDatabaseTrait for MongoBackend {
    fn create_user(&self, username: &str, password_hash: &str) -> Result<User, String> {
        let db = self.get_database();
        let users_collection = db.collection::<Document>("users");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            // Check if user exists
            let existing = users_collection
                .find_one(doc! { "username": username }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            if existing.is_some() {
                return Err("Username already exists".to_string());
            }
            
            // Get next user ID using a counter collection
            let counter_collection = db.collection::<Document>("counters");
            let result = counter_collection
                .find_one_and_update(
                    doc! { "_id": "user_id" },
                    doc! { "$inc": { "value": 1 } },
                    mongodb::options::FindOneAndUpdateOptions::builder()
                        .upsert(true)
                        .return_document(mongodb::options::ReturnDocument::After)
                        .build()
                )
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            let user_id = result
                .and_then(|doc| doc.get_i64("value").ok())
                .unwrap_or(1);
            
            let now = get_unix_timestamp();
            
            let user = User {
                id: user_id,
                username: username.to_string(),
                password_hash: password_hash.to_string(),
                created_at: now,
            };
            
            // Insert user document
            let user_doc = to_document(&user)
                .map_err(|e| format!("Failed to serialize user: {}", e))?;
            
            users_collection
                .insert_one(user_doc, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            Ok(user)
        })
    }
    
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let db = self.get_database();
        let users_collection = db.collection::<Document>("users");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            let user_doc = users_collection
                .find_one(doc! { "username": username }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            match user_doc {
                Some(doc) => {
                    let user: User = from_document(doc)
                        .map_err(|e| format!("Failed to deserialize user: {}", e))?;
                    Ok(Some(user))
                }
                None => Ok(None),
            }
        })
    }
    
    fn create_session(&self, token: &str, user_id: i64, expires_at: i64) -> Result<(), String> {
        let db = self.get_database();
        let users_collection = db.collection::<Document>("users");
        let sessions_collection = db.collection::<Document>("sessions");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            // Get user to retrieve username
            let user_doc = users_collection
                .find_one(doc! { "id": user_id }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            let user: User = match user_doc {
                Some(doc) => from_document(doc)
                    .map_err(|e| format!("Failed to deserialize user: {}", e))?,
                None => return Err("User not found".to_string()),
            };
            
            let now = get_unix_timestamp();
            
            let session = Session {
                token: token.to_string(),
                user_id,
                username: user.username,
                created_at: now,
                expires_at,
            };
            
            // Convert expires_at to BSON DateTime for TTL index
            let session_doc = doc! {
                "token": &session.token,
                "user_id": session.user_id,
                "username": &session.username,
                "created_at": session.created_at,
                "expires_at": bson::DateTime::from_millis(expires_at * 1000),
            };
            
            // Insert or update session
            sessions_collection
                .update_one(
                    doc! { "token": token },
                    doc! { "$set": session_doc },
                    mongodb::options::UpdateOptions::builder()
                        .upsert(true)
                        .build()
                )
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            Ok(())
        })
    }
    
    fn get_session(&self, token: &str) -> Result<Option<Session>, String> {
        let db = self.get_database();
        let sessions_collection = db.collection::<Document>("sessions");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            let session_doc = sessions_collection
                .find_one(doc! { "token": token }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            match session_doc {
                Some(doc) => {
                    // Extract expires_at as timestamp
                    let expires_at = if let Ok(dt) = doc.get_datetime("expires_at") {
                        dt.timestamp_millis() / 1000
                    } else {
                        doc.get_i64("expires_at").unwrap_or(0)
                    };
                    
                    let session = Session {
                        token: doc.get_str("token").unwrap_or("").to_string(),
                        user_id: doc.get_i64("user_id").unwrap_or(0),
                        username: doc.get_str("username").unwrap_or("").to_string(),
                        created_at: doc.get_i64("created_at").unwrap_or(0),
                        expires_at,
                    };
                    
                    // Check if session is expired
                    let now = get_unix_timestamp();
                    if session.expires_at < now {
                        // Delete expired session
                        let _ = sessions_collection
                            .delete_one(doc! { "token": token }, None)
                            .await;
                        Ok(None)
                    } else {
                        Ok(Some(session))
                    }
                }
                None => Ok(None),
            }
        })
    }
    
    fn delete_session(&self, token: &str) -> Result<(), String> {
        let db = self.get_database();
        let sessions_collection = db.collection::<Document>("sessions");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            sessions_collection
                .delete_one(doc! { "token": token }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            Ok(())
        })
    }
    
    fn delete_expired_sessions(&self) -> Result<(), String> {
        // MongoDB TTL index handles this automatically
        // But we can manually clean up if needed
        let db = self.get_database();
        let sessions_collection = db.collection::<Document>("sessions");
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        rt.block_on(async {
            let now = bson::DateTime::from_millis(get_unix_timestamp() * 1000);
            sessions_collection
                .delete_many(doc! { "expires_at": { "$lt": now } }, None)
                .await
                .map_err(|e| format!("MongoDB error: {}", e))?;
            
            Ok(())
        })
    }
}
