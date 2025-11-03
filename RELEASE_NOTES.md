# GeekCraft v0.2.0-alpha - Core Infrastructure Complete 🚀

> **Pre-release Alpha** - Ready for early testing and feedback!

This pre-release version includes a fully functional authentication system, multiplayer support, procedural zone generation, and campaign management.

## ✅ Fully Implemented Features

### Authentication & Security
- ✅ User registration and login with token-based authentication
- ✅ Session management with 24-hour token expiration
- ✅ Password hashing with bcrypt
- ✅ Protected endpoints with authorization middleware
- ✅ WebSocket authentication for real-time connections

**Endpoints:** `/api/auth/register`, `/api/auth/login`, `/api/auth/logout`

### Database & Persistence
- ✅ Database abstraction layer with clean trait-based interface
- ✅ In-Memory database for fast development (default)
- ✅ MongoDB support for production deployments
- ✅ Environment-based configuration (`GEEKCRAFT_DB_BACKEND`)
- ✅ Automatic session cleanup with TTL indexes

### Procedural Zone Generation
- ✅ 30x30 tile zones with deterministic generation
- ✅ Three terrain types: Plain (~60%), Swamp (~25%), Obstacle (~15%)
- ✅ 2-4 exits per zone (North, South, East, West)
- ✅ Player-specific zones with unique IDs
- ✅ Zone persistence and retrieval

**Endpoints:** `POST /api/zone/generate`, `GET /api/zone/:zone_id`, `GET /api/zones`

### Campaign System
- ✅ Campaign run management (start, stop, tick)
- ✅ Local file persistence in JSON format
- ✅ Save/load functionality for campaign runs
- ✅ Run state tracking (tick count, status, timestamps)

**Endpoints:** `POST /api/campaign/start`, `GET /api/campaign/state`, `POST /api/campaign/stop`, `POST /api/campaign/save`, `POST /api/campaign/load`, `GET /api/campaign/saves`

### Network & Communication
- ✅ Complete Axum-based HTTP REST API
- ✅ WebSocket server for real-time bidirectional communication
- ✅ CORS support with middleware
- ✅ Request logging with Tower tracing
- ✅ Concurrent authenticated users support

**Server:** http://localhost:3030 | **WebSocket:** ws://localhost:3030/ws

### Code Submission
- ✅ Code validation (1MB size limit)
- ✅ Per-player code storage in scripting engine
- ✅ Code retrieval for any player
- ✅ Protected submission requiring authentication

**Endpoint:** `POST /api/submit`

### HTML Viewer
- ✅ Functional visualization client with authentication
- ✅ **Zone/landscape display** with 30x30 terrain tiles
- ✅ **Terrain visualization** (Plain, Swamp, Obstacle)
- ✅ **Exit markers** with directional indicators
- ✅ Zone information panel with statistics
- ✅ Zoom and pan controls
- ✅ Real-time game state updates

### Examples & Documentation
- ✅ 8+ working JavaScript examples
- ✅ Complete API reference documentation
- ✅ Browser and Node.js client examples
- ✅ Bot templates for players

## 🚧 Known Limitations

**Not Yet Implemented:**
- ⚠️ JavaScript sandbox execution (code stored but not executed - awaiting Boa/Deno)
- ⚠️ Entity system (units, buildings - structure exists but not integrated)
- ⚠️ Movement and pathfinding
- ⚠️ Combat mechanics
- ⚠️ Resource collection system
- ⚠️ Real-time game tick simulation
- ⚠️ Zone interconnection

**Development-Only:**
- CORS allows all origins (restrict for production)
- In-Memory database loses data on restart (use MongoDB for production)

## 🎯 What You Can Test

With v0.2.0-alpha, you can:
- ✅ Register and authenticate users
- ✅ Submit bot code (stored but not executed yet)
- ✅ Generate procedural zones
- ✅ Create and manage campaigns
- ✅ Connect multiple clients via WebSocket
- ✅ Visualize zones in the HTML viewer
- ✅ Test all API endpoints

## 🚀 Quick Start

```bash
# Clone repository
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft

# Build and run (In-Memory database)
cargo run --release

# Or with MongoDB
export GEEKCRAFT_DB_BACKEND=MONGODB
export MONGODB_URL=mongodb://localhost:27017/geekcraft
cargo run --release

# Open viewer
open examples/viewer/index.html
```

Server runs on http://localhost:3030

## 📚 Documentation

- [README.md](https://github.com/xelfe/GeekCraft/blob/v0.2.0/README.md) - Main overview
- [FEATURES.md](https://github.com/xelfe/GeekCraft/blob/v0.2.0/FEATURES.md) - Complete feature status
- [CHANGELOG.md](https://github.com/xelfe/GeekCraft/blob/v0.2.0/CHANGELOG.md) - Version history
- [examples/README.md](https://github.com/xelfe/GeekCraft/blob/v0.2.0/examples/README.md) - API usage guide
- [QUICKSTART.md](https://github.com/xelfe/GeekCraft/blob/v0.2.0/QUICKSTART.md) - Getting started

## 🔧 Configuration

### Environment Variables
- `GEEKCRAFT_DB_BACKEND` - Database backend (`INMEMORY` or `MONGODB`)
- `MONGODB_URL` - MongoDB connection string
- `GEEKCRAFT_SAVE_DIR` - Campaign save directory (default: `./saves`)

## 📝 Recent Changes (v0.2.0)

Based on merged PRs and commits:
- **#25**: Aligned v0.2.0-alpha for pre-release with docs sync, viewer landscape display, deployment guide
- **#24**: Removed dead code and optimized for performance
- **#21**: Added procedural zone generation for player starting areas
- **#19**: Added Campaign system with save/load to disk and HTTP API
- **#18**: Migrated backend from Redis to MongoDB with persistent schema

## 🎯 Next Steps (v0.3.0)

1. **JavaScript Sandbox** - Integrate Boa or Deno for bot code execution
2. **Real-Time Game Loop** - Auto-incrementing tick system
3. **Entity System** - Units and buildings integration
4. **Movement System** - Basic pathfinding

## 🤝 Contributing

Contributions welcome! Most impactful areas:
- JavaScript sandbox integration (Boa/Deno)
- Game loop implementation
- Entity system integration
- Additional testing

## 📦 Dependencies

Key dependencies:
- `axum 0.7` - Web framework
- `tokio` - Async runtime
- `mongodb 2.8` - Database driver
- `bcrypt 0.15` - Password hashing
- `serde` - JSON serialization

## ⚠️ Pre-release Notice

This is a **pre-release alpha** version. The core infrastructure is complete and stable, but gameplay features (bot execution, movement, combat) are planned for v0.3.0 and beyond.

Perfect for:
- Early adopters wanting to explore the API
- Developers interested in contributing
- Testing authentication and zone generation
- Experimenting with campaign management

Not ready for:
- Production game servers
- Competitive gameplay
- Full bot automation

---

**Full Changelog:** https://github.com/xelfe/GeekCraft/blob/v0.2.0/CHANGELOG.md

**GeekCraft** - Your game, your code, your vision! 🎮🚀