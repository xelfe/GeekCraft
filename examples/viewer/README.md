# GeekCraft Viewer - Visualization Client Example

This HTML viewer is a basic example of a visualization client for GeekCraft. It demonstrates how to connect to the game server and display the game state in real-time.

## Features

- ✅ WebSocket connection to the server
- ✅ Real-time display of the game world
- ✅ HTML5 canvas for rendering
- ✅ Responsive user interface
- ✅ Integrated log console
- ✅ Camera controls (zoom, scroll)
- ✅ Unit selection
- ✅ Detailed information on players and units

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

The viewer communicates with the server via WebSocket in JSON:

### Incoming Messages (server → client)

```javascript
// Complete game state
{
    "type": "gameState",
    "data": {
        "tick": 123,
        "players": [...],
        "units": [...],
        "buildings": [...],
        "resources": [...]
    }
}

// Incremental update
{
    "type": "gameUpdate",
    "data": {
        "tick": 124,
        "units": [...]
    }
}

// Game event
{
    "type": "event",
    "data": {
        "eventType": "unitCreated",
        "unitId": 42
    }
}
```

### Outgoing Messages (client → server)

```javascript
// Request game state
{
    "type": "getGameState"
}

// Change game speed
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

The viewer is designed to be easily extensible:

1. **New entity type** : Add a `render[Type]()` method in `viewer.js`
2. **New command** : Add a button and use `sendCommand()`
3. **New panel** : Modify `index.html` and `style.css`

### Example: Add a Pause Button

```html
<!-- In index.html -->
<button id="pause-btn" class="btn btn-sm">⏸️ Pause</button>
```

```javascript
// In viewer.js, setupEventListeners() method
document.getElementById('pause-btn').addEventListener('click', () => {
    this.sendCommand({ type: 'pause' });
});
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
- Check that the GeekCraft server is running
- Check the WebSocket URL in the interface
- Look at the browser console (F12) for errors

### Nothing Displays on the Canvas
- Check that you are receiving data (bottom console)
- Check that the server is sending game data
- Adjust the zoom if elements are out of view

### Poor Performance
- Reduce the server update frequency
- Optimize rendering (only redraw what changes)
- Use a smaller canvas

## Resources

- [WebSocket API MDN](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
- [Canvas API MDN](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [GeekCraft API Reference](../API_REFERENCE.md)

## License

This example viewer is provided under the MIT license, like the rest of the GeekCraft project.
