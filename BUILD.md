# Build Guide - GeekCraft

## Prerequisites

Before building GeekCraft, make sure you have:

- **Rust 1.70+**: [Installation](https://rustup.rs)
- **Cargo** (included with Rust)
- **Git** (to clone the repository)

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
- Downloads all dependencies
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

## Exécution

### Mode Debug

```bash
cargo run
```

ou directement :

```bash
./target/debug/geekcraft
```

### Mode Release

```bash
cargo run --release
```

ou directement :

```bash
./target/release/geekcraft
```

### Avec logs détaillés

```bash
RUST_LOG=debug cargo run
```

Niveaux de log disponibles : `error`, `warn`, `info`, `debug`, `trace`

## Tests

### Exécuter tous les tests

```bash
cargo test
```

### Tests avec sortie détaillée

```bash
cargo test -- --nocapture
```

### Tests d'un module spécifique

```bash
cargo test game::
cargo test network::
```

### Tests d'intégration

```bash
cargo test --test integration_tests
```

## Documentation

### Générer la documentation

```bash
cargo doc --no-deps
```

### Ouvrir la documentation dans le navigateur

```bash
cargo doc --open
```

## Nettoyage

### Nettoyer les builds

```bash
cargo clean
```

Cela supprime le dossier `target/` (utile si vous rencontrez des problèmes de compilation).

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
