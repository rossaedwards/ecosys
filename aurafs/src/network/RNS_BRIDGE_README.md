# RNS Bridge - Reticulum Network Stack Integration

## Overview

This bridge allows AuraFS (Rust) to communicate with Reticulum Network Stack (RNS) via a Python intermediary process. The bridge uses ZeroMQ for efficient, bidirectional communication between the Rust core and Python RNS process.

## Architecture

```
┌─────────────────┐         ZeroMQ         ┌─────────────────┐
│   AuraFS Rust   │ ←───────────────────→ │  Python Bridge  │
│   (rns_client)  │     (IPC/TCP)         │  (rns_bridge.py) │
└─────────────────┘                        └─────────────────┘
                                                   │
                                                   │ RNS Protocol
                                                   ↓
                                           ┌─────────────────┐
                                           │  Reticulum      │
                                           │  Network Stack  │
                                           └─────────────────┘
```

## Components

### 1. Python Bridge (`rns_bridge.py`)

**Features:**
- RNS Identity management (loads or creates identity)
- Packet listener for incoming RNS packets
- ZeroMQ communication with Rust core
- Automatic restart on failure (up to 10 attempts with exponential backoff)
- Comprehensive error handling and logging
- Statistics tracking

**Setup:**
```bash
# Install dependencies
pip install rns pyzmq

# Run the bridge directly
python src/network/rns_bridge.py
```

### 2. Python Daemon (`rns_daemon.py`)

**Features:**
- Windows service compatible wrapper
- Improved signal handling and logging
- PID file management (Unix)
- Clean shutdown handling
- Better process management

**Setup:**
```bash
# Run as daemon (recommended for production)
python src/network/rns_daemon.py

# Windows: Install as service (requires pywin32)
python src/network/rns_daemon.py install
python src/network/rns_daemon.py start
```

### 3. Rust Bridge Manager (`rns_bridge.rs`)

**Features:**
- Spawns and manages Python bridge process from Rust
- Auto-restart with exponential backoff
- Health monitoring and status tracking
- Graceful shutdown handling
- Process output capture (stdout/stderr)

**Usage:**
```rust
use aurafs::network::{RNSBridgeManager, ProcessStatus};

// Create manager with script path
let manager = RNSBridgeManager::new("src/network/rns_bridge.py")?;

// Start the Python bridge process
manager.start().await?;

// Check status
if manager.is_running().await {
    println!("Bridge is running");
}

// Get process status
match manager.get_status().await {
    ProcessStatus::Running => println!("Process is running"),
    ProcessStatus::Failed => println!("Process failed"),
    _ => {}
}

// Stop the bridge
manager.stop().await?;
```

**Configuration:**
- ZeroMQ endpoints:
  - Unix: IPC sockets at `/tmp/aurafs_rns_send` and `/tmp/aurafs_rns_recv`
  - Windows: TCP sockets at `tcp://127.0.0.1:5555` and `tcp://127.0.0.1:5556`
- RNS Identity: Stored at `~/.reticulum/aurafs_identity`
- Logs: Written to `~/.aurafs/rns_bridge.log`

### 2. Rust Client (`rns_client.rs`)

**Features:**
- ZeroMQ client for Python bridge communication
- Async packet handling
- Background receiver task
- Statistics tracking
- Graceful shutdown

**Usage:**
```rust
use aurafs::network::{RNSClient, NetworkPacket, PacketType};

// Create and initialize client
let client = RNSClient::new();
client.initialize().await?;

// Send a packet via RNS
let packet = NetworkPacket::new(
    PacketType::ShardData,
    0,
    None,
    source_id,
    destination_id,
    payload,
);

client.send_packet(&packet, &destination_hash).await?;

// Get statistics
let stats = client.get_stats().await;

// Shutdown
client.shutdown().await?;
```

## Message Protocol

### Rust → Python (via ZeroMQ PUSH/PULL)

**Send Packet:**
```json
{
  "type": "send_packet",
  "destination_hash": "hex_encoded_destination_hash",
  "data": "hex_encoded_packet_data"
}
```

**Get Stats:**
```json
{
  "type": "get_stats"
}
```

### Python → Rust (via ZeroMQ PUSH/PULL)

**RNS Packet Received:**
```json
{
  "type": "rns_packet_received",
  "destination_hash": "hex_hash",
  "source_hash": "hex_hash",
  "data": "hex_encoded_data",
  "data_len": 1024,
  "hop_count": 3,
  "timestamp": 1234567890.123,
  "sequence": 42
}
```

**Stats Response:**
```json
{
  "type": "stats_response",
  "stats": {
    "packets_received": 100,
    "packets_sent": 50,
    "errors": 2,
    "start_time": 1234567890.123
  },
  "rns_identity": "hex_identity_hash",
  "destination_hash": "hex_destination_hash",
  "packet_handler_stats": {
    "packets_received": 100,
    "errors": 0,
    "last_packet_time": 1234567890.123
  }
}
```

## Error Handling

### Python Bridge
- **Automatic Restart**: On crash, automatically restarts up to 10 times with exponential backoff
- **Signal Handling**: Graceful shutdown on SIGINT/SIGTERM
- **ZeroMQ Errors**: Handles buffer full conditions and connection errors
- **RNS Errors**: Logs and continues operation on packet handling errors

### Rust Client
- **Connection Errors**: Returns `NetworkError::ConfigError` with details
- **Send Errors**: Handles buffer full and timeout conditions
- **Receive Errors**: Continues operation, logs errors, marks connection status

## Production Deployment

### Requirements

**Python:**
- Python 3.7+
- `rns` package (`pip install rns`)
- `pyzmq` package (`pip install pyzmq`)

**Rust:**
- `zmq` crate (already added to `Cargo.toml`)
- `hex` crate (already in workspace dependencies)
- `serde_json` (already in workspace dependencies)

### Process Management

The Python bridge should be run as a separate process or service:

**Systemd Service (Linux):**
```ini
[Unit]
Description=AuraFS RNS Bridge
After=network.target

[Service]
Type=simple
User=aurafs
WorkingDirectory=/path/to/aurafs
ExecStart=/usr/bin/python3 /path/to/aurafs/src/network/rns_bridge.py
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**Windows Service:**
Use NSSM (Non-Sucking Service Manager) or similar to run the Python script as a service.

### Security Considerations

1. **ZeroMQ Socket Permissions**: IPC sockets on Unix are created with `0o600` permissions (owner read/write only)
2. **RNS Identity**: The identity file is stored in user's home directory, protected by filesystem permissions
3. **Network Isolation**: ZeroMQ sockets are local-only (IPC on Unix, localhost TCP on Windows)

## Troubleshooting

### Python bridge won't start
- Check if RNS is installed: `pip list | grep rns`
- Check if pyzmq is installed: `pip list | grep pyzmq`
- Check log file: `~/.aurafs/rns_bridge.log`

### ZeroMQ connection errors
- **Unix**: Ensure `/tmp` directory is writable and old sockets are cleaned up
- **Windows**: Ensure ports 5555 and 5556 are not in use
- Check if Python bridge is running: `ps aux | grep rns_bridge.py`

### Packet delivery issues
- Check RNS network connectivity
- Verify destination hash is correct
- Check RNS logs for routing information
- Use `client.request_stats().await` to get bridge statistics

## Future Enhancements

- [ ] Connection health monitoring and automatic reconnection
- [ ] Packet encryption for ZeroMQ communication
- [ ] Metrics export (Prometheus format)
- [ ] Multi-threaded packet processing in Python bridge
- [ ] Destination caching to reduce RNS lookups
- [ ] Support for RNS announce/listen patterns

