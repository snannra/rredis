# 🧠 rredis — A Redis Clone Written in Rust

**rredis** is a high-performance, in-memory key-value data store inspired by Redis and built entirely in **Rust**.  
It provides a fully asynchronous TCP server implementing the **RESP2 protocol**, supporting real-time reads, writes, expiration (TTL), persistence, replication, and pipelined requests — all designed for learning, performance, and correctness.

---

## 🚀 Features

### 💾 Core
- In-Memory Key-Value Store backed by `Arc<RwLock<HashMap<>>>>` for concurrent access.
- Full RESP2 Protocol support for client compatibility.
- Command Set includes:
  - `PING`, `ECHO`
  - `SET`, `GET`, `DEL`, `EXISTS`
  - `EXPIRE`, `TTL`
  - `INCR`, `DECR`
  - `MSET`, `MGET`
  - `FLUSHALL`, `KEYS`

### ⚙️ Architecture
- **Tokio-based async server** for efficient non-blocking networking.
- **Command parser** that decodes nested arrays and bulk strings using the RESP2 spec.
- **Database engine** with automatic key expiration and background cleanup.

### ⏱️ Advanced
- **TTL Wheel** for efficient time-based key expiration.
- **Persistence layer** supporting:
  - RDB snapshots
  - Append-only (AOF) mode
- **Replication** between master and replica nodes.
- **Pipelining** for multiple commands in a single request.
- **Metrics** for latency, key count, and cache hit/miss tracking.

### 🔒 Reliability
- Graceful shutdown support.
- Configurable maximum connection limits.
- Snapshot and AOF recovery for durability.
