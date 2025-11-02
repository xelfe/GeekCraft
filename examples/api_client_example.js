/**
 * GeekCraft API Client Example
 * 
 * This example demonstrates how to interact with the GeekCraft server
 * using both HTTP REST API and WebSocket connections with authentication.
 * 
 * @author GeekCraft Team
 * @version 2.0.0
 */

// Configuration
const SERVER_URL = 'http://localhost:3030';
const WS_URL = 'ws://localhost:3030/ws';

// Authentication state
let authToken = null;
let username = null;

/**
 * Example Bot Code
 * This is the code that will be submitted to the server
 */
const MY_BOT_CODE = `
class MyBot {
    constructor() {
        this.name = "MyBot";
        this.version = "1.0.0";
    }

    onInit() {
        console.log(\`\${this.name} v\${this.version} initialized!\`);
    }

    onTick(gameState) {
        const myUnits = gameState.getMyUnits();
        
        for (const unit of myUnits) {
            if (unit.isIdle()) {
                const nearestResource = gameState.findNearestResource(unit.position);
                if (nearestResource) {
                    unit.moveTo(nearestResource.position);
                }
            }
        }
    }
}

module.exports = MyBot;
`;

// ============================================================================
// HTTP REST API Examples
// ============================================================================

/**
 * Register a new user account
 * @param {string} username - Username (3-32 characters)
 * @param {string} password - Password (minimum 6 characters)
 */
async function register(username, password) {
    try {
        const response = await fetch(`${SERVER_URL}/api/auth/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password })
        });
        
        const data = await response.json();
        console.log('Registration response:', data);
        return data;
    } catch (error) {
        console.error('Error registering:', error);
        throw error;
    }
}

/**
 * Login and obtain authentication token
 * @param {string} username - Username
 * @param {string} password - Password
 */
async function login(username, password) {
    try {
        const response = await fetch(`${SERVER_URL}/api/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password })
        });
        
        const data = await response.json();
        console.log('Login response:', data);
        
        if (data.success && data.token) {
            authToken = data.token;
            username = data.username;
            console.log('Authentication successful. Token saved.');
        }
        
        return data;
    } catch (error) {
        console.error('Error logging in:', error);
        throw error;
    }
}

/**
 * Check server health (public endpoint)
 */
async function checkHealth() {
    try {
        const response = await fetch(`${SERVER_URL}/api/health`);
        const data = await response.json();
        console.log('Server health:', data);
        return data;
    } catch (error) {
        console.error('Error checking health:', error);
        throw error;
    }
}

/**
 * Submit bot code to the server (requires authentication)
 * @param {string} code - Your bot code
 */
async function submitCode(code) {
    try {
        if (!authToken) {
            throw new Error('Not authenticated. Please login first.');
        }

        const response = await fetch(`${SERVER_URL}/api/submit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${authToken}`
            },
            body: JSON.stringify({ code })
        });
        
        const data = await response.json();
        console.log('Code submission response:', data);
        return data;
    } catch (error) {
        console.error('Error submitting code:', error);
        throw error;
    }
}

/**
 * Get list of all players (requires authentication)
 */
async function getPlayers() {
    try {
        if (!authToken) {
            throw new Error('Not authenticated. Please login first.');
        }

        const response = await fetch(`${SERVER_URL}/api/players`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        const data = await response.json();
        console.log('Players:', data);
        return data;
    } catch (error) {
        console.error('Error getting players:', error);
        throw error;
    }
}

/**
 * Get current game state (requires authentication)
 */
async function getGameState() {
    try {
        if (!authToken) {
            throw new Error('Not authenticated. Please login first.');
        }

        const response = await fetch(`${SERVER_URL}/api/gamestate`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        const data = await response.json();
        console.log('Game state:', data);
        return data;
    } catch (error) {
        console.error('Error getting game state:', error);
        throw error;
    }
}

// ============================================================================
// WebSocket Examples
// ============================================================================

/**
 * Connect to the WebSocket server with authentication
 * @param {function} onMessage - Callback for incoming messages
 */
function connectWebSocket(onMessage) {
    const ws = new WebSocket(WS_URL);
    
    ws.onopen = () => {
        console.log('WebSocket connected, waiting for welcome message...');
    };
    
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        console.log('WebSocket message received:', data);
        
        // Handle welcome message and authenticate
        if (data.type === 'welcome') {
            console.log('Received welcome message, authenticating...');
            if (!authToken) {
                console.error('No auth token available. Please login first.');
                ws.close();
                return;
            }
            // Send authentication
            ws.send(JSON.stringify({ type: 'auth', token: authToken }));
        }
        
        if (onMessage) {
            onMessage(data);
        }
    };
    
    ws.onerror = (error) => {
        console.error('WebSocket error:', error);
    };
    
    ws.onclose = () => {
        console.log('WebSocket disconnected');
    };
    
    return ws;
}

/**
 * Send a command via WebSocket
 * @param {WebSocket} ws - The WebSocket connection
 * @param {object} command - The command to send
 */
function sendCommand(ws, command) {
    if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify(command));
        console.log('Command sent:', command);
    } else {
        console.error('WebSocket is not open');
    }
}

