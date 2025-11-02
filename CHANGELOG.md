# Changelog - GeekCraft

All notable changes to this project are documented here.

## [0.2.0] - 2025-11-02

### Network Server Implementation 🌐

Major update implementing the complete network infrastructure with Axum and WebSocket support.

### Added ✅

#### Axum HTTP/WebSocket Server
- `src/network/server.rs` - Complete Axum-based server implementation
  - HTTP REST API endpoints
  - WebSocket real-time communication
  - CORS middleware for cross-origin requests
  - Tracing middleware for request logging
  - Shared application state with Arc/RwLock

#### REST API Endpoints
- `GET /` - API information and available endpoints
- `GET /api/health` - Health check endpoint
- `POST /api/submit` - Submit player code with validation
- `GET /api/players` - List all registered players
- `GET /api/gamestate` - Get current game state (tick count, players)

#### WebSocket Communication
- Real-time bidirectional communication
- Welcome message on connection
- Command-based protocol (JSON)
- Supported commands:
  - `getPlayers` - Retrieve player list
  - `getGameState` - Get current game state
- Automatic error handling and client disconnect detection

#### Code Validation
- Maximum code size limit (1MB)
- Player ID validation
- Secure code storage in scripting engine
- Error reporting for invalid submissions

#### World Simulation
- Basic tick counter
- Terrain system with resource types (Wood, Stone, Gold)
- Resource counting and management
- Foundation for entity system

#### Scripting Engine
- Code storage per player
- Player code retrieval
- Player listing functionality
- Code validation and size limits
- Variable storage for future sandbox integration

#### Integration Tests
- `tests/integration_tests.rs` - Game world initialization test
- Test infrastructure for future test expansion

#### Client Examples
- `examples/api_client_example.js` - Browser/Node.js HTTP and WebSocket example
- `examples/node_client_example.js` - Complete Node.js client example
- `examples/README.md` - Comprehensive API usage documentation

### Modified 🔧

- `Cargo.toml` - Added dependencies:
  - `axum 0.7` - Web framework with WebSocket support
  - `axum-extra 0.9` - Additional Axum features
  - `tower 0.4` - Service abstraction
  - `tower-http 0.5` - CORS and tracing middleware
  - `tokio-tungstenite 0.21` - WebSocket protocol
  - `futures-util 0.3` - Async utilities
  - `anyhow 1.0` - Error handling

- `src/main.rs` - Entry point implementation
  - Async runtime with Tokio
  - Logger initialization with env_logger
  - Game world creation and initialization
  - Scripting engine initialization
  - Network server startup
  - Informative startup messages

- `src/lib.rs` - Library module exports
  - Module documentation
  - Configuration constants (port, host, timeouts)
  - Version constant

- `README.md` - Updated documentation
  - Current project structure
  - Axum server information
  - HTTP and WebSocket API documentation
  - Quick start guide with API examples
  - Updated build and run instructions

### Technical Highlights

#### Architecture
- Async/await throughout with Tokio runtime
- Thread-safe shared state using Arc<RwLock<T>>
- Type-safe request/response handling with Serde
- Middleware-based request processing

#### Security
- CORS configured (permissive for development)
- Code size validation to prevent abuse
- Input validation on all endpoints
- Secure WebSocket message handling

#### Performance
- Async I/O for high concurrency
- Efficient state sharing with Arc
- Minimal memory allocation
- Fast JSON serialization with Serde

### Development Workflow Improvements

- Proper error handling with anyhow
- Comprehensive logging with env_logger
- Clear API structure and documentation
- Examples for all API endpoints
- Integration test framework

### Known Limitations

- JavaScript sandbox not yet fully implemented (code is stored but not executed)
- World simulation is basic (no movement, combat, or building)
- No authentication or authorization
- No multiplayer synchronization
- CORS allows all origins (development only)

### Next Steps

1. **Complete JavaScript Sandbox**
   - Integrate Boa or Deno for code execution
   - Implement security limitations
   - Add execution timeout and memory limits

2. **Enhance World Simulation**
   - Entity system for units and buildings
   - Movement system
   - Combat mechanics
   - Resource collection

3. **Expand API**
   - Player authentication
   - Game commands via WebSocket
   - Real-time game state updates
   - Replay functionality

4. **Production Readiness**
   - Restrict CORS origins
   - Add rate limiting
   - Implement proper authentication
   - Add monitoring and metrics

## [0.1.0] - 2025-11-01

### Headless Architecture 🎮

The project has been refactored to adopt a **headless** architecture (no integrated graphics).

### Added ✅

#### JavaScript Examples for Bots
- `examples/basic_bot.js` - Simple bot for beginners
  - Movement to resources
  - Basic unit management
  - Event handling
  
- `examples/advanced_bot.js` - Advanced bot with complex strategies
  - Role system (workers, soldiers)
  - Advanced resource management
  - Expansion base construction
  - Group combat tactics
  
- `examples/template_bot.js` - Empty template to get started
  - Basic bot structure
  - Methods to implement
  - Explanatory comments

#### Basic HTML Viewer
- `examples/viewer/index.html` - Modern user interface
  - Responsive layout
  - Configurable panels
  - Integrated log console
  
- `examples/viewer/viewer.js` - Complete WebSocket client
  - Server connection
  - 2D Canvas rendering
  - Camera management (zoom, movement)
  - Unit selection
  - Real-time updates
  
