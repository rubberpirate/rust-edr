# 🔧 WebSocket Connection Error Fix

## Error Seen:
```
Connection error: WebSocket protocol error: Handshake not finished
New connection from: 10.1.33.187:45142
```

## Root Cause:

The IP address `10.1.33.187` shows you're connecting from a **different machine/VM**. The WebSocket handshake is failing because of one of these issues:

### 1️⃣ Server Binding Issue

**Problem**: Server might be binding to `127.0.0.1` (localhost only)

**Solution**: Server MUST bind to `0.0.0.0` to accept external connections

```bash
# ❌ WRONG (only localhost)
./edr-server --bind 127.0.0.1:8080

# ✅ CORRECT (all interfaces)
./edr-server --bind 0.0.0.0:8080
```

### 2️⃣ Agent Connection String Issue

**Problem**: Agent might be using `ws://localhost` or `ws://127.0.0.1`

**Solution**: Agent must use the **actual IP address** of the server

```bash
# ❌ WRONG (on remote machine)
./edr-agent --server ws://127.0.0.1:8080 --endpoint-id vm-01

# ✅ CORRECT (use server's real IP)
./edr-agent --server ws://192.168.1.100:8080 --endpoint-id vm-01
```

### 3️⃣ Firewall Blocking

**Problem**: Firewall blocking port 8080

**Solution**: Open port on server

```bash
sudo ufw allow 8080/tcp
sudo ufw status
```

## Step-by-Step Fix:

### On Server Machine (Main OS):

**1. Get server IP address:**
```bash
ip addr show | grep "inet " | grep -v 127.0.0.1
# Example output: inet 192.168.1.100/24
```

**2. Open firewall:**
```bash
sudo ufw allow 8080/tcp
```

**3. Start server (bind to all interfaces):**
```bash
cd /home/rubberpirate/rust-edr/edr-dashboard
./target/release/edr-server --bind 0.0.0.0:8080
```

**4. Verify server is listening:**
```bash
# In another terminal
netstat -tuln | grep 8080
# Should show: tcp 0 0.0.0.0:8080 0.0.0.0:* LISTEN
```

### On Agent Machine (VM):

**1. Test network connectivity:**
```bash
# Replace 192.168.1.100 with your server IP
ping 192.168.1.100
telnet 192.168.1.100 8080
```

**2. Start agent with correct server IP:**
```bash
# Replace IP with your actual server IP
./edr-agent --server ws://192.168.1.100:8080 --endpoint-id vm-01
```

## Network Setup for VirtualBox:

### Recommended: Bridged Adapter

**Settings:**
- VM → Settings → Network
- Attached to: **Bridged Adapter**
- Promiscuous Mode: Allow All

**Benefits:**
- VM gets IP on same network as host
- Direct communication
- No port forwarding needed

**Get VM IP:**
```bash
# On VM
ip addr show | grep inet
```

### Alternative: Host-Only Network

**Setup:**
1. File → Host Network Manager → Create
2. Note the network (e.g., 192.168.56.0/24)
3. VM → Settings → Network → Host-Only Adapter
4. Server binds to host-only IP
5. Agent connects to host-only IP

## Quick Test:

**Run this on server to test:**
```bash
cd /home/rubberpirate/rust-edr/edr-dashboard
./debug-connection.sh
```

## Still Not Working?

### Check Server Logs:

The server should show:
```
🛡️  EDR Central Monitoring Server
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Binding to: 0.0.0.0:8080
✅ Server started!
📡 Waiting for agents to connect...
🔌 New connection from: 10.1.33.187:XXXXX
```

If you see "Connection error: WebSocket protocol error", it means:
- Connection is reaching server (good!)
- But WebSocket handshake is failing (bad)

### Possible causes:

1. **Version mismatch**: Rebuild both server and agent
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Proxy interference**: Some networks block WebSocket upgrades
   - Try on different network
   - Check if corporate proxy is interfering

3. **Old connection hanging**: Kill all instances and restart
   ```bash
   pkill edr-server
   pkill edr-agent
   # Then restart
   ```

4. **Port conflict**: Something else using port 8080
   ```bash
   # Check what's using the port
   lsof -i :8080
   # Or use different port
   ./edr-server --bind 0.0.0.0:9090
   ./edr-agent --server ws://SERVER_IP:9090 --endpoint-id vm-01
   ```

## Expected Successful Output:

**Server:**
```
🛡️  EDR Central Monitoring Server
Binding to: 0.0.0.0:8080
✅ Server started!
📡 Waiting for agents to connect...
🔌 New connection from: 10.1.33.187:45142
(No error - connection successful!)
```

**Agent:**
```
🛡️  EDR Agent Starting...
Endpoint ID: vm-01
Server: ws://192.168.1.100:8080
Heartbeat: 5s
🔌 Connecting to server...
✅ Connected!
(Keeps running - sending heartbeats every 5s)
```

**TUI Dashboard:**
- Should show endpoint in "📡 Endpoints" panel
- Status: 🟢 Online
- Live CPU, memory, process stats

## Your Specific Case (from screenshot):

You have:
- Connection from: `10.1.33.187:45142`
- This is NOT localhost (10.x.x.x is likely VM network)

**What to do:**

1. **Make sure server binds to 0.0.0.0:**
   ```bash
   ./edr-server --bind 0.0.0.0:8080
   ```

2. **Find what IP the server is actually on:**
   ```bash
   ip addr | grep "10.1"
   # Might show: inet 10.1.33.XXX
   ```

3. **Agent should connect to that IP:**
   ```bash
   ./edr-agent --server ws://10.1.33.XXX:8080 --endpoint-id test
   ```

4. **Check both firewall AND network settings:**
   - Firewall: `sudo ufw allow 8080/tcp`
   - VirtualBox: Use Bridged or Host-Only adapter
   - NOT NAT (NAT requires port forwarding)

## Summary:

The error means the WebSocket connection is being attempted but the HTTP upgrade handshake is failing. This is almost always due to:

✅ **Solution**: Server must bind to `0.0.0.0:8080`, not `127.0.0.1:8080`
✅ **Solution**: Agent must use server's real IP address, not localhost
✅ **Solution**: Firewall must allow port 8080

Try these steps and it should work!
