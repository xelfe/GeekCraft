/**
 * Zone Generation and Visualization Example with Improved Clustering
 *
 * This example demonstrates how to:
 * 1. Generate a new zone via the API
 * 2. Retrieve zone data
 * 3. Visualize the map in ASCII format
 * 4. Analyze tile clustering
 * 5. Work with both solo and online modes
 *
 * Tile Types:
 * - . (Plain):    Walkable, standard movement
 * - ~ (Swamp):    Walkable, slower movement
 * - # (Obstacle): Not walkable, blocks movement
 *
 * Exits:
 * - N/S/E/W: Zone exits on edges
 */

// Configuration for both solo campaign and online modes
const CONFIG = {
    solo: {
        apiUrl: 'http://localhost:3030/api',
        mode: 'local campaign'
    },
    online: {
        apiUrl: 'http://gameserver.example.com/api',
        mode: 'online multiplayer'
    }
};

// Use solo mode by default (local campaign)
const currentMode = CONFIG.solo;

/**
 * Main function to demonstrate zone generation and visualization
 */
async function demonstrateZoneGeneration() {
    try {
        console.log('=== GeekCraft Zone Generation Example ===\n');
        console.log(`Mode: ${currentMode.mode}`);
        console.log(`Server: ${currentMode.apiUrl}\n`);

        // Example 1: Generate a zone for a player
        console.log('1. Generating zone for player...');
        const playerId = 'example_player_' + Date.now();
        const zone = await generateZone(playerId);

        if (zone) {
            console.log(`✓ Zone generated: ${zone.id}\n`);

            // Example 2: Display the zone as ASCII map
            console.log('2. Zone Map (ASCII Visualization):');
            displayZoneMap(zone);
            console.log();

            // Example 3: Analyze terrain distribution
            console.log('3. Terrain Distribution Analysis:');
            analyzeTerrainDistribution(zone);
            console.log();

            // Example 4: Analyze tile clustering
            console.log('4. Tile Clustering Analysis:');
            analyzeClusteringQuality(zone);
            console.log();

            // Example 5: Display exit information
            console.log('5. Zone Exits:');
            displayExits(zone);
            console.log();

            // Example 6: Generate multiple zones for comparison
            console.log('6. Generating multiple zones for comparison...');
            await generateMultipleZones(5);
        }
    } catch (error) {
        console.error('Error during demonstration:', error);
    }
}

/**
 * Generate a new zone via the API
 * @param {string} playerId - Unique player identifier
 * @returns {Promise<Object>} Generated zone object
 */
async function generateZone(playerId) {
    try {
        const response = await fetch(`${currentMode.apiUrl}/zone/generate`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ player_id: playerId })
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Failed to generate zone:', error);
        return null;
    }
}

/**
 * Retrieve an existing zone by ID
 * @param {string} zoneId - Zone identifier
 * @returns {Promise<Object>} Zone object
 */
async function getZone(zoneId) {
    try {
        const response = await fetch(`${currentMode.apiUrl}/zone/${zoneId}`);

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Failed to retrieve zone:', error);
        return null;
    }
}

/**
 * Display zone as ASCII map with improved tile clustering visualization
 * @param {Object} zone - Zone object containing tiles array
 */
function displayZoneMap(zone) {
    // Map surface types to ASCII characters
    const tileSymbols = {
        'Plain': '.',
        'Swamp': '~',
        'Obstacle': '#'
    };

    // Add exits to visualization
    const exitMap = new Map();
    zone.exits.forEach(exit => {
        const key = `${exit.x},${exit.y}`;
        exitMap.set(key, exit.direction[0]); // N, S, E, W
    });

    console.log('┌' + '─'.repeat(zone.tiles[0].length) + '┐');

    zone.tiles.forEach((row, y) => {
        let line = '│';
        row.forEach((tile, x) => {
            const key = `${x},${y}`;
            if (exitMap.has(key)) {
                // Display exit direction
                line += exitMap.get(key);
            } else {
                // Display terrain type
                const symbol = tileSymbols[tile.surface_type] || '?';
                line += symbol;
            }
        });
        line += '│';
        console.log(line);
    });

    console.log('└' + '─'.repeat(zone.tiles[0].length) + '┘');
    console.log('\nLegend: . = Plain | ~ = Swamp | # = Obstacle | N/S/E/W = Exit');
}

/**
 * Analyze and display terrain type distribution
 * @param {Object} zone - Zone object
 */
