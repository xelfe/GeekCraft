use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Custom distribution implementation
pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

pub struct WeightedIndex {
    cumulative_weights: Vec<f64>,
}

impl WeightedIndex {
    pub fn new(weights: &[f64]) -> Result<Self, &'static str> {
        if weights.is_empty() {
            return Err("WeightedIndex requires non-empty weights");
        }

        let mut cumulative_weights = Vec::with_capacity(weights.len());
        let mut acc = 0.0;

        for &weight in weights {
            if weight < 0.0 {
                return Err("Negative weights not allowed");
            }
            acc += weight;
            cumulative_weights.push(acc);
        }

        if acc == 0.0 {
            return Err("All weights are zero");
        }

        Ok(WeightedIndex { cumulative_weights })
    }
}

impl Distribution<usize> for WeightedIndex {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let target = rng.gen::<f64>() * self.cumulative_weights.last().unwrap();

        // Binary search to find the index
        match self.cumulative_weights.binary_search_by(|w| {
            if *w < target {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }) {
            Ok(i) => i,
            Err(i) => i,
        }
    }
}

/// Configuration for map generation clustering behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringConfig {
    /// Base probability that an adjacent tile will be the same type (0.0 to 1.0)
    pub clustering_strength: f64,
    /// Minimum cluster size for each tile type
    pub min_cluster_size: usize,
    /// Maximum cluster size for each tile type
    pub max_cluster_size: usize,
    /// Falloff rate for clustering probability with distance (higher = faster falloff)
    pub distance_falloff: f64,
}

impl Default for ClusteringConfig {
    fn default() -> Self {
        Self {
            clustering_strength: 0.65,
            min_cluster_size: 4,
            max_cluster_size: 20,
            distance_falloff: 0.3,
        }
    }
}

/// Different game modes with their own clustering configurations
impl ClusteringConfig {
    pub fn solo_campaign() -> Self {
        Self {
            clustering_strength: 0.70,
            min_cluster_size: 6,
            max_cluster_size: 25,
            distance_falloff: 0.25,
        }
    }

    pub fn online_mode() -> Self {
        Self {
            clustering_strength: 0.60,
            min_cluster_size: 4,
            max_cluster_size: 18,
            distance_falloff: 0.35,
        }
    }
}

/// Tile types that can appear on the map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Forest,
    Mountain,
    Water,
    Desert,
    Snow,
}

impl TileType {
    /// Get all available tile types
    pub fn all_types() -> Vec<TileType> {
        vec![
            TileType::Grass,
            TileType::Forest,
            TileType::Mountain,
            TileType::Water,
            TileType::Desert,
            TileType::Snow,
        ]
    }

    /// Get base spawn weight for random selection
    pub fn base_weight(&self) -> f64 {
        match self {
            TileType::Grass => 30.0,
            TileType::Forest => 25.0,
            TileType::Mountain => 15.0,
            TileType::Water => 15.0,
            TileType::Desert => 10.0,
            TileType::Snow => 5.0,
        }
    }
}

/// Represents a single tile on the map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

/// Map structure containing all tiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
}

impl GameMap {
    /// Generate a new map with improved clustering algorithm
    pub fn generate(width: usize, height: usize, config: ClusteringConfig) -> Self {
        let mut rng = rand::rng();
        let mut tiles = vec![vec![TileType::Grass; width]; height];
        let mut visited = vec![vec![false; width]; height];

        // Track cluster sizes to ensure variety
        let mut cluster_counts: HashMap<TileType, usize> = HashMap::new();

        // Start with random seed points for each tile type
        let num_seeds = (width * height / 30).max(6);

        for _ in 0..num_seeds {
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);

            if !visited[y][x] {
                let tile_type = Self::weighted_random_tile(&mut rng);
                Self::grow_cluster(
                    &mut tiles,
                    &mut visited,
                    &mut cluster_counts,
                    x,
                    y,
                    tile_type,
                    &config,
                    &mut rng,
                );
            }
        }

