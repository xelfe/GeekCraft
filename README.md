# GeekCraft

<img src="https://img.shields.io/badge/version-0.1.0-blue">
<img src="https://img.shields.io/badge/rust-1.70+-orange">
<img src="https://img.shields.io/badge/license-MIT-green">

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
- Rust Game Engine
- JavaScript Programming
- WebSocket/REST API (Axum)
- Headless by Design
- Example HTML Viewer
- Secure JavaScript Sandbox
- World Simulation
- Open API

## Installation

Prerequisites
- Rust 1.70+ (rustup.rs)
- Cargo
- Optional: Node.js 18+ for Node examples

Build
```bash
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft
cargo build --release
```

Start the server
```bash
cargo run --release
# HTTP: http://localhost:3030
# WebSocket: ws://localhost:3030/ws
# Override port: export GEEKCRAFT_PORT=3030
```

## Quick Start (Server + Client)

1) Start the server
```bash
cargo run --release
```

2) Submit your bot code (HTTP API)
```bash
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "myplayer",
    "code": "class MyBot { onTick(gameState) { console.log(\\"Hello\\"); } }"
  }'
```

3) Connect a client
- Minimal WebSocket client:
```javascript
const ws = new WebSocket('ws://localhost:3030/ws');
ws.onopen = () => ws.send(JSON.stringify({ type: 'getGameState' }));
ws.onmessage = (e) => console.log('Received:', JSON.parse(e.data));
```
- HTML Viewer (example):
```bash
cd examples/viewer
# macOS: open index.html
# Linux: xdg-open index.html
# Windows: double-click index.html
```
- Node.js example:
```bash
npm install ws node-fetch
node examples/node_client_example.js
```

More details and examples: see examples/README.md.

## HTTP and WebSocket API
- Base URL: http://localhost:3030
- WebSocket: ws://localhost:3030/ws

Endpoints
- GET / — API info
- GET /api/health — Health check
- POST /api/submit — Submit player code (body: { "player_id": "string", "code": "string" })
- GET /api/players — List players
- GET /api/gamestate — Current game state

Note: CORS is permissive during development; restrict origins for production.

## Create Your First Bot
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
Submit via the HTTP API as shown above.

## Useful Commands

```bash
cargo run                     # Debug run
cargo build --release         # Release build
cargo test                    # Tests
cargo doc --open              # Docs

# Quick API checks
curl http://localhost:3030/api/health
curl http://localhost:3030/api/players
curl http://localhost:3030/api/gamestate
```

## Roadmap

- [x] Basic project structure
- [x] JavaScript API for bots
- [x] WebSocket/REST server (Axum)
- [ ] World simulation engine
- [ ] Secure JavaScript sandbox
- [ ] Basic graphical interface
- [ ] Resource system
- [ ] Combat system
- [ ] Multiplayer
- [ ] Replays and statistics

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