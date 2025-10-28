# 🔄 Complete EDR Workflow with Forensics

## 📊 Visual Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    1. THREAT DETECTION                          │
│                                                                 │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐│
│  │ Process  │    │   File   │    │ Network  │    │  Memory  ││
│  │ Monitor  │    │ Monitor  │    │ Monitor  │    │ Monitor  ││
│  └────┬─────┘    └────┬─────┘    └────┬─────┘    └────┬─────┘│
│       │               │               │               │       │
│       └───────────────┴───────────────┴───────────────┘       │
│                           │                                    │
│                           ▼                                    │
│                  ┌─────────────────┐                          │
│                  │ Detection Engine│                          │
│                  │  - 10 Rules     │                          │
│                  │  - 5 Patterns   │                          │
│                  │  - IOC Matcher  │                          │
│                  └────────┬────────┘                          │
└─────────────────────────┬─┬────────────────────────────────────┘
                          │ │
      ┌───────────────────┘ └───────────────────┐
      │                                         │
      ▼                                         ▼
┌─────────────────────────────────────┐ ┌──────────────────────┐
│   2. THREAT SCORING & CLASSIFICATION│ │  3. AUTOMATIC LOGGING│
│                                      │ │                      │
│  Score: 0-10                         │ │  threats_*.jsonl    │
│  Severity: Info/Low/Medium/High/Crit │ │  events_*.jsonl     │
│  IOC Matches: [...]                  │ │  responses_*.jsonl  │
│  Rule Matches: [...]                 │ │                      │
└──────────────┬───────────────────────┘ └──────────────────────┘
               │
               ▼
┌────────────────────────────────────────────────────────────────┐
│              4. RESPONSE ENGINE (Auto or Manual)               │
│                                                                │
│  ┌──────┐  ┌──────┐  ┌───────────┐  ┌──────┐  ┌────────────┐│
│  │Alert │  │Block │  │Quarantine │  │ Kill │  │  Isolate   ││
│  └──────┘  └──────┘  └───────────┘  └──────┘  └────────────┘│
└────────────────────────┬───────────────────────────────────────┘
                         │
                         ▼
┌────────────────────────────────────────────────────────────────┐
│         5. 🆕 AUTOMATIC FORENSICS CAPTURE                      │
│            (HIGH/CRITICAL threats only)                        │
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Forensic Snapshot Captured:                             │ │
│  │  ✓ All processes (PID, name, cmdline, UID, memory)     │ │
│  │  ✓ Network connections (TCP/UDP, addresses, ports)     │ │
│  │  ✓ Open files (first 100 for performance)              │ │
│  │  ✓ Memory maps (showing RWX permissions)               │ │
│  │  ✓ System info (kernel, uptime, load average)          │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Investigation Shell Created:                            │ │
│  │  ✓ Interactive bash script generated                    │ │
│  │  ✓ Threat context embedded                              │ │
│  │  ✓ Helper commands: info, snapshot, logs, archive      │ │
│  │  ✓ Colorized, formatted output                          │ │
│  └─────────────────────────────────────────────────────────┘ │
└────────────────────────┬───────────────────────────────────────┘
                         │
                         ▼
┌────────────────────────────────────────────────────────────────┐
│       6. 🆕 INVESTIGATION PHASE (Analyst-Driven)               │
│                                                                │
│  Analyst runs:                                                 │
│  $ bash /var/log/rust-edr/archives/sessions/investigation_*/  │
│         investigate.sh                                         │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ [edr-investigate] info                                    ││
│  │ ═══════════════════════════════════════════════          ││
│  │   RUST EDR - THREAT INVESTIGATION SESSION                ││
│  │ ═══════════════════════════════════════════════          ││
│  │ Threat ID: threat_abc123                                 ││
│  │ Type: SuspiciousProcess                                  ││
│  │ Severity: High                                           ││
│  │ Score: 8.50                                              ││
│  │ ...                                                      ││
│  └──────────────────────────────────────────────────────────┘│
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ [edr-investigate] snapshot                                ││
│  │ 📸 Capturing system snapshot...                          ││
│  │ ✅ Snapshot saved: snapshot_20251028_153045.txt          ││
│  └──────────────────────────────────────────────────────────┘│
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ [edr-investigate] logs                                    ││
│  │ {                                                         ││
│  │   "id": "threat_abc123",                                 ││
│  │   "threat_type": "SuspiciousProcess",                    ││
│  │   "severity": "High",                                    ││
│  │   ...                                                     ││
│  │ }                                                         ││
│  └──────────────────────────────────────────────────────────┘│
└────────────────────────┬───────────────────────────────────────┘
                         │
                         ▼