        // Fill remaining tiles with clustering awareness
        for y in 0..height {
            for x in 0..width {
                if !visited[y][x] {
                    let tile_type = Self::choose_tile_with_neighbors(
                        &tiles,
                        &visited,
                        x,
                        y,
                        width,
                        height,
                        &config,
                        &mut rng,
                    );
                    tiles[y][x] = tile_type;
                    visited[y][x] = true;
                }
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    /// Grow a cluster from a seed point using flood-fill approach
    fn grow_cluster<R: Rng>(
        tiles: &mut Vec<Vec<TileType>>,
        visited: &mut Vec<Vec<bool>>,
        cluster_counts: &mut HashMap<TileType, usize>,
        start_x: usize,
        start_y: usize,
        tile_type: TileType,
        config: &ClusteringConfig,
        rng: &mut R,
    ) {
        let height = tiles.len();
        let width = tiles[0].len();
        let mut stack = vec![(start_x, start_y, 0)]; // (x, y, distance_from_seed)

        let target_size = rng.random_range(config.min_cluster_size..=config.max_cluster_size);
        let mut current_size = 0;

        while let Some((x, y, distance)) = stack.pop() {
            if x >= width || y >= height || visited[y][x] || current_size >= target_size {
                continue;
            }

            // Calculate probability based on distance from seed
            let distance_factor = (-config.distance_falloff * distance as f64).exp();
            let placement_prob = config.clustering_strength * distance_factor;

            if rng.random::<f64>() > placement_prob && distance > 0 {
                continue;
            }

            tiles[y][x] = tile_type;
            visited[y][x] = true;
            current_size += 1;

            // Add neighbors to stack (4-directional connectivity)
            let neighbors = [
                (x.wrapping_sub(1), y, distance + 1),
                (x + 1, y, distance + 1),
                (x, y.wrapping_sub(1), distance + 1),
                (x, y + 1, distance + 1),
            ];

            for (nx, ny, nd) in neighbors {
                if nx < width && ny < height && !visited[ny][nx] {
                    stack.push((nx, ny, nd));
                }
            }
        }

        *cluster_counts.entry(tile_type).or_insert(0) += current_size;
    }

    /// Choose a tile type based on neighboring tiles and random weights
    fn choose_tile_with_neighbors<R: Rng>(
        tiles: &Vec<Vec<TileType>>,
        visited: &Vec<Vec<bool>>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        config: &ClusteringConfig,
        rng: &mut R,
    ) -> TileType {
        // use rand::distributions::{Distribution, WeightedIndex}; // Removed to use local implementation
        let mut neighbor_weights: HashMap<TileType, f64> = HashMap::new();

        // Check all 8 neighbors
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    // CRITICAL FIX: Only consider VISITED neighbors.
                    // Unvisited tiles are initialized to Grass by default, which causes massive bias.
                    if visited[ny as usize][nx as usize] {
                        let neighbor_type = tiles[ny as usize][nx as usize];
                        *neighbor_weights.entry(neighbor_type).or_insert(0.0) +=
                            config.clustering_strength * 10.0;
                    }
                }
            }
        }

        // Add base weights for variety
        for tile_type in TileType::all_types() {
            *neighbor_weights.entry(tile_type).or_insert(0.0) += tile_type.base_weight();
        }

        // Unzip to ensure keys and values are aligned
        let (types, weights): (Vec<TileType>, Vec<f64>) = neighbor_weights.into_iter().unzip();

        if let Ok(dist) = WeightedIndex::new(&weights) {
            types[dist.sample(rng)]
        } else {
            // Should not happen given we added base weights, but safe fallback
            TileType::Grass 
        }
    }

    /// Select a random tile type based on base weights
    fn weighted_random_tile<R: rand::Rng>(rng: &mut R) -> TileType {
        // use rand::distributions::{Distribution, WeightedIndex}; // Removed to use local implementation
        
        let types = [
            TileType::Grass,
            TileType::Forest,
            TileType::Mountain,
            TileType::Water,
            TileType::Desert,
            TileType::Snow,
        ];

        let weights: Vec<f64> = types.iter().map(|t| t.base_weight()).collect();
        
        // Unwrap is safe because base_weights are hardcoded and valid
        let dist = WeightedIndex::new(&weights).expect("Invalid weights");
        
        types[dist.sample(rng)]
    }

    /// Convert map to JSON for API response
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "width": self.width,
            "height": self.height,
            "tiles": self.tiles
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_generation() {
        let map = GameMap::generate(50, 50, ClusteringConfig::default());
        assert_eq!(map.width, 50);
        assert_eq!(map.height, 50);
        assert_eq!(map.tiles.len(), 50);
        assert_eq!(map.tiles[0].len(), 50);
    }

    #[test]
    fn test_clustering_configs() {
        let solo = ClusteringConfig::solo_campaign();
        let online = ClusteringConfig::online_mode();

        assert!(solo.clustering_strength > online.clustering_strength);
        assert!(solo.max_cluster_size > online.max_cluster_size);
    }

    #[test]
    fn test_tile_variety() {
        let map = GameMap::generate(30, 30, ClusteringConfig::default());
        let mut type_counts: HashMap<TileType, usize> = HashMap::new();

        for row in &map.tiles {
            for tile in row {
                *type_counts.entry(*tile).or_insert(0) += 1;
            }
        }

        // Ensure we have multiple tile types (variety)
        assert!(type_counts.len() >= 3);
    }
}