function analyzeTerrainDistribution(zone) {
    const distribution = {
        'Plain': 0,
        'Swamp': 0,
        'Obstacle': 0
    };

    // Count each surface type
    zone.tiles.forEach(row => {
        row.forEach(tile => {
            distribution[tile.surface_type]++;
        });
    });

    const totalTiles = zone.tiles.length * zone.tiles[0].length;

    console.log(`Total tiles: ${totalTiles}`);
    Object.entries(distribution).forEach(([type, count]) => {
        const percentage = ((count / totalTiles) * 100).toFixed(2);
        const bar = '█'.repeat(Math.floor(percentage / 2));
        console.log(`${type.padEnd(10)}: ${count.toString().padStart(3)} (${percentage.padStart(6)}%) ${bar}`);
    });
}

/**
 * Analyze tile clustering quality
 * Measures how well tiles of the same type are grouped together
 * @param {Object} zone - Zone object
 */
function analyzeClusteringQuality(zone) {
    let adjacentSameType = 0;
    let totalAdjacent = 0;

    // Count adjacent tiles of the same type
    for (let y = 0; y < zone.tiles.length - 1; y++) {
        for (let x = 0; x < zone.tiles[y].length - 1; x++) {
            const currentType = zone.tiles[y][x].surface_type;

            // Check right neighbor
            if (zone.tiles[y][x + 1].surface_type === currentType) {
                adjacentSameType++;
            }
            totalAdjacent++;

            // Check bottom neighbor
            if (zone.tiles[y + 1][x].surface_type === currentType) {
                adjacentSameType++;
            }
            totalAdjacent++;
        }
    }

    const clusteringRatio = (adjacentSameType / totalAdjacent * 100).toFixed(2);
    const randomExpected = 40; // Random distribution would be ~40%

    console.log(`Adjacent same-type tiles: ${adjacentSameType}/${totalAdjacent}`);
    console.log(`Clustering ratio: ${clusteringRatio}%`);
    console.log(`Random baseline: ~${randomExpected}%`);

    if (clusteringRatio > randomExpected + 10) {
        console.log(`✓ Excellent clustering! Tiles are well grouped.`);
    } else if (clusteringRatio > randomExpected) {
        console.log(`✓ Good clustering! Tiles show natural grouping.`);
    } else {
        console.log(`⚠ Clustering could be improved.`);
    }
}

/**
 * Display exit points for the zone
 * @param {Object} zone - Zone object
 */
function displayExits(zone) {
    console.log(`Total exits: ${zone.exits.length}`);
    zone.exits.forEach((exit, index) => {
        console.log(`Exit ${index + 1}: Position (${exit.x}, ${exit.y}) - Direction: ${exit.direction}`);
    });
}

/**
 * Generate multiple zones and compare their characteristics
 * @param {number} count - Number of zones to generate
 */
async function generateMultipleZones(count) {
    const zones = [];
    const clusteringRatios = [];

    for (let i = 0; i < count; i++) {
        const playerId = `comparison_player_${i}_${Date.now()}`;
        const zone = await generateZone(playerId);

        if (zone) {
            zones.push(zone);

            // Calculate clustering for this zone
            let adjacentSameType = 0;
            let totalAdjacent = 0;

            for (let y = 0; y < zone.tiles.length - 1; y++) {
                for (let x = 0; x < zone.tiles[y].length - 1; x++) {
                    const currentType = zone.tiles[y][x].surface_type;

                    if (zone.tiles[y][x + 1].surface_type === currentType) {
                        adjacentSameType++;
                    }
                    totalAdjacent++;

                    if (zone.tiles[y + 1][x].surface_type === currentType) {
                        adjacentSameType++;
                    }
                    totalAdjacent++;
                }
            }

            const ratio = (adjacentSameType / totalAdjacent * 100);
            clusteringRatios.push(ratio);

            console.log(`Zone ${i + 1}: ${zone.id} - Clustering: ${ratio.toFixed(2)}%`);
        }
    }

    // Display statistics
    if (clusteringRatios.length > 0) {
        const average = clusteringRatios.reduce((a, b) => a + b, 0) / clusteringRatios.length;
        const min = Math.min(...clusteringRatios);
        const max = Math.max(...clusteringRatios);

        console.log(`\nClustering Statistics:`);
        console.log(`Average: ${average.toFixed(2)}%`);
        console.log(`Min: ${min.toFixed(2)}%`);
        console.log(`Max: ${max.toFixed(2)}%`);
    }
}

// Run the demonstration when the script loads
demonstrateZoneGeneration().catch(console.error);