#!/bin/bash
# Lateral Movement Simulator
# Simulates network-based lateral movement techniques

echo "➡️  Lateral Movement Simulator"
echo "=============================="
echo ""

# Test 1: SSH key enumeration
echo "[Test 1] SSH key enumeration (recon)"
echo "Searching for SSH keys..."
find ~/.ssh -type f 2>/dev/null || echo "  No SSH directory"
sleep 2

# Test 2: Network scanning simulation
echo ""
echo "[Test 2] Internal network scanning"
echo "Scanning local subnet (simulation)..."
for i in {1..3}; do
    timeout 1 bash -c "cat < /dev/tcp/127.0.0.$i/22" 2>/dev/null && echo "  Host 127.0.0.$i:22 - Open" || echo "  Host 127.0.0.$i:22 - Closed"
done
sleep 2

# Test 3: SMB/CIFS enumeration
echo ""
echo "[Test 3] SMB/CIFS share enumeration"
echo "Checking for network shares..."
mount | grep cifs || echo "  No CIFS mounts found"
sleep 2

# Test 4: RDP connection attempt
echo ""
echo "[Test 4] RDP connection pattern (port 3389)"
timeout 1 bash -c 'cat < /dev/tcp/127.0.0.1/3389' 2>/dev/null || echo "  RDP connection attempted (failed as expected)"
sleep 2

# Test 5: WinRM connection attempt
echo ""
echo "[Test 5] WinRM connection pattern (port 5985)"
timeout 1 bash -c 'cat < /dev/tcp/127.0.0.1/5985' 2>/dev/null || echo "  WinRM connection attempted (failed as expected)"
sleep 2

# Test 6: SSH connection attempts
echo ""
echo "[Test 6] SSH lateral movement simulation"
for i in {1..3}; do
    echo "  Attempting SSH to 192.168.1.$i..."
    timeout 1 ssh -o ConnectTimeout=1 -o StrictHostKeyChecking=no user@192.168.1.$i 2>&1 | head -1
    sleep 1
done

# Test 7: Credential reuse pattern
echo ""
echo "[Test 7] Credential reuse pattern"
echo "Testing common credentials (simulation only)..."
for user in admin administrator root; do
    echo "  Testing user: $user"
    sleep 0.5
done

# Test 8: Port 445 (SMB) scanning
echo ""
echo "[Test 8] SMB port scanning (445)"
for i in {1..3}; do
    timeout 1 bash -c "cat < /dev/tcp/127.0.0.$i/445" 2>/dev/null && echo "  SMB port open on 127.0.0.$i" || echo "  SMB port closed on 127.0.0.$i"
done
sleep 2

# Test 9: WMI execution pattern (Linux equivalent)
echo ""
echo "[Test 9] Remote execution pattern"
echo "Simulating remote command execution..."
bash -c 'echo "Executing: wmic process call create calc.exe"'
sleep 2

echo ""
echo "✅ Lateral movement simulation completed"
echo ""
echo "🚨 Expected EDR Detections:"
echo "   - HIGH: Multiple network connection attempts"
echo "   - MEDIUM: SSH key enumeration"
echo "   - HIGH: Internal network scanning"
echo "   - MEDIUM: Credential testing patterns"
echo "   - HIGH: Lateral movement pattern correlation"
