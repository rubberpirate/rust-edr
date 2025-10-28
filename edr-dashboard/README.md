# EDR Central Monitoring Dashboard

A distributed EDR (Endpoint Detection and Response) system with a beautiful TUI (Terminal User Interface) dashboard for real-time monitoring of multiple endpoints.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Main OS (Host)                          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │         EDR Server - TUI Dashboard                    │  │
│  │  - Real-time monitoring                               │  │
│  │  - Endpoint status & stats                            │  │
│  │  - Event streams (Process/Network/File)               │  │
│  │  - Threat alerts                                      │  │
│  └───────────────────────────────────────────────────────┘  │
│                          ▲                                   │
│                          │ WebSocket                         │
└──────────────────────────┼───────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼─────┐      ┌────▼─────┐      ┌────▼─────┐
   │  VM #1   │      │  VM #2   │      │  VM #3   │
   │ EDR Agent│      │ EDR Agent│      │ EDR Agent│
   └──────────┘      └──────────┘      └──────────┘
```

## Components

### 1. **EDR Server** (runs on main OS)
- WebSocket server listening for agent connections
- Beautiful TUI dashboard showing:
  - 📡 Connected endpoints with live status
  - 📊 Aggregated system statistics
  - ⚠️  Real-time threat alerts
  - 📋 Process, network, and file events
  - 📈 CPU and memory gauges

### 2. **EDR Agent** (runs on VMs/endpoints)
- Lightweight monitoring agent
- Collects system statistics:
  - CPU usage, memory, disk
  - Process count
  - Network connections
- Sends data to server via WebSocket
- Heartbeat every 5 seconds

## Building

```bash
cd edr-dashboard
cargo build --release
```

This creates two binaries:
- `target/release/edr-server` - Central monitoring dashboard
- `target/release/edr-agent` - Endpoint agent

## Usage

### Step 1: Start the Server (on main OS)

```bash
# Start with TUI dashboard
./target/release/edr-server

# Or bind to specific address
./target/release/edr-server --bind 192.168.1.100:8080

# Console-only mode (no TUI)
./target/release/edr-server --no-tui
```

The server will:
- Start listening on port 8080 (default)
- Launch the TUI dashboard after 2 seconds
- Wait for agents to connect

### Step 2: Start Agents (on VMs)

On each VM/endpoint you want to monitor:

```bash
# Connect to server (replace with your server IP)
./target/release/edr-agent --server ws://192.168.1.100:8080 --endpoint-id vm-ubuntu-01

# Custom heartbeat interval
./target/release/edr-agent --server ws://SERVER_IP:8080 --endpoint-id vm-01 --heartbeat 10
```

**Agent Parameters:**
- `--server`: WebSocket URL of the server (e.g., `ws://192.168.1.100:8080`)
- `--endpoint-id`: Unique identifier for this endpoint
- `--heartbeat`: Heartbeat interval in seconds (default: 5)

### Step 3: Monitor in TUI

The TUI dashboard will show:

```
┌─────────────────────────────────────────────────────────────────┐
│ 🛡️  EDR Central Monitoring Dashboard                           │
├─────────────────┬───────────────────────────────────────────────┤
│ 📡 Endpoints    │ 📊 System Overview                            │
│                 │   Total Processes: 245                        │
│ ━━━━━━━━━━━━━━  │   Network Connections: 87                     │
│ ID: vm-01       │   Avg CPU Usage: 45.2%                        │
│ Host: ubuntu-vm │   Active Threats: 0                           │
│ Status: 🟢 Online│                                               │
│ CPU: 42.3%      │ ┌───────────────────────────────────────────┐ │
│ MEM: 2.1/4.0 GB │ │ CPU Usage      [████████░░░░░░░░] 42%    │ │
│ Proc: 124       │ │ Memory Usage   [█████████░░░░░░░] 52%    │ │
│                 │ └───────────────────────────────────────────┘ │
├─────────────────┼───────────────────────────────────────────────┤
│                 │ ⚠️  Recent Threats  │ 📋 Recent Events        │
│                 │                     │                         │
│                 │ ✅ No threats       │ 🟢 Process: bash        │
│                 │    detected         │   └─ PID 1234 on vm-01 │
│                 │                     │ 🔵 Network: 10.0.2.15..│
└─────────────────┴─────────────────────┴─────────────────────────┘
│ Press 'q' to quit | Real-time monitoring active                 │
└─────────────────────────────────────────────────────────────────┘
```

