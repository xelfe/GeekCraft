# GeekCraft - Final Project Structure (v0.2.0-alpha)

> **🚧 Pre-release Alpha** - This document describes the current structure of GeekCraft v0.2.0-alpha, with all implemented features clearly marked.

## Overview

The GeekCraft project has been updated to reflect its **headless** architecture (no integrated graphics), giving players complete freedom to create their own visualization interfaces.

## Current Structure

```
GeekCraft/
│
├── 📄 README.md              # Main project documentation
├── 📄 BUILD.md               # Detailed build guide
├── 📄 PROJECT_SUMMARY.md     # Complete project summary
├── 📄 Cargo.toml             # Cargo/Rust configuration
├── 📄 LICENSE                # MIT License
├── 📄 .gitignore             # Files to ignore in Git
│
├── 📁 src/                   # Rust source code
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Main library
│   │
│   ├── 📁 game/             # Game engine
│   │   ├── mod.rs           # Game module
│   │   ├── world.rs         # Game world (tick counter, zones)
│   │   ├── zone.rs          # Procedural zone generation (v0.2.0)
│   │   ├── campaign.rs      # Campaign system (v0.2.0)
│   │   └── (entities.rs planned for future)
│   │
│   ├── 📁 auth/             # Authentication system (v0.2.0)
│   │   ├── mod.rs           # Auth module
│   │   ├── models.rs        # User, Session models
│   │   ├── database.rs      # Database abstraction
│   │   └── service.rs       # Auth business logic
│   │
│   ├── 📁 network/          # Network server
│   │   ├── mod.rs           # Network module
│   │   ├── server.rs        # Axum WebSocket/REST server
│   │   ├── zone_routes.rs   # Zone API endpoints (v0.2.0)
│   │   └── campaign_routes.rs # Campaign API endpoints (v0.2.0)
│   │
│   └── 📁 scripting/        # JavaScript sandbox
│       ├── mod.rs           # Scripting module
│       └── sandbox.rs       # Code storage (execution pending)
│
├── 📁 examples/             # Examples and documentation
│   │
│   ├── 📄 API_REFERENCE.md  # Complete JavaScript API documentation
│   ├── 📄 README.md         # Examples guide and API usage
│   │
│   ├── 📄 basic_bot.js      # Basic bot to get started
│   ├── 📄 advanced_bot.js   # Advanced bot with strategies
│   ├── 📄 template_bot.js   # Empty template to create your bot
│   │
│   ├── 📄 auth_example.js   # Authentication example (v0.2.0)
│   ├── 📄 multiplayer_example.js # Multiplayer example (v0.2.0)
│   ├── 📄 api_client_example.js # HTTP/WebSocket client
│   ├── 📄 node_client_example.js # Node.js complete example (v0.2.0)
│   ├── 📄 zone_generation_example.js # Zone generation demo (v0.2.0)
│   ├── 📄 campaign_local_save_example.js # Campaign system demo (v0.2.0)
│   ├── 📄 online_campaign_example.js # Online campaign example
│   │
│   └── 📁 viewer/           # Basic HTML viewer (example)
│       ├── index.html       # User interface
│       ├── viewer.js        # Viewer logic
│       ├── style.css        # CSS styles
│       └── README.md        # Viewer documentation
│
├── 📁 tests/                # Tests
│   └── integration_tests.rs # Integration tests
│
└── 📁 assets/               # Resources (optional)
    └── textures             # Placeholder file
```

## Changes Made

### ✅ Removed (Headless Architecture)

- ❌ `src/graphics/` - Integrated graphics rendering module
- ❌ `assets/shaders/` - Graphics shaders
- ❌ `examples/basic_bot.rs` - Rust example (replaced by JS)

**Reason:** The engine is now headless. Players create their own viewers.

### ✅ Added (v0.2.0-alpha)

#### Authentication System
- ✅ `src/auth/` - Complete authentication module
- ✅ Token-based authentication with bcrypt
- ✅ Session management (24-hour expiration)
- ✅ Database abstraction (In-Memory, MongoDB)

#### Procedural Zone Generation
- ✅ `src/game/zone.rs` - Zone generation system
- ✅ 30x30 tile zones with terrain types
- ✅ Deterministic generation per player
- ✅ Exit placement system

