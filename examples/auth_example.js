// Authentication Example for GeekCraft
// This example demonstrates how to register, login, and submit code with authentication

const BASE_URL = 'http://localhost:3030';

/**
 * Register a new user
 * @param {string} username - Username (3-32 characters, alphanumeric, underscore, hyphen)
 * @param {string} password - Password (minimum 6 characters)
 * @returns {Promise<Object>} Response with success status and message
 */
async function register(username, password) {
  try {
    const response = await fetch(`${BASE_URL}/api/auth/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });

    const data = await response.json();
    console.log('Register response:', data);
    return data;
  } catch (error) {
    console.error('Registration error:', error);
    throw error;
  }
}

/**
 * Login with existing credentials
 * @param {string} username - Username
 * @param {string} password - Password
 * @returns {Promise<Object>} Response with token if successful
 */
async function login(username, password) {
  try {
    const response = await fetch(`${BASE_URL}/api/auth/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });

    const data = await response.json();
    console.log('Login response:', data);
    
    // Store token for later use
    if (data.success && data.token) {
      localStorage.setItem('geekcraft_token', data.token);
      localStorage.setItem('geekcraft_username', data.username);
    }
    
    return data;
  } catch (error) {
    console.error('Login error:', error);
    throw error;
  }
}

/**
 * Logout (invalidate current session)
 * @returns {Promise<Object>} Response with success status
 */
async function logout() {
  try {
    const token = localStorage.getItem('geekcraft_token');
    
    if (!token) {
      console.warn('No token found, already logged out');
      return { success: false, message: 'Not logged in' };
    }

    const response = await fetch(`${BASE_URL}/api/auth/logout`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    const data = await response.json();
    console.log('Logout response:', data);
    
    // Clear stored credentials
    localStorage.removeItem('geekcraft_token');
    localStorage.removeItem('geekcraft_username');
    
    return data;
  } catch (error) {
    console.error('Logout error:', error);
    throw error;
  }
}

/**
 * Submit bot code (requires authentication)
 * @param {string} code - JavaScript code for your bot
 * @returns {Promise<Object>} Response with success status
 */
async function submitCode(code) {
  try {
    const token = localStorage.getItem('geekcraft_token');
    
    if (!token) {
      throw new Error('Not authenticated. Please login first.');
    }

    const response = await fetch(`${BASE_URL}/api/submit`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify({ code }),
    });

    if (response.status === 401) {
      throw new Error('Authentication failed. Please login again.');
    }

    const data = await response.json();
    console.log('Code submission response:', data);
    return data;
  } catch (error) {
    console.error('Code submission error:', error);
    throw error;
  }
}

/**
 * Get list of players (requires authentication)
 * @returns {Promise<Object>} List of players
 */
async function getPlayers() {
  try {
    const token = localStorage.getItem('geekcraft_token');
    
    if (!token) {
      throw new Error('Not authenticated. Please login first.');
    }

    const response = await fetch(`${BASE_URL}/api/players`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    if (response.status === 401) {
      throw new Error('Authentication failed. Please login again.');
    }

    const data = await response.json();
    console.log('Players:', data);
    return data;
  } catch (error) {
    console.error('Get players error:', error);
    throw error;
  }
}

/**
 * Get game state (requires authentication)
 * @returns {Promise<Object>} Current game state
 */
async function getGameState() {
  try {
    const token = localStorage.getItem('geekcraft_token');
    
    if (!token) {
      throw new Error('Not authenticated. Please login first.');
    }

    const response = await fetch(`${BASE_URL}/api/gamestate`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    if (response.status === 401) {
      throw new Error('Authentication failed. Please login again.');
    }

    const data = await response.json();
    console.log('Game state:', data);
    return data;
  } catch (error) {
    console.error('Get game state error:', error);
    throw error;
  }
}

// Example usage
async function example() {
  console.log('=== GeekCraft Authentication Example ===\n');

  // Step 1: Register a new user
  console.log('Step 1: Registering user...');
  const registerResult = await register('player1', 'password123');
  
  if (!registerResult.success) {
    console.log('User might already exist, trying to login...');
  }

  // Step 2: Login
  console.log('\nStep 2: Logging in...');
  const loginResult = await login('player1', 'password123');
  
  if (!loginResult.success) {
    console.error('Login failed:', loginResult.message);
    return;
  }

  console.log('Login successful! Token:', loginResult.token);

  // Step 3: Submit bot code
  console.log('\nStep 3: Submitting bot code...');
  const botCode = `
    class MyBot {
      constructor() {
        this.name = "Player1Bot";
      }
      
      onTick(gameState) {
        console.log("Bot is running!");
        const myUnits = gameState.getMyUnits();
        console.log("My units:", myUnits.length);
      }
    }
    module.exports = MyBot;
  `;
  
  const submitResult = await submitCode(botCode);
  console.log('Code submitted:', submitResult.success);

  // Step 4: Get players
  console.log('\nStep 4: Getting players...');
  const players = await getPlayers();
  console.log('Active players:', players.players);

  // Step 5: Get game state
  console.log('\nStep 5: Getting game state...');
  const gameState = await getGameState();
  console.log('Current tick:', gameState.tick);

  // Step 6: Logout
  console.log('\nStep 6: Logging out...');
  const logoutResult = await logout();
  console.log('Logout:', logoutResult.success);
}

// For browser usage
if (typeof window !== 'undefined') {
  window.GeekCraftAuth = {
    register,
    login,
    logout,
    submitCode,
    getPlayers,
    getGameState,
    example,
  };
  console.log('GeekCraft Auth API loaded. Use GeekCraftAuth.example() to run example.');
}

// For Node.js usage
if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    register,
    login,
    logout,
    submitCode,
    getPlayers,
    getGameState,
    example,
  };
}
