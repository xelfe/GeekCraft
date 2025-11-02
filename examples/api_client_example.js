/**
 * GeekCraft API Client Example
 * 
 * This example demonstrates how to interact with the GeekCraft server
 * using both HTTP REST API and WebSocket connections.
 * 
 * @author GeekCraft Team
 * @version 1.0.0
 */

// Configuration
const SERVER_URL = 'http://localhost:3030';
const WS_URL = 'ws://localhost:3030/ws';

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
 * Check server health
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
 * Submit bot code to the server
 * @param {string} playerId - Your player ID
 * @param {string} code - Your bot code
 */
async function submitCode(playerId, code) {
    try {
        const response = await fetch(`${SERVER_URL}/api/submit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                player_id: playerId,
                code: code
            })
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
 * Get list of all players
 */
async function getPlayers() {
    try {
        const response = await fetch(`${SERVER_URL}/api/players`);
        const data = await response.json();
        console.log('Players:', data);
        return data;
    } catch (error) {
        console.error('Error getting players:', error);
        throw error;
    }
}

/**
 * Get current game state
 */
async function getGameState() {
    try {
        const response = await fetch(`${SERVER_URL}/api/gamestate`);
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
 * Connect to the WebSocket server
 * @param {function} onMessage - Callback for incoming messages
 */
function connectWebSocket(onMessage) {
    const ws = new WebSocket(WS_URL);
    
    ws.onopen = () => {
        console.log('WebSocket connected');
    };
    
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        console.log('WebSocket message received:', data);
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
 * Example: Using HTTP API
 */
async function httpApiExample() {
    console.log('\n=== HTTP API Example ===\n');
    
    // Check server health
    await checkHealth();
    
    // Submit your bot code
    await submitCode('player1', MY_BOT_CODE);
    
    // Get list of players
    await getPlayers();
    
    // Get game state
    await getGameState();
}

/**
 * Example: Using WebSocket
 */
function websocketExample() {
    console.log('\n=== WebSocket Example ===\n');
    
    // Connect to WebSocket
    const ws = connectWebSocket((message) => {
        // Handle incoming messages
        switch (message.type) {
            case 'welcome':
                console.log('Received welcome message:', message.message);
                
                // Request players list
                sendCommand(ws, { type: 'getPlayers' });
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
        ws.close();
    }, 5000);
}

// ============================================================================
// Main
// ============================================================================

/**
 * Run all examples
 * 
 * Note: This example assumes you're running it in a Node.js environment
 * with fetch API available (Node.js 18+) or with a polyfill.
 */
async function main() {
    console.log('GeekCraft API Client Example');
    console.log('=============================\n');
    
    try {
        // Run HTTP API examples
        await httpApiExample();
        
        // Wait a bit before WebSocket example
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Run WebSocket example
        // Note: This needs to be adapted for Node.js (use 'ws' package)
        // websocketExample();
        
    } catch (error) {
        console.error('Error running examples:', error);
    }
}

// Export functions for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
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

// Run main if executed directly (Node.js only)
if (typeof require !== 'undefined' && typeof module !== 'undefined' && require.main === module) {
    main();
}
