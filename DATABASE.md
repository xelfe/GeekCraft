# GeekCraft Database Options

GeekCraft supports multiple database backends for authentication and session management. Choose the right database based on your deployment scenario.

## Database Options

### 1. **SQLite** (Default - Recommended for Development)

**Best for:**
- Development and testing
- Single-server deployments
- Small to medium-scale games (<1000 concurrent users)
- Simple setup with no external dependencies

**Pros:**
- ✅ Zero configuration - works out of the box
- ✅ File-based - easy backup and migration
- ✅ No external server needed
- ✅ Perfect for getting started

**Cons:**
- ❌ Limited concurrent writes
- ❌ Single file - not suitable for horizontal scaling
- ❌ Not ideal for high-concurrency scenarios

**Usage:**
```bash
# Default - uses SQLite automatically
cargo run --release

# Or explicitly set
export GEEKCRAFT_DB_BACKEND=SQLITE
export SQLITE_PATH=geekcraft.db
cargo run --release
```

---

### 2. **Redis** (Recommended for Production MMO)

**Best for:**
- Production MMO servers
- High concurrency (>1000 concurrent users)
- Multi-server deployments
- Real-time applications

**Pros:**
- ✅ Very fast (in-memory)
- ✅ Excellent for sessions (automatic TTL expiration)
- ✅ Supports horizontal scaling
- ✅ Industry standard for real-time multiplayer games
- ✅ High concurrency support

**Cons:**
- ❌ Requires Redis server installation
- ❌ Data stored in RAM (configure persistence if needed)

**Installation:**

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install redis-server
sudo systemctl start redis
```

**macOS:**
```bash
brew install redis
brew services start redis
```

**Docker:**
```bash
docker run -d -p 6379:6379 redis:latest
```

**Usage:**
```bash
# Build with Redis support
cargo build --release --features redis_backend

# Run with Redis
export GEEKCRAFT_DB_BACKEND=REDIS
export REDIS_URL=redis://127.0.0.1:6379
cargo run --release --features redis_backend
```

**Production Redis Configuration:**
For production, configure Redis persistence:
```bash
# In redis.conf
save 900 1
save 300 10
save 60 10000
appendonly yes
```

---

### 3. **In-Memory** (Testing Only)

**Best for:**
- Automated testing
- Quick experiments
- Development when you don't need data persistence

**Pros:**
- ✅ Fastest option
- ✅ No dependencies
- ✅ Clean slate on every restart

**Cons:**
- ❌ All data lost on restart
- ❌ Not suitable for any production use

**Usage:**
```bash
export GEEKCRAFT_DB_BACKEND=INMEMORY
cargo run --release
```

---

## Comparison Table

| Feature | SQLite | Redis | In-Memory |
|---------|--------|-------|-----------|
| **Setup Complexity** | Easy | Medium | Easy |
| **External Dependencies** | None | Redis server | None |
| **Concurrent Users** | <1000 | >10,000 | <1000 |
| **Persistence** | Yes (file) | Yes (configurable) | No |
| **Speed** | Fast | Very Fast | Fastest |
| **Horizontal Scaling** | No | Yes | No |
| **Production Ready (MMO)** | Small-scale | ✅ Yes | ❌ No |
| **Memory Usage** | Low | High | Medium |
| **Session Auto-Expiration** | Manual cleanup | Automatic (TTL) | Manual cleanup |

---

## Recommendations by Deployment Type

### Development
```bash
# Use SQLite (default) - simplest option
cargo run --release
```

### Small Public Server (< 100 users)
```bash
# SQLite is fine
export GEEKCRAFT_DB_BACKEND=SQLITE
export SQLITE_PATH=/var/lib/geekcraft/data.db
cargo run --release
```

### Production MMO Server (> 1000 users)
```bash
# Use Redis for best performance and scaling
export GEEKCRAFT_DB_BACKEND=REDIS
export REDIS_URL=redis://your-redis-server:6379
cargo run --release --features redis_backend
```

### Multi-Server Cluster
```bash
# All servers connect to the same Redis instance
export GEEKCRAFT_DB_BACKEND=REDIS
export REDIS_URL=redis://central-redis.your-domain.com:6379
cargo run --release --features redis_backend
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `GEEKCRAFT_DB_BACKEND` | `SQLITE` | Database backend: `SQLITE`, `REDIS`, or `INMEMORY` |
| `SQLITE_PATH` | `geekcraft.db` | Path to SQLite database file |
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis connection URL |

---

## Migration Between Databases

Currently, there's no automatic migration tool between backends. To migrate:

### From SQLite to Redis

1. Export users and sessions from SQLite (implement export script)
2. Import into Redis
3. Switch backend and restart server

For production deployments, plan your database choice before launch. Redis is recommended for any serious MMO deployment.

---

## Future Database Support

The architecture supports adding more backends:
- **PostgreSQL** - For complex queries and relationships
- **MongoDB** - For document-based storage
- **MySQL** - Traditional relational database

To add a new backend, implement the `AuthDatabaseTrait` in `src/auth/database.rs`.

---

## Security Considerations

### All Backends
- Passwords are hashed with bcrypt
- Session tokens are UUIDs
- Sessions auto-expire after 24 hours

### Redis-Specific
- Use Redis ACL for access control
- Enable TLS for network encryption
- Use Redis password authentication
- Consider Redis Sentinel for high availability

### SQLite-Specific
- Set proper file permissions (600)
- Regular backups
- Consider encrypting the database file

---

## Performance Tuning

### Redis
```bash
# Increase max connections in redis.conf
maxclients 10000

# Configure memory limits
maxmemory 2gb
maxmemory-policy allkeys-lru
```

### SQLite
```sql
-- Enable WAL mode for better concurrency
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;
```

---

## Monitoring

### Redis
```bash
# Monitor Redis performance
redis-cli INFO stats
redis-cli MONITOR

# Check memory usage
redis-cli INFO memory
```

### SQLite
```bash
# Check database size
ls -lh geekcraft.db

# Vacuum to optimize
sqlite3 geekcraft.db "VACUUM;"
```

---

## Support

For database-related questions:
- Check the logs when starting the server
- Ensure Redis is running (if using Redis backend)
- Verify environment variables are set correctly
- See examples in `/examples/auth_example.js`
