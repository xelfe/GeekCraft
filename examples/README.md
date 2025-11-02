# GeekCraft API Examples

This directory contains examples for interacting with the GeekCraft server using HTTP REST API and WebSocket connections.

## Available Examples

### 1. `api_client_example.js`

A browser/Node.js example demonstrating both HTTP and WebSocket APIs.

**Features:**
- HTTP REST API examples (health check, code submission, player list, game state)
- WebSocket connection and real-time communication
- Proper error handling
- Detailed comments explaining each API call

**Usage (Browser):**
```html
<script src="api_client_example.js"></script>
<script>
  // Check health
  checkHealth();
  
  // Submit code
  submitCode('player1', yourBotCode);
  
  // Connect to WebSocket
  const ws = connectWebSocket((message) => {
    console.log('Received:', message);
  });
</script>
```

### 2. `node_client_example.js`

A complete Node.js example for connecting to the GeekCraft server.

**Prerequisites:**
```bash
npm install ws node-fetch
```

**Usage:**
```bash
node examples/node_client_example.js
```

**Features:**
- Tests all HTTP endpoints
- Tests WebSocket communication
- Complete example with proper async/await handling

### 3. Bot Examples

- `basic_bot.js` - Simple bot example
- `advanced_bot.js` - Advanced bot with strategies
- `template_bot.js` - Template for creating your own bot

## API Reference

### HTTP REST API

#### Base URL
```
http://localhost:3030
```

#### Endpoints

**GET /**
- Returns API information and available endpoints

**GET /api/health**
- Health check endpoint
- Response: `{ "status": "healthy", "service": "geekcraft" }`

**POST /api/submit**
- Submit player code
- Body: `{ "player_id": "string", "code": "string" }`
- Response: `{ "success": true, "message": "..." }`

**GET /api/players**
- Get list of all players
- Response: `{ "players": ["player1", "player2", ...] }`

**GET /api/gamestate**
- Get current game state
- Response: `{ "tick": 0, "players": [...] }`

### WebSocket API

#### URL
```
ws://localhost:3030/ws
```

#### Connection Flow

1. **Connect** to `ws://localhost:3030/ws`
2. **Receive** welcome message:
   ```json
   {
     "type": "welcome",
     "message": "Connected to GeekCraft server",
     "version": "0.1.0"
   }
   ```
3. **Send** commands as JSON strings
4. **Receive** responses

#### Available Commands

**Get Players**
```json
{
  "type": "getPlayers"
}
```
Response:
```json
{
  "type": "playersResponse",
  "players": ["player1", "player2"]
}
```

**Get Game State**
```json
{
  "type": "getGameState"
}
```
Response:
```json
{
  "type": "gameStateResponse",
  "tick": 0,
  "players": ["player1", "player2"]
}
```

**Set Speed** (from viewer)
```json
{
  "type": "setSpeed",
  "speed": 1.5
}
```

## Quick Start

### 1. Start the Server

```bash
cd GeekCraft
cargo run --release
```

The server will start on `http://localhost:3030`.

### 2. Submit Your Bot Code

Using curl:
```bash
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "myplayer",
    "code": "class MyBot { onTick(gameState) { console.log(\"Hello\"); } }"
  }'
```

Using the JavaScript example:
```javascript
const myBotCode = `
class MyBot {
  onTick(gameState) {
    // Your bot logic here
  }
}
module.exports = MyBot;
`;

submitCode('myplayer', myBotCode);
```

### 3. Connect to WebSocket

```javascript
const ws = new WebSocket('ws://localhost:3030/ws');

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};

ws.onopen = () => {
  // Request game state
  ws.send(JSON.stringify({ type: 'getGameState' }));
};
```

### 4. Use the Viewer

Open `viewer/index.html` in your browser:
```bash
cd examples/viewer
open index.html  # macOS
# or
xdg-open index.html  # Linux
# or double-click the file in File Explorer (Windows)
```

## Testing

Test the API using curl or the included examples:

```bash
# Health check
curl http://localhost:3030/api/health

# Submit code
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -d '{"player_id": "test", "code": "console.log(\"test\");"}'

# Get players
curl http://localhost:3030/api/players

# Get game state
curl http://localhost:3030/api/gamestate
```

## CORS

The server has CORS enabled for all origins, so you can connect from any web page during development.

## Production Deployment

For production deployment:

1. Build in release mode: `cargo build --release`
2. Set environment variables as needed
3. Configure CORS to restrict allowed origins
4. Use a reverse proxy (nginx, Apache) for SSL/TLS termination
5. Set up proper logging and monitoring

## Support

For more information:
- API Reference: `API_REFERENCE.md`
- Server documentation: Check the source code in `src/network/server.rs`
- Main README: `../README.md`
