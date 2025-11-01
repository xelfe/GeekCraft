pub struct Terrain {
    id: u32,
    name: String,
    resource_type: ResourceType,
    resource_amount: u32,
}

#[derive(PartialEq)]
pub enum ResourceType {
    Wood,
    Stone,
    Gold,
}

pub struct World {
    width: u32,
    height: u32,
    terrains: Vec<Terrain>,
}

impl World {
    pub fn new() -> Self {
        World {
            width: 100,
            height: 100,
            terrains: Vec::new(),
        }
    }

    fn add_terrain(&mut self, terrain: Terrain) {
        self.terrains.push(terrain);
    }

    fn get_terrain(&self, id: u32) -> Option<&Terrain> {
        self.terrains.iter().find(|&t| t.id == id)
    }

    fn resource_count(&self, resource_type: ResourceType) -> u32 {
        self.terrains.iter()
            .filter(|&t| t.resource_type == resource_type)
            .map(|t| t.resource_amount)
            .sum()
    }
}