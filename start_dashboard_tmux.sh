#!/bin/bash
# Simple Real-Time Monitor (Single Terminal with tmux)
# Alternative to multi-terminal dashboard

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo "❌ tmux not installed. Install with: sudo apt install tmux"
    exit 1
fi

# Create directories
sudo mkdir -p /var/log/rust-edr/archives/{threats,snapshots,sessions}
sudo mkdir -p /var/lib/rust-edr

# Kill existing session if any
tmux kill-session -t edr-dashboard 2>/dev/null

echo "🛡️  Starting Rust EDR Dashboard (tmux)"
echo "========================================"
echo ""
echo "Layout:"
echo "  ┌─────────────────┬─────────────────┐"
echo "  │   Main EDR      │  Threat Alerts  │"
echo "  ├─────────────────┼─────────────────┤"
echo "  │  File Monitor   │ Process Monitor │"
echo "  ├─────────────────┼─────────────────┤"
echo "  │ Network Monitor │  User Monitor   │"
echo "  └─────────────────┴─────────────────┘"
echo ""
echo "Controls:"
echo "  Ctrl+B then Arrow Keys - Navigate panes"
echo "  Ctrl+B then D - Detach (keep running)"
echo "  Ctrl+B then [ - Scroll mode (q to exit)"
echo "  tmux attach -t edr-dashboard - Reattach"
echo ""
read -p "Press Enter to start..."

# Create new tmux session
tmux new-session -d -s edr-dashboard

# Split into 6 panes
tmux split-window -h
tmux split-window -v
tmux select-pane -t 0
tmux split-window -v
tmux select-pane -t 2
tmux split-window -v
tmux select-pane -t 4
tmux split-window -v

# Pane 0: Main EDR System
tmux select-pane -t 0
tmux send-keys "cd /home/rubberpirate/rust-edr" C-m
tmux send-keys "echo '🛡️  MAIN EDR SYSTEM'; echo '=================='; sleep 2" C-m
tmux send-keys "sudo ./target/release/rust-edr start --verbose" C-m

# Pane 1: Threat Alerts
tmux select-pane -t 1
tmux send-keys "echo '🚨 THREAT ALERTS'; echo '==============='; echo 'Waiting for threats...'; echo ''" C-m
tmux send-keys "tail -f /var/log/rust-edr/threats_*.jsonl 2>/dev/null | jq -r '\"[\" + .timestamp + \"] 🚨 \" + .threat_type + \" | Score: \" + (.score | tostring) + \" | \" + .severity' || tail -f /var/log/rust-edr/threats_*.jsonl" C-m

# Pane 2: File Monitor
tmux select-pane -t 2
tmux send-keys "echo '📁 FILE MONITOR'; echo '==============='; echo 'Watching files...'; echo ''" C-m
tmux send-keys "tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'File' | jq -r '\"[\" + (.timestamp | split(\"T\")[1] | split(\".\")[0]) + \"] 📁 \" + .details.File.operation + \" - \" + .details.File.path' || tail -f /var/log/rust-edr/events_*.jsonl | grep File" C-m

# Pane 3: Process Monitor
tmux select-pane -t 3
tmux send-keys "echo '⚙️  PROCESS MONITOR'; echo '=================='; echo 'Watching processes...'; echo ''" C-m
tmux send-keys "tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'Process' | jq -r '\"[\" + (.timestamp | split(\"T\")[1] | split(\".\")[0]) + \"] ⚙️  \" + .event_type + \" | PID:\" + (.details.Process.pid | tostring) + \" | \" + .details.Process.name' || tail -f /var/log/rust-edr/events_*.jsonl | grep Process" C-m

# Pane 4: Network Monitor
tmux select-pane -t 4
tmux send-keys "echo '🌐 NETWORK MONITOR'; echo '=================='; echo 'Watching connections...'; echo ''" C-m
tmux send-keys "tail -f /var/log/rust-edr/events_*.jsonl 2>/dev/null | grep 'Network' | jq -r '\"[\" + (.timestamp | split(\"T\")[1] | split(\".\")[0]) + \"] 🌐 \" + .details.Network.local_addr + \":\" + (.details.Network.local_port | tostring) + \" → \" + .details.Network.remote_addr + \":\" + (.details.Network.remote_port | tostring)' || tail -f /var/log/rust-edr/events_*.jsonl | grep Network" C-m

# Pane 5: User & Response Monitor
tmux select-pane -t 5
tmux send-keys "echo '👤 USER & RESPONSES'; echo '=================='; echo 'Watching activity...'; echo ''" C-m
tmux send-keys "tail -f /var/log/rust-edr/events_*.jsonl /var/log/rust-edr/responses_*.jsonl 2>/dev/null | grep -E 'User|Response' | jq -r 'if .details.User then \"[\" + (.timestamp | split(\"T\")[1] | split(\".\")[0]) + \"] 👤 \" + .details.User.username + \" - \" + .details.User.action elif .action then \"[\" + (.timestamp | split(\"T\")[1] | split(\".\")[0]) + \"] ⚡ \" + .action + \" - \" + .message else . end' || tail -f /var/log/rust-edr/events_*.jsonl /var/log/rust-edr/responses_*.jsonl" C-m

# Attach to session
tmux attach -t edr-dashboard
