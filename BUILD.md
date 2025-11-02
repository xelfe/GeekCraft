# Build Guide - GeekCraft

## Prerequisites

Before building GeekCraft, make sure you have:

- **Rust 1.70+**: [Installation](https://rustup.rs)
- **Cargo** (included with Rust)
- **Git** (to clone the repository)

### Optional Dependencies

For running Node.js client examples:
- **Node.js 18+**: [Installation](https://nodejs.org)
- **npm** packages: `ws`, `node-fetch`

### Verify installation

```bash
rustc --version  # should display 1.70 or higher
cargo --version
```

## Building the Project

### 1. Clone the repository

```bash
git clone https://github.com/xelfe/GeekCraft.git
cd GeekCraft
```

### 2. Build in Debug mode (development)

```bash
cargo build
```

This command:
- Downloads all dependencies:
  - Tokio (async runtime)
  - Axum and Axum-extra (web framework)
  - Tower and Tower-HTTP (middleware)
  - Serde (JSON serialization)
  - tokio-tungstenite (WebSocket)
  - futures-util (async utilities)
  - log and env_logger (logging)
  - anyhow (error handling)
- Compiles the project in debug mode
- Creates the executable in `target/debug/geekcraft`

**Estimated time:** 2-5 minutes (first compilation)

### 3. Build in Release mode (production)

```bash
cargo build --release
```

This command:
- Compiles with full optimizations
- Creates the executable in `target/release/geekcraft`
- **Recommended for deployment**

**Estimated time:** 3-7 minutes

### 4. Verify the build

```bash
# In debug mode
./target/debug/geekcraft --version

# In release mode
./target/release/geekcraft --version
```

## Execution

### Debug Mode

```bash
cargo run
```

or directly:

```bash
./target/debug/geekcraft
```

The server will start on:
- HTTP: `http://localhost:3030`
- WebSocket: `ws://localhost:3030/ws`

### Release Mode

```bash
cargo run --release
```

or directly:

```bash
./target/release/geekcraft
```

### With Detailed Logs

```bash
RUST_LOG=debug cargo run
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

Example output:
```
[INFO] 🎮 Starting GeekCraft v0.2.0
[INFO] ✓ Game world initialized
[INFO] ✓ Scripting engine initialized
[INFO] ✓ Axum server listening on http://0.0.0.0:3030
[INFO] ✓ WebSocket endpoint: ws://0.0.0.0:3030/ws
[INFO] ✓ Network server started at http://localhost:3030
[INFO] 🚀 GeekCraft is ready!
```

## Tests

### Run All Tests

```bash
cargo test
```

This runs:
- Unit tests in `src/`
- Integration tests in `tests/`
- Documentation tests

### Tests with Detailed Output

```bash
cargo test -- --nocapture
```

### Test a Specific Module

```bash
cargo test game::
cargo test network::
cargo test scripting::
```

### Integration Tests Only

```bash
cargo test --test integration_tests
```

Example output:
```
running 1 test
test test_game_world_initialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

## Documentation

### Generate Documentation

```bash
cargo doc --no-deps
```

This generates API documentation for GeekCraft modules:
- `game::world` - World simulation
- `scripting::sandbox` - Script engine
- `network::server` - HTTP/WebSocket server
- `api` - Scripting API (planned)

### Open Documentation in Browser

```bash
cargo doc --open
```

## Cleanup

### Clean Builds

```bash
cargo clean
```

This removes the `target/` directory (useful if you encounter compilation issues).

## Common Issues

### Error: "linker not found"

**Linux:**
```bash
sudo apt-get install build-essential
```

**macOS:**
```bash
xcode-select --install
```

### Dependency errors

```bash
cargo update
cargo clean
cargo build
```

### Out of memory during compilation

```bash
# Limit parallelism
cargo build -j 2
```

## Build Scripts

### Quick build script (Linux/macOS)

Create a `build.sh` file:

```bash
#!/bin/bash
set -e

echo "🔨 Building GeekCraft..."
cargo build --release

echo "✓ Build complete!"
echo "📍 Executable: ./target/release/geekcraft"
```

Make it executable:
```bash
chmod +x build.sh
./build.sh
```

### Build and test script (Linux/macOS)

Create a `build-and-test.sh` file:

```bash
#!/bin/bash
set -e

echo "🔨 Building..."
cargo build --release

echo "🧪 Testing..."
cargo test

echo "📚 Documentation..."
cargo doc --no-deps

echo "✓ All done!"
```

## Build for Production

### Optimized build

```bash
cargo build --release --locked
```

### Executable size

Reduce executable size (optional):

Add to `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link Time Optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

Then:

```bash
cargo build --release
```

## Cross-Platform Build

### For Windows (from Linux)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### For Linux (from macOS)

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Useful Commands

```bash
# Check code without compiling
cargo check

# Format code
cargo fmt

# Linter (clippy)
cargo clippy

# Update dependencies
cargo update

# View dependency tree
cargo tree

# Project statistics
cargo count
```

## Environment Variables

```bash
# Server port
export GEEKCRAFT_PORT=3030

# Log level
export RUST_LOG=info

# Development mode
export GEEKCRAFT_DEV_MODE=true
```

## Next Steps

After building GeekCraft:

1. **Start the server**
   ```bash
   cargo run --release
   ```

2. **Open the example viewer**
   ```bash
   cd examples/viewer
   open index.html  # or double-click the file
   ```

3. **Create your first bot**
   ```bash
   cp examples/template_bot.js my_bot.js
   # Edit my_bot.js with your strategy
   ```

4. **Explore the documentation**
   - API Reference: `examples/API_REFERENCE.md`
   - Bot examples: `examples/basic_bot.js` and `examples/advanced_bot.js`
   - Viewer documentation: `examples/viewer/README.md`

5. **Create your own viewer** (optional)
   - The engine is headless - you are free to create your own interface
   - Use the HTML viewer as a reference
   - Suggested technologies: React, Unity, Godot, terminal, etc.

## Support

In case of problems:
- Check GitHub issues
- Consult Rust documentation
- Join our Discord

**Happy building! 🚀**
