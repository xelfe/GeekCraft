// This file defines the game entities, such as units and buildings, along with their behaviors.

pub struct Entity {
    pub id: u32,
    pub name: String,
    pub position: (f32, f32),
    pub health: f32,
}

impl Entity {
    pub fn new(id: u32, name: &str, position: (f32, f32), health: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            position,
            health,
        }
    }

    pub fn move_to(&mut self, new_position: (f32, f32)) {
        self.position = new_position;
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health -= amount;
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

pub struct Building {
    pub id: u32,
    pub name: String,
    pub position: (f32, f32),
    pub health: f32,
    pub resource_capacity: u32,
}

impl Building {
    pub fn new(id: u32, name: &str, position: (f32, f32), health: f32, resource_capacity: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            position,
            health,
            resource_capacity,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health -= amount;
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0.0
    }
}