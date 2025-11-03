# GeekCraft

<img src="https://img.shields.io/badge/version-0.2.0--alpha-blue">
<img src="https://img.shields.io/badge/rust-1.70+-orange">
<img src="https://img.shields.io/badge/license-MIT-green">
<img src="https://img.shields.io/badge/status-pre--release-yellow">

> **🚧 Pre-release Alpha Version 0.2.0** - This is an early alpha release. Core infrastructure is complete (authentication, multiplayer, zone generation, campaign system), but the full game simulation and JavaScript sandbox execution are still in development. See [Features](#features) for current implementation status, or check [FEATURES.md](FEATURES.md) for detailed feature documentation.

## Description

GeekCraft is a programming game inspired by Screeps and Starcraft, where players write JavaScript bots to control units in a real-time strategy environment.

The game engine is headless by design — it provides no integrated graphical interface. Players are free to create their own visualization using any technology they choose. A minimal HTML viewer is included as a simple example.

## Design Principles
- Server in Rust
- Player Freedom (any scripting language via API, JavaScript example provided)
- English First
- Working Examples
- Simple Viewer

## Project Structure
```
GeekCraft
├── src
│   ├── main.rs              # Entry point, initializes the server and starts the engine
│   ├── lib.rs               # Main library, exports modules
│   ├── game
│   │   ├── mod.rs
│   │   ├── world.rs
│   │   ├── entities.rs
│   │   └── simulation.rs
│   ├── api
│   │   ├── mod.rs
│   │   ├── scripting.rs     # Scripting system for bots (storage/execution)
│   │   └── events.rs
│   ├── network
│   │   ├── mod.rs
│   │   └── server.rs        # Axum HTTP + WebSocket server
│   └── scripting
│       ├── mod.rs
│       └── sandbox.rs
├── assets
│   └── textures
├── examples
│   ├── basic_bot.js
│   ├── advanced_bot.js
│   ├── template_bot.js
│   ├── API_REFERENCE.md
│   ├── README.md            # API usage and client quick start
│   ├── api_client_example.js
│   ├── node_client_example.js
│   └── viewer
│       ├── index.html
│       ├── viewer.js
│       └── style.css
├── tests
│   └── integration_tests.rs
├── Cargo.toml
├── BUILD.md
└── README.md
```

## Features

**✅ Implemented in v0.2.0:**
- ✅ Rust Game Engine with Tokio async runtime
- ✅ Axum HTTP/WebSocket Server
- ✅ **Authentication System** (token-based, bcrypt password hashing)
- ✅ **Multiplayer Support** (concurrent users with session management)
- ✅ **Flexible Database** (In-Memory for dev, MongoDB for production)
- ✅ **Procedural Zone Generation** (30x30 tile zones with terrain types and exits)
- ✅ **Campaign System** (start, stop, save, load campaign runs)
- ✅ REST API for code submission and game state
- ✅ WebSocket for real-time multiplayer communication
- ✅ Code validation and storage
- ✅ Basic world simulation (tick counter, terrain, resources)
- ✅ Integration tests
- ✅ Example HTML Viewer (with authentication)
- ✅ Comprehensive API documentation
- ✅ Working JavaScript examples (authentication, multiplayer, zones, campaigns)

**🚧 In Development:**
- 🚧 JavaScript Sandbox execution (code storage implemented, Boa/Deno execution pending)
- 🚧 Entity and movement systems (structure exists, integration pending)
- 🚧 Real-time game tick simulation

**📅 Planned:**
- 📅 Combat and building systems
- 📅 Resource collection mechanics
- 📅 Zone interconnection
- 📅 Full game loop with bot execution

## Installation

Prerequisites
- Rust 1.70+ (rustup.rs)
- Cargo
- Optional: Node.js 18+ for Node examples
- Optional: MongoDB server for production (see DATABASE.md)

Build
```bash
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft
cargo build --release
```

For production with MongoDB:
```bash
cargo build --release
```

Start the server
```bash
cargo run --release
# HTTP: http://localhost:3030
# WebSocket: ws://localhost:3030/ws
# Database: In-Memory (default, data lost on restart)
```

For production with MongoDB:
```bash
export GEEKCRAFT_DB_BACKEND=MONGODB
export MONGODB_URL=mongodb://localhost:27017/geekcraft
cargo run --release
```

## Quick Start (Authentication + Multiplayer)

1) **Start the server**
```bash
cargo run --release
```

2) **Register and login**
```bash
# Register
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "player1", "password": "mypassword"}'

# Login (get token)
curl -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "player1", "password": "mypassword"}'
# Response: {"success":true,"token":"...","username":"player1"}
```

3) **Submit your bot code** (requires authentication)
```bash
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{"code": "class MyBot { onTick(gameState) { console.log(\"Hello\"); } }"}'
```

4) **Connect via WebSocket** (multiplayer)
```javascript
// Browser or Node.js
const ws = new WebSocket('ws://localhost:3030/ws');

ws.onmessage = (e) => {
  const msg = JSON.parse(e.data);
  console.log('Received:', msg);
  
  // Authenticate after welcome
  if (msg.type === 'welcome') {
    ws.send(JSON.stringify({ 
      type: 'auth', 
      token: 'YOUR_TOKEN_HERE' 
    }));
  }
};

// Request game state
ws.send(JSON.stringify({ type: 'getGameState' }));
```

5) **Try the working examples**

All examples are tested and working in v0.2.0-alpha:

**Node.js Examples** (require `npm install ws node-fetch@2` in project root):
- `node examples/node_client_example.js` - Complete example with all features
- `node examples/zone_generation_example.js` - Zone generation demo
- `node examples/campaign_local_save_example.js` - Campaign system demo

