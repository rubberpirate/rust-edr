# 🔬 Forensics & Investigation Guide

## 🎯 Overview

The Rust EDR now includes **automatic forensics** capabilities that:

✅ **Automatically capture snapshots** when HIGH/CRITICAL threats detected  
✅ **Create investigation shells** for each threat with helper commands  
✅ **Compress and archive** threat sessions for long-term storage  
✅ **Provide CLI tools** for forensic analysis and log management  

---

## 🚀 Quick Start

### 1. Start EDR (Forensics Auto-Enabled)

```bash
sudo ./target/release/rust-edr start --verbose
```

**What happens when a threat is detected:**
- 📸 Forensic snapshot captured automatically (processes, network, memory, files)
- 🐚 Investigation shell script created in `/var/log/rust-edr/archives/sessions/`
- 📝 All related events logged

### 2. Trigger a Test Threat

```bash
# Run test script from /tmp (triggers HIGH severity alert)
echo '#!/bin/bash
echo "Testing EDR"' > /tmp/test.sh
chmod +x /tmp/test.sh
sudo /tmp/test.sh
```

### 3. Check Investigation Artifacts

```bash
# Find your investigation session
ls -la /var/log/rust-edr/archives/sessions/

# Example output:
# drwxr-xr-x 2 root root 4096 Oct 28 15:30 investigation_threat_abc123/

# Enter investigation shell
cd /var/log/rust-edr/archives/sessions/investigation_threat_abc123/
bash investigate.sh
```

---

## 🐚 Investigation Shell Commands

When you run an investigation shell, you get an interactive environment:

```bash
bash /var/log/rust-edr/archives/sessions/investigation_<threat-id>/investigate.sh
```

### Available Commands

| Command | Description |
|---------|-------------|
| `info` | Show complete threat information |
| `events` | List all related events (JSON) |
| `snapshot` | Capture new system snapshot |
| `logs` | View recent EDR threat logs |
| `archive` | Archive this investigation session |
| `help` | Show all available commands |
| `clear` | Clear screen and show banner |
| `exit` | Close investigation shell |

### Example Session

```bash
[edr-investigate] info
═══════════════════════════════════════════════
  RUST EDR - THREAT INVESTIGATION SESSION
═══════════════════════════════════════════════
Threat ID: threat_abc123
Type: SuspiciousProcess
Severity: High
Score: 8.50
Timestamp: 2025-10-28T15:30:00Z
═══════════════════════════════════════════════

DESCRIPTION:
Event: ProcessCreated | Rule matches: suspicious_process_location

RELATED EVENTS (2):
  - ProcessCreated: /tmp/test.sh
  - FileCreated: /tmp/test.sh

RULE MATCHES (1):
  - suspicious_process_location

[edr-investigate] snapshot
📸 Capturing system snapshot...
✅ Snapshot saved: snapshot_20251028_153045.txt

[edr-investigate] logs
📋 Recent EDR Logs:
{
  "id": "threat_abc123",
  "threat_type": "SuspiciousProcess",
  "severity": "High",
  "score": 8.5,
  ...
}

[edr-investigate] archive
🗜️ Archiving session...
✅ Session archived: investigation_threat_abc123_20251028_153100.tar.gz

[edr-investigate] exit
👋 Closing investigation shell...
```

---

## 📦 Forensics CLI Commands

### Archive Management

#### List All Archives
```bash
sudo ./target/release/rust-edr forensics list
```

Output:
```
📚 Available Archives:

  📦 threat_abc123_20251028_153000.tar.gz (5.2 MB) - Created: 2025-10-28 15:30:00
  📦 threat_def456_20251028_140000.tar.gz (3.8 MB) - Created: 2025-10-28 14:00:00
```

#### Archive a Threat Session
```bash
sudo ./target/release/rust-edr forensics archive <threat-id>
```

This creates a compressed archive containing:
- All log files related to the threat
- Captured snapshots
- Investigation shell scripts
- Event timeline

#### Extract an Archive
```bash
sudo ./target/release/rust-edr forensics extract \
  /var/log/rust-edr/archives/threats/threat_abc123_*.tar.gz \
  --output /tmp/investigation
```

### Log Management

#### Compress Old Logs
```bash
# Compress logs older than 7 days
sudo ./target/release/rust-edr forensics compress --days 7
```

