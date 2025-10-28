#!/bin/bash
# Ransomware Behavior Simulator
# Simulates ransomware-like file operations

echo "💀 Ransomware Behavior Simulator"
echo "================================="
echo ""
echo "⚠️  This simulates ransomware file encryption patterns"
echo ""

TEST_DIR="/tmp/ransomware_simulation_$$"
mkdir -p "$TEST_DIR"

echo "📁 Creating test directory: $TEST_DIR"
echo ""

# Create many test files
echo "[Phase 1] Creating 25 victim files..."
for i in {1..25}; do
    echo "Important data file #$i" > "$TEST_DIR/document_$i.txt"
    echo "Spreadsheet data $i" > "$TEST_DIR/spreadsheet_$i.xlsx"
done
sleep 1

# Rapid file modifications (encryption simulation)
echo "[Phase 2] Encrypting files (rapid modifications)..."
for i in {1..25}; do
    # Simulate encryption by overwriting with random data
    echo "ENCRYPTED_DATA_$(date +%s%N)" > "$TEST_DIR/document_$i.txt"
    echo "LOCKED_$(date +%s%N)" > "$TEST_DIR/spreadsheet_$i.xlsx"
done
sleep 1

# Create ransom note
echo "[Phase 3] Creating ransom note..."
cat > "$TEST_DIR/README_DECRYPT.txt" << 'EOF'
YOUR FILES HAVE BEEN ENCRYPTED
===============================

All your important files have been encrypted with strong encryption.

To decrypt your files, you need to pay 1 BTC to the following address:
1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

After payment, contact: ransomware@evil.com

Your personal ID: SIMULATION-TEST-ONLY
EOF
sleep 1

# File deletion simulation
echo "[Phase 4] Deleting original backups..."
for i in {1..10}; do
    rm -f "$TEST_DIR/document_$i.txt" 2>/dev/null
done
sleep 1

echo ""
echo "✅ Ransomware simulation completed"
echo ""
echo "📊 Statistics:"
echo "   - Files created: 50"
echo "   - Files modified: 50"
echo "   - Files deleted: 10"
echo "   - Ransom note created: 1"
echo ""
echo "🚨 Expected EDR Detection:"
echo "   - CRITICAL: Ransomware behavior (rapid file modifications)"
echo "   - HIGH: Mass file operations"
echo "   - MEDIUM: Suspicious file patterns"
echo ""
echo "🧹 Cleanup: rm -rf $TEST_DIR"
