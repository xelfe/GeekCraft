# GeekCraft - Project Summary (v0.2.0-alpha)

> **🚧 Pre-release Alpha** - Core infrastructure complete. Authentication, multiplayer, zone generation, and campaign systems are working. Full game simulation with JavaScript bot execution is in development.

## Overview

**GeekCraft** is a programming game inspired by **Screeps** and **Starcraft**, where players program bots in JavaScript to control units in a real-time strategy environment.

### Key Concept

The game engine is **headless by design** - it provides **no integrated graphics rendering**. Players are **completely free** to create their own visualization interface using the technology of their choice.

## Architecture

### Backend - Game Engine (Rust)

The GeekCraft server is written in Rust for performance and security reasons:

```
GeekCraft (Rust)
├── Simulation engine
│   ├── Game world (grid, resources)
│   ├── Entities (units, buildings)
│   └── Game logic (movement, combat, resources)
│
├── JavaScript sandbox
│   ├── Secure bot execution
│   ├── Time and memory limitations
│   └── Scripting API
│
└── Network server
    ├── WebSocket for real-time
    ├── REST API for requests
    └── Event broadcasting
```

**Features:**
- ⚡ High performance
- 🔒 Secure script execution
- 🌐 Real-time communication
- 📊 Deterministic simulation

### Frontend - Player Bots (JavaScript)

Players program their bots in JavaScript:

```javascript
class MyBot {
    onTick(gameState) {
        // Bot strategy
        const units = gameState.getMyUnits();
        for (const unit of units) {
            // Control logic
        }
    }
}
```

**Available API:**
- 🎮 Unit control
- 📊 Game state access
- 🏗️ Building construction
- ⚔️ Combat management
- 💎 Resource collection

### Visualization - Custom Client (Your choice)

Players create their own interface:

**Available options:**

#### Web
- **HTML5 Canvas** (example provided)
- **React/Vue/Angular**
- **Three.js** (3D)
- **Phaser/PixiJS** (2D gaming)

#### Desktop
- **Unity**
- **Godot**
- **Electron**
- **Tauri**

#### Terminal
- **Blessed** (Node.js)
- **Rich** (Python)
- **Cursive** (Rust)

#### Other
- Anything that can connect via WebSocket!

## Communication

### WebSocket Protocol

```
Player Bot (JS) ←→ Server (Rust) ←→ Viewer (Free)
                     ↓
                 Simulation
```

**Message types:**

```javascript
// Server → Client
{
    "type": "gameState",
    "data": {
        "tick": 123,
        "players": [...],
        "units": [...],
        "resources": [...]
    }
}

// Client → Server  
{
    "type": "command",
    "data": {
        "unitId": 42,
        "action": "moveTo",
        "target": {"x": 100, "y": 200}
    }
}
```

## Game Flow

1. **Player codes their bot** in JavaScript
2. **Bot submitted to server** via API
3. **Server executes bot** each tick
4. **Game state updated** based on actions
5. **Server broadcasts state** via WebSocket
6. **Viewer displays** the game (if connected)

```
[Bot Code] → [Server] → [Simulation]
                ↓
           [WebSocket]
                ↓
          [Your Viewer]
```

## Provided Examples

### 1. JavaScript Bots

#### `basic_bot.js`
Simple bot to get started:
- Movement to resources
- Event handling
- Basic management

#### `advanced_bot.js`
Advanced strategies:
- Unit roles (workers, soldiers)
- Resource management
- Base construction
- Combat tactics

#### `template_bot.js`
Empty template to start with

### 2. HTML Viewer

A complete web client example:
- Modern interface (HTML/CSS/JS)
- Canvas for 2D rendering
- WebSocket connection
- Camera controls
- Unit selection
- Log console

**Location:** `examples/viewer/`

### 3. Documentation

#### `API_REFERENCE.md`
Complete JavaScript API documentation for bots

## Project Structure

