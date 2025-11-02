# 🚀 GeekCraft - Quick Start Guide

## Building the Project

To build GeekCraft, follow these simple steps:

### Prerequisites

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Verify installation**
   ```bash
   rustc --version  # Must be 1.70+
   cargo --version
   ```

### Building

```bash
# 1. Navigate to the project
cd GeekCraft

# 2. Build in release mode (optimized)
cargo build --release

# ⏳ First compilation: 3-7 minutes
# Subsequent builds will be much faster
```

### Running

```bash
# Start the server
cargo run --release

# Server starts on:
# - HTTP: http://localhost:3030
# - WebSocket: ws://localhost:3030/ws
```

You should see:
```
[INFO] 🎮 Starting GeekCraft v0.2.0
[INFO] ✓ Game world initialized
[INFO] ✓ Scripting engine initialized
[INFO] ✓ Axum server listening on http://0.0.0.0:3030
[INFO] ✓ WebSocket endpoint: ws://0.0.0.0:3030/ws
[INFO] ✓ API endpoints:
[INFO]   - GET  /
[INFO]   - GET  /api/health
[INFO]   - POST /api/submit
[INFO]   - GET  /api/players
[INFO]   - GET  /api/gamestate
[INFO] 🚀 GeekCraft is ready!
```

## Authenticate and Submit Your First Bot

GeekCraft uses token-based authentication. You need to register and login before submitting code.

```bash
# 1. Register a new account
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"myplayer","password":"mypassword123"}'
  # Note: Use your own username and password

# Expected response:
# {"success":true,"message":"User registered successfully"}

# 2. Login and get your authentication token
curl -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"myplayer","password":"mypassword123"}'
  # Note: Use your own credentials

# Expected response:
# {"success":true,"token":"YOUR_TOKEN_HERE","username":"myplayer"}

# 3. Save your token (Linux/macOS)
TOKEN=$(curl -s -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"myplayer","password":"mypassword123"}' \
  | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

echo "Your token: $TOKEN"
```

## Create Your First Bot

```bash
# 1. Copy the template
cp examples/template_bot.js my_bot.js

# 2. Edit with your strategy
nano my_bot.js  # or your favorite editor

# 3. Submit your bot via HTTP API (requires authentication)
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code":"class MyBot{onTick(){console.log(\"Hello!\");}}module.exports=MyBot;"}'

# Expected response:
# {"success":true,"message":"Code submitted successfully"}
```

### Test Your Bot Submission

```bash
# Check if your bot was registered (requires authentication)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/players

# Expected response:
# {"players":["myplayer"]}

# Get current game state (requires authentication)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/gamestate

# Expected response:
# {"tick":0,"players":["myplayer"]}
```

## Test the Viewer

Once the server is running:

```bash
# In a new terminal
cd examples/viewer

# Open the HTML viewer
open index.html  # macOS
xdg-open index.html  # Linux
start index.html  # Windows

# Or use a local HTTP server (recommended)
python3 -m http.server 8000
# Then open http://localhost:8000 in your browser
```

The viewer workflow:
1. **Register** - Create a new account (first time only)
2. **Login** - Authenticate and receive a token
3. **Connect** - Connect to WebSocket at `ws://localhost:3030/ws`
4. **Auto-authenticate** - Viewer sends your token to authenticate WebSocket
5. **View game state** - See real-time tick count and player list

**Note**: All WebSocket connections require authentication. The viewer handles this automatically after you login.

## Useful Commands

```bash
# Quick build (debug)
cargo build

# Optimized build (release)
cargo build --release

# Run directly
cargo run

# Run in release
cargo run --release

# Run with logs
RUST_LOG=info cargo run

# Tests
cargo test

# Check API health (public endpoint, no auth required)
curl http://localhost:3030/api/health

# Check available endpoints (public endpoint, no auth required)
curl http://localhost:3030/

# Authenticated API calls (replace $TOKEN with your actual token)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/players
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/gamestate

# Documentation
cargo doc --open

# Clean
cargo clean

# Check without compiling
cargo check

# Format code
cargo fmt

# Linter
cargo clippy
```

## Project Structure

```
GeekCraft/
├── src/              # Rust source code (engine)
├── examples/         # JS bots and HTML viewer
│   ├── *.js         # Bot examples
│   └── viewer/      # Basic HTML viewer
├── tests/           # Tests
└── assets/          # Resources (optional)
```

## Documentation

- **README.md** - Main documentation
- **BUILD.md** - Detailed build guide
- **PROJECT_SUMMARY.md** - Project summary
- **STRUCTURE.md** - Complete structure
- **examples/API_REFERENCE.md** - JavaScript API
- **examples/viewer/README.md** - Viewer documentation

## Development Workflow

```
1. Edit code → 2. Compile → 3. Test → 4. Repeat
       ↓            ↓          ↓
  src/**/*.rs   cargo build  cargo test
```

## Troubleshooting

### Compilation error

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Linker not found (Linux)

```bash
sudo apt-get install build-essential
```

### Linker not found (macOS)

```bash
xcode-select --install
```

### Outdated dependencies

```bash
cargo update
```

## Next Steps

1. ✅ Build the server (you are here)
2. 📖 Read the API Reference (`examples/API_REFERENCE.md`)
3. 🤖 Create your bot (`examples/template_bot.js`)
4. 🎨 Test the viewer (`examples/viewer/`)
5. 🚀 Create your own viewer (optional)

## Support

- **Documentation**: See `.md` files
- **Issues**: https://github.com/xelfe/GeekCraft/issues
- **Discussions**: https://github.com/xelfe/GeekCraft/discussions

## Summary

```bash
# 1. Build once
cd GeekCraft
cargo build --release

# 2. Run the server
cargo run --release

# 3. Test the API (health check is public)
curl http://localhost:3030/api/health

# 4. Register and login
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"me","password":"password123"}'

# Get authentication token
TOKEN=$(curl -s -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"me","password":"password123"}' \
  | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

# 5. Open the viewer (in browser)
open examples/viewer/index.html
# Then login with your credentials in the viewer

# 6. Create and submit your bot (requires authentication)
cp examples/template_bot.js my_bot.js
# Edit my_bot.js with your strategy!

# Submit via API
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code":"class MyBot{onTick(){console.log(\"Hi!\");}}module.exports=MyBot;"}'

# 7. Check your bot was registered
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/players
```

**That's it! You're ready to code! 🎮🚀**

---

*For more details, see `BUILD.md`*
