# GeekCraft - Final Project Structure

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
│   │   ├── world.rs         # Game world
│   │   ├── entities.rs      # Entities (units, buildings)
│   │   └── simulation.rs    # Simulation logic
│   │
│   ├── 📁 api/              # Scripting API
│   │   ├── mod.rs           # API module
│   │   ├── scripting.rs     # Scripting interface
│   │   └── events.rs        # Event system
│   │
│   ├── 📁 network/          # Network server
│   │   ├── mod.rs           # Network module
│   │   └── server.rs        # WebSocket/REST server
│   │
│   └── 📁 scripting/        # JavaScript sandbox
│       ├── mod.rs           # Scripting module
│       └── sandbox.rs       # Secure environment
│
├── 📁 examples/             # Examples and documentation
│   │
│   ├── 📄 API_REFERENCE.md  # Complete JavaScript API documentation
│   ├── 📄 basic_bot.js      # Basic bot to get started
│   ├── 📄 advanced_bot.js   # Advanced bot with strategies
│   ├── 📄 template_bot.js   # Empty template to create your bot
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

### ✅ Added

#### JavaScript Examples
- ✅ `examples/basic_bot.js` - Simple bot to get started
- ✅ `examples/advanced_bot.js` - Advanced strategies
- ✅ `examples/template_bot.js` - Starter template
- ✅ `examples/API_REFERENCE.md` - Complete API documentation

#### Basic HTML Viewer
- ✅ `examples/viewer/index.html` - Modern user interface
- ✅ `examples/viewer/viewer.js` - WebSocket client with Canvas rendering
- ✅ `examples/viewer/style.css` - Dark theme styles
- ✅ `examples/viewer/README.md` - Viewer documentation

#### Documentation
- ✅ `PROJECT_SUMMARY.md` - Complete project summary
- ✅ Updated `README.md` - Headless architecture
- ✅ Updated `BUILD.md` - Build guide

### ✅ Modified

- 🔧 `src/lib.rs` - Removed graphics module
- 🔧 `README.md` - Complete overhaul for headless architecture
- 🔧 `BUILD.md` - Added startup examples

## Key Concepts

### 1. Headless Engine

The GeekCraft server provides **no graphics rendering**. It is responsible for:
- ✅ Game world simulation
- ✅ JavaScript bot execution
- ✅ Resource and entity management
- ✅ Network communication (WebSocket/REST)

### 2. Visualization Freedom

Players are **completely free** to create their own interface:
- 🌐 **Web**: React, Vue, Three.js, Canvas, etc.
- 🖥️ **Desktop**: Unity, Godot, Electron, Tauri
- 💻 **Terminal**: ASCII art, Blessed, Rich
- 📱 **Mobile**: React Native, Flutter
- 🎨 **Other**: Anything that can connect via WebSocket

### 3. JavaScript Programming

Players program their bots in JavaScript with:
- 📝 Intuitive and documented API
- 🔒 Secure execution (sandbox)
- ⚡ Real-time events
- 🎮 Full control of units

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

## Next Steps

### Current Phase: Foundations ✅

- [x] Project structure
- [x] Complete documentation
- [x] JavaScript bot examples
- [x] Basic HTML viewer
- [x] Headless architecture defined

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

3. **Network Server** 🚧
   - WebSocket server
   - REST API
   - Authentication
   - Multiplayer

4. **Gameplay** 📅
   - Combat
   - Construction
   - Tech tree
   - Balance

## Technologies

### Backend (Rust)
- **Rust** 1.70+
- **Tokio** - Async runtime
- **Warp/Actix** - Web framework (to be decided)
- **Boa/Deno** - JavaScript engine (to be decided)
- **Serde** - JSON serialization

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