- `examples/viewer/style.css` - Modern styles
  - Dark theme
  - Responsive design
  - Subtle animations
  
- `examples/viewer/README.md` - Viewer documentation
  - Usage guide
  - Customization
  - WebSocket protocol
  - Creating alternative viewers

#### Complete Documentation
- `examples/API_REFERENCE.md` - JavaScript API documentation
  - GameState API
  - Unit API
  - Usage examples
  - Best practices
  
- `PROJECT_SUMMARY.md` - Complete project summary
  - Overview
  - Architecture
  - Technologies
  - Roadmap
  
- `STRUCTURE.md` - Detailed structure
  - Complete file tree
  - Changes made
  - Workflow
  
- `QUICKSTART.md` - Quick start guide
  - Installation
  - Building
  - First steps
  - Useful commands

### Modified 🔧

- `README.md` - Complete overhaul
  - Headless description
  - New features
  - Updated structure
  - JavaScript examples
  
- `BUILD.md` - Update
  - Improved "Next Steps" section
  - Viewer references
  - Custom viewer guide
  
- `src/lib.rs` - Removed graphics module
  - Headless architecture
  - Updated modules

### Removed ❌

#### Graphics Modules (Headless Architecture)
- `src/graphics/` - Integrated rendering module
  - `src/graphics/mod.rs`
  - `src/graphics/renderer.rs`
  - `src/graphics/ui.rs`
  
- `assets/shaders/` - Graphics shaders
  
- `examples/basic_bot.rs` - Rust example (replaced by JavaScript)

**Reason:** The engine is now headless. Players create their own visualization clients.

### Project Philosophy

#### Total Visualization Freedom
Players can create their own interface with:
- Web technologies (React, Vue, Canvas, Three.js)
- Game engines (Unity, Godot)
- Desktop applications (Electron, Tauri)
- Terminal interfaces (ASCII art)
- Any technology supporting WebSocket

#### Open Source and Educational
- Teach programming
- Promote Rust and JavaScript
- Build a community
- Share knowledge

### Roadmap

#### Phase 1: Foundations ✅ (Current)
- [x] Project structure
- [x] Complete documentation
- [x] JavaScript bot examples
- [x] Basic HTML viewer
- [x] Headless architecture

#### Phase 2: Base Engine 🚧 (Coming)
- [ ] World simulation
- [ ] Entity system
- [ ] Resource management
- [ ] Movement system

#### Phase 3: Scripting ⏳
- [ ] JavaScript sandbox (Boa/Deno)
- [ ] Complete scripting API
- [ ] Event system
- [ ] Security limitations

#### Phase 4: Networking ⏳
- [ ] WebSocket server
- [ ] REST API
- [ ] Authentication
- [ ] Multiplayer

#### Phase 5: Gameplay 📅
- [ ] Combat system
- [ ] Building construction
- [ ] Tech tree
- [ ] Fog of war

#### Phase 6: Advanced 📅
- [ ] Replay system
- [ ] Leaderboards
- [ ] Tournaments
- [ ] Reference AI

### Technologies Used

#### Backend
- **Rust** 1.70+ - Main language
- **Tokio** - Async runtime (planned)
- **Warp/Actix** - Web framework (to be decided)
- **Boa/Deno** - JavaScript engine (to be decided)
- **Serde** - JSON serialization (planned)

#### Frontend (Examples)
- **HTML5/CSS3/JavaScript** - Basic viewer
- **Canvas API** - 2D rendering
- **WebSocket API** - Real-time communication

### Project Files

#### Documentation
- `README.md` - Main documentation
- `BUILD.md` - Build guide
- `PROJECT_SUMMARY.md` - Complete summary
- `STRUCTURE.md` - Detailed structure
- `QUICKSTART.md` - Quick start
- `CHANGELOG.md` - This file
- `LICENSE` - MIT License

#### Source Code (Rust)
- `src/main.rs` - Entry point
- `src/lib.rs` - Library
- `src/game/*` - Game engine
- `src/api/*` - Scripting API
- `src/network/*` - Network server
- `src/scripting/*` - JavaScript sandbox

#### Examples (JavaScript)
- `examples/basic_bot.js` - Simple bot
- `examples/advanced_bot.js` - Advanced bot
- `examples/template_bot.js` - Template
- `examples/API_REFERENCE.md` - API docs

#### Viewer (HTML/CSS/JS)
- `examples/viewer/index.html` - UI
- `examples/viewer/viewer.js` - Logic
- `examples/viewer/style.css` - Styles
- `examples/viewer/README.md` - Docs

#### Tests
- `tests/integration_tests.rs` - Integration tests

#### Configuration
- `Cargo.toml` - Cargo configuration
- `.gitignore` - Files ignored by Git

### Contribution

Contributions are welcome in these areas:
- 🎮 Simulation engine implementation
- 🔒 Secure JavaScript sandbox
- 🌐 WebSocket/REST server
- 📚 Documentation and tutorials
- 🎨 Alternative viewers (React, Unity, etc.)
- 🧪 Unit and integration tests

### License

MIT License - See `LICENSE` file

### Contact

- **Repository**: https://github.com/xelfe/GeekCraft
- **Issues**: https://github.com/xelfe/GeekCraft/issues
- **Discussions**: https://github.com/xelfe/GeekCraft/discussions

---

**GeekCraft** - Your game, your code, your vision! 🎮🚀

*Format inspired by [Keep a Changelog](https://keepachangelog.com/)*
