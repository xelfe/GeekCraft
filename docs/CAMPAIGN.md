# Campaign System

## Overview

The Campaign System provides a minimal framework for managing game campaign runs with support for starting/stopping runs, ticking game state, and persisting runs to disk as JSON files.

## Architecture

The campaign system consists of the following components:

### Core Components

1. **CampaignRun**: Represents a single campaign run instance
   - `run_id`: Unique identifier for the run
   - `tick`: Current game tick counter
   - `running`: Boolean flag indicating if the run is active
   - `created_at`: Unix timestamp of creation time

2. **InMemoryRunStore**: In-memory storage for active campaign runs
   - Manages a hash map of run_id to CampaignRun
   - Provides CRUD operations for runs

3. **CampaignManager**: Main interface for campaign operations
   - Manages run lifecycle (start, stop, tick)
   - Handles persistence (save, load, list)
   - Uses environment variable `GEEKCRAFT_SAVE_DIR` for save location (defaults to `./saves`)

### HTTP API Endpoints

All campaign endpoints are accessible without authentication:

- **POST /api/campaign/start**: Start a new campaign run
- **GET /api/campaign/state?run_id=...**: Get current state of a run
- **POST /api/campaign/stop**: Stop a running campaign
- **POST /api/campaign/save**: Save a run to disk (JSON format)
- **GET /api/campaign/saves**: List all available saved runs
- **POST /api/campaign/load**: Load a run from disk into memory

## Usage

### Starting a Campaign Run

```bash
curl -X POST http://localhost:3030/api/campaign/start \
  -H "Content-Type: application/json" \
  -d '{"run_id": "my_first_campaign"}'
```

Response:
```json
{
  "success": true,
  "message": "Campaign run my_first_campaign started successfully",
  "run_id": "my_first_campaign"
}
```

### Getting Run State

```bash
curl "http://localhost:3030/api/campaign/state?run_id=my_first_campaign"
```

Response:
```json
{
  "success": true,
  "message": "Retrieved state for run my_first_campaign",
  "run": {
    "run_id": "my_first_campaign",
    "tick": 0,
    "running": true,
    "created_at": 1698765432
  }
}
```

### Stopping a Campaign Run

```bash
curl -X POST http://localhost:3030/api/campaign/stop \
  -H "Content-Type: application/json" \
  -d '{"run_id": "my_first_campaign"}'
```

### Saving a Run to Disk

```bash
curl -X POST http://localhost:3030/api/campaign/save \
  -H "Content-Type: application/json" \
  -d '{"run_id": "my_first_campaign"}'
```

The run will be saved to `./saves/my_first_campaign.json` (or the directory specified by `GEEKCRAFT_SAVE_DIR`).

### Listing Available Saves

```bash
curl http://localhost:3030/api/campaign/saves
```

Response:
```json
{
  "success": true,
  "message": "Found 2 saved runs",
  "saves": ["my_first_campaign", "another_campaign"]
}
```

### Loading a Run from Disk

```bash
curl -X POST http://localhost:3030/api/campaign/load \
  -H "Content-Type: application/json" \
  -d '{"run_id": "my_first_campaign"}'
```

## Save File Format

Campaign runs are saved as JSON files with the following structure:

```json
{
  "run_id": "my_first_campaign",
  "tick": 42,
  "running": false,
  "created_at": 1698765432
}
```

## Configuration

### Environment Variables

- `GEEKCRAFT_SAVE_DIR`: Directory for saving campaign runs (default: `./saves`)

Example:
```bash
export GEEKCRAFT_SAVE_DIR=/var/geekcraft/campaigns
cargo run
```

## JavaScript Examples

See the following example files in the `examples/` directory:

- `campaign_local_save_example.js`: Demonstrates local save/load workflow
- `online_campaign_example.js`: Demonstrates online campaign management

## Future Enhancements

This is a minimal implementation. Future enhancements could include:

- Integration with actual game simulation (tick processing)
- Authentication and authorization for campaign operations
- Multi-player campaign support
- Campaign templates and scenarios
- Web UI for campaign management
- Automatic periodic saves
- Campaign metrics and statistics
