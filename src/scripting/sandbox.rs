// This file provides a secure environment for executing player scripts, isolating their code from the rest of the system. 

use std::collections::HashMap;

pub struct Sandbox {
    // A map to store the variables accessible in the sandbox
    variables: HashMap<String, f64>,
}

pub type ScriptEngine = Sandbox;

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            variables: HashMap::new(),
        }
    }

    // Method to set a variable in the sandbox
    pub fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    // Method to get a variable from the sandbox
    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }

    // Method to execute a script in the sandbox
    pub fn execute_script(&self, _script: &str) -> Result<(), String> {
        // Here you would implement the logic to safely execute the script
        // For now, we will just return Ok to indicate success
        Ok(())
    }
}