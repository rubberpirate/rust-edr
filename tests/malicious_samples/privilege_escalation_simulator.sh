#!/bin/bash
# Privilege Escalation Simulator
# Simulates various privilege escalation techniques

echo "⬆️  Privilege Escalation Simulator"
echo "=================================="
echo ""

current_user=$(whoami)
echo "Current user: $current_user"
echo ""

# Test 1: Sudo escalation
echo "[Test 1] Sudo privilege escalation"
echo "Attempting: sudo whoami"
sudo whoami 2>/dev/null || echo "  ❌ Sudo not available or password required"
sleep 2

# Test 2: Su escalation attempt
echo ""
echo "[Test 2] Su privilege escalation attempt"
echo "Simulating: su - root"
echo "  (not actually executing - would need password)"
sleep 2

# Test 3: SUID binary enumeration (recon for privilege escalation)
echo ""
echo "[Test 3] SUID binary enumeration (recon)"
echo "Searching for SUID binaries..."
find /usr/bin /usr/sbin -perm -4000 -type f 2>/dev/null | head -5
sleep 2

# Test 4: Sudoers file read attempt
echo ""
echo "[Test 4] Sudoers file access attempt"
sudo cat /etc/sudoers 2>/dev/null | head -5 || echo "  ❌ No access to sudoers"
sleep 2

# Test 5: Password file access
echo ""
echo "[Test 5] Password file access"
cat /etc/passwd | grep -E "root|$current_user"
sleep 2

# Test 6: Shadow file access attempt
echo ""
echo "[Test 6] Shadow file access attempt"
sudo cat /etc/shadow 2>/dev/null | head -3 || echo "  ❌ No access to shadow file"
sleep 2

# Test 7: Kernel exploit check
echo ""
echo "[Test 7] Kernel version check (exploit recon)"
uname -a
sleep 2

# Test 8: Capabilities check
echo ""
echo "[Test 8] Checking file capabilities"
getcap /usr/bin/* 2>/dev/null | head -5 || echo "  ℹ️  No special capabilities found"
sleep 2

echo ""
echo "✅ Privilege escalation simulation completed"
echo ""
echo "🚨 Expected EDR Detections:"
echo "   - HIGH: Privilege escalation (sudo/su usage)"
echo "   - CRITICAL: /etc/shadow access"
echo "   - HIGH: /etc/passwd access"
echo "   - MEDIUM: SUID binary enumeration"
