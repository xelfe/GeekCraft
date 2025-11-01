/**
 * GeekCraft Viewer - Client de visualisation basique
 * 
 * Ce viewer se connecte au serveur GeekCraft via WebSocket
 * et affiche l'état du jeu en temps réel sur un canvas HTML5.
 */

class GeekCraftViewer {
    constructor() {
        this.ws = null;
        this.canvas = document.getElementById('game-canvas');
        this.ctx = this.canvas.getContext('2d');
        this.gameState = null;
        this.selectedUnit = null;
        
        // Paramètres de vue
        this.camera = {
            x: 0,
            y: 0,
            zoom: 1.0
        };
        
        // État de la connexion
        this.connected = false;
        
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.startRenderLoop();
        this.log('Viewer initialisé. Connectez-vous à un serveur pour commencer.');
    }

    setupEventListeners() {
        // Boutons de connexion
        document.getElementById('connect-btn').addEventListener('click', () => this.connect());
        document.getElementById('disconnect-btn').addEventListener('click', () => this.disconnect());
        
        // Contrôles de caméra
        document.getElementById('zoom-in').addEventListener('click', () => this.zoom(1.2));
        document.getElementById('zoom-out').addEventListener('click', () => this.zoom(0.8));
        document.getElementById('reset-view').addEventListener('click', () => this.resetView());
        
        // Contrôle de vitesse
        document.getElementById('game-speed').addEventListener('input', (e) => {
            document.getElementById('speed-value').textContent = e.target.value + 'x';
            this.sendCommand({ type: 'setSpeed', speed: parseFloat(e.target.value) });
        });
        
        // Sélection sur le canvas
        this.canvas.addEventListener('click', (e) => this.handleCanvasClick(e));
        
        // Défilement avec la souris
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
        this.log(`Connexion à ${serverUrl}...`);
        
        try {
            this.ws = new WebSocket(serverUrl);
            
            this.ws.onopen = () => this.onConnected();
            this.ws.onmessage = (event) => this.onMessage(event);
            this.ws.onerror = (error) => this.onError(error);
            this.ws.onclose = () => this.onDisconnected();
            
        } catch (error) {
            this.log(`Erreur de connexion: ${error.message}`, 'error');
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
        this.log('✓ Connecté au serveur', 'success');
        
        // Demander l'état initial du jeu
        this.sendCommand({ type: 'getGameState' });
    }

    onDisconnected() {
        this.connected = false;
        this.updateConnectionStatus(false);
        this.log('✗ Déconnecté du serveur', 'warning');
    }

    onError(error) {
        this.log(`Erreur WebSocket: ${error}`, 'error');
    }

    onMessage(event) {
        try {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        } catch (error) {
            this.log(`Erreur de parsing: ${error.message}`, 'error');
        }
    }

    handleMessage(message) {
        switch (message.type) {
            case 'gameState':
                this.gameState = message.data;
                this.updateUI();
                break;
            
            case 'gameUpdate':
                // Mise à jour incrémentale
                this.applyUpdate(message.data);
                this.updateUI();
                break;
            
            case 'event':
                this.handleGameEvent(message.data);
                break;
            
            default:
                console.log('Message non géré:', message);
        }
    }

    applyUpdate(update) {
        if (!this.gameState) {
            this.gameState = update;
            return;
        }
        
        // Mettre à jour le tick
        if (update.tick !== undefined) {
            this.gameState.tick = update.tick;
        }
        
        // Mettre à jour les unités
        if (update.units) {
            this.gameState.units = update.units;
        }
        
        // Mettre à jour les joueurs
        if (update.players) {
            this.gameState.players = update.players;
        }
    }

    handleGameEvent(event) {
        this.log(`Événement: ${event.type} - ${JSON.stringify(event.data)}`, 'info');
    }

    sendCommand(command) {
        if (this.ws && this.connected) {
            this.ws.send(JSON.stringify(command));
        }
    }

    updateUI() {
        if (!this.gameState) return;
        
        // Mettre à jour les informations
        document.getElementById('game-tick').textContent = this.gameState.tick || 0;
        document.getElementById('player-count').textContent = this.gameState.players?.length || 0;
        document.getElementById('unit-count').textContent = this.gameState.units?.length || 0;
        
        // Mettre à jour la liste des joueurs
        this.updatePlayersList();
    }

    updatePlayersList() {
        const playersList = document.getElementById('players-list');
        if (!this.gameState || !this.gameState.players) {
            playersList.innerHTML = '<p class="empty-state">Aucun joueur</p>';
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
            text.textContent = 'Connecté';
            connectBtn.disabled = true;
            disconnectBtn.disabled = false;
        } else {
            indicator.className = 'status-disconnected';
            text.textContent = 'Déconnecté';
            connectBtn.disabled = false;
            disconnectBtn.disabled = true;
        }
    }

    // Rendu
    startRenderLoop() {
        const render = () => {
            this.render();
            requestAnimationFrame(render);
        };
        requestAnimationFrame(render);
    }

    render() {
        // Effacer le canvas
        this.ctx.fillStyle = '#1a1a1a';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        
        if (!this.gameState) {
            this.renderNoData();
            return;
        }
        
        // Appliquer la transformation de caméra
        this.ctx.save();
        this.ctx.translate(this.camera.x, this.camera.y);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        
        // Dessiner la grille
        this.renderGrid();
        
        // Dessiner les ressources
        this.renderResources();
        
        // Dessiner les unités
        this.renderUnits();
        
        // Dessiner les bâtiments
        this.renderBuildings();
        
        this.ctx.restore();
        
        // Dessiner l'interface (sans transformation)
        this.renderUI();
    }

    renderNoData() {
        this.ctx.fillStyle = '#666';
        this.ctx.font = '20px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.fillText('En attente de données...', this.canvas.width / 2, this.canvas.height / 2);
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
            
            // Afficher la quantité
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
            
            // Couleur du joueur
            this.ctx.fillStyle = unit.playerColor || '#4CAF50';
            
            // Cercle de sélection
            if (isSelected) {
                this.ctx.strokeStyle = '#FFF';
                this.ctx.lineWidth = 2;
                this.ctx.beginPath();
                this.ctx.arc(unit.x, unit.y, 18, 0, Math.PI * 2);
                this.ctx.stroke();
            }
            
            // Unité
            this.ctx.beginPath();
            this.ctx.arc(unit.x, unit.y, 12, 0, Math.PI * 2);
            this.ctx.fill();
            
            // Barre de vie
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
            
            // Nom du bâtiment
            this.ctx.fillStyle = '#FFF';
            this.ctx.font = '10px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.fillText(building.type, building.x, building.y + 25);
        });
    }

    renderUI() {
        // Afficher les FPS
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
        
        // Trouver l'unité cliquée
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
        this.log(`Unité sélectionnée: ${unit.id}`, 'info');
    }

    updateSelectionInfo() {
        const infoPanel = document.getElementById('selection-info');
        
        if (!this.selectedUnit) {
            infoPanel.innerHTML = '<p class="empty-state">Sélectionnez une unité pour voir ses détails</p>';
            return;
        }
        
        const unit = this.selectedUnit;
        infoPanel.innerHTML = `
            <div class="unit-details">
                <h3>Unité #${unit.id}</h3>
                <div class="detail-row">
                    <span>Type:</span>
                    <span>${unit.type || 'Unknown'}</span>
                </div>
                <div class="detail-row">
                    <span>Santé:</span>
                    <span>${unit.health} / ${unit.maxHealth}</span>
                </div>
                <div class="detail-row">
                    <span>Position:</span>
                    <span>(${Math.round(unit.x)}, ${Math.round(unit.y)})</span>
                </div>
                <div class="detail-row">
                    <span>État:</span>
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

// Initialiser le viewer au chargement de la page
document.addEventListener('DOMContentLoaded', () => {
    window.viewer = new GeekCraftViewer();
});
