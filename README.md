# GeekCraft

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Description

GeekCraft is a programming game inspired by **Screeps** and **Starcraft**, where players write **JavaScript** bots to control units in a real-time strategy environment.

**The game engine is headless by design** - it provides no integrated graphical interface. Players are **free to create their own visualization** using any technology they choose (web, desktop, terminal, etc.). A **basic HTML viewer** is provided as an example and starting point.

## Project Structure
```
GeekCraft
├── src
│   ├── main.rs          # Entry point, initializes the server and starts the engine
│   ├── lib.rs           # Main library, exports modules
│   ├── game
│   │   ├── mod.rs       # Game module
│   │   ├── world.rs     # Game world structures and functions
│   │   ├── entities.rs   # Defines entities (units, buildings)
│   │   └── simulation.rs # Simulation logic (time, interactions)
│   ├── api
│   │   ├── mod.rs       # API module
│   │   ├── scripting.rs  # Scripting system for bots
│   │   └── events.rs     # Game events
│   ├── network
│   │   ├── mod.rs       # Network module
│   │   └── server.rs     # Client-server communication (WebSocket/REST)
│   └── scripting
│       ├── mod.rs       # Scripting module
│       └── sandbox.rs    # Secure environment for script execution
├── assets
│   └── textures          # Example textures (optional)
├── examples
│   ├── basic_bot.js      # Basic JavaScript bot
│   ├── advanced_bot.js   # Advanced bot with strategies
│   ├── template_bot.js   # Template for creating your bot
│   ├── API_REFERENCE.md  # JavaScript API documentation
│   └── viewer
│       ├── index.html    # Basic HTML viewer
│       ├── viewer.js     # Viewer logic
│       └── style.css     # Viewer styles
├── tests
│   └── integration_tests.rs # Integration tests
├── Cargo.toml            # Cargo configuration
├── BUILD.md              # Detailed build guide
└── README.md             # Main documentation
```

## Features

- 🎮 **Rust Game Engine** - Performance and reliability
- 🤖 **JavaScript Programming** - Create bots with a familiar language
- 🌐 **WebSocket/REST API** - Real-time communication with your clients
- 🎨 **Headless by Design** - No integrated graphics, total visualization freedom
- 🖥️ **Example HTML Viewer** - Starting point for creating your own interface
- 🔒 **Secure JavaScript Sandbox** - Safe execution of user scripts
- 📊 **World Simulation** - Resource management, units, and combat
- 🔌 **Open API** - Create your client with any technology (React, Unity, terminal, etc.)

## Installation

### Prerequisites
- Rust 1.70+ ([install Rust](https://rustup.rs))
- Cargo (included with Rust)

### Installation Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/xelfe/GeekCraft.git
   cd GeekCraft/GeekCraft
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Start the game server**
   ```bash
   cargo run --release
   ```

4. **Test with an example bot**
   ```bash
   # Server starts on http://localhost:3030
   # Submit your bot via API or web interface
   ```

## Quick Start

### Create Your First Bot

1. Copy the bot template:
   ```bash
   cp examples/template_bot.js my_bot.js
   ```

2. Edit `my_bot.js` with your strategy:
   ```javascript
   class MyBot {
       onTick(gameState) {
           const units = gameState.getMyUnits();
           // Your logic here
       }
   }
   module.exports = MyBot;
   ```

3. Submit your bot to the server via the API

### Available Examples

- **`basic_bot.js`** - Simple bot to get started
- **`advanced_bot.js`** - Advanced strategies (roles, construction, combat)
- **`template_bot.js`** - Empty template for creating your bot
- **`API_REFERENCE.md`** - Complete API documentation

## Technical Architecture

### Backend (Rust)
- **Game Engine**: World simulation, entity management
- **API Server**: WebSocket + REST for communication
- **JavaScript Sandbox**: Secure execution of user scripts
- **Headless**: No integrated graphics rendering

### JavaScript API (for bots)
- Access to game state
- Unit control
- Resource management
- Building construction
- Combat strategies

### Visualization Clients
- **Basic HTML viewer** provided as example
- **Create your own**: React, Vue, Unity, Godot, ASCII terminal, etc.
- **WebSocket API** to receive real-time updates
- **Total freedom** of design and technology

## Useful Commands

```bash
# Development
cargo run                    # Run in debug mode
cargo build --release        # Optimized build
cargo test                   # Run tests
cargo doc --open            # Generate and open documentation

# Examples
cargo run --example basic_bot
```

## Roadmap

- [x] Basic project structure
- [x] JavaScript API for bots
- [ ] World simulation engine
- [ ] WebSocket/REST server
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