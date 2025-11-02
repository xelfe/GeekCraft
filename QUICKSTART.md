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

# Server starts on ws://localhost:3030
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

# Or use a local HTTP server
python3 -m http.server 8000
# Then open http://localhost:8000
```

## Create Your First Bot

```bash
# 1. Copy the template
cp examples/template_bot.js my_bot.js

# 2. Edit with your strategy
nano my_bot.js  # or your favorite editor

# 3. Test your bot
# (via the server API or web interface)
```

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

# Tests
cargo test

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
# Build once
cd GeekCraft
cargo build --release

# Run
cargo run --release

# Test
open examples/viewer/index.html

# Code
cp examples/template_bot.js my_bot.js
# Edit my_bot.js with your strategy!
```

**That's it! You're ready to code! 🎮🚀**

---

*For more details, see `BUILD.md`*
