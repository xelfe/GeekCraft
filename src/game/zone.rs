// Zone generation module for procedural landscape generation
// Each player starts in their own 30x30 tile zone with procedurally generated terrain

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Size of each zone in tiles
pub const ZONE_SIZE: usize = 30;

/// Surface types that can appear in a zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceType {
    /// Plain surface - walkable, standard movement
    Plain,
    /// Swamp surface - walkable, slower movement
    Swamp,
    /// Obstacle - not walkable, blocks movement
    Obstacle,
}

/// Represents a single tile in a zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    /// X coordinate within the zone (0-29)
    pub x: usize,
    /// Y coordinate within the zone (0-29)
    pub y: usize,
    /// Surface type of this tile
    pub surface_type: SurfaceType,
}

/// Represents an exit point from a zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exit {
    /// X coordinate of the exit
    pub x: usize,
    /// Y coordinate of the exit
    pub y: usize,
    /// Direction of the exit (North, South, East, West)
    pub direction: ExitDirection,
}

/// Cardinal directions for exits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExitDirection {
    North,
    South,
    East,
    West,
}

/// Represents a procedurally generated zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Unique identifier for this zone
    pub id: String,
    /// 2D grid of tiles (30x30)
    pub tiles: Vec<Vec<Tile>>,
    /// List of exits (2-4 per zone)
    pub exits: Vec<Exit>,
}

impl Zone {
    /// Generate a new zone with procedural landscape
    /// 
    /// # Arguments
    /// * `zone_id` - Unique identifier for the zone
    /// * `seed` - Seed for random generation (use zone_id hash for deterministic generation)
    pub fn generate(zone_id: String, seed: u64) -> Self {
        let mut rng = SimpleRng::new(seed);
        
        // Generate tiles with procedural algorithm
        let mut tiles = Vec::with_capacity(ZONE_SIZE);
        
        for y in 0..ZONE_SIZE {
            let mut row = Vec::with_capacity(ZONE_SIZE);
            for x in 0..ZONE_SIZE {
                let surface_type = Self::generate_surface_type(x, y, &mut rng);
                row.push(Tile {
                    x,
                    y,
                    surface_type,
                });
            }
            tiles.push(row);
        }
        
        // Generate 2-4 exits
        let num_exits = 2 + (rng.next() % 3) as usize; // 2, 3, or 4 exits
        let exits = Self::generate_exits(num_exits, &mut rng);
        
        Zone {
            id: zone_id,
            tiles,
            exits,
        }
    }
    
    /// Generate surface type for a tile using procedural algorithm
    fn generate_surface_type(x: usize, y: usize, rng: &mut SimpleRng) -> SurfaceType {
        // Use Perlin-like noise approximation for natural-looking terrain
        let noise_value = Self::noise(x, y, rng.seed);
        
        // Distribution: ~60% Plain, ~25% Swamp, ~15% Obstacle
        if noise_value < 0.60 {
            SurfaceType::Plain
        } else if noise_value < 0.85 {
            SurfaceType::Swamp
        } else {
            SurfaceType::Obstacle
        }
    }
    
    /// Simple 2D noise function for terrain generation
    fn noise(x: usize, y: usize, seed: u64) -> f64 {
        // Combine position with seed for deterministic pseudo-random values
        let hash = (x as u64).wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add(seed);
        
        // Mix the bits
        let hash = hash ^ (hash >> 13);
        let hash = hash.wrapping_mul(1274126177);
        let hash = hash ^ (hash >> 16);
        
        // Convert to 0.0-1.0 range
        (hash % 10000) as f64 / 10000.0
    }
    
