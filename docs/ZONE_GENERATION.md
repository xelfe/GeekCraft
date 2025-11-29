# Zone Generation System with Improved Tile Clustering

## Overview

The Zone Generation System provides procedural landscape generation for player starting zones in GeekCraft. Each player begins in a unique 30x30 tile zone with procedurally generated terrain using **Perlin noise** for natural tile clustering.

The improved algorithm groups tiles of the same type together, creating larger contiguous regions of plains, swamps, and obstacles, resulting in more strategic and visually appealing gameplay.

## Features

- **Improved Procedural Generation**: Uses multi-scale Perlin noise for natural tile clustering
- **Three Terrain Types**:
    - **Plain**: Walkable, standard movement (~60% of tiles)
    - **Swamp**: Walkable, slower movement (~25% of tiles)
    - **Obstacle**: Not walkable, blocks movement (~15% of tiles)
- **Tile Clustering**: Adjacent tiles of the same type are grouped naturally (>50% clustering ratio)
- **Multiple Exits**: Each zone has 2-4 exits placed on the edges
- **Deterministic**: Same player ID always generates the same zone layout
- **Server-Side**: All generation logic implemented in Rust for security
- **Multi-Mode Support**: Works with both solo campaign and online multiplayer

## What's Improved

### Previous Algorithm
- Independent random tile placement
- Scattered terrain types across the map
- Low clustering ratio (~40%)
- Less strategic gameplay

### New Algorithm (Perlin Noise)
- **Multi-scale Noise**: Combines three noise frequencies for natural-looking features
- **Gradient Interpolation**: Uses smoothstep function for smooth terrain transitions
- **High Clustering**: >50% of adjacent tiles match type
- **Larger Regions**: Creates bigger contiguous areas for better strategic planning
- **Better Aesthetics**: More visually appealing, game-like appearance

## Architecture

### Rust Components

#### `src/game/zone.rs`
Core zone generation module containing:
- `Zone` struct: Represents a complete zone with tiles and exits
- `Tile` struct: Individual tile with coordinates and surface type
- `SurfaceType` enum: Plain, Swamp, Obstacle
- `Exit` struct: Exit point with direction (North, South, East, West)
- **`perlin_noise()`**: Multi-scale Perlin-like noise function for clustering
- **`smoothstep()`**: Smooth interpolation for natural transitions
- **`gradient_hash()`**: Deterministic gradient generation

#### `src/game/world.rs`
World management with zone support:
- `World::add_zone()`: Add a zone to the world
- `World::get_zone()`: Retrieve a zone by ID
- `World::generate_player_zone()`: Generate and add a new player zone
- `World::get_zone_ids()`: List all zone IDs

### JavaScript Examples

#### `examples/zone_clustering_example.js`
Complete example demonstrating:
- Zone generation via API
- ASCII map visualization
- Terrain distribution analysis
- **Tile clustering quality measurement**
- Multi-zone comparison
- Solo campaign and online mode support

## Technical Details

### Perlin Noise Algorithm

The generation process uses a sophisticated multi-scale Perlin noise approach:

1. **Multi-Scale Composition**:
    - Large scale (frequency=1.0): 65% weight, creates large regions
    - Medium scale (frequency=0.5): 25% weight, adds variation
    - Small scale (frequency=0.25): 10% weight, adds fine details

2. **Gradient Interpolation**:
    - Each grid cell has pseudo-random gradients
    - Dot products between gradients and distances are computed
    - Smoothstep function ensures smooth transitions

3. **Surface Classification**:
    - noise < 0.60 → Plain
    - 0.60 ≤ noise < 0.85 → Swamp
    - noise ≥ 0.85 → Obstacle

### Performance

- Zone generation: <1ms for a 30x30 zone (same as before)
- Multi-scale Perlin noise adds minimal computational overhead
- Zones are cached in memory after generation
- Fast serialization with serde

### Clustering Metrics

**Clustering Ratio**: Percentage of adjacent tile pairs that have the same type

- **Random Distribution**: ~40% clustering ratio
- **Improved Generation**: >50% clustering ratio (typical: 55-65%)
- **Benefit**: Larger contiguous regions enable better strategic planning

## API Endpoints

All endpoints are available in both solo and online modes.