// ============================================================================
// Usage Examples
// ============================================================================

/**
 * Example: Using HTTP API with authentication
 */
async function httpApiExample() {
    console.log('\n=== HTTP API Example (with Authentication) ===\n');
    
    // Step 1: Check server health (public endpoint)
    await checkHealth();
    
    // Step 2: Register a new user (or skip if already registered)
    console.log('\nRegistering user...');
    try {
        await register('player1', 'password123');  // Example credentials - use your own
    } catch (error) {
        console.log('User might already exist, continuing...');
    }
    
    // Step 3: Login to get authentication token
    console.log('\nLogging in...');
    await login('player1', 'password123');  // Example credentials - use your own
    
    if (!authToken) {
        console.error('Login failed, cannot continue');
        return;
    }
    
    // Step 4: Submit your bot code
    console.log('\nSubmitting bot code...');
    await submitCode(MY_BOT_CODE);
    
    // Step 5: Get list of players
    console.log('\nGetting players list...');
    await getPlayers();
    
    // Step 6: Get game state
    console.log('\nGetting game state...');
    await getGameState();
}

/**
 * Example: Using WebSocket with authentication
 */
function websocketExample() {
    console.log('\n=== WebSocket Example (with Authentication) ===\n');
    
    if (!authToken) {
        console.error('Please login first using httpApiExample()');
        return;
    }
    
    // Connect to WebSocket
    const ws = connectWebSocket((message) => {
        // Handle incoming messages
        switch (message.type) {
            case 'welcome':
                console.log('Server welcome:', message.message);
                break;
                
            case 'authResponse':
                if (message.success) {
                    console.log('WebSocket authenticated successfully');
                    // Now we can request game data
                    sendCommand(ws, { type: 'getPlayers' });
                } else {
                    console.error('WebSocket authentication failed:', message.message);
                }
                break;
                
            case 'playersResponse':
                console.log('Current players:', message.players);
                // Request game state
                sendCommand(ws, { type: 'getGameState' });
                break;
                
            case 'gameStateResponse':
                console.log('Game tick:', message.tick);
                console.log('Players in game:', message.players);
                break;
                
            case 'error':
                console.error('Server error:', message.message);
                break;
        }
    });
    
    // Clean up on exit
    setTimeout(() => {
        console.log('\nClosing WebSocket connection...');
        ws.close();
    }, 5000);
}

// ============================================================================
// Main
// ============================================================================

/**
 * Run all examples
 * 
 * Note: This example assumes you're running it in a browser environment.
 * For Node.js, use node_client_example.js instead.
 */
async function main() {
    console.log('GeekCraft API Client Example');
    console.log('=============================\n');
    
    try {
        // Run HTTP API examples with authentication
        await httpApiExample();
        
        // Wait a bit before WebSocket example
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Run WebSocket example (uses token from httpApiExample)
        websocketExample();
        
    } catch (error) {
        console.error('Error running examples:', error);
    }
}

// Export functions for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        register,
        login,
        checkHealth,
        submitCode,
        getPlayers,
        getGameState,
        connectWebSocket,
        sendCommand,
        httpApiExample,
        websocketExample
    };
}

// Run main if executed directly (browser console)
if (typeof window !== 'undefined') {
    console.log('GeekCraft API Client loaded.');
    console.log('Run main() to execute examples, or use individual functions:');
    console.log('  - register(username, password)');
    console.log('  - login(username, password)');
    console.log('  - submitCode(code)');
    console.log('  - getPlayers()');
    console.log('  - getGameState()');
    console.log('  - connectWebSocket(onMessage)');
}