**Browser Examples** (load directly in browser):
- `examples/auth_example.js` - Authentication workflow
- `examples/multiplayer_example.js` - Multiplayer client
- `examples/api_client_example.js` - HTTP/WebSocket client

**HTML Viewer:**
```bash
cd examples/viewer
# macOS: open index.html
# Linux: xdg-open index.html
# Windows: double-click index.html
```
node examples/node_client_example.js
```

More details and examples: see examples/README.md.

## Procedural Zone Generation

GeekCraft features a procedural zone generation system where each player starts in their own unique 30x30 tile zone:

```bash
# Generate a zone for a player
curl -X POST http://localhost:3030/api/zone/generate \
  -H "Content-Type: application/json" \
  -d '{"player_id": "alice"}'

# Get zone data
curl http://localhost:3030/api/zone/player_alice_zone

# List all zones
curl http://localhost:3030/api/zones

# Run the complete example
node examples/zone_generation_example.js
```

**Zone Features:**
- **30x30 tiles** with procedural terrain
- **Three terrain types**: Plain (~60%), Swamp (~25%), Obstacle (~15%)
- **2-4 exits** per zone for future zone interconnection
- **Deterministic generation**: Same player ID always generates same zone
- **Server-side**: All generation in Rust for security

See [docs/ZONE_GENERATION.md](docs/ZONE_GENERATION.md) for complete documentation.

## HTTP and WebSocket API
- Base URL: http://localhost:3030
- WebSocket: ws://localhost:3030/ws

### Authentication Endpoints (Public)
- `POST /api/auth/register` — Register new user (body: `{"username": "string", "password": "string"}`)
- `POST /api/auth/login` — Login (body: `{"username": "string", "password": "string"}`) → Returns token

### Protected Endpoints (Require `Authorization: Bearer TOKEN`)
- `POST /api/auth/logout` — Logout and invalidate session
- `POST /api/submit` — Submit player code (body: `{"code": "string"}`)
- `GET /api/players` — List players
- `GET /api/gamestate` — Current game state

### Public Endpoints
- `GET /` — API info
- `GET /api/health` — Health check

### Zone Generation Endpoints (Public)
- `POST /api/zone/generate` — Generate zone for player (body: `{"player_id": "string"}`)
- `GET /api/zone/:zone_id` — Get zone data
- `GET /api/zones` — List all zone IDs

### WebSocket Commands
- `{"type": "auth", "token": "YOUR_TOKEN"}` — Authenticate WebSocket connection
- `{"type": "getPlayers"}` — Get list of players (requires auth)
- `{"type": "getGameState"}` — Get current game state (requires auth)

Note: CORS is permissive during development; restrict origins for production.

## Create Your First Bot

1. **Register and login**
```bash
# Register
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "myplayer", "password": "mypassword"}'

# Login and get token
TOKEN=$(curl -s -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "myplayer", "password": "mypassword"}' \
  | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
```

2. **Create your bot**
```bash
cp examples/template_bot.js my_bot.js
```
```javascript
class MyBot {
  onTick(gameState) {
    const units = gameState.getMyUnits();
    // Your logic here
  }
}
module.exports = MyBot;
```

3. **Submit your bot** (using the token from step 1)
```bash
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code": "class MyBot { onTick(gs) { console.log(\"Hello\"); } } module.exports = MyBot;"}'
```

## Useful Commands

```bash
# Development
cargo run                          # Debug run (In-Memory database)
cargo build --release              # Release build
cargo test                         # Run tests
cargo doc --open                   # Generate and open docs

# Production (with MongoDB)
cargo build --release
export GEEKCRAFT_DB_BACKEND=MONGODB
export MONGODB_URL=mongodb://localhost:27017/geekcraft
cargo run --release

# Quick API checks (require authentication)
curl http://localhost:3030/api/health
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/players
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/gamestate
```

## Database Configuration

See [DATABASE.md](DATABASE.md) for detailed database options:
- **In-Memory** (default): Development, zero config, data lost on restart
- **MongoDB** (production): Persistent storage, horizontal scaling, rich queries

## Roadmap

### ✅ Completed (v0.2.0 Alpha)
- [x] Basic project structure
- [x] Complete documentation
- [x] JavaScript API for bots
- [x] Axum HTTP/WebSocket server
- [x] REST API endpoints (health, submit, players, gamestate)
- [x] WebSocket real-time communication  
- [x] **Authentication and authorization (token-based)**
- [x] **Multiplayer session management**
- [x] **Flexible database (In-Memory, MongoDB)**
- [x] **Procedural zone generation (30x30 tiles, terrain types, exits)**
- [x] **Campaign system (start, stop, save, load)**
- [x] Code validation and storage
- [x] Basic world simulation (tick, terrain, resources)
- [x] Integration tests
- [x] **JavaScript examples for all implemented features**
- [x] **HTML viewer with authentication support**

### 🚧 In Progress
- [ ] Full JavaScript sandbox with execution (Boa/Deno)
- [ ] Complete world simulation engine with real-time ticks
- [ ] Entity system for units and buildings (full integration)

### 📅 Planned Features
- [ ] Zone interconnection and world navigation
- [ ] Resource collection and management (gameplay mechanics)
- [ ] Movement system with pathfinding
- [ ] Combat system
- [ ] Building/construction system
- [ ] Tech tree and progression
- [ ] Replays and statistics
- [ ] Tournament system

## Contributing

Contributions are welcome! Here's how to participate:

1. Fork the project
2. Create a branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contact

Project: [https://github.com/xelfe/GeekCraft](https://github.com/xelfe/GeekCraft)