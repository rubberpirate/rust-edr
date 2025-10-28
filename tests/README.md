# EDR Malicious Test Suite

This directory contains safe simulations of malicious behavior to test the EDR system.

## ⚠️ WARNING

These scripts simulate malicious activities and should **ONLY** be run in:
- Virtual machines (recommended)
- Isolated test environments
- Development systems where you understand the implications

**DO NOT run on production systems!**

## 📁 Test Files

### Main Test Suite
- **`run_malicious_tests.sh`** - Comprehensive test suite (20 tests)
  - Runs all detection scenarios
  - Safe to execute, no actual harm
  - Tests all EDR components

### Individual Simulators

1. **`reverse_shell_simulator.sh`** - Reverse shell patterns
   - Tests: Command pattern detection
   - Expected: HIGH severity alerts

2. **`ransomware_simulator.sh`** - Ransomware behavior
   - Tests: Rapid file modifications, mass deletions
   - Expected: CRITICAL severity alerts

3. **`privilege_escalation_simulator.sh`** - Privilege escalation
   - Tests: sudo/su usage, SUID enumeration
   - Expected: HIGH severity alerts

4. **`data_exfiltration_simulator.sh`** - Data exfiltration
   - Tests: Sensitive file access, network transfers
   - Expected: HIGH severity alerts

5. **`rootkit_simulator.sh`** - Rootkit behavior
   - Tests: Hidden files/processes, kernel modules
   - Expected: CRITICAL severity alerts

6. **`lateral_movement_simulator.sh`** - Lateral movement
   - Tests: Network scanning, credential testing
   - Expected: HIGH severity alerts

## 🚀 Quick Start

### 1. Make scripts executable
```bash
chmod +x tests/run_malicious_tests.sh
chmod +x tests/malicious_samples/*.sh
```

### 2. Start the EDR
```bash
# In one terminal
sudo ./target/release/rust-edr start --verbose
```

### 3. Run the test suite
```bash
# In another terminal
./tests/run_malicious_tests.sh
```

### 4. Check results
```bash
# View threats in real-time
tail -f /var/log/rust-edr/threats_*.jsonl | jq .

# View recent alerts
sudo ./target/release/rust-edr alerts --recent 20

# Check status
sudo ./target/release/rust-edr status
```

## 🧪 Running Individual Tests

```bash
# Test specific behavior
./tests/malicious_samples/ransomware_simulator.sh

# Test reverse shells
./tests/malicious_samples/reverse_shell_simulator.sh

# Test privilege escalation
./tests/malicious_samples/privilege_escalation_simulator.sh
```

## 📊 Expected Detection Results

| Test Category | Expected Severity | Detection Rules Triggered |
|---------------|-------------------|---------------------------|
| Suspicious Process Location | HIGH | suspicious_process_location |
| Critical File Modification | CRITICAL | critical_file_modification |
| Privilege Escalation | HIGH | privilege_escalation |
| Ransomware Behavior | CRITICAL | Correlation: ransomware_behavior |
| Reverse Shell Patterns | HIGH | suspicious_cmdline |
| Network Suspicious Ports | MEDIUM/HIGH | uncommon_port_connection |
| Hidden File Execution | MEDIUM | hidden_file_execution |
| Rootkit Indicators | CRITICAL | RootkitDetected events |
| Data Exfiltration | HIGH | Correlation: data_exfiltration |
| Lateral Movement | HIGH | Correlation: lateral_movement |

## 🎯 Test Coverage

The test suite covers:
- ✅ All 10 behavioral detection rules
- ✅ All 5 event correlation patterns
- ✅ All 6 monitoring agents (process, file, network, memory, user, rootkit)
- ✅ IOC matching
- ✅ Threat scoring
- ✅ Response engine triggers

## 📝 Test Logging

All test activities are logged to:
- `/var/log/rust-edr/events_YYYYMMDD.jsonl` - Raw events
- `/var/log/rust-edr/threats_YYYYMMDD.jsonl` - Detected threats
- `/var/log/rust-edr/responses_YYYYMMDD.jsonl` - Response actions

## 🔍 Analyzing Results

### View threat details with jq
```bash
# Pretty print threats
cat /var/log/rust-edr/threats_*.jsonl | jq .

# Filter by severity
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.severity == "Critical")'

# Filter by threat type
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.threat_type == "Ransomware")'

# Count threats by type
cat /var/log/rust-edr/threats_*.jsonl | jq -r '.threat_type' | sort | uniq -c
```

### Check correlation events
```bash
# View correlated threats (attacks detected from multiple events)
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.events | length > 1)'
```

## 🧹 Cleanup

The test suite automatically cleans up after itself, but you can manually clean:

```bash
# Remove test artifacts
rm -f /tmp/suspicious_process.sh
rm -f /dev/shm/shm_malware.sh
rm -f /tmp/.hidden_malware
rm -rf /tmp/ransomware_test
rm -rf /tmp/ransomware_simulation_*
rm -rf /tmp/exfil_staging_*
rm -rf /tmp/.hidden_rootkit

# Clear EDR logs (if needed)
sudo rm -f /var/log/rust-edr/*.jsonl
```

## 💡 Tips

1. **Run in VM**: Best practice for safety
2. **Monitor in real-time**: Keep `tail -f` running on threat logs
3. **Adjust threshold**: Lower threshold (e.g., 3.0) to catch more events
4. **Enable auto-response**: Test with `--auto-response` flag (carefully!)
5. **Compare outputs**: Run tests with different configurations

## 🐛 Troubleshooting

### No detections showing up
```bash
# Check if EDR is running
ps aux | grep rust-edr

# Check permissions
ls -l /var/log/rust-edr/

# Verify monitors are active
# Should see process scanning messages in verbose output
```

### Permission errors
```bash
# Some tests require sudo
sudo ./tests/malicious_samples/privilege_escalation_simulator.sh

# Fix log directory permissions
sudo chown -R $USER:$USER /var/log/rust-edr
```

### Tests run too fast
```bash
# Add delays in test scripts or run individually
# Each test has built-in waits for detection
```

## 📚 Learning Resources

These tests demonstrate:
- MITRE ATT&CK techniques
- Common malware behaviors
- Attacker tradecraft
- Detection engineering

Use them to understand:
- How EDR systems work
- What suspicious behavior looks like
- How correlation detects complex attacks

---

**Remember: These are simulations for educational purposes only!**
