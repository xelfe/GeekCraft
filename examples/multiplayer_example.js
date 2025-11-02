// Multiplayer WebSocket Example for GeekCraft
// Demonstrates real-time multiplayer communication with authentication

const BASE_URL = 'http://localhost:3030';
const WS_URL = 'ws://localhost:3030/ws';

class GeekCraftClient {
  constructor() {
    this.ws = null;
    this.token = null;
    this.username = null;
    this.authenticated = false;
    this.messageHandlers = {};
  }

  /**
   * Register for a new account
   */
  async register(username, password) {
    const response = await fetch(`${BASE_URL}/api/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    
    const data = await response.json();
    console.log('Registration:', data.success ? '✓' : '✗', data.message);
    return data;
  }

  /**
   * Login and obtain authentication token
   */
  async login(username, password) {
    const response = await fetch(`${BASE_URL}/api/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    
    const data = await response.json();
    
    if (data.success && data.token) {
      this.token = data.token;
      this.username = data.username;
      console.log('Login successful for:', this.username);
      console.log('Token:', this.token);
    } else {
      console.log('Login failed:', data.message);
    }
    
    return data;
  }

  /**
   * Connect to WebSocket server and authenticate
   */
  async connectWebSocket() {
    return new Promise((resolve, reject) => {
      if (!this.token) {
        reject(new Error('Not logged in. Please login first.'));
        return;
      }

      this.ws = new WebSocket(WS_URL);

      this.ws.onopen = () => {
        console.log('WebSocket connected');
      };

      this.ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log('Received:', message);

        // Handle welcome message
        if (message.type === 'welcome') {
          console.log('Server says:', message.message);
          // Authenticate after receiving welcome
          this.authenticateWebSocket().then(resolve).catch(reject);
        }
        // Handle auth response
        else if (message.type === 'authResponse') {
          if (message.success) {
            this.authenticated = true;
            console.log('WebSocket authenticated as:', message.username);
          } else {
            console.error('WebSocket authentication failed:', message.message);
            reject(new Error(message.message));
          }
        }
        // Handle custom message handlers
        else if (this.messageHandlers[message.type]) {
          this.messageHandlers[message.type](message);
        }
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        reject(error);
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.authenticated = false;
      };
    });
  }

  /**
   * Authenticate the WebSocket connection
   */
  async authenticateWebSocket() {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket not connected');
    }

    this.send({ type: 'auth', token: this.token });
    
    // Wait for auth response
    return new Promise((resolve) => {
      const originalHandler = this.messageHandlers['authResponse'];
      this.messageHandlers['authResponse'] = (message) => {
        if (originalHandler) originalHandler(message);
        resolve(message.success);
      };
    });
  }

  /**
   * Send a message through WebSocket
   */
  send(message) {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket not connected');
    }

    this.ws.send(JSON.stringify(message));
  }

  /**
   * Register a handler for a specific message type
   */
  on(messageType, handler) {
    this.messageHandlers[messageType] = handler;
  }

  /**
   * Get list of players (via WebSocket)
   */
  getPlayers() {
    this.send({ type: 'getPlayers' });
  }

  /**
   * Get current game state (via WebSocket)
   */
  getGameState() {
    this.send({ type: 'getGameState' });
  }

  /**
   * Submit bot code (via HTTP with authentication)
   */
  async submitCode(code) {
    if (!this.token) {
      throw new Error('Not authenticated. Please login first.');
    }

    const response = await fetch(`${BASE_URL}/api/submit`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.token}`,
      },
      body: JSON.stringify({ code }),
    });

    const data = await response.json();
    console.log('Code submission:', data.success ? '✓' : '✗', data.message);
    return data;
  }

  /**
   * Logout (via HTTP)
   */
  async logout() {
    if (!this.token) {
      return { success: false, message: 'Not logged in' };
    }

    const response = await fetch(`${BASE_URL}/api/auth/logout`, {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${this.token}` },
    });

    const data = await response.json();
    
    if (data.success) {
      this.token = null;
      this.username = null;
      this.authenticated = false;
      if (this.ws) {
        this.ws.close();
        this.ws = null;
      }
      console.log('Logged out successfully');
    }
    
    return data;
  }

  /**
   * Disconnect WebSocket
   */
  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
      this.authenticated = false;
    }
  }
}

// Example usage demonstrating multiplayer

async function multiplayerExample() {
  console.log('=== GeekCraft Multiplayer Example ===\n');

  // Create two clients simulating two players
  const player1 = new GeekCraftClient();
  const player2 = new GeekCraftClient();

  try {
    // Player 1: Register and login
    console.log('\n--- Player 1 Setup ---');
    await player1.register('player1', 'password123');
    await player1.login('player1', 'password123');

    // Player 2: Register and login
    console.log('\n--- Player 2 Setup ---');
    await player2.register('player2', 'password456');
    await player2.login('player2', 'password456');

    // Both players connect via WebSocket
    console.log('\n--- Connecting to Game ---');
    await player1.connectWebSocket();
    await player2.connectWebSocket();

    // Register message handlers
    player1.on('playersResponse', (msg) => {
      console.log('[Player1] Active players:', msg.players);
    });

    player1.on('gameStateResponse', (msg) => {
      console.log('[Player1] Game state - Tick:', msg.tick, 'Players:', msg.players.length);
    });

    player2.on('playersResponse', (msg) => {
      console.log('[Player2] Active players:', msg.players);
    });

    player2.on('gameStateResponse', (msg) => {
      console.log('[Player2] Game state - Tick:', msg.tick, 'Players:', msg.players.length);
    });

    // Player 1 submits code
    console.log('\n--- Player 1 Submits Bot Code ---');
    const botCode1 = `
      class Player1Bot {
        onTick(gameState) {
          console.log("Player1Bot running");
        }
      }
      module.exports = Player1Bot;
    `;
    await player1.submitCode(botCode1);

    // Player 2 submits code
    console.log('\n--- Player 2 Submits Bot Code ---');
    const botCode2 = `
      class Player2Bot {
        onTick(gameState) {
          console.log("Player2Bot running");
        }
      }
      module.exports = Player2Bot;
    `;
    await player2.submitCode(botCode2);

    // Both players request game state
    console.log('\n--- Requesting Game State ---');
    player1.getPlayers();
    player2.getGameState();

    // Wait a bit for responses
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Cleanup
    console.log('\n--- Cleanup ---');
    await player1.logout();
    await player2.logout();

    console.log('\n=== Example Complete ===');

  } catch (error) {
    console.error('Error in multiplayer example:', error);
  }
}

// For browser usage
if (typeof window !== 'undefined') {
  window.GeekCraftClient = GeekCraftClient;
  window.multiplayerExample = multiplayerExample;
  console.log('GeekCraft Multiplayer Client loaded.');
  console.log('Usage:');
  console.log('  const client = new GeekCraftClient();');
  console.log('  await client.login("username", "password");');
  console.log('  await client.connectWebSocket();');
  console.log('Or run: multiplayerExample()');
}

// For Node.js usage
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { GeekCraftClient, multiplayerExample };
}
