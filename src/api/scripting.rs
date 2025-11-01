// This file manages the scripting system, allowing players to program behaviors for their entities.

use std::collections::HashMap;

pub struct Script {
    pub code: String,
}

pub struct ScriptManager {
    scripts: HashMap<String, Script>,
}

impl ScriptManager {
    pub fn new() -> Self {
        ScriptManager {
            scripts: HashMap::new(),
        }
    }

    pub fn add_script(&mut self, name: String, code: String) {
        let script = Script { code };
        self.scripts.insert(name, script);
    }

    pub fn execute_script(&self, name: &str) {
        if let Some(script) = self.scripts.get(name) {
            // Here you would implement the logic to execute the script.
            // This is a placeholder for the actual execution logic.
            println!("Executing script: {}", script.code);
        } else {
            println!("Script not found: {}", name);
        }
    }
}