**TUI Controls:**
- `q` - Quit dashboard
- Auto-refreshes every 250ms

## Dashboard Features

### Endpoint Panel (Left)
- Lists all connected endpoints
- Shows real-time status (🟢 Online / 🔴 Offline)
- Displays current CPU, memory, and process count per endpoint

### System Overview (Top Right)
- **Total Processes**: Sum across all endpoints
- **Network Connections**: Total active connections
- **Avg CPU Usage**: Average CPU across all endpoints
- **Active Threats**: Current threat count

### Gauges (Middle Right)
- CPU Usage gauge for first endpoint
- Memory Usage gauge for first endpoint

### Recent Threats (Bottom Left)
- Color-coded by severity (🔴 Critical, 🟡 High, 🟢 Medium, ⚪ Low)
- Shows threat type and description
- Displays which endpoint detected it

### Recent Events (Bottom Right)
- 🟢 Process created
- 🔴 Process terminated
- 🔵 Network connection established
- ⚫ Network connection closed
- 📡 Data transfer

## Testing Setup

### Single Machine Testing

You can test the system on a single machine using multiple terminal sessions:

**Terminal 1: Server**
```bash
./target/release/edr-server --bind 127.0.0.1:8080
```

**Terminal 2: Agent 1**
```bash
./target/release/edr-agent --server ws://127.0.0.1:8080 --endpoint-id local-test-1
```

**Terminal 3: Agent 2**
```bash
./target/release/edr-agent --server ws://127.0.0.1:8080 --endpoint-id local-test-2
```

### VM Setup

1. **Build the agent on host:**
   ```bash
   cargo build --release --bin edr-agent
   ```

2. **Copy agent to VMs:**
   ```bash
   scp target/release/edr-agent user@vm-ip:/home/user/
   ```

3. **On each VM:**
   ```bash
   chmod +x edr-agent
   ./edr-agent --server ws://HOST_IP:8080 --endpoint-id vm-ubuntu-01
   ```

## Network Configuration

### Firewall Rules

**On Host (Server):**
```bash
# Allow incoming WebSocket connections on port 8080
sudo ufw allow 8080/tcp
```

**On VMs (Agents):**
```bash
# Allow outgoing connections to server
# Usually no configuration needed for outbound
```

### Port Forwarding (VirtualBox)

If VMs are in NAT mode, you may need port forwarding or use Bridged Adapter:

1. **Option 1: Bridged Adapter (Recommended)**
   - VM Settings → Network → Attached to: Bridged Adapter
   - VMs get IPs on same network as host
   - Direct communication

2. **Option 2: Host-Only Adapter**
   - Create host-only network in VirtualBox
   - VMs and host communicate on private network

## Performance

- **Server**: Minimal CPU usage, handles 100+ concurrent agents
- **Agent**: ~5-10 MB memory, <1% CPU overhead
- **Network**: ~1-2 KB/s per agent (heartbeat traffic)

## Future Enhancements

- [ ] Historical data graphing
- [ ] Alert rules configuration
- [ ] Agent commands (kill process, block connection)
- [ ] Multi-page TUI navigation
- [ ] Export logs/reports
- [ ] Authentication/TLS encryption
- [ ] Integration with original EDR detection rules

## Troubleshooting

### Agent can't connect to server

1. Check firewall rules
2. Verify server IP and port
3. Test with `telnet SERVER_IP 8080`
4. Check server logs

### TUI not displaying properly

1. Terminal must support UTF-8
2. Minimum terminal size: 80x24
3. Try different terminal emulator

### High CPU usage

1. Increase heartbeat interval: `--heartbeat 10`
2. Limit number of stored events (modify code)

## License

MIT License - See parent project for details
