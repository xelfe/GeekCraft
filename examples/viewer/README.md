# GeekCraft Viewer - Visualization Client Example

This HTML viewer is a basic example of a visualization client for GeekCraft. It demonstrates how to connect to the game server and display the game state in real-time.

## Current Server Support (v0.2.0)

The server currently provides only basic game data:
- ✅ **DYNAMIC**: Game tick counter (updated in real-time)
- ✅ **DYNAMIC**: List of player IDs (strings)
- 🚧 **PLACEHOLDER**: Units, buildings, resources, and detailed player stats are not yet implemented on the server

The viewer displays real-time data for supported features and shows "N/A" for features awaiting server implementation.

## Features

- ✅ WebSocket connection to the server
- ✅ Real-time display of tick and player count
- ✅ HTML5 canvas for rendering
- ✅ Responsive user interface
- ✅ Integrated log console
- ✅ Camera controls (zoom, scroll)
- 🚧 Unit selection (placeholder for future development)
- 🚧 Detailed information on players and units (placeholder for future development)

## Usage

### 1. Start the GeekCraft Server

```bash
cd GeekCraft
cargo run --release
```

The server starts by default on `ws://localhost:3030/ws`

### 2. Open the Viewer

Simply open `index.html` in your web browser:

```bash
cd examples/viewer
# Method 1: Double-click on index.html
# Method 2: Use a local HTTP server
python3 -m http.server 8000
# Then open http://localhost:8000
```

### 3. Connect

1. Verify that the server URL is correct (default: `ws://localhost:3030/ws`)
2. Click the "Connect" button
3. The viewer will display the game state in real-time

## Controls

### Camera
- **Zoom +/- :** Buttons in the control bar
- **Mouse wheel :** Zoom in/out
- **Center :** Reset the view

### Selection
- **Click on a unit :** See its details in the right panel
- **Click elsewhere :** Deselect

### Console
- **Clear :** Empty the console logs

## Code Structure

### index.html
Main user interface with:
- Header (title, connection status)
- Left sidebar (configuration, game info, players)
- Central area (game canvas, controls)
- Right panel (selection details)
- Bottom console (logs)

### viewer.js
Main viewer logic:
- **GeekCraftViewer** : Main class
- **WebSocket** : Communication with the server
- **Canvas Rendering** : Game display
- **Event Management** : User interactions

### style.css
Modern styles with:
- Dark theme
- Responsive layout
- Subtle animations
- Custom scrollbars

## WebSocket Protocol

The viewer communicates with the server via WebSocket in JSON.

### Current Implementation (v0.2.0)

#### Incoming Messages (server → client)

**Welcome message:**
```javascript
{
    "type": "welcome",
    "message": "Connected to GeekCraft server",
    "version": "0.2.0"
}
```

**Game state response (DYNAMIC - currently supported):**
```javascript
{
    "type": "gameStateResponse",
    "tick": 123,              // Game tick counter
    "players": ["player1", "player2"]  // Array of player IDs (strings)
}
```

**Players list response (DYNAMIC - currently supported):**
```javascript
{
    "type": "playersResponse",
    "players": ["player1", "player2"]  // Array of player IDs (strings)
}
```

**Error response:**
```javascript
{
    "type": "error",
    "message": "Error description"
}
```

#### Outgoing Messages (client → server)

**Request game state:**
```javascript
{
    "type": "getGameState"
}
```

**Request players list:**
```javascript
{
    "type": "getPlayers"
}
```

### Future Protocol Extensions (PLACEHOLDER)

The following message types are prepared in the viewer code but not yet implemented on the server:

```javascript
// Complete game state (future)
{
    "type": "gameState",
    "data": {
        "tick": 123,
        "players": [...],
        "units": [...],      // Not yet implemented
        "buildings": [...],  // Not yet implemented
        "resources": [...]   // Not yet implemented
    }
}

// Incremental update (future)
{
    "type": "gameUpdate",
    "data": {
        "tick": 124,
        "units": [...]
    }
}

// Game event (future)
{
    "type": "event",
    "data": {
        "eventType": "unitCreated",
        "unitId": 42
    }
}

// Change game speed (future)
{
    "type": "setSpeed",
    "speed": 2.0
}
```

## Customization

### Modify the Appearance

Edit `style.css` to change colors, sizes, etc.

```css
:root {
    --primary-color: #2196F3;  /* Primary color */
    --bg-dark: #1a1a1a;        /* Dark background */
    /* ... */
}
```

### Add Features

The viewer is designed to be easily extensible. Many placeholder functions are already included for future development:

1. **New entity type** : Uncomment the corresponding `render[Type]()` method in `viewer.js` when server support is added
2. **New command** : Add a button and use `sendCommand()`
3. **New panel** : Modify `index.html` and `style.css`

**Important**: Before adding new features to the viewer, ensure the server provides the necessary data via the WebSocket protocol.

### Example: Enable Unit Rendering (when server support is added)

```javascript
// In viewer.js, render() method
// Uncomment this line:
this.renderUnits();
```

## Create Your Own Viewer

This HTML viewer is just an example! You can create your own client with:

### Web Technologies
- **React/Vue/Angular** : Modern web applications
- **Three.js** : 3D rendering in the browser
- **Phaser** : 2D game framework
- **PixiJS** : High-performance 2D rendering

### Desktop Technologies
- **Electron** : Desktop web application
- **Unity** : 3D game engine
- **Godot** : Open-source game engine
- **Tauri** : Lightweight alternative to Electron

### Terminal Technologies
- **Blessed/Ink** (Node.js) : Terminal interface
- **Rich** (Python) : Terminal with colors
- **Cursive** (Rust) : TUI in Rust

### Minimal Example

```javascript
// viewer_minimal.js
const ws = new WebSocket('ws://localhost:3030/ws');

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Game state:', data);
    
    // Display as you want!
    // Terminal, canvas, 3D, etc.
};

ws.onopen = () => {
    ws.send(JSON.stringify({ type: 'getGameState' }));
};
```

## Troubleshooting

### The Viewer Won't Connect
- Check that the GeekCraft server is running (`cargo run --release`)
- Check the WebSocket URL in the interface (default: `ws://localhost:3030/ws`)
- Look at the browser console (F12) for errors

### Nothing Displays on the Canvas
- The canvas currently shows only a grid as units, buildings, and resources are not yet implemented on the server
- Check that you are receiving data in the console (bottom panel)
- Verify that the "Tick" and "Players" fields in the left sidebar are updating

### Players Show "N/A" for Stats
- This is expected behavior - the server currently only provides player IDs
- Resources and unit counts will be displayed when server support is added

### Poor Performance
- Reduce the server update frequency
- Use a smaller canvas
- The performance will improve once the full game state is implemented

## Resources

- [WebSocket API MDN](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
- [Canvas API MDN](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [GeekCraft API Reference](../API_REFERENCE.md)

## License

This example viewer is provided under the MIT license, like the rest of the GeekCraft project.