Output:
```
🗜️ Compressing logs older than 7 days
  ✓ Compressed and removed: threats_20251020.jsonl
  ✓ Compressed and removed: events_20251020.jsonl
✅ Compressed 2 log files
```

#### Cleanup Old Archives
```bash
# Remove archives older than 90 days
sudo ./target/release/rust-edr forensics cleanup --days 90
```

### Snapshot Capture

#### Manual Snapshot
```bash
sudo ./target/release/rust-edr forensics snapshot <threat-id>
```

This captures:
- ✅ All running processes (PID, name, cmdline, UID, memory)
- ✅ Network connections (TCP/UDP, local/remote addresses, states)
- ✅ Open files (first 100 for performance)
- ✅ Memory maps (sample from each process)
- ✅ System information (hostname, kernel, load average, uptime)

Output saved as:
- `/var/log/rust-edr/archives/snapshots/snapshot_<threat-id>.json` (machine-readable)
- `/var/log/rust-edr/archives/snapshots/snapshot_<threat-id>.txt` (human-readable)

---

## 📁 File Structure

```
/var/log/rust-edr/
├── events_*.jsonl              # All system events
├── threats_*.jsonl             # Detected threats
├── responses_*.jsonl           # Response actions
└── archives/
    ├── threats/                # Archived threat sessions (.tar.gz)
    ├── snapshots/              # Forensic snapshots (JSON + TXT)
    ├── sessions/               # Active investigation sessions
    │   └── investigation_<id>/
    │       ├── session_info.txt        # Threat details
    │       ├── investigate.sh          # Interactive shell script
    │       ├── snapshot_*.txt          # System snapshots
    │       └── snapshot_*.json         # Snapshot data
    └── *.jsonl.gz              # Compressed old logs
```

---

## 🔍 Forensic Snapshot Contents

### Example snapshot.json structure:
```json
{
  "timestamp": "2025-10-28T15:30:00Z",
  "threat_id": "threat_abc123",
  "processes": [
    {
      "pid": 1234,
      "name": "suspicious_process",
      "cmdline": ["/tmp/suspicious_process", "--arg1"],
      "uid": 1000,
      "parent_pid": 1000,
      "status": "running",
      "memory_kb": 5120
    }
  ],
  "network_connections": [
    {
      "protocol": "TCP",
      "local_addr": "192.168.1.100",
      "local_port": 54321,
      "remote_addr": "45.33.32.156",
      "remote_port": 4444,
      "state": "ESTABLISHED",
      "pid": 1234
    }
  ],
  "open_files": [
    {
      "pid": 1234,
      "path": "/etc/passwd",
      "mode": "r"
    }
  ],
  "memory_maps": [
    {
      "pid": 1234,
      "address": "7f1234567000-7f1234568000",
      "permissions": "rwx",
      "path": "/tmp/suspicious_lib.so",
      "size_kb": 4
    }
  ],
  "system_info": {
    "hostname": "edr-test-vm",
    "kernel": "Linux version 5.15.0",
    "uptime_seconds": 12345,
    "load_average": [0.5, 0.3, 0.2],
    "total_processes": 156
  }
}
```

---

## 🎬 Complete Workflow Example

### Scenario: Detecting and Investigating Ransomware

#### 1. Start EDR
```bash
sudo ./target/release/rust-edr start --verbose --threshold 7.0
```

#### 2. Simulate Ransomware Attack
```bash
./tests/malicious_samples/ransomware_simulator.sh
```

#### 3. EDR Detects Threat (Automatic)
```
🚨 THREAT DETECTED: RansomwareBehavior - Score: 10.00
📸 Forensic snapshot captured for threat: threat_xyz789
🐚 Investigation shell created: /var/log/rust-edr/archives/sessions/investigation_threat_xyz789/investigate.sh
   Run with: bash /var/log/rust-edr/archives/sessions/investigation_threat_xyz789/investigate.sh
```

#### 4. Investigate
```bash
cd /var/log/rust-edr/archives/sessions/investigation_threat_xyz789/
bash investigate.sh

[edr-investigate] info
# Shows: RansomwareBehavior, Score 10.0, 50 rapid file modifications

[edr-investigate] snapshot
# Captures current system state

[edr-investigate] logs
# Shows all related EDR events

[edr-investigate] archive
# Archives entire investigation
```

