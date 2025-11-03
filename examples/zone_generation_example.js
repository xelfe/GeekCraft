// Zone Generation Example for GeekCraft
// This example demonstrates how to generate and interact with procedurally generated zones
// Run with: node examples/zone_generation_example.js

const BASE_URL = 'http://localhost:3030';

/**
 * Generate a new zone for a player
 * @param {string} playerId - Unique identifier for the player
 * @returns {Promise<Object>} Response with zone_id
 */
async function generateZone(playerId) {
  try {
    const response = await fetch(`${BASE_URL}/api/zone/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ player_id: playerId }),
    });

    const data = await response.json();
    console.log('Generate zone response:', data);
    return data;
  } catch (error) {
    console.error('Error generating zone:', error);
    throw error;
  }
}

/**
 * Get zone data by zone ID
 * @param {string} zoneId - Zone identifier
 * @returns {Promise<Object>} Response with zone data
 */
async function getZone(zoneId) {
  try {
    const response = await fetch(`${BASE_URL}/api/zone/${zoneId}`);
    const data = await response.json();
    console.log(`Zone ${zoneId} data retrieved`);
    return data;
  } catch (error) {
    console.error('Error getting zone:', error);
    throw error;
  }
}

/**
 * List all zones
 * @returns {Promise<Object>} Response with list of zone IDs
 */
async function listZones() {
  try {
    const response = await fetch(`${BASE_URL}/api/zones`);
    const data = await response.json();
    console.log('List zones response:', data);
    return data;
  } catch (error) {
    console.error('Error listing zones:', error);
    throw error;
  }
}

/**
 * Analyze zone terrain distribution
 * @param {Object} zone - Zone object with tiles
 */
function analyzeZone(zone) {
  console.log('\n=== Zone Analysis ===');
  console.log(`Zone ID: ${zone.id}`);
  console.log(`Size: ${zone.tiles.length}x${zone.tiles[0].length} tiles`);
  console.log(`Exits: ${zone.exits.length}`);
  
  // Count surface types
  let plainCount = 0;
  let swampCount = 0;
  let obstacleCount = 0;
  
  for (const row of zone.tiles) {
    for (const tile of row) {
      switch (tile.surface_type) {
        case 'Plain':
          plainCount++;
          break;
        case 'Swamp':
          swampCount++;
          break;
        case 'Obstacle':
          obstacleCount++;
          break;
      }
    }
  }
  
  const total = plainCount + swampCount + obstacleCount;
  
  console.log('\nTerrain Distribution:');
  console.log(`  Plain:    ${plainCount} tiles (${((plainCount / total) * 100).toFixed(1)}%)`);
  console.log(`  Swamp:    ${swampCount} tiles (${((swampCount / total) * 100).toFixed(1)}%)`);
  console.log(`  Obstacle: ${obstacleCount} tiles (${((obstacleCount / total) * 100).toFixed(1)}%)`);
  
  console.log('\nExit Locations:');
  for (const exit of zone.exits) {
    console.log(`  ${exit.direction}: (${exit.x}, ${exit.y})`);
  }
}

/**
 * Visualize zone terrain as ASCII art
 * @param {Object} zone - Zone object with tiles
 */
function visualizeZone(zone) {
  console.log('\n=== Zone Visualization ===');
  console.log('Legend: . = Plain, ~ = Swamp, # = Obstacle, E = Exit\n');
  
  // Create a map of exit positions
  const exitMap = new Map();
  for (const exit of zone.exits) {
    exitMap.set(`${exit.x},${exit.y}`, true);
  }
  
  // Print top border
  console.log('┌' + '─'.repeat(zone.tiles[0].length) + '┐');
  
  // Print each row
  for (let y = 0; y < zone.tiles.length; y++) {
    let row = '│';
    for (let x = 0; x < zone.tiles[y].length; x++) {
      // Check if this is an exit
      if (exitMap.has(`${x},${y}`)) {
        row += 'E';
      } else {
        const tile = zone.tiles[y][x];
        switch (tile.surface_type) {
          case 'Plain':
            row += '.';
            break;
          case 'Swamp':
            row += '~';
            break;
          case 'Obstacle':
            row += '#';
            break;
        }
      }
    }
    row += '│';
    console.log(row);
  }
  
  // Print bottom border
  console.log('└' + '─'.repeat(zone.tiles[0].length) + '┘');
}

/**
 * Main demonstration function
 */
async function main() {
  console.log('=== GeekCraft Zone Generation Example ===\n');

  try {
    // Step 1: Generate a zone for player1
    console.log('Step 1: Generating zone for player1...');
    const generateResponse = await generateZone('player1');
    const zoneId = generateResponse.zone_id;
    console.log(`Zone generated: ${zoneId}\n`);

    // Step 2: Retrieve the zone data
    console.log('Step 2: Retrieving zone data...');
    const zoneResponse = await getZone(zoneId);
    const zone = zoneResponse.zone;
    
    // Step 3: Analyze the zone
    console.log('Step 3: Analyzing zone terrain...');
    analyzeZone(zone);
    
    // Step 4: Visualize the zone
    console.log('\nStep 4: Visualizing zone...');
    visualizeZone(zone);

    // Step 5: Generate zones for more players
    console.log('\n\nStep 5: Generating zones for additional players...');
    await generateZone('player2');
    await generateZone('player3');
    console.log('');

    // Step 6: List all zones
    console.log('Step 6: Listing all zones...');
    const listResponse = await listZones();
    console.log(`Total zones in world: ${listResponse.zone_ids.length}`);
    console.log('');

    console.log('=== Example completed successfully! ===');
  } catch (error) {
    console.error('Example failed:', error);
    process.exit(1);
  }
}

// Run the example
if (require.main === module) {
  main();
}

// Export functions for use in other scripts
module.exports = {
  generateZone,
  getZone,
  listZones,
  analyzeZone,
  visualizeZone,
};
