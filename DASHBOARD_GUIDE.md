# 🖥️ Real-Time Monitoring Dashboard Guide

## 🎯 Overview

The Rust EDR now includes **two dashboard options** for real-time monitoring:

1. **Multi-Terminal Dashboard** - Spawns 8 separate terminal windows
2. **Tmux Dashboard** - Single terminal with 6 panes (recommended)

---

## 🚀 Quick Start

### Option 1: Multi-Terminal Dashboard (Requires GUI)

```bash
./start_dashboard.sh
```

**What it does:**
- Spawns 8 separate terminal windows
- Each shows a different monitoring aspect
- Best for multi-monitor setups
- Requires: `gnome-terminal`, `xterm`, `konsole`, or `xfce4-terminal`

**Terminal Layout:**
1. 🛡️  **Main EDR System** - Complete system logs
2. 🚨 **Threat Detection** - Only detected threats
3. 📁 **File Monitor** - File create/modify/delete in real-time
4. ⚙️  **Process Monitor** - Process spawn/kill events
5. 🌐 **Network Monitor** - Network connections (in/out)
6. 👤 **User Monitor** - Login/logout, privilege escalation
7. ⚡ **Response Engine** - Automated response actions
8. 🔬 **Forensics** - Investigation sessions created

### Option 2: Tmux Dashboard (Recommended)

```bash
./start_dashboard_tmux.sh
```

**What it does:**
- Creates single terminal split into 6 panes
- All monitors visible at once
- Easy to navigate with keyboard
- Works over SSH
- Requires: `tmux` (install with `sudo apt install tmux`)

**Pane Layout:**
```
┌─────────────────┬─────────────────┐
│   Main EDR      │  Threat Alerts  │
│   (all logs)    │  (threats only) │
├─────────────────┼─────────────────┤
│  File Monitor   │ Process Monitor │
│  (file ops)     │  (proc events)  │
├─────────────────┼─────────────────┤
│ Network Monitor │ User & Response │
│  (connections)  │ (user + actions)│
└─────────────────┴─────────────────┘
```

---

## 🎮 Tmux Dashboard Controls