#### 5. Generate Report
```bash
sudo ./target/release/rust-edr forensics archive threat_xyz789
```

Output: `/var/log/rust-edr/archives/threats/threat_xyz789_20251028_160000.tar.gz`

#### 6. Share with Security Team
```bash
# Extract archive on another system
tar -xzf threat_xyz789_20251028_160000.tar.gz

# Review snapshots
cat investigation_threat_xyz789/snapshot_*.txt
cat investigation_threat_xyz789/session_info.txt

# Analyze with jq
jq . investigation_threat_xyz789/snapshot_*.json
```

---

## 🔧 Advanced Features

### Custom Snapshot Analysis

```bash
# Capture snapshot
sudo ./target/release/rust-edr forensics snapshot threat_custom_001

# Extract specific data with jq
cd /var/log/rust-edr/archives/snapshots

# Find processes with RWX memory
jq '.memory_maps[] | select(.permissions == "rwx")' snapshot_threat_custom_001.json

# Find suspicious network connections
jq '.network_connections[] | select(.remote_port == 4444)' snapshot_threat_custom_001.json

# List processes by memory usage
jq '.processes | sort_by(.memory_kb) | reverse | .[0:10]' snapshot_threat_custom_001.json
```

### Automated Archiving

Create a cron job to archive and compress regularly:

```bash
# Add to crontab
sudo crontab -e

# Archive old logs daily at 2 AM
0 2 * * * /usr/local/bin/rust-edr forensics compress --days 7

# Cleanup old archives monthly
0 3 1 * * /usr/local/bin/rust-edr forensics cleanup --days 90
```

---

## 📊 Storage Management

### Disk Usage Estimates

| Component | Size per Day | Compressed | Notes |
|-----------|--------------|------------|-------|
| Events Log | ~50-100 MB | ~5-10 MB | Depends on activity |
| Threats Log | ~1-10 MB | ~100 KB - 1 MB | Depends on detections |
| Snapshots | ~500 KB - 2 MB each | ~100-500 KB | Per threat |
| Archives | Varies | 70-90% reduction | gzip compression |

### Recommended Retention

- **Active Logs**: 7 days uncompressed
- **Compressed Logs**: 30-90 days
- **Threat Archives**: 90-180 days
- **Critical Incidents**: Indefinite (move to external storage)

---

## 🛡️ Security Considerations

### File Permissions

All forensic data is stored with restricted permissions:
```bash
# Only root can access
sudo chown -R root:root /var/log/rust-edr/archives/
sudo chmod -R 700 /var/log/rust-edr/archives/
```

### Sensitive Data

Snapshots may contain:
- ⚠️ Process command lines (may include passwords)
- ⚠️ Open file paths (may reveal sensitive directories)
- ⚠️ Network connections (may include internal IPs)

**Always encrypt archives before sharing:**
```bash
# Encrypt archive
gpg --encrypt --recipient security@company.com \
    threat_abc123_20251028.tar.gz
```

---

## 🎯 Benefits of Forensic Features

### For Security Teams

✅ **Complete Evidence Chain**: All events captured and preserved  
✅ **Timeline Reconstruction**: Understand attack progression  
✅ **Rapid Response**: Investigation shells provide instant tooling  
✅ **Compliance**: Compressed archives meet retention requirements  

### For Incident Response

✅ **No Data Loss**: Automatic capture on detection  
✅ **Isolated Analysis**: Each threat gets own investigation environment  
✅ **Reproducible**: Archives can be extracted and re-analyzed  
✅ **Shareable**: Compressed format easy to transfer  

### For Forensic Analysis

✅ **Rich Context**: Process, network, file, memory state  
✅ **Machine-Readable**: JSON format for automated analysis  
✅ **Human-Readable**: Text summaries for quick review  
✅ **Correlated**: Events linked to specific threats  

---

## 📚 Further Reading

- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - EDR command reference
- [TESTING_GUIDE.md](TESTING_GUIDE.md) - Test the forensics system
- [LIMITATIONS_AND_ROADMAP.md](LIMITATIONS_AND_ROADMAP.md) - Production considerations

---

**🔬 Happy Investigating!**
