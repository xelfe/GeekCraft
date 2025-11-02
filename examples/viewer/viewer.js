/**
 * GeekCraft Viewer - Basic Visualization Client
 * 
 * This viewer connects to the GeekCraft server via WebSocket
 * and displays the game state in real-time on an HTML5 canvas.
 */

class GeekCraftViewer {
    constructor() {
        this.ws = null;
        this.canvas = document.getElementById('game-canvas');
        this.ctx = this.canvas.getContext('2d');
        this.gameState = null;
        this.selectedUnit = null;
        
        // View parameters
        this.camera = {
            x: 0,
            y: 0,
            zoom: 1.0
        };
        
        // Connection state
        this.connected = false;
        
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.startRenderLoop();
        this.log('Viewer initialized. Connect to a server to begin.');
    }

    setupEventListeners() {
        // Connection buttons
        document.getElementById('connect-btn').addEventListener('click', () => this.connect());
        document.getElementById('disconnect-btn').addEventListener('click', () => this.disconnect());
        
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

    connect() {
        const serverUrl = document.getElementById('server-url').value;
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
        if (this.ws) {
            this.ws.close();
        }
    }

    onConnected() {
        this.connected = true;
        this.updateConnectionStatus(true);
        this.log('✓ Connected to server', 'success');
        
        // Request initial game state
        this.sendCommand({ type: 'getGameState' });
    }

    onDisconnected() {
        this.connected = false;
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
                // Handle welcome message from new server
                this.log(`Server: ${message.message}`, 'success');
                // Request initial game state
                this.sendCommand({ type: 'getGameState' });
                break;
            
            case 'gameStateResponse':
                // Handle game state response from new server
                if (!this.gameState) {
                    this.gameState = { tick: 0, players: [], units: [] };
                }
                this.gameState.tick = message.tick || 0;
                this.gameState.players = message.players || [];
                this.updateUI();
                break;
            
            case 'playersResponse':
                // Handle players list response
                if (!this.gameState) {
                    this.gameState = { tick: 0, players: [], units: [] };
                }
                this.gameState.players = message.players || [];
                this.updateUI();
                break;
            
            case 'gameState':
                this.gameState = message.data;
                this.updateUI();
                break;
            
            case 'gameUpdate':
                // Incremental update
                this.applyUpdate(message.data);
                this.updateUI();
                break;
            
            case 'event':
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
        if (!this.gameState) {
            this.gameState = update;
            return;
        }
        
        // Update tick
        if (update.tick !== undefined) {
            this.gameState.tick = update.tick;
        }
        
        // Update units
        if (update.units) {
            this.gameState.units = update.units;
        }
        
        // Update players
        if (update.players) {
            this.gameState.players = update.players;
        }
    }

    handleGameEvent(event) {
        this.log(`Event: ${event.type} - ${JSON.stringify(event.data)}`, 'info');
    }

    sendCommand(command) {
        if (this.ws && this.connected) {
            this.ws.send(JSON.stringify(command));
        }
    }

    updateUI() {
        if (!this.gameState) return;
        
        // Update information
        document.getElementById('game-tick').textContent = this.gameState.tick || 0;
        document.getElementById('player-count').textContent = this.gameState.players?.length || 0;
        document.getElementById('unit-count').textContent = this.gameState.units?.length || 0;
        
        // Update players list
        this.updatePlayersList();
    }

    updatePlayersList() {
        const playersList = document.getElementById('players-list');
        if (!this.gameState || !this.gameState.players) {
            playersList.innerHTML = '<p class="empty-state">No players</p>';
            return;
        }
        
        playersList.innerHTML = this.gameState.players.map(player => `
            <div class="player-item" style="border-left: 3px solid ${player.color}">
                <div class="player-name">${player.name}</div>
                <div class="player-stats">
                    <span>⚡ ${player.resources?.minerals || 0}</span>
                    <span>👥 ${player.unitCount || 0}</span>
                </div>
            </div>
        `).join('');
    }

    updateConnectionStatus(connected) {
        const indicator = document.getElementById('status-indicator');
        const text = document.getElementById('status-text');
        const connectBtn = document.getElementById('connect-btn');
        const disconnectBtn = document.getElementById('disconnect-btn');
        
        if (connected) {
            indicator.className = 'status-connected';
            text.textContent = 'Connected';
            connectBtn.disabled = true;
            disconnectBtn.disabled = false;
        } else {
            indicator.className = 'status-disconnected';
            text.textContent = 'Disconnected';
            connectBtn.disabled = false;
            disconnectBtn.disabled = true;
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
        
        // Draw resources
        this.renderResources();
        
        // Draw units
        this.renderUnits();
        
        // Draw buildings
        this.renderBuildings();
        
        this.ctx.restore();
        
        // Draw interface (without transformation)
        this.renderUI();
    }

    renderNoData() {
        this.ctx.fillStyle = '#666';
        this.ctx.font = '20px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.fillText('Waiting for data...', this.canvas.width / 2, this.canvas.height / 2);
    }

    renderGrid() {
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

    renderResources() {
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
        this.selectedUnit = unit;
        this.updateSelectionInfo();
        this.log(`Unit selected: ${unit.id}`, 'info');
    }

    updateSelectionInfo() {
        const infoPanel = document.getElementById('selection-info');
        
        if (!this.selectedUnit) {
            infoPanel.innerHTML = '<p class="empty-state">Select a unit to see its details</p>';
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
