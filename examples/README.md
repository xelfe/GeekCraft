# GeekCraft API Examples

This directory contains examples for interacting with the GeekCraft server using HTTP REST API and WebSocket connections with authentication.

## Available Examples

### 1. `auth_example.js`

A comprehensive example demonstrating authentication and API usage.

**Features:**
- User registration
- Login and token management
- Authenticated code submission
- Authenticated API calls (players, game state)
- Logout and session management
- Proper error handling

**Usage (Browser):**
```html
<script src="auth_example.js"></script>
<script>
  // Run the complete example
  GeekCraftAuth.example();
  
  // Or use individual functions
  await GeekCraftAuth.register('player1', 'password123');
  await GeekCraftAuth.login('player1', 'password123');
  await GeekCraftAuth.submitCode(botCode);
  await GeekCraftAuth.getPlayers();
  await GeekCraftAuth.logout();
</script>
```

### 2. `multiplayer_example.js`

A complete multiplayer example with WebSocket authentication.

**Features:**
- Multiple client simulation
- WebSocket authentication flow
- Real-time game state updates
- Message handlers for different event types
- Complete workflow from registration to gameplay

**Usage (Browser):**
```html
<script src="multiplayer_example.js"></script>
<script>
  // Run the full multiplayer example
  multiplayerExample();
  
  // Or create your own client
  const client = new GeekCraftClient();
  await client.login('player1', 'password123');
  await client.connectWebSocket();
  client.getGameState();
</script>
```

### 3. `api_client_example.js`

A browser/Node.js example demonstrating both HTTP and WebSocket APIs with authentication.

**Features:**
- HTTP REST API examples with authentication
- WebSocket connection with authentication
- Proper error handling
- Detailed comments explaining each API call

**Usage (Browser):**
```html
<script src="api_client_example.js"></script>
<script>
  // Run all examples
  main();
  
  // Or use individual functions
  await register('player1', 'password123');
  await login('player1', 'password123');
  await submitCode(yourBotCode);
  const ws = connectWebSocket((message) => {
    console.log('Received:', message);
  });
</script>
```

### 4. `node_client_example.js`

A complete Node.js example for connecting to the GeekCraft server with authentication.

**Prerequisites:**
```bash
npm install ws node-fetch
```

**Usage:**
```bash
node examples/node_client_example.js
```

**Features:**
- Tests all HTTP endpoints with authentication
- Tests WebSocket communication with authentication
- Complete example with proper async/await handling

### 5. Bot Examples

- `basic_bot.js` - Simple bot example
- `advanced_bot.js` - Advanced bot with strategies
- `template_bot.js` - Template for creating your own bot

## API Reference

### Authentication Flow

**Required for all protected endpoints and WebSocket connections**

1. **Register** (first time only)
2. **Login** to get authentication token
3. **Use token** for HTTP requests (Bearer token) or WebSocket authentication

### HTTP REST API

#### Base URL
```
http://localhost:3030
```

#### Public Endpoints (No Authentication Required)

