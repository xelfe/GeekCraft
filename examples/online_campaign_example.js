// Online Campaign Example for GeekCraft
// This example demonstrates an online workflow for managing campaigns
// with continuous monitoring and real-time updates
// Run with: node examples/online_campaign_example.js

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
    return data;
  } catch (error) {
    console.error('Error saving campaign:', error);
    throw error;
  }
}

/**
 * Monitor campaign progress with periodic status checks
 * @param {string} runId - Campaign run identifier
 * @param {number} duration - How long to monitor (in milliseconds)
 * @param {number} interval - Check interval (in milliseconds)
 */
async function monitorCampaign(runId, duration = 10000, interval = 2000) {
  console.log(`Monitoring campaign "${runId}" for ${duration / 1000} seconds...`);
  
  const startTime = Date.now();
  const endTime = startTime + duration;
  
  while (Date.now() < endTime) {
    const state = await getCampaignState(runId);
    
    if (state.success && state.run) {
      const elapsed = Math.floor((Date.now() - startTime) / 1000);
      console.log(`[${elapsed}s] Tick: ${state.run.tick}, Running: ${state.run.running}`);
    } else {
      console.log('Campaign not found or error occurred');
      break;
    }
    
    // Wait for the next interval
    await new Promise(resolve => setTimeout(resolve, interval));
  }
  
  console.log('Monitoring complete');
}

/**
 * Demonstrate a typical online campaign workflow
 */
async function demonstrateOnlineWorkflow() {
  console.log('=== GeekCraft Online Campaign Example ===\n');

  const runId = `online_campaign_${Date.now()}`;

  try {
    // Step 1: Start a campaign
    console.log('Step 1: Starting online campaign...');
    const startResult = await startCampaign(runId);
    console.log(`Campaign "${runId}" started:`, startResult.success);
    console.log('');

    // Step 2: Monitor the campaign for a period
    console.log('Step 2: Monitoring campaign progress...');
    await monitorCampaign(runId, 10000, 2000);
    console.log('');

    // Step 3: Create a checkpoint save
    console.log('Step 3: Creating checkpoint save...');
    const saveResult = await saveCampaign(runId);
    console.log('Save result:', saveResult.success ? 'SUCCESS' : 'FAILED');
    console.log('');

    // Step 4: Continue running
    console.log('Step 4: Continuing campaign...');
    await monitorCampaign(runId, 5000, 2000);
    console.log('');

    // Step 5: Stop the campaign
    console.log('Step 5: Stopping campaign...');
    const stopResult = await stopCampaign(runId);
    console.log('Stop result:', stopResult.success ? 'SUCCESS' : 'FAILED');
    console.log('');

    // Step 6: Final save
    console.log('Step 6: Creating final save...');
    await saveCampaign(runId);
    console.log('');

    console.log('=== Online campaign workflow completed! ===');
  } catch (error) {
    console.error('Workflow failed:', error);
    process.exit(1);
  }
}

/**
 * Demonstrate managing multiple campaigns simultaneously
 */
async function demonstrateMultipleCampaigns() {
  console.log('=== Managing Multiple Campaigns ===\n');

  const campaigns = [
    `campaign_alpha_${Date.now()}`,
    `campaign_beta_${Date.now()}`,
    `campaign_gamma_${Date.now()}`,
  ];

  try {
    // Start all campaigns
    console.log('Starting multiple campaigns...');
    for (const runId of campaigns) {
      const result = await startCampaign(runId);
      console.log(`- ${runId}: ${result.success ? 'started' : 'failed'}`);
    }
    console.log('');

    // Monitor all campaigns briefly
    console.log('Checking status of all campaigns...');
    for (const runId of campaigns) {
      const state = await getCampaignState(runId);
      if (state.success && state.run) {
        console.log(`- ${runId}: tick=${state.run.tick}, running=${state.run.running}`);
      }
    }
    console.log('');

    // Save all campaigns
    console.log('Saving all campaigns...');
    for (const runId of campaigns) {
      const result = await saveCampaign(runId);
      console.log(`- ${runId}: ${result.success ? 'saved' : 'failed'}`);
    }
    console.log('');

    // Stop all campaigns
    console.log('Stopping all campaigns...');
    for (const runId of campaigns) {
      const result = await stopCampaign(runId);
      console.log(`- ${runId}: ${result.success ? 'stopped' : 'failed'}`);
    }
    console.log('');

    console.log('=== Multiple campaigns workflow completed! ===');
  } catch (error) {
    console.error('Multiple campaigns workflow failed:', error);
    process.exit(1);
  }
}

/**
 * Main function - choose which demo to run
 */
async function main() {
  const args = process.argv.slice(2);
  const mode = args[0] || 'online';

  if (mode === 'multiple') {
    await demonstrateMultipleCampaigns();
  } else {
    await demonstrateOnlineWorkflow();
  }
}

// Run the example
if (require.main === module) {
  console.log('Usage: node online_campaign_example.js [online|multiple]');
  console.log('');
  main();
}

// Export functions for use in other scripts
module.exports = {
  startCampaign,
  getCampaignState,
  stopCampaign,
  saveCampaign,
  monitorCampaign,
  demonstrateOnlineWorkflow,
  demonstrateMultipleCampaigns,
};