    /// Generate exits for the zone
    fn generate_exits(num_exits: usize, rng: &mut SimpleRng) -> Vec<Exit> {
        let mut exits = Vec::with_capacity(num_exits);
        let mut used_directions = HashSet::new();
        
        for _ in 0..num_exits {
            // Pick a random direction that hasn't been used
            let direction = loop {
                let dir = match rng.next() % 4 {
                    0 => ExitDirection::North,
                    1 => ExitDirection::South,
                    2 => ExitDirection::East,
                    _ => ExitDirection::West,
                };
                
                if !used_directions.contains(&dir) {
                    used_directions.insert(dir);
                    break dir;
                }
                
                // If all directions are used, allow duplicates
                if used_directions.len() >= 4 {
                    break dir;
                }
            };
            
            // Place exit on the edge based on direction
            let (x, y) = match direction {
                ExitDirection::North => (rng.next() as usize % ZONE_SIZE, 0),
                ExitDirection::South => (rng.next() as usize % ZONE_SIZE, ZONE_SIZE - 1),
                ExitDirection::East => (ZONE_SIZE - 1, rng.next() as usize % ZONE_SIZE),
                ExitDirection::West => (0, rng.next() as usize % ZONE_SIZE),
            };
            
            exits.push(Exit { x, y, direction });
        }
        
        exits
    }
    
    /// Get a tile at specific coordinates
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < ZONE_SIZE && y < ZONE_SIZE {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }
    
    /// Count tiles by surface type
    pub fn count_surface_type(&self, surface_type: SurfaceType) -> usize {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| tile.surface_type == surface_type)
            .count()
    }
}

/// Simple pseudo-random number generator for deterministic zone generation
struct SimpleRng {
    seed: u64,
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self {
            seed,
            state: seed,
        }
    }
    
    fn next(&mut self) -> u64 {
        // Linear congruential generator
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_generation() {
        let zone = Zone::generate("test_zone".to_string(), 12345);
        
        // Verify zone properties
        assert_eq!(zone.id, "test_zone");
        assert_eq!(zone.tiles.len(), ZONE_SIZE);
        assert_eq!(zone.tiles[0].len(), ZONE_SIZE);
        assert!(zone.exits.len() >= 2 && zone.exits.len() <= 4);
    }
    
    #[test]
    fn test_deterministic_generation() {
        let zone1 = Zone::generate("zone1".to_string(), 12345);
        let zone2 = Zone::generate("zone1".to_string(), 12345);
        
        // Same seed should produce same zone
        assert_eq!(zone1.tiles[0][0].surface_type, zone2.tiles[0][0].surface_type);
        assert_eq!(zone1.tiles[15][15].surface_type, zone2.tiles[15][15].surface_type);
        assert_eq!(zone1.exits.len(), zone2.exits.len());
    }
    
    #[test]
    fn test_different_seeds() {
        let zone1 = Zone::generate("zone1".to_string(), 12345);
        let zone2 = Zone::generate("zone1".to_string(), 54321);
        
        // Different seeds should produce different zones
        let same_tiles = zone1.tiles.iter()
            .zip(zone2.tiles.iter())
            .flat_map(|(row1, row2)| row1.iter().zip(row2.iter()))
            .filter(|(t1, t2)| t1.surface_type == t2.surface_type)
            .count();
        
        // Not all tiles should be the same
        assert!(same_tiles < ZONE_SIZE * ZONE_SIZE);
    }
    
    #[test]
    fn test_exit_count() {
        for seed in 0..100 {
            let zone = Zone::generate(format!("zone_{}", seed), seed);
            assert!(zone.exits.len() >= 2 && zone.exits.len() <= 4,
                "Zone should have 2-4 exits, got {}", zone.exits.len());
        }
    }
    
    #[test]
    fn test_get_tile() {
        let zone = Zone::generate("test_zone".to_string(), 12345);
        
        // Valid coordinates
        assert!(zone.get_tile(0, 0).is_some());
        assert!(zone.get_tile(29, 29).is_some());
        
        // Invalid coordinates
        assert!(zone.get_tile(30, 0).is_none());
        assert!(zone.get_tile(0, 30).is_none());
    }
    
    #[test]
    fn test_surface_type_distribution() {
        let zone = Zone::generate("test_zone".to_string(), 12345);
        
        let plains = zone.count_surface_type(SurfaceType::Plain);
        let swamps = zone.count_surface_type(SurfaceType::Swamp);
        let obstacles = zone.count_surface_type(SurfaceType::Obstacle);
        
        // Total should equal zone size
        assert_eq!(plains + swamps + obstacles, ZONE_SIZE * ZONE_SIZE);
        
        // Each type should be present
        assert!(plains > 0, "Should have some plains");
        assert!(swamps > 0, "Should have some swamps");
        assert!(obstacles > 0, "Should have some obstacles");
    }
}
