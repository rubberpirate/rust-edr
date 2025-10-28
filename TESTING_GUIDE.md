# 🧪 Quick Test Guide for Rust EDR

## 🚀 3-Step Testing Process

### Step 1: Start the EDR (Terminal 1)
```bash
cd /home/rubberpirate/rust-edr
sudo mkdir -p /var/log/rust-edr /var/lib/rust-edr
sudo chown -R $USER:$USER /var/log/rust-edr /var/lib/rust-edr
sudo ./target/release/rust-edr start --verbose
```

### Step 2: Run Tests (Terminal 2)
```bash
cd /home/rubberpirate/rust-edr
./tests/run_malicious_tests.sh
```

### Step 3: View Results (Terminal 3)
```bash
# Real-time threat monitoring
tail -f /var/log/rust-edr/threats_*.jsonl | jq .

# Or view recent alerts
sudo ./target/release/rust-edr alerts --recent 20
```

---

## 🎯 Available Tests

### Main Test Suite (20 tests)
```bash
./tests/run_malicious_tests.sh
```
Covers all detection scenarios in one run (~2 minutes)

### Individual Test Suites

#### 1. Ransomware Test 💀
```bash
./tests/malicious_samples/ransomware_simulator.sh
```
- Creates 50 files
- Rapid modifications (encryption simulation)
- Mass deletions
- **Expected**: CRITICAL ransomware correlation alert

#### 2. Reverse Shell Test 🔴
```bash
./tests/malicious_samples/reverse_shell_simulator.sh
```
- Netcat patterns
- Bash /dev/tcp patterns
- Python/Perl reverse shells
- **Expected**: HIGH suspicious command alerts

#### 3. Privilege Escalation Test ⬆️
```bash
./tests/malicious_samples/privilege_escalation_simulator.sh
```
- sudo/su usage
- /etc/shadow access
- SUID enumeration
- **Expected**: HIGH/CRITICAL privilege escalation alerts

#### 4. Data Exfiltration Test 📤
```bash
./tests/malicious_samples/data_exfiltration_simulator.sh
```
- Sensitive file collection
- Network connections
- Data staging
- **Expected**: HIGH exfiltration correlation

#### 5. Rootkit Test 👻
```bash
./tests/malicious_samples/rootkit_simulator.sh
```
- Hidden files/processes
- Kernel module operations
- Process obfuscation
- **Expected**: CRITICAL rootkit detection

#### 6. Lateral Movement Test ➡️
```bash
./tests/malicious_samples/lateral_movement_simulator.sh
```
- Network scanning
- SSH attempts
- Credential testing
- **Expected**: HIGH lateral movement correlation

---

## 📊 Expected Results Summary

| Test | Severity | Alerts Expected | Correlation |
|------|----------|-----------------|-------------|
| Ransomware | CRITICAL | 10+ | ✅ Yes |
| Reverse Shell | HIGH | 5+ | ❌ No |
| Privilege Escalation | HIGH/CRITICAL | 5+ | ✅ Yes |
| Data Exfiltration | HIGH | 5+ | ✅ Yes |
| Rootkit | CRITICAL | 3+ | ✅ Yes |
| Lateral Movement | HIGH | 8+ | ✅ Yes |
| **Main Suite (All)** | MIXED | **30+** | **5 types** |

---

## 🔍 How to Analyze Results

### Option 1: Real-time Monitoring
```bash
# Watch threats appear live
tail -f /var/log/rust-edr/threats_*.jsonl | jq .
```

### Option 2: CLI Queries
```bash
# View recent threats
sudo ./target/release/rust-edr alerts --recent 20

# Check EDR status
sudo ./target/release/rust-edr status
```

### Option 3: Manual Log Analysis
```bash
# Count total threats
cat /var/log/rust-edr/threats_*.jsonl | wc -l

# Count by severity
cat /var/log/rust-edr/threats_*.jsonl | jq -r '.severity' | sort | uniq -c

# Count by threat type
cat /var/log/rust-edr/threats_*.jsonl | jq -r '.threat_type' | sort | uniq -c

# Show only CRITICAL threats
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.severity == "Critical")'

# Show threats with high scores (>7.0)
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.score > 7.0)'

# Show correlation-based threats (multiple events)
cat /var/log/rust-edr/threats_*.jsonl | jq 'select(.events | length > 2)'
```

