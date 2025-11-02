/**
 * GeekCraft Viewer - Basic Visualization Client
 * 
 * This viewer connects to the GeekCraft server via WebSocket
 * and displays the game state in real-time on an HTML5 canvas.
 * 
 * CURRENT SERVER SUPPORT (v0.2.0+):
 * - AUTHENTICATION: Token-based authentication required
 * - DYNAMIC: tick (game tick counter) and players (list of player IDs)
 * - MULTIPLAYER: Multiple authenticated users can connect concurrently
 * - PLACEHOLDER: units, buildings, resources, etc. are not yet implemented
 *   on the server side. UI elements for these features are kept as placeholders
 *   for future development.
 */

class GeekCraftViewer {
    constructor() {
        this.ws = null;
        this.canvas = document.getElementById('game-canvas');
        this.ctx = this.canvas.getContext('2d');
        
        // Authentication state
        this.token = null;
        this.username = null;
        this.authenticated = false;
        
        // Game state (only tick and players are currently supported by server)
        this.gameState = null;
        
        // Placeholder state for future features (not yet implemented on server)
        this.selectedUnit = null;
        
        // View parameters
        this.camera = {
            x: 0,
            y: 0,
            zoom: 1.0
        };
        
        // Connection state
        this.connected = false;
        
        // Polling interval for auto-updates
        this.pollingInterval = null;
        
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.startRenderLoop();
        this.log('Viewer initialized. Connect to a server to begin.');
    }