┌────────────────────────────────────────────────────────────────┐
│        7. 🆕 ARCHIVAL & LONG-TERM STORAGE                      │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐│
│  │ [edr-investigate] archive                                 ││
│  │ 🗜️ Archiving session...                                   ││
│  │ ✅ Session archived: investigation_threat_abc123.tar.gz  ││
│  └──────────────────────────────────────────────────────────┘│
│                                                                │
│  OR use CLI:                                                   │
│  $ sudo ./target/release/rust-edr forensics archive <id>      │
│  $ sudo ./target/release/rust-edr forensics compress --days 7 │
│                                                                │
│  Archive contains:                                             │
│   ├─ session_info.txt                                         │
│   ├─ investigate.sh                                           │
│   ├─ snapshot.json                                            │
│   ├─ snapshot.txt                                             │
│   └─ all related log entries                                  │
│                                                                │
│  Compression: 70-90% size reduction (gzip)                    │
│  Retention: 90 days default (configurable)                    │
└────────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌────────────────────────────────────────────────────────────────┐
│         8. 🆕 ANALYSIS & REPORTING                             │
│                                                                │
│  Extract archive on any system:                                │
│  $ tar -xzf threat_abc123_20251028.tar.gz                     │
│                                                                │
│  Analyze with standard tools:                                  │
│  $ cat investigation_*/session_info.txt                        │
│  $ jq . investigation_*/snapshot.json                          │
│  $ jq '.processes[] | select(.uid == 0)' snapshot.json        │
│                                                                │
│  Share with team:                                              │
│  $ scp threat_abc123.tar.gz analyst@soc:~/incoming/           │
│                                                                │
│  Generate reports:                                             │
│  $ sudo ./target/release/rust-edr forensics list              │
│  $ sudo ./target/release/rust-edr forensics cleanup --days 90 │
└────────────────────────────────────────────────────────────────┘
```

---

## 🔄 Data Flow Summary

```
System Activity
    ↓
Monitoring Agents (process, file, network, memory, user, rootkit)
    ↓
SystemEvent objects → mpsc channel
    ↓
Detection Engine
├─→ IOC Matcher
├─→ Rule Engine (10 rules)
├─→ Correlator (5 patterns)
└─→ Threat Scorer
    ↓
Threat object (score, severity, IOCs, rules)
    ↓
Response Engine
├─→ Alert (always)
├─→ Block (if auto-response + threshold met)
├─→ Quarantine (HIGH severity)
├─→ Kill (CRITICAL severity)
└─→ 🆕 Capture Forensics (HIGH/CRITICAL)
         ↓
    ┌────┴────┐
    │         │
    ▼         ▼
Snapshot   Investigation Shell
(JSON/TXT)  (interactive bash)
    │         │
    └────┬────┘
         ▼
Session Directory
(/var/log/rust-edr/archives/sessions/investigation_<id>/)
         ↓
Manual Investigation
(analyst runs investigate.sh)
         ↓
Archive Session
(tar.gz compression)
         ↓
Long-term Storage
(/var/log/rust-edr/archives/threats/)
         ↓
Analysis & Reporting
(extract, review, share)
```

---

## 📊 File Organization

```
/var/log/rust-edr/
│
├── events_20251028.jsonl          ← All system events (today)
├── threats_20251028.jsonl         ← Detected threats (today)
├── responses_20251028.jsonl       ← Response actions (today)
│
├── archives/
│   │
│   ├── threats/                   ← Compressed threat sessions
│   │   ├── threat_abc123_20251028_153000.tar.gz
│   │   └── threat_def456_20251028_140000.tar.gz
│   │
│   ├── snapshots/                 ← Forensic snapshots
│   │   ├── snapshot_threat_abc123.json  (machine-readable)
│   │   └── snapshot_threat_abc123.txt   (human-readable)
│   │
│   ├── sessions/                  ← Active investigation sessions
│   │   └── investigation_threat_abc123/
│   │       ├── session_info.txt         (threat details)
│   │       ├── investigate.sh           (interactive shell - executable)
│   │       ├── snapshot.json            (initial snapshot)
│   │       ├── snapshot.txt             (initial snapshot)
│   │       └── snapshot_20251028_*.txt  (additional snapshots)
│   │
│   └── *.jsonl.gz                 ← Compressed old logs
│       ├── events_20251020.jsonl.gz
│       └── threats_20251020.jsonl.gz
│
└── /var/lib/rust-edr/
    └── events.db                  ← Persistent sled database
