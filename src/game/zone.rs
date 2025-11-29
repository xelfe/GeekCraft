//! Zone generation module for procedural landscape generation with improved tile clustering
//!
//! Each player starts in their own 30x30 tile zone with procedurally generated terrain.
//! Uses Perlin noise for natural-looking tile clustering and grouping.
//! Zones feature three surface types (Plain, Swamp, Obstacle) and 2-4 exits for zone interconnection.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Size of each zone in tiles
pub const ZONE_SIZE: usize = 50;

/// Surface types that can appear in a zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// X coordinate of the exit (start position)
    pub x: usize,
    /// Y coordinate of the exit (start position)
    pub y: usize,
    /// Direction of the exit (North, South, East, West)
    pub direction: ExitDirection,
    /// Width of the exit in tiles (1-4)
    pub width: usize,
}

/// Cardinal directions for exits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExitDirection {
    North,
    South,
    East,
    West,
}

/// Represents a procedurally generated zone with improved tile clustering
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
    /// Generate a new zone with procedural landscape and improved tile clustering
    ///
    /// Uses Perlin noise-like algorithm to create natural grouping of tiles by type,
    /// resulting in larger contiguous regions of plains, swamps, and obstacles.
    /// Guarantees that all 3 surface types appear in every zone.
    ///
    /// # Arguments
    /// * `zone_id` - Unique identifier for the zone
    /// * `seed` - Seed for random generation (use zone_id hash for deterministic generation)
    pub fn generate(zone_id: String, seed: u64) -> Self {
        let mut rng = SimpleRng::new(seed);

        // Generate tiles with improved Perlin noise algorithm
        let mut tiles = Vec::with_capacity(ZONE_SIZE);

        // Pre-calculate noise values for all positions using multi-scale Perlin noise
        let mut noise_values = vec![vec![0.0; ZONE_SIZE]; ZONE_SIZE];

        for y in 0..ZONE_SIZE {
            for x in 0..ZONE_SIZE {
                // Combine multiple noise scales for better clustering
                // Larger scale (lower frequency) dominates for bigger regions
                let noise1 = Self::perlin_noise(x as f64, y as f64, seed, 0.1);
                let noise2 = Self::perlin_noise(x as f64, y as f64, seed, 0.2);
                let noise3 = Self::perlin_noise(x as f64, y as f64, seed, 0.4);

                // Weight the scales: larger regions have more influence
                // Use raw average to maintain full 0.0-1.0 range
                noise_values[y][x] = noise1 * 0.6 + noise2 * 0.3 + noise3 * 0.1;
            }
        }

        // Create tiles based on noise values
        for y in 0..ZONE_SIZE {
            let mut row = Vec::with_capacity(ZONE_SIZE);
            for x in 0..ZONE_SIZE {
                let noise_value = noise_values[y][x];
                let surface_type = Self::classify_terrain(noise_value);

                row.push(Tile {
                    x,
                    y,
                    surface_type,
                });
            }
            tiles.push(row);
        }

        // Generate 2-4 exits first (before adding border)
        let num_exits = 2 + (rng.next() % 3) as usize; // 2, 3, or 4 exits
        let exits = Self::generate_exits(num_exits, &mut rng);

        // Surround zone with obstacles except at exits
        Self::add_obstacle_border(&mut tiles, &exits);

        // Ensure all 3 surface types are present
        Self::ensure_all_surface_types(&mut tiles, &mut rng);

        Zone {
            id: zone_id,
            tiles,
            exits,
        }
    }

    /// Add obstacle border around the zone, except at exit locations
    fn add_obstacle_border(tiles: &mut Vec<Vec<Tile>>, exits: &[Exit]) {
        // Create a set of exit tile positions for quick lookup
        let mut exit_positions = std::collections::HashSet::new();

        for exit in exits {
            match exit.direction {
                ExitDirection::North | ExitDirection::South => {
                    // Horizontal exits
                    for i in 0..exit.width {
                        let x = exit.x + i;
                        if x < ZONE_SIZE {
                            exit_positions.insert((x, exit.y));
                        }
                    }
                }
                ExitDirection::East | ExitDirection::West => {
                    // Vertical exits
                    for i in 0..exit.width {
                        let y = exit.y + i;
                        if y < ZONE_SIZE {
                            exit_positions.insert((exit.x, y));
                        }
                    }
                }
            }
        }

        // Set border tiles to obstacles, except at exits
        for y in 0..ZONE_SIZE {
            for x in 0..ZONE_SIZE {
                // Check if this is a border tile
                let is_border = x == 0 || x == ZONE_SIZE - 1 || y == 0 || y == ZONE_SIZE - 1;

                if is_border && !exit_positions.contains(&(x, y)) {
                    tiles[y][x].surface_type = SurfaceType::Obstacle;
                }
            }
        }
    }

    /// Ensure all surface types are present in the zone
    /// If any type is missing, convert some tiles to that type while maintaining clustering
    fn ensure_all_surface_types(tiles: &mut Vec<Vec<Tile>>, rng: &mut SimpleRng) {
        use std::collections::HashMap;

        // Count existing surface types
        let mut type_counts: HashMap<SurfaceType, usize> = HashMap::new();
        for row in tiles.iter() {
            for tile in row.iter() {
                *type_counts.entry(tile.surface_type).or_insert(0) += 1;
            }
        }

        // Check which types are missing
        let all_types = [SurfaceType::Plain, SurfaceType::Swamp, SurfaceType::Obstacle];
        let missing_types: Vec<SurfaceType> = all_types
            .iter()
            .filter(|t| !type_counts.contains_key(t) || type_counts[t] == 0)
            .copied()
            .collect();

        // For each missing type, place a small cluster
        for missing_type in missing_types {
            // Find a random location
            let x = (rng.next() % ZONE_SIZE as u64) as usize;
            let y = (rng.next() % ZONE_SIZE as u64) as usize;

            // Create a small cluster (3-5 tiles) of the missing type
            let cluster_size = 3 + (rng.next() % 3) as usize;
            Self::place_surface_cluster(tiles, x, y, missing_type, cluster_size);
        }
    }

    /// Place a small cluster of a specific surface type at the given location
    fn place_surface_cluster(
        tiles: &mut Vec<Vec<Tile>>,
        start_x: usize,
        start_y: usize,
        surface_type: SurfaceType,
        size: usize,
    ) {
        let mut placed = 0;
        let mut stack = vec![(start_x, start_y)];
        let mut visited = HashSet::new();

        while let Some((x, y)) = stack.pop() {
            if placed >= size {
                break;
            }

            if x >= ZONE_SIZE || y >= ZONE_SIZE || visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
            tiles[y][x].surface_type = surface_type;
            placed += 1;

            // Add neighbors
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < ZONE_SIZE {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < ZONE_SIZE {
                stack.push((x, y + 1));
            }
        }
    }

    /// Classify terrain based on noise value with proper distribution
    ///
    /// The ranges are tuned to maintain approximately:
    /// - ~60% Plain
    /// - ~25% Swamp
    /// - ~15% Obstacle
    fn classify_terrain(noise_value: f64) -> SurfaceType {
        // Distribution optimized for gameplay balance and clustering
        if noise_value < 0.60 {
            SurfaceType::Plain
        } else if noise_value < 0.85 {
            SurfaceType::Swamp
        } else {
            SurfaceType::Obstacle
        }
    }

    /// Improved Perlin-like noise function for better terrain clustering
    ///
    /// This implementation uses gradient interpolation to create smooth noise
    /// that produces natural-looking clusters of terrain types.
    ///
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `seed` - Base seed for deterministic generation
    /// * `frequency` - Controls the scale of features (higher = smaller features)
    fn perlin_noise(x: f64, y: f64, seed: u64, frequency: f64) -> f64 {
        // Scale coordinates by frequency
        let x_scaled = x * frequency;
        let y_scaled = y * frequency;

        // Find grid cell corners
        let xi = x_scaled.floor() as i32;
        let yi = y_scaled.floor() as i32;

        // Position within the cell (0.0 to 1.0)
        let xf = x_scaled - xi as f64;
        let yf = y_scaled - yi as f64;

        // Get pseudo-random gradients at each corner
        let g00 = Self::gradient_hash(xi, yi, seed);
        let g10 = Self::gradient_hash(xi + 1, yi, seed);
        let g01 = Self::gradient_hash(xi, yi + 1, seed);
        let g11 = Self::gradient_hash(xi + 1, yi + 1, seed);

        // Calculate dot products
        let n00 = Self::dot_product(g00, xf, yf);
        let n10 = Self::dot_product(g10, xf - 1.0, yf);
        let n01 = Self::dot_product(g01, xf, yf - 1.0);
        let n11 = Self::dot_product(g11, xf - 1.0, yf - 1.0);

        // Smooth interpolation function (Perlin's improved version)
        let u = Self::smoothstep(xf);
        let v = Self::smoothstep(yf);

        // Bilinear interpolation
        let nx0 = Self::lerp(n00, n10, u);
        let nx1 = Self::lerp(n01, n11, u);
        let result = Self::lerp(nx0, nx1, v);

        // Normalize to 0.0-1.0 range
        ((result + 1.0) / 2.0).clamp(0.0, 1.0)
    }

    /// Smoothstep interpolation function for smooth transitions
    /// Uses the improved Perlin smoothstep: 3t² - 2t³
    fn smoothstep(t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }

    /// Linear interpolation between two values
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    /// Compute dot product of pseudo-random gradient and distance vector
    fn dot_product(gradient: (f64, f64), x: f64, y: f64) -> f64 {
        gradient.0 * x + gradient.1 * y
    }

    /// Generate pseudo-random gradient vector at a grid point
    /// Returns a unit-like vector based on hash of coordinates
    fn gradient_hash(x: i32, y: i32, seed: u64) -> (f64, f64) {
        // Combine coordinates and seed for deterministic randomness
        let hash = (x as u64).wrapping_mul(374761393)
            .wrapping_add((y as u64).wrapping_mul(668265263))
            .wrapping_add(seed);

        // Mix the bits
        let hash = hash ^ (hash >> 13);
        let hash = hash.wrapping_mul(1274126177);
        let hash = hash ^ (hash >> 16);

        // Select one of 4 simple gradients based on hash
        let gradient_index = (hash % 4) as usize;

        match gradient_index {
            0 => (1.0, 0.0),      // Right
            1 => (-1.0, 0.0),     // Left
            2 => (0.0, 1.0),      // Up
            _ => (0.0, -1.0),     // Down
        }
    }

    /// Generate exits for the zone on the edges with procedural width (1-4 tiles)
    fn generate_exits(num_exits: usize, rng: &mut SimpleRng) -> Vec<Exit> {
        let mut exits = Vec::with_capacity(num_exits);
        let mut used_directions = HashSet::new();

        for _ in 0..num_exits {
            // Pick a random direction that hasn't been used yet
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

            // Random exit width between 1 and 4 tiles
            let width = 1 + (rng.next() % 4) as usize;

            // Place exit on the appropriate edge based on direction
            // Ensure the exit doesn't go out of bounds
            let (x, y) = match direction {
                ExitDirection::North => {
                    let max_x = if ZONE_SIZE > width { ZONE_SIZE - width } else { 0 };
                    ((rng.next() as usize % (max_x + 1)).min(max_x), 0)
                }
                ExitDirection::South => {
                    let max_x = if ZONE_SIZE > width { ZONE_SIZE - width } else { 0 };
                    ((rng.next() as usize % (max_x + 1)).min(max_x), ZONE_SIZE - 1)
                }
                ExitDirection::East => {
                    let max_y = if ZONE_SIZE > width { ZONE_SIZE - width } else { 0 };
                    (ZONE_SIZE - 1, (rng.next() as usize % (max_y + 1)).min(max_y))
                }
                ExitDirection::West => {
                    let max_y = if ZONE_SIZE > width { ZONE_SIZE - width } else { 0 };
                    (0, (rng.next() as usize % (max_y + 1)).min(max_y))
                }
            };

            exits.push(Exit { x, y, direction, width });
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
        self.tiles.iter()
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

    /// Generate next pseudo-random number using Linear Congruential Generator
    fn next(&mut self) -> u64 {
        // Standard LCG parameters for good randomness
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

    #[test]
    fn test_tile_clustering() {
        // Test that tiles of the same type tend to be grouped together
        let zone = Zone::generate("test_zone".to_string(), 12345);

        // Count adjacent same-type tiles
        let mut adjacent_same_type = 0;
        let mut total_adjacent = 0;

        for y in 0..ZONE_SIZE - 1 {
            for x in 0..ZONE_SIZE - 1 {
                let current = zone.tiles[y][x].surface_type;

                // Check right neighbor
                if zone.tiles[y][x + 1].surface_type == current {
                    adjacent_same_type += 1;
                }
                total_adjacent += 1;

                // Check bottom neighbor
                if zone.tiles[y + 1][x].surface_type == current {
                    adjacent_same_type += 1;
                }
                total_adjacent += 1;
            }
        }

        // With Perlin noise, at least 50% of adjacent pairs should be the same type
        // (much higher than random which would be ~40%)
        let clustering_ratio = adjacent_same_type as f64 / total_adjacent as f64;
        assert!(clustering_ratio > 0.50,
                "Clustering ratio should be > 0.50, got {}", clustering_ratio);
    }
}