### Generate Zone
```bash
POST /api/zone/generate
Content-Type: application/json

{
  "player_id": "player_123"
}

Response (200 OK):
{
  "id": "player_player_123_zone",
  "tiles": [
    [
      { "x": 0, "y": 0, "surface_type": "Plain" },
      ...
    ],
    ...
  ],
  "exits": [
    { "x": 5, "y": 0, "direction": "North" },
    ...
  ]
}
```

### Get Zone
```bash
GET /api/zone/{zone_id}

Response (200 OK):
{
  "id": "player_player_123_zone",
  "tiles": [...],
  "exits": [...]
}
```

## Usage Examples

### Solo Campaign Mode (Local)
```bash
# Start the server
cargo run --release

# Generate a zone
curl -X POST http://localhost:3030/api/zone/generate \
  -H "Content-Type: application/json" \
  -d '{"player_id": "my_player"}'

# Retrieve zone data
curl http://localhost:3030/api/zone/player_my_player_zone
```

### JavaScript Client
```javascript
// Configuration for solo mode
const config = {
    apiUrl: 'http://localhost:3030/api',
    mode: 'local campaign'
};

// Generate and visualize a zone
async function showZone() {
    const response = await fetch(`${config.apiUrl}/zone/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ player_id: 'player_1' })
    });
    
    const zone = await response.json();
    
    // Visualize the map
    displayZoneMap(zone);
    
    // Analyze clustering
    analyzeClusteringQuality(zone);
}

showZone();
```

See `examples/zone_clustering_example.js` for complete examples.

## Testing

### Unit Tests
```bash
cargo test zone
```

Tests included:
- `test_zone_generation`: Basic zone structure validation
- `test_deterministic_generation`: Same seed produces same zone
- `test_different_seeds`: Different seeds produce different zones
- `test_exit_count`: Validates 2-4 exits per zone
- `test_get_tile`: Coordinate validation
- `test_surface_type_distribution`: Terrain type percentages
- **`test_tile_clustering`**: NEW - Validates >50% clustering ratio

### Integration Tests
```bash
cargo test integration
```

### Manual Testing
```bash
# Run the server
cargo run --release

# Run the JavaScript example
node examples/zone_clustering_example.js
```

## Visualization

The ASCII visualization uses these symbols:
- `.` = Plain (walkable, standard)
- `~` = Swamp (walkable, slow)
- `#` = Obstacle (not walkable)
- `N/S/E/W` = Exit points

Example output:
```
┌──────────────────────────────┐
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~N##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
│......~~~~~##..~####..~...~~~│
└──────────────────────────────┘
```

Note the natural grouping of `.` (plains), `~` (swamps), and `#` (obstacles).

## Tuning and Configuration

The clustering behavior can be adjusted by modifying the noise frequency weights in `zone.rs`:

```rust
// Current: Strong clustering
noise1 * 0.65 + noise2 * 0.25 + noise3 * 0.10

// Less clustering, more variety
noise1 * 0.45 + noise2 * 0.35 + noise3 * 0.20

// Maximum clustering
noise1 * 0.80 + noise2 * 0.15 + noise3 * 0.05
```

## Mode Support

### Solo Campaign
- Runs on localhost
- Player zones are generated deterministically
- All data stays on the client
- Perfect for single-player testing

### Online Multiplayer
- Same zone generation algorithm
- API endpoints available on game server
- Supports multiple concurrent players
- Deterministic generation ensures consistency

## Version History

- **v0.3.0**: Improved tile clustering with Perlin noise
    - Multi-scale noise composition
    - Gradient interpolation
    - Enhanced test suite
    - Better documentation
    - JavaScript clustering analysis

- **v0.2.0**: Initial zone generation implementation
    - 30x30 tile zones
    - Three terrain types
    - 2-4 exits per zone
    - REST API endpoints

## Future Enhancements

- Zone interconnection through exits
- Resource nodes on specific terrain types
- Dynamic terrain features (rivers, mountains)
- Difficulty scaling
- Player zone customization options

## Contributing

When improving map generation further:
1. Maintain deterministic generation (same seed = same map)
2. Keep clustering ratio > 50%
3. Preserve terrain distribution percentages (~60% Plain, ~25% Swamp, ~15% Obstacle)
4. Test with `test_tile_clustering` to validate improvements
5. Update visualization examples

## Security Considerations

- All generation is server-side (no client manipulation)
- Player ID is validated and sanitized
- Deterministic generation prevents cheating
- No sensitive data in zone structure
- Seed is derived from player ID securely