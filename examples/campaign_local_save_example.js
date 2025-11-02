// Campaign Local Save Example for GeekCraft
// This example demonstrates how to start a campaign, save it, list saves, and load it back
// Run with: node examples/campaign_local_save_example.js

const BASE_URL = 'http://localhost:3030';

/**
 * Start a new campaign run
 * @param {string} runId - Unique identifier for the campaign run
 * @returns {Promise<Object>} Response with success status and run details
 */
async function startCampaign(runId) {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/start`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ run_id: runId }),
    });

    const data = await response.json();
    console.log('Start campaign response:', data);
    return data;
  } catch (error) {
    console.error('Error starting campaign:', error);
    throw error;
  }
}

/**
 * Get current state of a campaign run
 * @param {string} runId - Campaign run identifier
 * @returns {Promise<Object>} Response with run state
 */
async function getCampaignState(runId) {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/state?run_id=${runId}`);
    const data = await response.json();
    console.log('Campaign state:', data);
    return data;
  } catch (error) {
    console.error('Error getting campaign state:', error);
    throw error;
  }
}

/**
 * Stop a running campaign
 * @param {string} runId - Campaign run identifier
 * @returns {Promise<Object>} Response with success status
 */
async function stopCampaign(runId) {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/stop`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ run_id: runId }),
    });

    const data = await response.json();
    console.log('Stop campaign response:', data);
    return data;
  } catch (error) {
    console.error('Error stopping campaign:', error);
    throw error;
  }
}

/**
 * Save a campaign run to disk
 * @param {string} runId - Campaign run identifier
 * @returns {Promise<Object>} Response with success status
 */
async function saveCampaign(runId) {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/save`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ run_id: runId }),
    });

    const data = await response.json();
    console.log('Save campaign response:', data);
    return data;
  } catch (error) {
    console.error('Error saving campaign:', error);
    throw error;
  }
}

/**
 * List all saved campaigns
 * @returns {Promise<Object>} Response with list of saved campaign IDs
 */
async function listSaves() {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/saves`);
    const data = await response.json();
    console.log('List saves response:', data);
    return data;
  } catch (error) {
    console.error('Error listing saves:', error);
    throw error;
  }
}

/**
 * Load a saved campaign from disk
 * @param {string} runId - Campaign run identifier
 * @returns {Promise<Object>} Response with loaded run details
 */
async function loadCampaign(runId) {
  try {
    const response = await fetch(`${BASE_URL}/api/campaign/load`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ run_id: runId }),
    });

    const data = await response.json();
    console.log('Load campaign response:', data);
    return data;
  } catch (error) {
    console.error('Error loading campaign:', error);
    throw error;
  }
}

/**
 * Main demonstration function
 */
async function main() {
  console.log('=== GeekCraft Campaign Local Save Example ===\n');

  const runId = `demo_campaign_${Date.now()}`;

  try {
    // Step 1: Start a new campaign
    console.log('Step 1: Starting campaign...');
    await startCampaign(runId);
    console.log('');

    // Step 2: Check campaign state
    console.log('Step 2: Checking campaign state...');
    await getCampaignState(runId);
    console.log('');

    // Step 3: Stop the campaign
    console.log('Step 3: Stopping campaign...');
    await stopCampaign(runId);
    console.log('');

    // Step 4: Save the campaign to disk
    console.log('Step 4: Saving campaign to disk...');
    await saveCampaign(runId);
    console.log('');

    // Step 5: List all saves
    console.log('Step 5: Listing all saved campaigns...');
    await listSaves();
    console.log('');

    // Step 6: Load the campaign back
    console.log('Step 6: Loading campaign from disk...');
    await loadCampaign(runId);
    console.log('');

    // Step 7: Verify loaded state
    console.log('Step 7: Verifying loaded campaign state...');
    await getCampaignState(runId);
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
  startCampaign,
  getCampaignState,
  stopCampaign,
  saveCampaign,
  listSaves,
  loadCampaign,
};