#### Campaign System
- ✅ `src/game/campaign.rs` - Campaign management
- ✅ `src/network/campaign_routes.rs` - Campaign API
- ✅ Start, stop, save, load functionality
- ✅ Local file persistence (JSON)

#### JavaScript Examples
- ✅ `examples/auth_example.js` - Authentication workflow
- ✅ `examples/multiplayer_example.js` - Multiplayer client
- ✅ `examples/node_client_example.js` - Complete Node.js example
- ✅ `examples/zone_generation_example.js` - Zone generation demo
- ✅ `examples/campaign_local_save_example.js` - Campaign demo
- ✅ `examples/online_campaign_example.js` - Online campaign
- ✅ `examples/api_client_example.js` - Browser/Node.js HTTP and WebSocket

#### Basic HTML Viewer
- ✅ `examples/viewer/index.html` - Modern user interface with authentication
- ✅ `examples/viewer/viewer.js` - WebSocket client with auth support
- ✅ `examples/viewer/style.css` - Dark theme styles
- ✅ `examples/viewer/README.md` - Updated viewer documentation

#### Documentation
- ✅ `DATABASE.md` - Database configuration guide
- ✅ `IMPLEMENTATION_SUMMARY.md` - Auth & multiplayer implementation
- ✅ `docs/ZONE_GENERATION.md` - Zone generation documentation
- ✅ `docs/CAMPAIGN.md` - Campaign system documentation
- ✅ Updated `README.md` - Authentication and multiplayer
- ✅ Updated `examples/README.md` - API usage with auth

### ✅ Modified (v0.2.0-alpha)

- 🔧 `src/lib.rs` - Added auth module export
- 🔧 `src/main.rs` - Initialize auth database, auth service, and startup messages
- 🔧 `src/network/server.rs` - Added auth endpoints, WebSocket auth, protected routes
- 🔧 `src/game/world.rs` - Added zone management
- 🔧 `Cargo.toml` - Added dependencies (mongodb, bcrypt, uuid, bson, chrono)
- 🔧 `README.md` - Complete overhaul for authentication and v0.2.0-alpha features
- 🔧 `BUILD.md` - Updated with current dependencies and features
- 🔧 `.gitignore` - Added node_modules exclusion

## Key Concepts (v0.2.0-alpha)

### 1. Headless Engine

The GeekCraft server provides **no graphics rendering**. It is responsible for:
- ✅ Game world simulation (zones, tick counter)
- ✅ Authentication and session management
- ✅ Campaign management
- ✅ Code storage (execution pending)
- ✅ Network communication (WebSocket/REST)
- 🚧 Entity management (structure exists, integration pending)
- 🚧 JavaScript bot execution (storage implemented, Boa/Deno pending)

### 2. Visualization Freedom

Players are **completely free** to create their own interface:
- 🌐 **Web**: React, Vue, Three.js, Canvas, etc.
- 🖥️ **Desktop**: Unity, Godot, Electron, Tauri
- 💻 **Terminal**: ASCII art, Blessed, Rich
- 📱 **Mobile**: React Native, Flutter
- 🎨 **Other**: Anything that can connect via WebSocket

### 3. JavaScript Programming (Storage Phase)

Players program their bots in JavaScript:
- 📝 Code submission and validation
- 💾 Secure code storage per player
- 🔒 Size limits (1MB max)
- ⚡ Ready for execution (Boa/Deno integration pending)
- 🚧 Bot execution not yet active in v0.2.0-alpha

## Workflow

```
1. Player writes their bot in JavaScript
   ↓
2. Bot submitted to Rust server
   ↓
3. Server executes bot (sandbox)
   ↓
4. World simulation updated
   ↓
5. State broadcast via WebSocket
   ↓
6. Custom viewer displays the game
```

## Getting Started

### 1. Build the Server

```bash
cd GeekCraft
cargo build --release
```

### 2. Start the Server

```bash
cargo run --release
# Server starts on ws://localhost:3030
```

### 3. Test the Viewer

```bash
cd examples/viewer
open index.html  # or double-click
```

### 4. Create Your Bot

