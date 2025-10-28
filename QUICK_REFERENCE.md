# Rust EDR - Quick Reference

## 🚀 Quick Start

```bash
# 1. Create directories
sudo mkdir -p /var/log/rust-edr /var/lib/rust-edr
sudo chown -R $USER:$USER /var/log/rust-edr /var/lib/rust-edr

# 2. Build
cargo build --release

# 3. Run
sudo ./target/release/rust-edr start --verbose
```

## 📋 Components Overview

| Component | File | Function |
|-----------|------|----------|
| **IOC Matcher** | `detection/ioc.rs` | Match against known bad indicators |
| **Rule Engine** | `detection/rules.rs` | 10 behavioral detection rules |
| **Threat Scorer** | `detection/scoring.rs` | Calculate threat scores (0-10) |
| **Correlator** | `detection/correlator.rs` | Detect complex attack patterns |
| **Process Monitor** | `monitors/process.rs` | Track process creation/termination |
| **File Monitor** | `monitors/file.rs` | Monitor file operations (inotify) |
| **Network Monitor** | `monitors/network.rs` | Track network connections |
| **Memory Monitor** | `monitors/memory.rs` | Detect RWX memory pages |
| **User Monitor** | `monitors/user.rs` | Track logins and privilege escalation |
| **Rootkit Monitor** | `monitors/rootkit.rs` | Detect kernel modules & hidden processes |
| **Response Engine** | `response/actions.rs` | Block, Kill, Quarantine, Alert |
| **Telemetry** | `telemetry/logger.rs` | Log all events to JSONL files |
| **Event Store** | `telemetry/event_store.rs` | Persistent sled database |

## 🎯 Detection Rules (10 Built-in)

1. **suspicious_process_location** - Process from /tmp or /dev/shm
2. **critical_file_modification** - Modify /etc/passwd, /etc/shadow, SSH keys
3. **uncommon_port_connection** - Connect to ports 4444, 31337, 1337, etc.
4. **privilege_escalation** - sudo or su usage
5. **remote_root_login** - Root login from remote IP
6. **suspicious_cmdline** - wget/curl piped to bash, nc, /dev/tcp
7. **memory_injection** - RWX memory permissions
8. **hidden_file_execution** - Execute files starting with '.'
9. **high_volume_transfer** - Transfer >100MB
10. **root_process_spawn** - New root process created

## 🔍 Correlation Patterns (5 Built-in)

1. **Privilege Escalation Chain** - User elevation → Process spawn → File mod
2. **Data Exfiltration** - File access → Large network transfer
3. **Lateral Movement** - Network connections + Process spawns + Auth
4. **Ransomware Behavior** - Rapid file modifications (10+) or deletions (5+)
5. **Rootkit Installation** - System file mod + Kernel module + Hidden processes

## 📊 Threat Severity & Scoring

| Score | Severity | Auto-Response (if enabled) |
|-------|----------|----------------------------|
| 0-2 | Info | Alert only |
| 2-4 | Low | Alert only |
| 4-6 | Medium | Alert + Block |
| 6-8 | High | Alert + Block + Quarantine |
| 8-10 | Critical | Alert + Block + Quarantine + Kill |

## 🛠️ CLI Commands

```bash
# Start with defaults
sudo ./target/release/rust-edr start

# Start with threshold
sudo ./target/release/rust-edr start --threshold 5.0

# Enable auto-response
sudo ./target/release/rust-edr start --auto-response

# Select modules
sudo ./target/release/rust-edr start --modules process,file,user

# View status
sudo ./target/release/rust-edr status

# View recent threats
sudo ./target/release/rust-edr alerts --recent 20

# Show config
./target/release/rust-edr config --show
```

## 📁 File Locations

| Path | Contents |
|------|----------|
| `/var/log/rust-edr/events_*.jsonl` | All system events (JSONL) |
| `/var/log/rust-edr/threats_*.jsonl` | Detected threats (JSONL) |
| `/var/log/rust-edr/responses_*.jsonl` | Response actions (JSONL) |
| `/var/lib/rust-edr/events.db` | Persistent event database (sled) |

## 🧪 Test Commands

```bash
# Test process monitoring
echo '#!/bin/bash' > /tmp/test.sh && chmod +x /tmp/test.sh && /tmp/test.sh

# Test file monitoring
sudo touch /etc/test.conf && sudo rm /etc/test.conf

# Test privilege escalation
sudo whoami

# Test network (requires internet)
timeout 1 bash -c 'cat < /dev/tcp/8.8.8.8/80' 2>/dev/null || true

# View logs
tail -f /var/log/rust-edr/threats_*.jsonl | jq .
```

## 🔧 Customization

### Add Custom IOC
Edit `src/detection/ioc.rs`, add to `add_default_iocs()`:
```rust
self.add_ioc(IOC {
    id: "custom_malware".to_string(),
    ioc_type: IOCType::ProcessName,
    value: "malware.exe".to_string(),
    description: "Known malware".to_string(),
    severity: Severity::Critical,
    tags: vec!["malware".to_string()],
});
```

### Add Custom Rule
Edit `src/detection/rules.rs`, add to `add_default_rules()`:
```rust
self.rules.push(BehavioralRule {
    id: "my_custom_rule".to_string(),
    name: "My Custom Detection".to_string(),
    description: "Detects something specific".to_string(),
    severity: Severity::High,
    enabled: true,
    conditions: vec![],
});
```

## 💡 Performance Tuning

```bash
# Low resource mode (process + user only)
sudo ./target/release/rust-edr start --modules process,user --threshold 8.0

# High sensitivity mode (all modules, low threshold)
sudo ./target/release/rust-edr start --threshold 3.0

# Production mode (auto-response on)
sudo ./target/release/rust-edr start --auto-response --threshold 7.0
```

## 🐛 Common Issues

| Issue | Solution |
|-------|----------|
| Permission denied | Run with `sudo` |
| Directory not found | Create: `sudo mkdir -p /var/log/rust-edr /var/lib/rust-edr` |
| Too many events | Increase threshold: `--threshold 8.0` |
| Build fails | Run: `cargo clean && cargo build --release` |
| Can't write logs | Fix perms: `sudo chown -R $USER /var/log/rust-edr` |

## 📚 Architecture Flow

```
Events → Monitors → Detection Engine → Threat Scorer → Response Engine
                          ↓                              ↓
                    Event Correlator              Telemetry Logger
                          ↓                              ↓
                   Threat Detection                  Event Store
```

## 🎯 Monitored Activities

- ✅ Process creation/termination
- ✅ File create/modify/delete in critical paths
- ✅ Network connections (TCP/UDP)
- ✅ Memory operations (RWX pages)
- ✅ User logins/logouts
- ✅ Privilege escalations (sudo)
- ✅ Kernel module loading
- ✅ Hidden processes/files
- ✅ Suspicious command patterns
- ✅ Critical file modifications

---

**Happy Hunting! 🔍🛡️**
