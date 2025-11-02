# GeekCraft Database Options

GeekCraft supports multiple database backends for authentication and session management. Choose the right database based on your deployment scenario.

## Database Options

### 1. **In-Memory** (Default - Development/Testing)

**Best for:**
- Development and testing
- Quick experiments
- Automated testing

**Pros:**
- ✅ Fastest option
- ✅ No dependencies or setup
- ✅ Zero configuration
- ✅ Clean slate on every restart

**Cons:**
- ❌ All data lost on restart
- ❌ Not suitable for production

**Usage:**
```bash
# Default - uses In-Memory automatically
cargo run --release

# Or explicitly set
export GEEKCRAFT_DB_BACKEND=INMEMORY
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
- ✅ Data persistence (configurable)

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

## Comparison Table

| Feature | In-Memory | Redis |
|---------|-----------|-------|
| **Setup Complexity** | Easy | Medium |
| **External Dependencies** | None | Redis server |
| **Concurrent Users** | <1000 | >10,000 |
| **Persistence** | No | Yes (configurable) |
| **Speed** | Fastest | Very Fast |
| **Horizontal Scaling** | No | Yes |
| **Production Ready (MMO)** | ❌ No | ✅ Yes |
| **Memory Usage** | Medium | High |
| **Session Auto-Expiration** | Manual cleanup | Automatic (TTL) |

---

## Recommendations by Deployment Type

### Development/Testing
```bash
# Use In-Memory (default) - simplest option
cargo run --release
```

### Production MMO Server
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
| `GEEKCRAFT_DB_BACKEND` | `INMEMORY` | Database backend: `REDIS` or `INMEMORY` |
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis connection URL (when using Redis) |

---

## Why No SQL Database?

For an MMO server, we chose to support only NoSQL (Redis) and In-Memory databases because:

1. **Sessions are temporary** - Don't need complex queries or relations
2. **High concurrency** - Redis handles thousands of concurrent connections better than SQL
3. **Speed** - In-memory databases are faster for session management
4. **Simplicity** - Simpler data model (key-value, hash maps)
5. **Auto-expiration** - Redis TTL automatically removes expired sessions
6. **Horizontal scaling** - Redis supports clustering and replication

For user data that requires persistence, you can add a separate SQL database in the future.

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

---

## Monitoring

### Redis
```bash
# Monitor Redis performance
redis-cli INFO stats
redis-cli MONITOR

# Check memory usage
redis-cli INFO memory

# Check connected clients
redis-cli CLIENT LIST
```

---

## Support

For database-related questions:
- Check the logs when starting the server
- Ensure Redis is running (if using Redis backend)
- Verify environment variables are set correctly
- See examples in `/examples/auth_example.js`