```bash
cp examples/template_bot.js my_bot.js
# Edit my_bot.js with your strategy
```

### 5. (Optional) Create Your Viewer

Use the technology of your choice! The provided HTML viewer is just an example.

## Available Documentation

| File | Description |
|---------|-------------|
| `README.md` | Main documentation |
| `BUILD.md` | Build guide |
| `PROJECT_SUMMARY.md` | Project summary |
| `examples/API_REFERENCE.md` | JavaScript API for bots |
| `examples/viewer/README.md` | HTML viewer documentation |

## Next Steps (Post v0.2.0-alpha)

### Current Phase: Complete Game Simulation 🚧

#### Phase 1: Foundations ✅ (v0.2.0-alpha - Completed)
- [x] Project structure
- [x] Complete documentation
- [x] JavaScript bot examples
- [x] Basic HTML viewer
- [x] Headless architecture defined

#### Phase 2: Base Engine & Database ✅ (v0.2.0-alpha - Completed)
- [x] Basic world simulation (tick counter, terrain)
- [x] Resource types (Wood, Stone, Gold)
- [x] Procedural zone generation (30x30 tiles)
- [x] Campaign system (start, stop, save, load)
- [x] In-Memory and MongoDB database support
- [ ] Complete entity system (structure exists)
- [ ] Advanced resource management (gameplay mechanics)
- [ ] Movement logic

#### Phase 3: Scripting 🚧 (In Progress)
- [x] Code storage and validation
- [x] Basic scripting engine
- [ ] Full JS engine integration (Boa/Deno) - Next Priority
- [ ] Bot execution on tick
- [ ] Complete functional scripting API
- [ ] Advanced security limitations
- [ ] Event management

#### Phase 4: Networking ✅ (v0.2.0-alpha - Completed)
- [x] Axum HTTP server
- [x] WebSocket server
- [x] REST API endpoints
- [x] CORS configuration
- [x] Token-based authentication
- [x] Session management
- [ ] Real-time multiplayer synchronization (auto-incrementing ticks)

#### Phase 5: Gameplay 📅 (Planned)
- [ ] Combat system
- [ ] Building construction
- [ ] Tech tree
- [ ] Zone interconnection
- [ ] Balance and tuning
- [x] Axum HTTP server
- [x] WebSocket server
- [x] REST API endpoints
- [x] CORS configuration
- [ ] Authentication
- [ ] Multiplayer synchronization

#### Phase 5: Gameplay 📅 (Planned)
- [ ] Combat
- [ ] Construction
- [ ] Tech tree
- [ ] Balance

### Upcoming Phases

1. **Simulation Engine** 🚧
   - Game world implementation
   - Entity system
   - Resource management
   - Movement logic

2. **JavaScript Sandbox** 🚧
   - JS engine integration (Boa/Deno)
   - Functional scripting API
   - Security limitations
   - Event management

3. **Network Server** ✅
   - WebSocket server (Completed)
   - REST API (Completed)
   - Authentication (Planned)
   - Multiplayer (Planned)

4. **Gameplay** 📅
   - Combat
   - Construction
   - Tech tree
   - Balance

## Technologies

### Backend (Rust)
- **Rust** 1.70+
- **Tokio** - Async runtime
- **Axum 0.7** - Web framework with WebSocket support
- **Tower-HTTP** - CORS and tracing middleware
- **Serde** - JSON serialization
- **Anyhow** - Error handling
- **Future**: Boa/Deno for full JavaScript sandbox

### Frontend (Examples)
- **HTML5/CSS3/JavaScript** - Basic viewer
- **Canvas API** - 2D rendering
- **WebSocket API** - Real-time communication

## License

MIT License - See `LICENSE` file

## Contribution

Contributions are welcome! Priority areas:
- 🎮 Simulation engine implementation
- 🔒 Secure JavaScript sandbox
- 🌐 WebSocket/REST server
- 📚 Documentation and tutorials
- 🎨 Alternative viewers (React, Unity, etc.)

## Contact

- **Repository**: https://github.com/xelfe/GeekCraft
- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions

---

**GeekCraft** - Your game, your code, your vision! 🎮🚀

*Last updated: November 1st, 2025*
