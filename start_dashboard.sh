#!/bin/bash
# Real-Time EDR Monitoring Dashboard
# Spawns separate terminals for each monitoring type

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🛡️  Starting Rust EDR Multi-Terminal Dashboard${NC}"
echo "=================================================="

# Create log directory if it doesn't exist
sudo mkdir -p /var/log/rust-edr/archives/{threats,snapshots,sessions}
sudo mkdir -p /var/lib/rust-edr

# Function to spawn terminal
spawn_terminal() {
    local title="$1"
    local command="$2"
    
    # Try different terminal emulators
    if command -v gnome-terminal &> /dev/null; then
        gnome-terminal --title="$title" -- bash -c "$command; exec bash" &
    elif command -v xterm &> /dev/null; then
        xterm -T "$title" -e bash -c "$command; exec bash" &
    elif command -v konsole &> /dev/null; then
        konsole --title "$title" -e bash -c "$command; exec bash" &
    elif command -v xfce4-terminal &> /dev/null; then
        xfce4-terminal --title="$title" -e "bash -c '$command; exec bash'" &
    else
        echo "⚠️  No terminal emulator found. Run commands manually."
        return 1
    fi
    
    echo -e "${BLUE}✓${NC} Spawned: $title"
    sleep 0.5
}

echo ""
echo -e "${YELLOW}Spawning monitoring terminals...${NC}"
echo ""

# 1. Main EDR System Terminal
spawn_terminal "🛡️ EDR Main System" \
"echo '🛡️  RUST EDR - MAIN SYSTEM'; \
echo '================================'; \
echo 'Starting EDR with all monitors...'; \
echo ''; \
cd /home/rubberpirate/rust-edr && \
sudo ./target/release/rust-edr start --verbose"

sleep 2

# 2. Threat Detection Terminal
spawn_terminal "🚨 Threat Detection" \
"echo '🚨 THREAT DETECTION MONITOR'; \
echo '================================'; \
echo 'Watching for detected threats...'; \
echo ''; \
tail -f /var/log/rust-edr/threats_*.jsonl 2>/dev/null | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .threat_type + \" - Score: \" + (.score | tostring) + \" - \" + .severity + \" - \" + .description' 2>/dev/null || echo \"\$line\"
done"

# 3. File Monitoring Terminal
spawn_terminal "📁 File Monitor" \
"echo '📁 FILE SYSTEM MONITOR'; \
echo '================================'; \
echo 'Watching file operations...'; \
echo ''; \
tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'File' | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .event_type + \" - \" + .details.File.path + \" (\" + .details.File.operation + \")\"' 2>/dev/null || echo \"\$line\"
done"

# 4. Process Monitoring Terminal
spawn_terminal "⚙️ Process Monitor" \
"echo '⚙️  PROCESS MONITOR'; \
echo '================================'; \
echo 'Watching process activity...'; \
echo ''; \
tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'Process' | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .event_type + \" - PID:\" + (.details.Process.pid | tostring) + \" \" + .details.Process.name + \" (UID:\" + (.details.Process.uid | tostring) + \")\"' 2>/dev/null || echo \"\$line\"
done"

# 5. Network Monitoring Terminal  
spawn_terminal "🌐 Network Monitor" \
"echo '🌐 NETWORK MONITOR'; \
echo '================================'; \
echo 'Watching network connections...'; \
echo ''; \
tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'Network' | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .event_type + \" - \" + .details.Network.local_addr + \":\" + (.details.Network.local_port | tostring) + \" -> \" + .details.Network.remote_addr + \":\" + (.details.Network.remote_port | tostring)' 2>/dev/null || echo \"\$line\"
done"

# 6. User Activity Monitor
spawn_terminal "👤 User Monitor" \
"echo '👤 USER ACTIVITY MONITOR'; \
echo '================================'; \
echo 'Watching user actions...'; \
echo ''; \
tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'User' | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .event_type + \" - User:\" + .details.User.username + \" (UID:\" + (.details.User.uid | tostring) + \") - \" + .details.User.action' 2>/dev/null || echo \"\$line\"
done"

# 7. Response Actions Monitor
spawn_terminal "⚡ Response Engine" \
"echo '⚡ RESPONSE ENGINE'; \
echo '================================'; \
echo 'Watching automated responses...'; \
echo ''; \
tail -f /var/log/rust-edr/responses_*.jsonl 2>/dev/null | while read line; do
    echo \"\$line\" | jq -r '\"[\" + .timestamp + \"] \" + .action + \" - \" + .message + \" (Success: \" + (.success | tostring) + \")\"' 2>/dev/null || echo \"\$line\"
done"

# 8. Forensics & Snapshots Monitor
spawn_terminal "🔬 Forensics" \
"echo '🔬 FORENSICS & INVESTIGATION'; \
echo '================================'; \
echo 'Investigation sessions created:'; \
echo ''; \
while true; do
    ls -lt /var/log/rust-edr/archives/sessions/ 2>/dev/null | head -20
    echo ''
    echo 'Watching for new snapshots...'
    sleep 5
done"

echo ""
echo -e "${GREEN}✅ All monitoring terminals spawned!${NC}"
echo ""
echo -e "${YELLOW}Dashboard Layout:${NC}"
echo "  1. 🛡️  Main EDR System (all logs)"
echo "  2. 🚨 Threat Detection (threats only)"
echo "  3. 📁 File Monitor (file operations)"
echo "  4. ⚙️  Process Monitor (process activity)"
echo "  5. 🌐 Network Monitor (connections)"
echo "  6. 👤 User Monitor (user actions)"
echo "  7. ⚡ Response Engine (automated responses)"
echo "  8. 🔬 Forensics (investigation sessions)"
echo ""
echo -e "${BLUE}Tip: Arrange terminals in a grid for dashboard view${NC}"
echo ""
echo "Press Enter to continue or Ctrl+C to exit..."
read

echo ""
echo "🎯 Now run test scenarios in a new terminal:"
echo "   ./tests/run_malicious_tests.sh"
echo ""
echo "Or trigger individual tests:"
echo "   ./tests/malicious_samples/ransomware_simulator.sh"
echo "   ./tests/malicious_samples/reverse_shell_simulator.sh"
echo ""