```
GeekCraft/
├── src/                    # Rust source code
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library
│   ├── game/              # Game engine
│   │   ├── world.rs       # Game world
│   │   ├── entities.rs    # Entities
│   │   └── simulation.rs  # Simulation
│   ├── api/               # Scripting API
│   │   ├── scripting.rs   # Bot interface
│   │   └── events.rs      # Events
│   ├── network/           # Network server
│   │   └── server.rs      # WebSocket/REST
│   └── scripting/         # JS sandbox
│       └── sandbox.rs     # Secure execution
│
├── examples/              # Examples
│   ├── basic_bot.js      # Simple bot
│   ├── advanced_bot.js   # Advanced bot
│   ├── template_bot.js   # Template
│   ├── API_REFERENCE.md  # API docs
│   └── viewer/           # HTML viewer example
│       ├── index.html
│       ├── viewer.js
│       ├── style.css
│       └── README.md
│
├── tests/                # Tests
├── assets/               # Resources (optional)
├── Cargo.toml           # Rust config
├── BUILD.md             # Build guide
└── README.md            # Documentation
```

## Roadmap

### ✅ Phase 1: Foundations (v0.2.0-alpha - Completed)
- [x] Project structure
- [x] Complete documentation
- [x] JS bot examples
- [x] HTML viewer example
- [x] Headless architecture

### ✅ Phase 2: Base Engine (v0.2.0-alpha - Partially Complete)
- [x] Basic world simulation (tick counter, terrain)
- [x] Resource types (Wood, Stone, Gold)
- [x] Procedural zone generation (30x30 tiles)
- [ ] Complete entity system
- [ ] Advanced resource management
- [ ] Movement logic

### 🚧 Phase 3: Scripting (In Progress)
- [x] Code storage and validation
- [x] Basic scripting engine
- [ ] Full JS engine integration (Boa/Deno)
- [ ] Complete functional scripting API
- [ ] Advanced security limitations
- [ ] Event management

### ✅ Phase 4: Networking (v0.2.0-alpha - Completed)
- [x] Axum HTTP server
- [x] WebSocket server
- [x] REST API endpoints
- [x] CORS configuration
- [x] Authentication (token-based)
- [ ] Multiplayer synchronization (real-time ticks)

### ✅ Phase 5: Database (v0.2.0-alpha - Completed)
- [x] In-Memory database for development
- [x] MongoDB support for production
- [x] Database abstraction layer

### ✅ Phase 6: Campaign System (v0.2.0-alpha - Completed)
- [x] Campaign run management
- [x] Local persistence (save/load)
- [x] Campaign API endpoints

### 📅 Phase 7: Gameplay (Planned)
- [ ] Combat
- [ ] Construction
- [ ] Tech tree
- [ ] Balance

### Phase 6: Advanced 📅
- [ ] Replays
- [ ] Leaderboards
- [ ] Tournaments
- [ ] Reference AI

## Technologies Used

### Backend
- **Rust** 1.70+ - Main language
- **Tokio** - Async runtime
- **Axum 0.7** - Web framework with WebSocket support
- **Tower-HTTP** - CORS and tracing middleware
- **Serde** - JSON serialization
- **Future**: Boa/Deno for full JavaScript sandbox

### Frontend Examples
- **HTML5/CSS3/JavaScript** - Basic viewer
- **Canvas API** - 2D rendering
- **WebSocket API** - Real-time communication

## How to Contribute

1. **Game engine**: Implement simulation
2. **JavaScript API**: Enrich bot features
3. **Documentation**: Improve guides
4. **Examples**: Create more bots and viewers
5. **Tests**: Add unit and integration tests

## Project Philosophy

### Visualization Freedom

GeekCraft imposes **no graphical constraints**. You are free to:
- Create spectacular 3D rendering
- Make a minimalist terminal interface
- Develop a mobile app
- Even display nothing and just log!

### Open Source and Educational

The project aims to:
- Teach programming
- Promote Rust and JavaScript
- Build a developer community
- Share knowledge

### Performance & Security

- Rust code for speed
- Sandbox for security
- Clear and documented API
- Testing and validation

## Quick Start

```bash
# 1. Clone the project
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft

# 2. Build the server
cargo build --release

# 3. Start the server
cargo run --release

# 4. Open the example viewer
open examples/viewer/index.html

# 5. Create your bot
cp examples/template_bot.js my_bot.js
# Edit my_bot.js with your code

# 6. Submit your bot (via API or interface)
```

## Resources

- **Documentation**: `README.md`, `BUILD.md`, `API_REFERENCE.md`
- **Examples**: `examples/` folder
- **Repository**: https://github.com/xelfe/GeekCraft
- **License**: MIT

## Contact and Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Contributions**: Pull Requests welcome!

---

**GeekCraft** - Your game, your code, your vision! 🎮🚀