**GET /**
- Returns API information and available endpoints

**GET /api/health**
- Health check endpoint
- Response: `{ "status": "healthy", "service": "geekcraft" }`

**POST /api/auth/register**
- Register a new user
- Body: `{ "username": "string", "password": "string" }`
- Response: `{ "success": true, "message": "..." }`

**POST /api/auth/login**
- Login and get authentication token
- Body: `{ "username": "string", "password": "string" }`
- Response: `{ "success": true, "token": "...", "username": "..." }`

#### Protected Endpoints (Require `Authorization: Bearer TOKEN`)

**POST /api/auth/logout**
- Logout and invalidate session
- Headers: `Authorization: Bearer YOUR_TOKEN`
- Response: `{ "success": true, "message": "..." }`

**POST /api/submit**
- Submit player code
- Headers: `Authorization: Bearer YOUR_TOKEN`
- Body: `{ "code": "string" }`
- Response: `{ "success": true, "message": "..." }`

**GET /api/players**
- Get list of all players
- Headers: `Authorization: Bearer YOUR_TOKEN`
- Response: `{ "players": ["player1", "player2", ...] }`

**GET /api/gamestate**
- Get current game state
- Headers: `Authorization: Bearer YOUR_TOKEN`
- Response: `{ "tick": 0, "players": [...] }`

### WebSocket API

#### URL
```
ws://localhost:3030/ws
```

#### Connection Flow (Authentication Required)

1. **Connect** to `ws://localhost:3030/ws`
2. **Receive** welcome message:
   ```json
   {
     "type": "welcome",
     "message": "Connected to GeekCraft server. Send auth command to authenticate.",
     "version": "0.2.0",
     "requiresAuth": true
   }
   ```
3. **Authenticate** with your token:
   ```json
   {
     "type": "auth",
     "token": "YOUR_TOKEN_FROM_LOGIN"
   }
   ```
4. **Receive** authentication response:
   ```json
   {
     "type": "authResponse",
     "success": true,
     "username": "player1"
   }
   ```
5. **Send/Receive** game commands (only after authentication)

#### Available Commands (Require Authentication)

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

**Error Response**
```json
{
  "type": "error",
  "message": "Authentication required. Send auth command first."
}
```

## Quick Start

### 1. Start the Server

```bash
cd GeekCraft
cargo run --release
```

The server will start on `http://localhost:3030`.

### 2. Register and Login

Using curl:
```bash
# Register a new user
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "myplayer", "password": "mypassword123"}'
  # Note: Use your own username and password

# Login to get authentication token
curl -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "myplayer", "password": "mypassword123"}'
  # Note: Use your own credentials

# Response includes your token:
# {"success":true,"token":"YOUR_TOKEN_HERE","username":"myplayer"}

# Save the token for subsequent requests
export TOKEN="YOUR_TOKEN_HERE"
```

Using JavaScript:
```javascript
const response = await fetch('http://localhost:3030/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ username: 'myplayer', password: 'mypassword123' })
});
const data = await response.json();
const token = data.token; // Save this for authenticated requests
```

### 3. Submit Your Bot Code

Using curl (requires authentication):
```bash
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "code": "class MyBot { onTick(gameState) { console.log(\"Hello\"); } }"
  }'
```

Using JavaScript (requires authentication):
```javascript
const myBotCode = `
class MyBot {
  onTick(gameState) {
    // Your bot logic here
  }
}
module.exports = MyBot;
`;

const response = await fetch('http://localhost:3030/api/submit', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({ code: myBotCode })
});
```

### 4. Connect to WebSocket

```javascript
const ws = new WebSocket('ws://localhost:3030/ws');

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
  
  // Authenticate after welcome
  if (message.type === 'welcome') {
    ws.send(JSON.stringify({ 
      type: 'auth', 
      token: yourTokenFromLogin 
    }));
  }
  
  // After authentication, request game state
  if (message.type === 'authResponse' && message.success) {
    ws.send(JSON.stringify({ type: 'getGameState' }));
  }
};
```

### 5. Use the Viewer

Open `viewer/index.html` in your browser:
```bash
cd examples/viewer
open index.html  # macOS
# or
xdg-open index.html  # Linux
# or double-click the file in File Explorer (Windows)
```

Then:
1. Enter your username and password
2. Click "Register" (first time) or "Login"
3. Click "Connect" to connect to the WebSocket server
4. The viewer will authenticate automatically and show real-time game state

## Testing

Test the API using curl or the included examples:

```bash
# Health check (public, no authentication required)
curl http://localhost:3030/api/health

# Register a new user
curl -X POST http://localhost:3030/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "password": "test123"}'

# Login to get token
curl -X POST http://localhost:3030/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "password": "test123"}'

# Save the token from the response
export TOKEN="your_token_here"

# Submit code (requires authentication)
curl -X POST http://localhost:3030/api/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code": "console.log(\"test\");"}'

# Get players (requires authentication)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/players

# Get game state (requires authentication)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3030/api/gamestate
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