```

---

## 🎯 CLI Command Tree

```
rust-edr
├── start                         Start EDR agent
│   ├── --verbose                 Enable verbose logging
│   ├── --threshold <score>       Set threat threshold (0-10)
│   ├── --auto-response           Enable automated responses
│   └── --modules <list>          Select monitoring modules
│
├── stop                          Stop EDR agent
│
├── status                        Check agent status
│
├── alerts                        View recent threats
│   └── --recent <n>              Number of recent alerts
│
├── config                        Configuration management
│   └── --show                    Show current config
│
└── forensics                     🆕 Forensics tools
    ├── archive <threat-id>       Archive a threat session
    ├── list                      List all archives
    ├── extract <archive>         Extract an archive
    │   └── --output <dir>        Extraction directory
    ├── compress                  Compress old logs
    │   └── --days <n>            Logs older than N days
    ├── cleanup                   Remove old archives
    │   └── --days <n>            Archives older than N days
    └── snapshot <threat-id>      Capture manual snapshot
```

---

## 🔐 Security & Compliance

### Evidence Chain

1. **Detection** → Event logged with timestamp, source, severity
2. **Correlation** → Multiple events linked by time window + pattern
3. **Threat** → Scored, classified, IOCs/rules recorded
4. **Response** → Actions logged with success/failure
5. **Forensics** → Complete system state captured
6. **Investigation** → Analyst actions recorded in session
7. **Archive** → Compressed, immutable record created
8. **Retention** → Stored per policy (90 days default)

### Audit Trail

Every step is logged:
- System events → `events_*.jsonl`
- Detected threats → `threats_*.jsonl`
- Response actions → `responses_*.jsonl`
- Forensic snapshots → `snapshots/*.json`
- Investigation sessions → `sessions/investigation_*/`
- Archived sessions → `threats/*.tar.gz`

### Compliance Features

✅ Tamper-evident (logs are append-only JSONL)  
✅ Complete audit trail (all actions logged)  
✅ Long-term retention (compressed archives)  
✅ Exportable format (standard JSON + TXT)  
✅ Chain of custody (timestamps, analyst actions)  

---

## 📈 Performance Characteristics

### Resource Usage

| Component | CPU | Memory | Disk I/O |
|-----------|-----|--------|----------|
| Process Monitor | Low (2s polling) | ~5 MB | Low |
| File Monitor | Medium (inotify) | ~10 MB | Medium |
| Network Monitor | Low (10s polling) | ~5 MB | Low |
| Detection Engine | Low | ~20 MB | Low |
| Forensics Capture | High (spike) | ~50 MB | High (spike) |

### Storage

| Data Type | Size per Day | Compressed | Retention |
|-----------|--------------|------------|-----------|
| Events | 50-100 MB | 5-10 MB | 7 days |
| Threats | 1-10 MB | 100 KB - 1 MB | 30 days |
| Snapshots | 500 KB - 2 MB each | 100-500 KB | 90 days |
| Archives | Varies | 70-90% reduction | 90 days |

---

## 🎓 Educational Value

### Demonstrates

1. **Systems Programming**
   - Process management (procfs)
   - File system monitoring (inotify)
   - Network analysis (/proc/net/tcp)
   - Memory inspection

2. **Async Rust**
   - Tokio runtime
   - mpsc channels
   - Concurrent monitoring agents
   - Background tasks

3. **Security Concepts**
   - Behavioral detection
   - Event correlation
   - Threat scoring
   - Incident response

4. **Forensics**
   - Evidence preservation
   - System state capture
   - Timeline reconstruction
   - Archive management

5. **Software Engineering**
   - Modular architecture
   - Error handling (anyhow, Result)
   - CLI design (clap)
   - Documentation

---

**🔬 Complete EDR Platform with Forensics! 🛡️**