---

## 🎬 Example Test Run

```bash
# Terminal 1: Start EDR
$ sudo ./target/release/rust-edr start --verbose
🛡️  Starting Rust EDR System...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Threat threshold: 7.0
Auto-response: disabled
Enabled modules: process,file,network,user,rootkit
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting process monitor...
Starting file monitor...
Starting network monitor...
Starting user monitor...
Starting rootkit monitor...
✅ EDR System running. Press Ctrl+C to stop.

# Terminal 2: Run tests
$ ./tests/run_malicious_tests.sh
🧪 Rust EDR Test Suite - Malicious Behavior Simulator
==================================================
⚠️  WARNING: This script simulates malicious behavior
    Only run in a controlled environment (VM recommended)

Press Enter to continue or Ctrl+C to cancel...

Test 1: Suspicious Process Location
Description: Execute script from /tmp directory
---
I'm running from /tmp - this should trigger HIGH alert!
⏱️  Waiting 3 seconds for EDR detection...

# Terminal 3: View detections
$ tail -f /var/log/rust-edr/threats_*.jsonl | jq .
{
  "id": "threat_abc123",
  "timestamp": "2025-10-28T10:30:00Z",
  "threat_type": "SuspiciousProcess",
  "severity": "High",
  "score": 7.5,
  "description": "Event: ProcessCreated | Rule matches: suspicious_process_location",
  ...
}
```

---

## ⚡ Quick Commands Cheat Sheet

```bash
# Start EDR
sudo ./target/release/rust-edr start --verbose

# Run all tests
./tests/run_malicious_tests.sh

# View threats live
tail -f /var/log/rust-edr/threats_*.jsonl | jq .

# View recent alerts
sudo ./target/release/rust-edr alerts --recent 20

# Stop EDR
Ctrl+C (in EDR terminal)

# Clean logs
sudo rm -f /var/log/rust-edr/*.jsonl

# Clean test artifacts
rm -rf /tmp/ransomware_* /tmp/exfil_* /tmp/.hidden_* /tmp/suspicious_*
```

---

## 🐛 Common Issues

### Issue: No threats detected
**Solution**: 
- Check if EDR is running: `ps aux | grep rust-edr`
- Lower threshold: `--threshold 3.0`
- Check logs exist: `ls -l /var/log/rust-edr/`

### Issue: Permission denied
**Solution**:
- Run EDR with sudo: `sudo ./target/release/rust-edr start`
- Fix permissions: `sudo chown -R $USER /var/log/rust-edr`

### Issue: Tests run but no output in logs
**Solution**:
- Wait longer (some monitors have 2-10s polling intervals)
- Check verbose output from EDR terminal
- Verify log directory: `ls /var/log/rust-edr/`

---

## 📈 Success Metrics

After running the full test suite, you should see:

✅ **30+ threats detected**  
✅ **At least 3 CRITICAL severity alerts** (ransomware, rootkit)  
✅ **At least 10 HIGH severity alerts**  
✅ **5 correlation patterns triggered**  
✅ **All 6 monitoring agents active**  
✅ **All 10 detection rules matched**  

---

## 🎓 What to Look For

### Good Signs ✅
- Threats appearing in logs within seconds
- Correlation alerts (multiple events combined)
- Accurate severity scoring
- Detailed event information
- No false negatives (all tests detected)

### Red Flags ❌
- No threats after 30 seconds
- Only INFO/LOW severity (threshold too high)
- Missing correlation alerts
- Monitors not starting
- Log files not created

---

## 🚀 Advanced Testing

### Test with Auto-Response
```bash
sudo ./target/release/rust-edr start --auto-response --threshold 7.0
./tests/run_malicious_tests.sh
# Check /var/log/rust-edr/responses_*.jsonl
```

### Test Specific Modules
```bash
# Only process and file monitors
sudo ./target/release/rust-edr start --modules process,file
./tests/run_malicious_tests.sh
```

### Test with Lower Threshold
```bash
# Catch more threats (including low severity)
sudo ./target/release/rust-edr start --threshold 3.0 --verbose
./tests/run_malicious_tests.sh
```

---

**Happy Testing! 🛡️🔬**

Remember: These are **safe simulations** for testing only!