### Navigation
- **Ctrl+B** then **Arrow Keys** - Move between panes
- **Ctrl+B** then **Z** - Zoom current pane (fullscreen)
- **Ctrl+B** then **[** - Scroll mode (press Q to exit)

### Session Management
- **Ctrl+B** then **D** - Detach (keep running in background)
- `tmux attach -t edr-dashboard` - Reattach to dashboard
- `tmux kill-session -t edr-dashboard` - Stop dashboard

### Useful Commands
- **Ctrl+B** then **?** - Show all keybindings
- **Ctrl+B** then **:** - Enter tmux command mode

---

## 📊 What Each Monitor Shows

### 1. Main EDR System (Pane 0)
```
🛡️  Starting Rust EDR System...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Threat threshold: 7.0
Auto-response: disabled
Enabled modules: process,file,network,memory,user,rootkit
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting process monitor...
Starting file monitor...
✅ EDR System running.
```

### 2. Threat Alerts (Pane 1)
```
🚨 THREAT ALERTS
===============
[15:30:25] 🚨 SuspiciousProcess | Score: 8.5 | High
[15:31:10] 🚨 RansomwareBehavior | Score: 10.0 | Critical
[15:32:05] 🚨 DataExfiltration | Score: 7.8 | High
```

### 3. File Monitor (Pane 2)
```
📁 FILE MONITOR
===============
[15:30:12] 📁 created - /tmp/suspicious.sh
[15:30:15] 📁 modified - /etc/passwd
[15:30:18] 📁 deleted - /tmp/test.txt
[15:30:25] 📁 accessed - /home/user/.ssh/id_rsa
```

### 4. Process Monitor (Pane 3)
```
⚙️  PROCESS MONITOR
==================
[15:30:10] ⚙️  ProcessCreated | PID:1234 | /tmp/malicious.sh
[15:30:15] ⚙️  ProcessCreated | PID:1235 | bash
[15:30:20] ⚙️  ProcessTerminated | PID:1234 | /tmp/malicious.sh
```

### 5. Network Monitor (Pane 4)
```
🌐 NETWORK MONITOR
==================
[15:30:05] 🌐 192.168.1.100:54321 → 45.33.32.156:4444
[15:30:10] 🌐 192.168.1.100:55123 → 8.8.8.8:443
[15:30:15] 🌐 192.168.1.100:22 → 192.168.1.50:51234
```

### 6. User & Response Monitor (Pane 5)
```
👤 USER & RESPONSES
==================
[15:30:01] 👤 john - login
[15:30:10] 👤 john - sudo command executed
[15:30:15] ⚡ Alert - SuspiciousProcess threat detected
[15:30:16] ⚡ Block - Network connection blocked
```

---

## 🎬 Complete Demo Workflow

### Terminal 1: Start Dashboard
```bash
cd /home/rubberpirate/rust-edr
./start_dashboard_tmux.sh
```

### Terminal 2: Run Tests (in another terminal)
```bash
cd /home/rubberpirate/rust-edr
./tests/run_malicious_tests.sh
```

### Watch in Real-Time
- **Main pane**: See EDR processing events
- **Threat pane**: Alerts pop up as threats detected
- **File pane**: File operations stream by
- **Process pane**: Process spawns/kills shown
- **Network pane**: Connections displayed
- **User pane**: Responses executed

---

## 🔧 Troubleshooting

### Issue: "No such file or directory" when saving snapshots

**Solution:**
```bash
sudo mkdir -p /var/log/rust-edr/archives/{threats,snapshots,sessions}
sudo chown -R $USER:$USER /var/log/rust-edr
```

### Issue: Multi-terminal dashboard doesn't spawn

**Solution:** Install a terminal emulator:
```bash
sudo apt install gnome-terminal  # Ubuntu/Debian
# OR
sudo apt install xterm          # Lightweight alternative
```

### Issue: Tmux dashboard shows "command not found"

**Solution:** Install tmux:
```bash
sudo apt install tmux
```

### Issue: No logs appearing in panes

**Solution:** Wait a few seconds for log files to be created, or trigger an event:
```bash
echo '#!/bin/bash' > /tmp/test.sh
chmod +x /tmp/test.sh
sudo /tmp/test.sh
```

### Issue: Permission denied errors

**Solution:** Run EDR with sudo (it needs root for system monitoring):
```bash
# Stop existing dashboard
tmux kill-session -t edr-dashboard

# Restart
./start_dashboard_tmux.sh
# (It will prompt for sudo password)
```

---

## 🎨 Customization

### Change Log Filters

Edit `start_dashboard_tmux.sh` to customize what each pane shows:

```bash
# Show only CRITICAL threats
tail -f /var/log/rust-edr/threats_*.jsonl | jq 'select(.severity == "Critical")'

# Show only network connections to port 4444
tail -f /var/log/rust-edr/events_*.jsonl | jq 'select(.details.Network.remote_port == 4444)'

# Show only file modifications (not creates)
tail -f /var/log/rust-edr/events_*.jsonl | jq 'select(.details.File.operation == "modify")'
```

### Add More Panes

```bash
# In tmux, split current pane horizontally
Ctrl+B then "

# In tmux, split current pane vertically
Ctrl+B then %
```

---

## 📊 Performance Tips

### For Low-Resource Systems

Use minimal monitoring:
```bash
# Edit start_dashboard_tmux.sh
# Change the main EDR command to:
sudo ./target/release/rust-edr start --modules process,file --threshold 8.0
```

### For High-Activity Systems

Reduce log noise by increasing threshold:
```bash
sudo ./target/release/rust-edr start --threshold 9.0
```

### Limit Log File Size

```bash
# Rotate logs daily
sudo crontab -e

# Add this line:
0 0 * * * mv /var/log/rust-edr/events_*.jsonl /var/log/rust-edr/archives/
```

---

## 🎯 Real-World Usage Examples

### Example 1: Ransomware Detection Demo

**Terminal 1 - Dashboard:**
```bash
./start_dashboard_tmux.sh
```

**Terminal 2 - Trigger:**
```bash
./tests/malicious_samples/ransomware_simulator.sh
```

**Watch:**
- **File pane**: Rapid file modifications streaming
- **Threat pane**: RansomwareBehavior alert (Score 10.0)
- **Response pane**: Alert + Quarantine actions
- **Main pane**: Forensic snapshot captured

### Example 2: Privilege Escalation Detection

**Terminal 1 - Dashboard:**
```bash
./start_dashboard_tmux.sh
```

**Terminal 2 - Trigger:**
```bash
./tests/malicious_samples/privilege_escalation_simulator.sh
```

**Watch:**
- **User pane**: sudo command executed
- **File pane**: /etc/shadow access attempt
- **Process pane**: Root process spawn
- **Threat pane**: PrivilegeEscalationChain correlation

### Example 3: Network Exfiltration

**Terminal 1 - Dashboard:**
```bash
./start_dashboard_tmux.sh
```

**Terminal 2 - Trigger:**
```bash
./tests/malicious_samples/data_exfiltration_simulator.sh
```

**Watch:**
- **File pane**: Sensitive file accesses
- **Network pane**: Connection to suspicious port (4444)
- **Threat pane**: DataExfiltration correlation
- **Main pane**: Investigation shell created

---

## 📚 Advanced Features

### Background Monitoring

Start dashboard in background:
```bash
./start_dashboard_tmux.sh
# Press Ctrl+B then D to detach

# Continue working in terminal
# Dashboard keeps running

# Reattach anytime:
tmux attach -t edr-dashboard
```

### Remote Monitoring (SSH)

```bash
# On remote server
ssh user@edr-server
cd rust-edr
./start_dashboard_tmux.sh

# Detach with Ctrl+B, D
# Logout

# Reconnect later
ssh user@edr-server
tmux attach -t edr-dashboard
```

### Multi-User Monitoring

```bash
# User 1 starts dashboard
./start_dashboard_tmux.sh
Ctrl+B, D  # Detach

# User 2 (or same user in different session) attaches read-only
tmux attach -t edr-dashboard -r  # -r = read-only
```

### Recording Sessions

```bash
# Start recording
tmux pipe-pane -o 'cat >> /tmp/edr-session-$(date +%Y%m%d_%H%M%S).log'

# Stop recording
tmux pipe-pane
```

---

## 🎓 Dashboard Benefits

### For Development
✅ **Instant feedback** - See detection logic in action  
✅ **Debug visibility** - Spot issues immediately  
✅ **Performance monitoring** - Watch system load  

### For Demonstrations
✅ **Visual impact** - Multiple monitors show activity  
✅ **Real-time detection** - Threats appear instantly  
✅ **Professional presentation** - Organized layout  

### For Operations
✅ **24/7 monitoring** - Runs in tmux background  
✅ **Remote access** - Works over SSH  
✅ **Low overhead** - Text-based, lightweight  

---

## 🚀 Quick Reference

### Start Dashboards
```bash
./start_dashboard.sh        # Multi-terminal (8 windows)
./start_dashboard_tmux.sh   # Tmux (6 panes) ⭐ Recommended
```

### Tmux Shortcuts
```bash
Ctrl+B → Arrow  # Navigate panes
Ctrl+B → Z      # Zoom pane
Ctrl+B → [      # Scroll mode
Ctrl+B → D      # Detach
Ctrl+B → ?      # Help
```

### Common Tasks
```bash
# Create directories
sudo mkdir -p /var/log/rust-edr/archives/{threats,snapshots,sessions}

# Reattach to dashboard
tmux attach -t edr-dashboard

# Stop dashboard
tmux kill-session -t edr-dashboard

# Run tests
./tests/run_malicious_tests.sh

# View investigation shells
ls /var/log/rust-edr/archives/sessions/
```

---

## 📝 Summary

**Two dashboard options:**

1. **Multi-Terminal** (`start_dashboard.sh`)
   - 8 separate windows
   - Best for multi-monitor setups
   - Requires GUI terminal emulator

2. **Tmux** (`start_dashboard_tmux.sh`) ⭐
   - 6 panes in one terminal
   - Works over SSH
   - Professional, organized layout
   - Easy navigation
   - Can run in background

**What you see in real-time:**
- 📁 File operations (create, modify, delete)
- ⚙️  Process events (spawn, kill)
- 🌐 Network connections (in, out)
- 👤 User activity (login, sudo)
- 🚨 Threat alerts (as detected)
- ⚡ Response actions (automated)
- 🔬 Forensics (snapshots, shells)

**Perfect for:**
- Live demonstrations
- Development/debugging
- Security operations
- Training sessions
- System monitoring

---

**🖥️ Happy Monitoring! 🛡️**
