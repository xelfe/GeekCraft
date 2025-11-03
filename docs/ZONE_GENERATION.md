# Zone Generation System

## Overview

The Zone Generation System provides procedural landscape generation for player starting zones in GeekCraft. Each player begins in a unique 30x30 tile zone with procedurally generated terrain.

## Features

- **Procedural Generation**: Each zone is generated using a deterministic algorithm based on the player ID
- **Three Terrain Types**:
  - **Plain**: Walkable, standard movement (~60% of tiles)
  - **Swamp**: Walkable, slower movement (~25% of tiles)
  - **Obstacle**: Not walkable, blocks movement (~15% of tiles)
- **Multiple Exits**: Each zone has 2-4 exits placed on the edges for future zone interconnection
- **Deterministic**: Same player ID always generates the same zone layout
- **Server-Side**: All generation logic implemented in Rust for security and performance

## Architecture

### Rust Components

#### `src/game/zone.rs`
Core zone generation module containing:
- `Zone` struct: Represents a complete zone with tiles and exits
- `Tile` struct: Individual tile with coordinates and surface type
- `SurfaceType` enum: Plain, Swamp, Obstacle
- `Exit` struct: Exit point with direction (North, South, East, West)
- Procedural generation algorithm using pseudo-random number generation

#### `src/game/world.rs`
World management with zone support:
- `World::add_zone()`: Add a zone to the world
- `World::get_zone()`: Retrieve a zone by ID
- `World::generate_player_zone()`: Generate and add a new player zone
- `World::get_zone_ids()`: List all zone IDs
- HashMap-based zone storage for future multi-zone world support

#### `src/network/zone_routes.rs`
API endpoints for zone operations:
- `POST /api/zone/generate`: Generate a new zone for a player
- `GET /api/zone/:zone_id`: Retrieve zone data
- `GET /api/zones`: List all zone IDs

### JavaScript Examples

#### `examples/zone_generation_example.js`
Complete example demonstrating:
- Zone generation
- Zone data retrieval
- Terrain analysis
- ASCII visualization
- Multi-player zone management

## API Reference

### Generate Zone

Generate a new zone for a player.

**Endpoint**: `POST /api/zone/generate`

**Request**:
```json
{
  "player_id": "player1"
}
```

**Response**:
```json
{
  "success": true,
  "message": "Zone generated successfully for player player1",
  "zone_id": "player_player1_zone"
}
```

### Get Zone

Retrieve zone data by ID.

**Endpoint**: `GET /api/zone/:zone_id`

**Response**:
```json
{
  "success": true,
  "message": "Zone player_player1_zone retrieved successfully",
  "zone": {
    "id": "player_player1_zone",
    "tiles": [
      [
        {
          "x": 0,
          "y": 0,
          "surface_type": "Plain"
        },
        ...
      ],
      ...
    ],
    "exits": [
      {
        "x": 29,
        "y": 15,
        "direction": "East"
      },
      ...
    ]
  }
}
```

### List Zones

Get all zone IDs in the world.

**Endpoint**: `GET /api/zones`

**Response**:
```json
{
  "success": true,
  "message": "Found 3 zones",
  "zone_ids": [
    "player_player1_zone",
    "player_player2_zone",
    "player_player3_zone"
  ]
}
```

## Zone Structure

### Tile Grid
- 30x30 tiles (900 tiles total)
- Each tile has x, y coordinates (0-29)
- Each tile has a surface type

### Surface Type Distribution
Based on the procedural algorithm:
- **Plain**: ~60% of tiles
- **Swamp**: ~25% of tiles
- **Obstacle**: ~15% of tiles

### Exits
- 2-4 exits per zone
- Placed on zone edges (top, bottom, left, right)
- Each exit has a direction (North, South, East, West)
- Exits are evenly distributed across different edges

## Usage Examples

### Basic Zone Generation

```javascript
const response = await fetch('http://localhost:3030/api/zone/generate', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ player_id: 'alice' })
});

const data = await response.json();
console.log('Generated zone:', data.zone_id);
// Output: "Generated zone: player_alice_zone"
```

### Retrieve and Analyze Zone

