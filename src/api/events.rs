// This file defines the game events, allowing scripts to react to specific actions. 

pub struct GameEvent {
    pub event_type: String,
    pub data: EventData,
}

pub enum EventData {
    UnitCreated { unit_id: u32, position: (f32, f32) },
    UnitMoved { unit_id: u32, new_position: (f32, f32) },
    UnitDestroyed { unit_id: u32 },
    ResourceCollected { resource_id: u32, amount: u32 },
}

impl GameEvent {
    pub fn new(event_type: String, data: EventData) -> Self {
        GameEvent { event_type, data }
    }
}