    setupEventListeners() {
        // User menu toggle
        document.getElementById('user-menu-btn').addEventListener('click', () => this.openUserMenu());
        document.getElementById('close-menu-btn').addEventListener('click', () => this.closeUserMenu());
        document.getElementById('user-menu-modal').addEventListener('click', (e) => {
            if (e.target.id === 'user-menu-modal') {
                this.closeUserMenu();
            }
        });
        
        // Authentication buttons (from menu)
        document.getElementById('login-btn-menu').addEventListener('click', () => this.login());
        document.getElementById('register-btn-menu').addEventListener('click', () => this.register());
        document.getElementById('logout-btn-menu').addEventListener('click', () => this.logout());
        
        // Connection buttons (from menu)
        document.getElementById('connect-btn-menu').addEventListener('click', () => this.connect());
        document.getElementById('disconnect-btn-menu').addEventListener('click', () => this.disconnect());
        
        // Camera controls
        document.getElementById('zoom-in').addEventListener('click', () => this.zoom(1.2));
        document.getElementById('zoom-out').addEventListener('click', () => this.zoom(0.8));
        document.getElementById('reset-view').addEventListener('click', () => this.resetView());
        
        // Speed control
        document.getElementById('game-speed').addEventListener('input', (e) => {
            document.getElementById('speed-value').textContent = e.target.value + 'x';
            this.sendCommand({ type: 'setSpeed', speed: parseFloat(e.target.value) });
        });
        
        // Selection on canvas
        this.canvas.addEventListener('click', (e) => this.handleCanvasClick(e));
        
        // Mouse wheel scroll
        this.canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            this.zoom(e.deltaY > 0 ? 0.9 : 1.1);
        });
        
        // Console
        document.getElementById('clear-console').addEventListener('click', () => {
            document.getElementById('console-output').innerHTML = '';
        });
    }
    
    openUserMenu() {
        document.getElementById('user-menu-modal').style.display = 'flex';
    }
    
    closeUserMenu() {
        document.getElementById('user-menu-modal').style.display = 'none';
    }

    async register() {
        const username = document.getElementById('username-menu').value.trim();
        const password = document.getElementById('password-menu').value;
        const apiUrl = document.getElementById('api-url-menu').value;
        
        if (!username || !password) {
            this.log('Please enter username and password', 'error');
            return;
        }
        
        try {
            const response = await fetch(`${apiUrl}/api/auth/register`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password }),
            });
            
            const data = await response.json();
            
            if (data.success) {
                this.log(`✓ Registration successful for ${username}`, 'success');
                this.log('Please login to continue', 'info');
            } else {
                this.log(`Registration failed: ${data.message}`, 'error');
            }
        } catch (error) {
            this.log(`Registration error: ${error.message}`, 'error');
        }
    }
    
    async login() {
        const username = document.getElementById('username-menu').value.trim();
        const password = document.getElementById('password-menu').value;
        const apiUrl = document.getElementById('api-url-menu').value;
        
        if (!username || !password) {
            this.log('Please enter username and password', 'error');
            return;
        }
        
        try {
            const response = await fetch(`${apiUrl}/api/auth/login`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password }),
            });
            
            const data = await response.json();
            
            if (data.success && data.token) {
                this.token = data.token;
                this.username = data.username;
                this.updateAuthStatus(true);
                this.log(`✓ Logged in as ${this.username}`, 'success');
                this.log('You can now connect to the game server', 'info');
            } else {
                this.log(`Login failed: ${data.message}`, 'error');
            }
        } catch (error) {
            this.log(`Login error: ${error.message}`, 'error');
        }
    }
    
    async logout() {
        if (!this.token) {
            this.log('Not logged in', 'warning');
            return;
        }
        
        const apiUrl = document.getElementById('api-url-menu').value;
        
        try {
            const response = await fetch(`${apiUrl}/api/auth/logout`, {
                method: 'POST',
                headers: { 'Authorization': `Bearer ${this.token}` },
            });
            
            const data = await response.json();
            
            if (data.success) {
                this.log('✓ Logged out successfully', 'success');
            }
        } catch (error) {
            this.log(`Logout error: ${error.message}`, 'error');
        } finally {
            // Clear auth state regardless
            this.token = null;
            this.username = null;
            this.authenticated = false;
            this.updateAuthStatus(false);
            this.disconnect();
        }
    }
    
    connect() {
        if (!this.token) {
            this.log('Please login first before connecting to the game server', 'error');
            this.openUserMenu(); // Open menu to prompt login
            return;
        }
        
        const serverUrl = document.getElementById('server-url-menu').value;
        this.log(`Connecting to ${serverUrl}...`);
        
        try {
            this.ws = new WebSocket(serverUrl);
            
            this.ws.onopen = () => this.onConnected();
            this.ws.onmessage = (event) => this.onMessage(event);
            this.ws.onerror = (error) => this.onError(error);
            this.ws.onclose = () => this.onDisconnected();
            
        } catch (error) {
            this.log(`Connection error: ${error.message}`, 'error');
        }
    }

    disconnect() {
        this.stopPolling();
        if (this.ws) {
            this.ws.close();
        }
    }

    onConnected() {
        this.connected = true;
        this.updateConnectionStatus(true);
        this.log('✓ Connected to WebSocket', 'success');
        this.log('Waiting for welcome message...', 'info');
    }

    onDisconnected() {
        this.connected = false;
        this.stopPolling();
        this.updateConnectionStatus(false);
        this.log('✗ Disconnected from server', 'warning');
    }

    onError(error) {
        this.log(`WebSocket error: ${error}`, 'error');
    }

    onMessage(event) {
        try {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        } catch (error) {
            this.log(`Parsing error: ${error.message}`, 'error');
        }
    }

    handleMessage(message) {
        switch (message.type) {
            case 'welcome':
                // Handle welcome message from server
                this.log(`Server: ${message.message}`, 'success');
                // Authenticate with token
                this.log('Authenticating...', 'info');
                this.sendCommand({ type: 'auth', token: this.token });
                break;
            
            case 'authResponse':
                // Handle authentication response
                if (message.success) {
                    this.authenticated = true;
                    this.log(`✓ Authenticated as ${message.username}`, 'success');
                    // Request initial game state
                    this.sendCommand({ type: 'getGameState' });
                    // Start polling for updates
                    this.startPolling();
                } else {
                    this.log(`✗ Authentication failed: ${message.message}`, 'error');
                    this.disconnect();
                }
                break;
            
            case 'gameStateResponse':
                // DYNAMIC: Handle game state response from server
                // Server currently provides: tick and players (array of player ID strings)
                if (!this.gameState) {
                    this.gameState = { tick: 0, players: [] };
                }
                this.gameState.tick = message.tick || 0;
                this.gameState.players = message.players || [];
                this.updateUI();
                break;
            
            case 'playersResponse':
                // DYNAMIC: Handle players list response
                if (!this.gameState) {
                    this.gameState = { tick: 0, players: [] };
                }
                this.gameState.players = message.players || [];
                this.updateUI();
                break;
            
            case 'gameState':
                // Legacy/future message type (placeholder)
                this.gameState = message.data;
                this.updateUI();
                break;
            
            case 'gameUpdate':
                // PLACEHOLDER: Incremental update (not yet implemented on server)
                this.applyUpdate(message.data);
                this.updateUI();
                break;
            
            case 'event':
                // PLACEHOLDER: Game events (not yet implemented on server)
                this.handleGameEvent(message.data);
                break;
            
            case 'error':
                this.log(`Server error: ${message.message}`, 'error');
                break;
            
            default:
                console.log('Unhandled message:', message);
        }
    }

    applyUpdate(update) {
        // PLACEHOLDER: This function is for future incremental updates
        // Currently not used as the server does not send incremental updates
        if (!this.gameState) {
            this.gameState = update;
            return;
        }
        
        // Update tick
        if (update.tick !== undefined) {
            this.gameState.tick = update.tick;
        }
        
        // Update players (only supported field currently)
        if (update.players) {
            this.gameState.players = update.players;
        }
        
        // PLACEHOLDER: Units, buildings, resources (not yet implemented)
        // These will be handled when server support is added
    }

    handleGameEvent(event) {
        // PLACEHOLDER: Game events are not yet implemented on the server
        this.log(`Event: ${event.type} - ${JSON.stringify(event.data)}`, 'info');
    }

    sendCommand(command) {
        if (this.ws && this.connected) {
            this.ws.send(JSON.stringify(command));
        }
    }

    startPolling() {
        // Clear any existing interval to prevent duplicates
        this.stopPolling();
        
        this.pollingInterval = setInterval(() => {
            if (this.ws && this.connected) {
                this.sendCommand({ type: 'getGameState' });
            }
        }, 1000);
    }

    stopPolling() {
        if (this.pollingInterval) {
            clearInterval(this.pollingInterval);
            this.pollingInterval = null;
        }
    }

    updateUI() {
        if (!this.gameState) return;
        
        // DYNAMIC: Update tick (from server)
        document.getElementById('game-tick').textContent = this.gameState.tick || 0;
        
        // DYNAMIC: Update player count (from server)
        document.getElementById('player-count').textContent = this.gameState.players?.length || 0;
        
        // PLACEHOLDER: Unit count not yet implemented on server
        document.getElementById('unit-count').textContent = 'N/A';
        
        // Update players list
        this.updatePlayersList();
    }

    updatePlayersList() {
        const playersList = document.getElementById('players-list');
        if (!this.gameState || !this.gameState.players) {
            playersList.innerHTML = '<p class="empty-state">No players</p>';
            return;
        }
        
        // DYNAMIC: Display player IDs from server
        // PLACEHOLDER: Resources and unit counts are not yet implemented
        playersList.innerHTML = this.gameState.players.map((playerId, index) => {
            // Generate a simple color based on player index using golden angle
            const hue = (index * 137.507764) % 360; // Golden angle for better color distribution
            const color = `hsl(${hue}, 70%, 60%)`;
            
            return `
            <div class="player-item" style="border-left: 3px solid ${color}">
                <div class="player-name">${playerId}</div>
                <div class="player-stats">
                    <span>⚡ N/A</span>
                    <span>👥 N/A</span>
                </div>
            </div>
        `}).join('');
    }

    updateAuthStatus(loggedIn) {
        const loginBtn = document.getElementById('login-btn-menu');
        const registerBtn = document.getElementById('register-btn-menu');
        const logoutBtn = document.getElementById('logout-btn-menu');
        const connectBtn = document.getElementById('connect-btn-menu');
        const usernameField = document.getElementById('username-menu');
        const passwordField = document.getElementById('password-menu');
        const authStatus = document.getElementById('auth-status-menu');
        
        if (loggedIn) {
            loginBtn.disabled = true;
            registerBtn.disabled = true;
            logoutBtn.disabled = false;
            connectBtn.disabled = false;
            usernameField.disabled = true;
            passwordField.disabled = true;
            authStatus.textContent = `Logged in as ${this.username}`;
            authStatus.className = 'auth-status auth-logged-in';
        } else {
            loginBtn.disabled = false;
            registerBtn.disabled = false;
            logoutBtn.disabled = true;
            connectBtn.disabled = true;
            usernameField.disabled = false;
            passwordField.disabled = false;
            authStatus.textContent = 'Not logged in';
            authStatus.className = 'auth-status auth-logged-out';
        }
    }
    
    updateConnectionStatus(connected) {
        const indicator = document.getElementById('status-indicator');
        const text = document.getElementById('status-text');
        const connectBtn = document.getElementById('connect-btn-menu');
        const disconnectBtn = document.getElementById('disconnect-btn-menu');
        
        if (connected) {
            indicator.className = 'status-connected';
            text.textContent = this.authenticated ? 'Connected & Authenticated' : 'Connected (authenticating...)';
            connectBtn.disabled = true;
            disconnectBtn.disabled = false;
        } else {
            indicator.className = 'status-disconnected';
            text.textContent = 'Disconnected';
            connectBtn.disabled = !this.token; // Can only connect if logged in
            disconnectBtn.disabled = true;
            this.authenticated = false;
        }
    }

    // Rendering
    startRenderLoop() {
        const render = () => {
            this.render();
            requestAnimationFrame(render);
        };
        requestAnimationFrame(render);
    }

    render() {
        // Clear canvas
        this.ctx.fillStyle = '#1a1a1a';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        
        if (!this.gameState) {
            this.renderNoData();
            return;
        }
        
        // Apply camera transformation
        this.ctx.save();
        this.ctx.translate(this.camera.x, this.camera.y);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        
        // Draw grid
        this.renderGrid();
        
        // PLACEHOLDER: Resources, units, and buildings are not yet implemented
        // The rendering functions are kept for future development
        // Uncomment these when server support is added:
        // this.renderResources();
        // this.renderUnits();
        // this.renderBuildings();
        
        this.ctx.restore();
        
        // Draw interface (without transformation)
        this.renderUI();
    }

    renderNoData() {
        this.ctx.fillStyle = '#666';
        this.ctx.font = '20px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.fillText('Waiting for server data...', this.canvas.width / 2, this.canvas.height / 2);
        this.ctx.fillText('(Connect to see tick and player information)', this.canvas.width / 2, this.canvas.height / 2 + 30);
    }

    renderGrid() {
        // Draw a simple grid as a placeholder background
        const gridSize = 50;
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = 1;
        
        for (let x = 0; x < this.canvas.width; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }
        
        for (let y = 0; y < this.canvas.height; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.canvas.width, y);
            this.ctx.stroke();
        }
    }

    // PLACEHOLDER RENDERING FUNCTIONS
    // These functions are kept for future implementation when server support is added
    // They are not currently called in the render loop

    renderResources() {
        // PLACEHOLDER: Resource rendering (not yet implemented on server)
        if (!this.gameState.resources) return;
        
        this.gameState.resources.forEach(resource => {
            this.ctx.fillStyle = '#FFD700';
            this.ctx.beginPath();
            this.ctx.arc(resource.x, resource.y, 8, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Display quantity
            this.ctx.fillStyle = '#FFF';
            this.ctx.font = '10px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(resource.amount, resource.x, resource.y - 12);
        });
    }

    renderUnits() {
        // PLACEHOLDER: Unit rendering (not yet implemented on server)
        if (!this.gameState.units) return;
        
        this.gameState.units.forEach(unit => {
            const isSelected = this.selectedUnit && this.selectedUnit.id === unit.id;
            
            // Player color
            this.ctx.fillStyle = unit.playerColor || '#4CAF50';
            
            // Selection circle
            if (isSelected) {
                this.ctx.strokeStyle = '#FFF';
                this.ctx.lineWidth = 2;
                this.ctx.beginPath();
                this.ctx.arc(unit.x, unit.y, 18, 0, Math.PI * 2);
                this.ctx.stroke();
            }
            
            // Unit
            this.ctx.beginPath();
            this.ctx.arc(unit.x, unit.y, 12, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Health bar
            const healthPercent = unit.health / unit.maxHealth;
            this.ctx.fillStyle = healthPercent > 0.5 ? '#4CAF50' : healthPercent > 0.25 ? '#FFA500' : '#F44336';
            this.ctx.fillRect(unit.x - 10, unit.y - 20, 20 * healthPercent, 3);
        });
    }

    renderBuildings() {
        // PLACEHOLDER: Building rendering (not yet implemented on server)
        if (!this.gameState.buildings) return;
        
        this.gameState.buildings.forEach(building => {
            this.ctx.fillStyle = building.playerColor || '#2196F3';
            this.ctx.fillRect(building.x - 15, building.y - 15, 30, 30);
            
            // Building name
            this.ctx.fillStyle = '#FFF';
            this.ctx.font = '10px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(building.type, building.x, building.y + 25);
        });
    }

    renderUI() {
        // Display FPS
        this.ctx.fillStyle = '#FFF';
        this.ctx.font = '12px monospace';
        this.ctx.textAlign = 'left';
        this.ctx.fillText(`Zoom: ${this.camera.zoom.toFixed(2)}x`, 10, 20);
    }

    // Interactions
    handleCanvasClick(event) {
        // PLACEHOLDER: Unit selection (not yet implemented on server)
        // Kept for future development when units are supported
        if (!this.gameState || !this.gameState.units) return;
        
        const rect = this.canvas.getBoundingClientRect();
        const x = (event.clientX - rect.left - this.camera.x) / this.camera.zoom;
        const y = (event.clientY - rect.top - this.camera.y) / this.camera.zoom;
        
        // Find clicked unit
        const clickedUnit = this.gameState.units.find(unit => {
            const dx = unit.x - x;
            const dy = unit.y - y;
            return Math.sqrt(dx * dx + dy * dy) < 12;
        });
        
        if (clickedUnit) {
            this.selectUnit(clickedUnit);
        } else {
            this.selectedUnit = null;
            this.updateSelectionInfo();
        }
    }

    selectUnit(unit) {
        // PLACEHOLDER: Unit selection (not yet implemented on server)
        this.selectedUnit = unit;
        this.updateSelectionInfo();
        this.log(`Unit selected: ${unit.id}`, 'info');
    }

    updateSelectionInfo() {
        // PLACEHOLDER: Unit details (not yet implemented on server)
        const infoPanel = document.getElementById('selection-info');
        
        if (!this.selectedUnit) {
            infoPanel.innerHTML = '<p class="empty-state">Unit selection not yet supported by server</p>';
            return;
        }
        
        const unit = this.selectedUnit;
        infoPanel.innerHTML = `
            <div class="unit-details">
                <h3>Unit #${unit.id}</h3>
                <div class="detail-row">
                    <span>Type:</span>
                    <span>${unit.type || 'Unknown'}</span>
                </div>
                <div class="detail-row">
                    <span>Health:</span>
                    <span>${unit.health} / ${unit.maxHealth}</span>
                </div>
                <div class="detail-row">
                    <span>Position:</span>
                    <span>(${Math.round(unit.x)}, ${Math.round(unit.y)})</span>
                </div>
                <div class="detail-row">
                    <span>State:</span>
                    <span>${unit.state || 'idle'}</span>
                </div>
            </div>
        `;
    }

    zoom(factor) {
        this.camera.zoom *= factor;
        this.camera.zoom = Math.max(0.5, Math.min(3, this.camera.zoom));
    }

    resetView() {
        this.camera.x = 0;
        this.camera.y = 0;
        this.camera.zoom = 1.0;
    }

    log(message, type = 'info') {
        const console = document.getElementById('console-output');
        const timestamp = new Date().toLocaleTimeString();
        const entry = document.createElement('div');
        entry.className = `console-entry console-${type}`;
        entry.textContent = `[${timestamp}] ${message}`;
        console.appendChild(entry);
        console.scrollTop = console.scrollHeight;
    }
}

// Initialize viewer on page load
document.addEventListener('DOMContentLoaded', () => {
    window.viewer = new GeekCraftViewer();
});