```javascript
const response = await fetch('http://localhost:3030/api/zone/player_alice_zone');
const data = await response.json();
const zone = data.zone;

// Count terrain types
let plainCount = 0;
let swampCount = 0;
let obstacleCount = 0;

for (const row of zone.tiles) {
  for (const tile of row) {
    if (tile.surface_type === 'Plain') plainCount++;
    if (tile.surface_type === 'Swamp') swampCount++;
    if (tile.surface_type === 'Obstacle') obstacleCount++;
  }
}

console.log('Plain tiles:', plainCount);
console.log('Swamp tiles:', swampCount);
console.log('Obstacle tiles:', obstacleCount);
console.log('Exits:', zone.exits.length);
```

### Complete Example

Run the included example:
```bash
node examples/zone_generation_example.js
```

This example demonstrates:
- Zone generation
- Zone retrieval
- Terrain analysis
- ASCII visualization
- Multi-player zones

## Future Enhancements

### Multi-Zone World (Planned)
- Zone interconnection through exits
- Player movement between zones
- World-level pathfinding
- Zone-to-zone resource trading

### Advanced Terrain Features (Planned)
- Resource nodes (minerals, energy)
- Strategic points
- Spawn locations for neutral units
- Terrain height levels

### Persistence
- Zones are currently stored in memory
- Can be integrated with the campaign save system
- MongoDB support for persistent world state

## Testing

### Unit Tests
Run zone generation tests:
```bash
cargo test game::zone::tests
```

Tests include:
- Zone generation
- Deterministic generation
- Different seeds produce different zones
- Exit count validation
- Surface type distribution
- Tile retrieval

### Integration Tests
Run world integration tests:
```bash
cargo test integration_tests
```

Tests include:
- Zone generation and world integration
- Multiple zones in world
- Deterministic generation for same player
- Zone listing and retrieval

### Manual Testing
1. Start the server:
   ```bash
   cargo run --release
   ```

2. Generate a zone:
   ```bash
   curl -X POST http://localhost:3030/api/zone/generate \
     -H "Content-Type: application/json" \
     -d '{"player_id": "test"}'
   ```

3. Retrieve zone data:
   ```bash
   curl http://localhost:3030/api/zone/player_test_zone
   ```

4. Run the JavaScript example:
   ```bash
   node examples/zone_generation_example.js
   ```

## Technical Details

### Procedural Generation Algorithm

The zone generation uses a simple but effective approach:

1. **Seed Generation**: Player ID is hashed to create a deterministic seed
2. **Pseudo-Random Number Generator**: Simple LCG (Linear Congruential Generator)
3. **Noise Function**: Position-based hash function for terrain distribution
4. **Surface Type Assignment**: 
   - noise < 0.60 → Plain
   - 0.60 ≤ noise < 0.85 → Swamp
   - noise ≥ 0.85 → Obstacle
5. **Exit Placement**: Random positions on edges with direction constraints

### Performance Considerations

- Zone generation is fast (<1ms for a 30x30 zone)
- Zones are cached in memory after generation
- Serialization to JSON is efficient with serde
- Future: Consider lazy loading for very large worlds

### Security

- All generation is server-side (no client manipulation)
- Player ID is validated and sanitized
- Deterministic generation prevents cheating
- No sensitive data in zone structure

## Code Structure

```
src/game/
├── zone.rs              # Zone generation core
├── world.rs             # World with zone management
└── mod.rs               # Module declarations

src/network/
├── zone_routes.rs       # Zone API endpoints
├── server.rs            # Server with zone routes
└── mod.rs               # Module declarations

examples/
└── zone_generation_example.js  # Complete usage example

tests/
└── integration_tests.rs        # Zone integration tests
```

## Contributing

When contributing to the zone generation system:

1. **Maintain Determinism**: Zone generation must be deterministic for same player ID
2. **Write Tests**: Add unit and integration tests for new features
3. **Document Changes**: Update this document and inline code comments
4. **Performance**: Keep generation fast (<10ms per zone)
5. **English**: All code and comments must be in English

## Version History

- **v0.2.0**: Initial zone generation implementation
  - 30x30 tile zones
  - Three terrain types (Plain, Swamp, Obstacle)
  - 2-4 exits per zone
  - Deterministic generation
  - REST API endpoints
  - JavaScript example
