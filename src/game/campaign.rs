use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::game::world::World;
use crate::scripting::sandbox::ScriptEngine;

/// Validate run_id to prevent path traversal attacks
fn validate_run_id(run_id: &str) -> Result<(), String> {
    if run_id.is_empty() {
        return Err("Run ID cannot be empty".to_string());
    }
    
    // Check for path separators and other potentially dangerous characters
    if run_id.contains('/') || run_id.contains('\\') || run_id.contains("..") {
        return Err("Run ID contains invalid characters (path separators or '..')".to_string());
    }
    
    // Ensure it's a reasonable length
    if run_id.len() > 255 {
        return Err("Run ID is too long (max 255 characters)".to_string());
    }
    
    // Only allow alphanumeric, underscore, hyphen, and dot
    if !run_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("Run ID can only contain alphanumeric characters, underscore, hyphen, and dot".to_string());
    }
    
    Ok(())
}

/// Represents a single campaign run instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignRun {
    pub run_id: String,
    pub tick: u64,
    pub running: bool,
    pub created_at: i64,
}

impl CampaignRun {
    pub fn new(run_id: String) -> Self {
        Self {
            run_id,
            tick: 0,
            running: false,
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn tick(&mut self) {
        if self.running {
            self.tick += 1;
        }
    }
}

/// In-memory store for active campaign runs
pub struct InMemoryRunStore {
    runs: HashMap<String, CampaignRun>,
}

impl InMemoryRunStore {
    pub fn new() -> Self {
        Self {
            runs: HashMap::new(),
        }
    }

    pub fn create_run(&mut self, run_id: String) -> CampaignRun {
        let run = CampaignRun::new(run_id.clone());
        self.runs.insert(run_id, run.clone());
        run
    }

    pub fn get_run(&self, run_id: &str) -> Option<&CampaignRun> {
        self.runs.get(run_id)
    }

    pub fn get_run_mut(&mut self, run_id: &str) -> Option<&mut CampaignRun> {
        self.runs.get_mut(run_id)
    }

    pub fn remove_run(&mut self, run_id: &str) -> Option<CampaignRun> {
        self.runs.remove(run_id)
    }

    pub fn insert_run(&mut self, run_id: String, run: CampaignRun) {
        self.runs.insert(run_id, run);
    }
}

/// Campaign manager handles campaign operations including persistence
pub struct CampaignManager {
    store: InMemoryRunStore,
    save_dir: PathBuf,
}

impl CampaignManager {
    pub fn new() -> Self {
        let save_dir = std::env::var("GEEKCRAFT_SAVE_DIR")
            .unwrap_or_else(|_| "./saves".to_string());
        
        let save_path = PathBuf::from(save_dir);
        
        // Create save directory if it doesn't exist
        if !save_path.exists() {
            if let Err(e) = fs::create_dir_all(&save_path) {
                log::warn!("Failed to create save directory {:?}: {}", save_path, e);
            } else {
                log::info!("Created save directory at {:?}", save_path);
            }
        }

        Self {
            store: InMemoryRunStore::new(),
            save_dir: save_path,
        }
    }

    pub fn start_run(&mut self, run_id: String) -> Result<CampaignRun, String> {
        validate_run_id(&run_id)?;
        
        if self.store.get_run(&run_id).is_some() {
            return Err(format!("Run {} already exists", run_id));
        }

        self.store.create_run(run_id.clone());
        let run = self.store.get_run_mut(&run_id)
            .ok_or_else(|| "Failed to retrieve created run".to_string())?;
        run.start();
        Ok(run.clone())
    }

    pub fn get_run_state(&self, run_id: &str) -> Option<CampaignRun> {
        self.store.get_run(run_id).cloned()
    }

    pub fn stop_run(&mut self, run_id: &str) -> Result<(), String> {
        let run = self.store.get_run_mut(run_id)
            .ok_or_else(|| format!("Run {} not found", run_id))?;
        
        run.stop();
        Ok(())
    }

    pub fn tick_run(&mut self, run_id: &str, _world: &mut World, _script_engine: &mut ScriptEngine) -> Result<(), String> {
        let run = self.store.get_run_mut(run_id)
            .ok_or_else(|| format!("Run {} not found", run_id))?;
        
        if !run.running {
            return Err(format!("Run {} is not running", run_id));
        }

        run.tick();
        
        // TODO: Actual game simulation logic would go here
        // For now, this is a minimal implementation
        
        Ok(())
    }

    pub fn save_run(&self, run_id: &str) -> Result<(), String> {
        validate_run_id(run_id)?;
        
        let run = self.store.get_run(run_id)
            .ok_or_else(|| format!("Run {} not found", run_id))?;

        let file_path = self.save_dir.join(format!("{}.json", run_id));
        
        let json = serde_json::to_string_pretty(run)
            .map_err(|e| format!("Failed to serialize run: {}", e))?;
        
        fs::write(&file_path, json)
            .map_err(|e| format!("Failed to write save file: {}", e))?;

        log::info!("Saved run {} to {:?}", run_id, file_path);
        Ok(())
    }

    pub fn load_run(&mut self, run_id: &str) -> Result<CampaignRun, String> {
        validate_run_id(run_id)?;
        
        let file_path = self.save_dir.join(format!("{}.json", run_id));
        
        if !file_path.exists() {
            return Err(format!("Save file not found for run {}", run_id));
        }

        let json = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read save file: {}", e))?;
        
        let run: CampaignRun = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize run: {}", e))?;

        self.store.insert_run(run_id.to_string(), run.clone());
        
        log::info!("Loaded run {} from {:?}", run_id, file_path);
        Ok(run)
    }

    pub fn list_all_saves(&self) -> Result<Vec<String>, String> {
        if !self.save_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.save_dir)
            .map_err(|e| format!("Failed to read save directory: {}", e))?;

        let mut saves = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        saves.push(file_name.to_string());
                    }
                }
            }
        }

        Ok(saves)
    }
}

/// Global campaign manager instance (used by HTTP handlers)
pub fn create_campaign_manager() -> Arc<RwLock<CampaignManager>> {
    Arc::new(RwLock::new(CampaignManager::new()))
}
