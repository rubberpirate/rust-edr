# Rust EDR - Build and Run Guide

## 📦 Complete Component List

### ✅ Detection Engine
- **IOC Matcher** (`src/detection/ioc.rs`) - Matches events against known Indicators of Compromise
- **Rule Engine** (`src/detection/rules.rs`) - 10+ behavioral detection rules
- **Threat Scorer** (`src/detection/scoring.rs`) - Calculates threat scores (0-10 scale)
- **Event Correlator** (`src/detection/correlator.rs`) - Detects complex attack patterns
- **Detection Engine** (`src/detection/engine.rs`) - Main coordinator

### ✅ Monitoring Agents
- **Process Monitor** (`src/monitors/process.rs`) - Tracks process creation/termination
- **File Monitor** (`src/monitors/file.rs`) - Monitors critical file operations using inotify
- **Network Monitor** (`src/monitors/network.rs`) - Tracks network connections
- **Memory Monitor** (`src/monitors/memory.rs`) - Detects suspicious memory operations (RWX pages)
- **User Monitor** (`src/monitors/user.rs`) - Tracks logins, logouts, privilege escalations
- **Rootkit Monitor** (`src/monitors/rootkit.rs`) - Detects kernel modules and hidden processes

### ✅ Telemetry System
- **Telemetry Logger** (`src/telemetry/logger.rs`) - JSONL logging to files
- **Event Store** (`src/telemetry/event_store.rs`) - Persistent storage with sled database

### ✅ Response Engine
- **Response Actions** (`src/response/actions.rs`) - Block, Allow, Quarantine, Alert, Kill, IsolateNetwork

---

## 🚀 Building the Project

### Step 1: Create Required Directories

```bash
# Create system directories (requires root)
sudo mkdir -p /var/log/rust-edr
sudo mkdir -p /var/lib/rust-edr
sudo mkdir -p /etc/rust-edr

# Set permissions
sudo chown -R $USER:$USER /var/log/rust-edr
sudo chown -R $USER:$USER /var/lib/rust-edr
```

### Step 2: Build the Project

```bash
cd /home/rubberpirate/rust-edr

# Build in debug mode (faster compilation)
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Step 3: Verify Build

```bash
# Check the binary was created
ls -lh target/release/rust-edr

# Test the CLI
./target/release/rust-edr --help
```

---

## 🎯 Running the EDR

### Basic Usage

```bash
# Start with default settings (requires root for full monitoring)
sudo ./target/release/rust-edr start

# Start with specific threshold
sudo ./target/release/rust-edr start --threshold 5.0

# Start with auto-response enabled
sudo ./target/release/rust-edr start --auto-response

# Start specific modules only
sudo ./target/release/rust-edr start --modules process,file,network

# Run in foreground with verbose output
sudo ./target/release/rust-edr start --foreground --verbose
```

### Check Status

```bash
# View agent status
sudo ./target/release/rust-edr status

# View recent alerts
sudo ./target/release/rust-edr alerts --recent 20

# Show configuration
./target/release/rust-edr config --show
```

---

## 🧪 Testing the EDR

### Test 1: Process Monitoring

```bash
# In another terminal, create a suspicious process
echo '#!/bin/bash\necho "test"' > /tmp/test.sh
chmod +x /tmp/test.sh
/tmp/test.sh
```

**Expected**: Alert for process execution from `/tmp/`

### Test 2: File Monitoring

```bash
# Create/modify a critical file
sudo touch /etc/test_file.conf
sudo rm /etc/test_file.conf
```

**Expected**: Alerts for file creation and deletion in `/etc/`

### Test 3: Network Monitoring

```bash
# Create a connection (may require nc installed)
timeout 1 bash -c 'cat < /dev/tcp/8.8.8.8/80' 2>/dev/null || true
```

**Expected**: Network connection alert

### Test 4: User Actions

```bash
# Trigger privilege escalation detection
sudo whoami
```

**Expected**: Privilege escalation alert

### Test 5: Kernel Module (if root)

```bash
# List kernel modules
lsmod

