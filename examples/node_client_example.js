/**
 * GeekCraft Node.js WebSocket Client Example
 * 
 * This example demonstrates how to connect to GeekCraft server from Node.js
 * with authentication support.
 * 
 * Prerequisites:
 *   npm install ws node-fetch
 * 
 * Usage:
 *   node examples/node_client_example.js
 * 
 * @author GeekCraft Team
 * @version 2.0.0
 */

const WebSocket = require('ws');
const fetch = require('node-fetch');

// Configuration
const SERVER_URL = 'http://localhost:3030';
const WS_URL = 'ws://localhost:3030/ws';

// Authentication state
let authToken = null;
let currentUsername = null;

// Example bot code
const MY_BOT_CODE = `
class MyBot {
    constructor() {
        this.name = "MyBot";
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

/**
 * HTTP API Examples
 */
async function testHttpApi() {
    console.log('\n=== Testing HTTP API ===\n');
    
    try {
        // Check server health
        console.log('1. Checking server health...');
        const healthResponse = await fetch(`${SERVER_URL}/api/health`);
        const health = await healthResponse.json();
        console.log('   Health:', health);
        
        // Register
        console.log('\n2. Registering new user...');
        try {
            const registerResponse = await fetch(`${SERVER_URL}/api/auth/register`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    username: 'testplayer',
                    password: 'password123'
                })
            });
            const registerResult = await registerResponse.json();
            console.log('   Registration result:', registerResult);
        } catch (error) {
            console.log('   User might already exist, continuing...');
        }
        
        // Login
        console.log('\n3. Logging in...');
        const loginResponse = await fetch(`${SERVER_URL}/api/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                username: 'testplayer',
                password: 'password123'
            })
        });
        const loginResult = await loginResponse.json();
        console.log('   Login result:', loginResult);
        
        if (loginResult.success && loginResult.token) {
            authToken = loginResult.token;
            currentUsername = loginResult.username;
            console.log('   ✓ Authenticated successfully');
        } else {
            console.error('   ✗ Login failed, cannot continue');
            return;
        }
        
        // Submit code
        console.log('\n4. Submitting bot code...');
        const submitResponse = await fetch(`${SERVER_URL}/api/submit`, {
            method: 'POST',
            headers: { 
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${authToken}`
            },
            body: JSON.stringify({
                code: MY_BOT_CODE
            })
        });
        const submitResult = await submitResponse.json();
        console.log('   Submission result:', submitResult);
        
        // Get players
        console.log('\n5. Getting players list...');
        const playersResponse = await fetch(`${SERVER_URL}/api/players`, {
            headers: { 'Authorization': `Bearer ${authToken}` }
        });
        const players = await playersResponse.json();
        console.log('   Players:', players);
        
        // Get game state
        console.log('\n6. Getting game state...');
        const stateResponse = await fetch(`${SERVER_URL}/api/gamestate`, {
            headers: { 'Authorization': `Bearer ${authToken}` }
        });
        const state = await stateResponse.json();
        console.log('   Game state:', state);
        
    } catch (error) {
        console.error('HTTP API Error:', error.message);
    }
}

/**
 * WebSocket Example
 */
function testWebSocket() {
    console.log('\n=== Testing WebSocket ===\n');
    
    if (!authToken) {
        console.error('No authentication token available. Please run testHttpApi() first.');
        return Promise.resolve();
    }
    
    return new Promise((resolve) => {
        const ws = new WebSocket(WS_URL);
        
        ws.on('open', () => {
            console.log('WebSocket connected, waiting for welcome message...');
        });
        
        ws.on('message', (data) => {
            const message = JSON.parse(data.toString());
            console.log('Received:', message);
            
            if (message.type === 'welcome') {
                console.log('Welcome message:', message.message);
                console.log('Authenticating WebSocket with token...');
                ws.send(JSON.stringify({ type: 'auth', token: authToken }));
            } else if (message.type === 'authResponse') {
                if (message.success) {
                    console.log('✓ WebSocket authenticated successfully');
                    console.log('\nSending getPlayers command...');
                    ws.send(JSON.stringify({ type: 'getPlayers' }));
                } else {
                    console.error('✗ WebSocket authentication failed:', message.message);
                    ws.close();
                }
            } else if (message.type === 'playersResponse') {
                console.log('Players:', message.players);
                
                // Request game state
                console.log('\nSending getGameState command...');
                ws.send(JSON.stringify({ type: 'getGameState' }));
            } else if (message.type === 'gameStateResponse') {
                console.log('Game state - Tick:', message.tick, 'Players:', message.players);
                
                // Close connection after testing
                console.log('\nClosing WebSocket connection...');
                ws.close();
            } else if (message.type === 'error') {
                console.error('Server error:', message.message);
            }
        });
        
        ws.on('error', (error) => {
            console.error('WebSocket error:', error.message);
            resolve();
        });
        
        ws.on('close', () => {
            console.log('WebSocket disconnected');
            resolve();
        });
    });
}

/**
 * Main function
 */
async function main() {
    console.log('GeekCraft Node.js Client Example');
    console.log('=================================');
    
    try {
        // Test HTTP API
        await testHttpApi();
        
        // Wait a bit
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Test WebSocket
        await testWebSocket();
        
        console.log('\n=== All tests completed ===\n');
        
    } catch (error) {
        console.error('Error:', error);
    }
}

// Run if executed directly
if (require.main === module) {
    main().catch(console.error);
}

module.exports = { testHttpApi, testWebSocket };
