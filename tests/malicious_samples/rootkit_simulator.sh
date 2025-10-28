#!/bin/bash
# Rootkit Behavior Simulator
# Simulates rootkit-like activities

echo "👻 Rootkit Behavior Simulator"
echo "============================="
echo ""
echo "⚠️  Simulating rootkit detection triggers"
echo ""

# Test 1: Kernel module enumeration
echo "[Test 1] Kernel module enumeration"
echo "Listing loaded kernel modules..."
lsmod | head -10
sleep 2

# Test 2: Hidden process simulation
echo ""
echo "[Test 2] Process hiding simulation"
echo "Creating process with kernel-like name..."
bash -c 'exec -a "[kworker/0:1]" sleep 5' &
HIDDEN_PID=$!
echo "  Created fake kernel process: $HIDDEN_PID"
ps aux | grep "$HIDDEN_PID" | grep -v grep
sleep 3
kill $HIDDEN_PID 2>/dev/null
sleep 1

# Test 3: Hidden file creation
echo ""
echo "[Test 3] Hidden file creation"
mkdir -p /tmp/.hidden_rootkit 2>/dev/null
echo "rootkit payload" > /tmp/.hidden_rootkit/.payload
echo "  Created hidden directory: /tmp/.hidden_rootkit"
ls -la /tmp/.hidden_rootkit/
sleep 2

# Test 4: System call hooking simulation
echo ""
echo "[Test 4] System call hooking indicators"
echo "Checking /proc/kallsyms for syscall table..."
sudo grep sys_call_table /proc/kallsyms 2>/dev/null | head -3 || echo "  ℹ️  No access to kallsyms"
sleep 2

# Test 5: Memory manipulation patterns
echo ""
echo "[Test 5] Memory manipulation patterns"
echo "Searching for RWX memory regions..."
for pid in $(pgrep -x bash | head -3); do
    echo "  Checking PID $pid..."
    sudo cat /proc/$pid/maps 2>/dev/null | grep "rwx" | head -2
done
sleep 2

# Test 6: Kernel module loading (if permitted)
echo ""
echo "[Test 6] Kernel module operations"
echo "Attempting to load test module (will likely fail without actual module)..."
sudo modprobe test_module 2>&1 | head -3 || echo "  ℹ️  Module load attempted (expected to fail)"
sleep 2

# Test 7: /dev/kmem access attempt
echo ""
echo "[Test 7] Kernel memory access attempt"
ls -l /dev/kmem 2>/dev/null || echo "  ℹ️  /dev/kmem not accessible (good security)"
sleep 2

# Test 8: LKM (Loadable Kernel Module) check
echo ""
echo "[Test 8] Checking for suspicious kernel modules"
lsmod | grep -E "rootkit|suspicious|hidden" || echo "  ✓ No obviously suspicious modules"
sleep 2

# Test 9: Process comparison (hidden process detection)
echo ""
echo "[Test 9] Process enumeration comparison"
echo "  /proc method: $(ls /proc | grep -E '^[0-9]+$' | wc -l) processes"
echo "  ps method:    $(ps aux | wc -l) processes"
sleep 2

# Cleanup
echo ""
echo "[Cleanup] Removing hidden files..."
rm -rf /tmp/.hidden_rootkit 2>/dev/null

echo ""
echo "✅ Rootkit simulation completed"
echo ""
echo "🚨 Expected EDR Detections:"
echo "   - CRITICAL: Hidden file in suspicious location"
echo "   - MEDIUM: Kernel module enumeration"
echo "   - HIGH: Process name obfuscation"
echo "   - MEDIUM: Memory region inspection"