# Load a module (example)
# sudo modprobe <module_name>
```

**Expected**: Module load detection

---

## 📊 Understanding the Output

### Event Log Format (JSONL)

Location: `/var/log/rust-edr/events_YYYYMMDD.jsonl`

```json
{
  "id": "proc_create_...",
  "timestamp": "2025-10-28T10:30:00Z",
  "event_type": "ProcessCreated",
  "severity": "High",
  "source": "process_monitor",
  "details": {
    "Process": {
      "pid": 12345,
      "name": "suspicious.sh",
      "path": "/tmp/suspicious.sh",
      "cmdline": ["/tmp/suspicious.sh"],
      "user": "testuser",
      "uid": 1000
    }
  }
}
```

### Threat Log Format

Location: `/var/log/rust-edr/threats_YYYYMMDD.jsonl`

```json
{
  "id": "threat_...",
  "timestamp": "2025-10-28T10:30:05Z",
  "threat_type": "SuspiciousProcess",
  "severity": "High",
  "score": 7.5,
  "description": "Event: ProcessCreated | Rule matches: suspicious_process_location",
  "events": [...],
  "ioc_matches": [],
  "rule_matches": ["suspicious_process_location"]
}
```

### Response Log Format

Location: `/var/log/rust-edr/responses_YYYYMMDD.jsonl`

```json
{
  "action": "Alert",
  "success": true,
  "message": "Alert: SuspiciousProcess threat detected - Score: 7.50 - ...",
  "timestamp": "2025-10-28T10:30:05Z"
}
```

---

## 🔧 Configuration

### Threat Threshold

- `0.0 - 2.0`: Info severity
- `2.0 - 4.0`: Low severity
- `4.0 - 6.0`: Medium severity
- `6.0 - 8.0`: High severity
- `8.0 - 10.0`: Critical severity

### Auto-Response Behavior

When enabled:
- **Critical** (8.0+): Kill process + Quarantine
- **High** (6.0-8.0): Block + Quarantine
- **Medium** (4.0-6.0): Block only
- **Low/Info** (<4.0): Alert only

---

## 🐛 Troubleshooting

### Permission Denied

```bash
# Most operations require root
sudo ./target/release/rust-edr start
```

### Directory Not Found

```bash
# Create required directories
sudo mkdir -p /var/log/rust-edr /var/lib/rust-edr
sudo chown -R $USER:$USER /var/log/rust-edr /var/lib/rust-edr
```

### Compilation Errors

```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release
```

### Too Many Events

Adjust threshold or disable noisy modules:

```bash
sudo ./target/release/rust-edr start --threshold 8.0 --modules process,user,rootkit
```

---

## 📈 Performance Considerations

### Resource Usage

- **Process Monitor**: ~1-2% CPU (polling every 2s)
- **File Monitor**: <1% CPU (event-driven)
- **Network Monitor**: ~1-2% CPU (polling every 5s)
- **Memory Monitor**: ~1% CPU (polling every 10s)
- **Detection Engine**: <1% CPU
- **Total Memory**: ~10-50 MB

### Optimization Tips

1. **Increase polling intervals** in monitor code
2. **Disable unused modules** with `--modules`
3. **Increase threshold** to reduce false positives
4. **Limit file watch paths** to critical directories only

---

## 🔒 Security Notes

⚠️ **This is a research/educational EDR system**

For production use:
- Add proper authentication/authorization
- Implement secure log rotation
- Add tamper protection
- Use systemd service for daemon mode
- Add TLS for remote logging
- Implement proper error recovery
- Add comprehensive testing

---

## 📚 Next Steps

1. **Run the EDR** and monitor output
2. **Generate test events** to verify detection
3. **Review logs** in `/var/log/rust-edr/`
4. **Tune threshold** based on your environment
5. **Add custom IOCs/rules** as needed

---

## 🎓 Learning Resources

### Detection Rules

- Review `src/detection/rules.rs` for behavioral rules
- Add custom rules by modifying the `add_default_rules()` function

### IOCs

- Review `src/detection/ioc.rs` for IOC matching
- Add custom IOCs in the `add_default_iocs()` function

### Correlation Patterns

- Review `src/detection/correlator.rs` for attack patterns
- Implements: privilege escalation chains, data exfiltration, ransomware, etc.

---

**Ready to start!** Run: `sudo cargo run --release -- start --verbose`
