// Shell Spawner - Launch investigation shells for threat analysis
use crate::types::{Threat, SystemEvent};
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use chrono::Utc;

/// Spawn an interactive investigation shell for a detected threat
pub fn spawn_investigation_shell(threat: &Threat, session_dir: &PathBuf) -> Result<()> {
    // Create session directory
    fs::create_dir_all(session_dir)?;
    
    // Create session info file
    let info_path = session_dir.join("session_info.txt");
    create_session_info(&info_path, threat)?;
    
    // Create investigation script
    let script_path = session_dir.join("investigate.sh");
    create_investigation_script(&script_path, threat, session_dir)?;
    
    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Spawn new terminal with investigation environment
    spawn_terminal(&script_path, &threat.id)?;
    
    println!("🐚 Investigation shell spawned for threat: {}", threat.id);
    Ok(())
}

/// Create session information file
fn create_session_info(path: &PathBuf, threat: &Threat) -> Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "═══════════════════════════════════════════════")?;
    writeln!(file, "  RUST EDR - THREAT INVESTIGATION SESSION")?;
    writeln!(file, "═══════════════════════════════════════════════")?;
    writeln!(file, "Threat ID: {}", threat.id)?;
    writeln!(file, "Type: {:?}", threat.threat_type)?;
    writeln!(file, "Severity: {:?}", threat.severity)?;
    writeln!(file, "Score: {:.2}", threat.score)?;
    writeln!(file, "Timestamp: {}", threat.timestamp.to_rfc3339())?;
    writeln!(file, "═══════════════════════════════════════════════\n")?;
    
    writeln!(file, "DESCRIPTION:")?;
    writeln!(file, "{}\n", threat.description)?;
    
    if !threat.events.is_empty() {
        writeln!(file, "RELATED EVENTS ({}):", threat.events.len())?;
        for event in &threat.events {
            writeln!(file, "  - {:?}: {}", event.event_type, event.source)?;
        }
        writeln!(file)?;
    }
    
    if !threat.ioc_matches.is_empty() {
        writeln!(file, "IOC MATCHES ({}):", threat.ioc_matches.len())?;
        for ioc in &threat.ioc_matches {
            writeln!(file, "  - {}", ioc)?;
        }
        writeln!(file)?;
    }
    
    if !threat.rule_matches.is_empty() {
        writeln!(file, "RULE MATCHES ({}):", threat.rule_matches.len())?;
        for rule in &threat.rule_matches {
            writeln!(file, "  - {}", rule)?;
        }
        writeln!(file)?;
    }
    
    writeln!(file, "═══════════════════════════════════════════════")?;
    
    Ok(())
}

/// Create investigation helper script
fn create_investigation_script(
    path: &PathBuf,
    threat: &Threat,
    session_dir: &PathBuf,
) -> Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "#!/bin/bash")?;
    writeln!(file, "# Rust EDR - Threat Investigation Shell")?;
    writeln!(file, "# Threat ID: {}", threat.id)?;
    writeln!(file, "# Generated: {}\n", Utc::now().to_rfc3339())?;
    
    writeln!(file, "# Session directory")?;
    writeln!(file, "SESSION_DIR=\"{}\"\n", session_dir.display())?;
    
    writeln!(file, "# Color codes")?;
    writeln!(file, "RED='\\033[0;31m'")?;
    writeln!(file, "GREEN='\\033[0;32m'")?;
    writeln!(file, "YELLOW='\\033[1;33m'")?;
    writeln!(file, "BLUE='\\033[0;34m'")?;
    writeln!(file, "NC='\\033[0m' # No Color\n")?;
    
    writeln!(file, "# Helper functions")?;
    writeln!(file, "function banner {{")?;
    writeln!(file, "    clear")?;
    writeln!(file, "    echo -e \"${{BLUE}}═══════════════════════════════════════════════${{NC}}\"")?;
    writeln!(file, "    echo -e \"${{BLUE}}  RUST EDR - THREAT INVESTIGATION SHELL${{NC}}\"")?;
    writeln!(file, "    echo -e \"${{BLUE}}═══════════════════════════════════════════════${{NC}}\"")?;
    writeln!(file, "    echo -e \"Threat ID: ${{YELLOW}}{}${{NC}}\"", threat.id)?;
    writeln!(file, "    echo -e \"Severity: ${{RED}}{:?}${{NC}}\"", threat.severity)?;
    writeln!(file, "    echo -e \"Score: ${{RED}}{:.2}${{NC}}\"", threat.score)?;
    writeln!(file, "    echo -e \"${{BLUE}}═══════════════════════════════════════════════${{NC}}\"")?;
    writeln!(file, "    echo \"\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function show_help {{")?;
    writeln!(file, "    echo -e \"${{GREEN}}Available Commands:${{NC}}\"")?;
    writeln!(file, "    echo \"  info       - Show threat information\"")?;
    writeln!(file, "    echo \"  events     - List related events\"")?;
    writeln!(file, "    echo \"  snapshot   - Capture system snapshot\"")?;
    writeln!(file, "    echo \"  logs       - View EDR logs\"")?;
    writeln!(file, "    echo \"  archive    - Archive this session\"")?;
    writeln!(file, "    echo \"  exit       - Close investigation shell\"")?;
    writeln!(file, "    echo \"\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function show_info {{")?;
    writeln!(file, "    cat \"$SESSION_DIR/session_info.txt\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function show_events {{")?;
    writeln!(file, "    echo -e \"${{YELLOW}}Related Events:${{NC}}\"")?;
    writeln!(file, "    if [ -f \"$SESSION_DIR/events.json\" ]; then")?;
    writeln!(file, "        cat \"$SESSION_DIR/events.json\" | jq .")?;
    writeln!(file, "    else")?;
    writeln!(file, "        echo \"No events file found\"")?;
    writeln!(file, "    fi")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function take_snapshot {{")?;
    writeln!(file, "    echo -e \"${{YELLOW}}Capturing system snapshot...${{NC}}\"")?;
    writeln!(file, "    SNAPSHOT_FILE=\"$SESSION_DIR/snapshot_$(date +%Y%m%d_%H%M%S).txt\"")?;
    writeln!(file, "    {{")?;
    writeln!(file, "        echo \"System Snapshot - $(date)\"")?;
    writeln!(file, "        echo \"═══════════════════════════════════════════════\"")?;
    writeln!(file, "        echo \"\"")?;
    writeln!(file, "        echo \"PROCESSES:\"")?;
    writeln!(file, "        ps auxf")?;
    writeln!(file, "        echo \"\"")?;
    writeln!(file, "        echo \"NETWORK CONNECTIONS:\"")?;
    writeln!(file, "        ss -tunap 2>/dev/null || netstat -tunap 2>/dev/null")?;
    writeln!(file, "        echo \"\"")?;
    writeln!(file, "        echo \"OPEN FILES:\"")?;
    writeln!(file, "        lsof 2>/dev/null | head -100")?;
    writeln!(file, "        echo \"\"")?;
    writeln!(file, "        echo \"RECENT LOGINS:\"")?;
    writeln!(file, "        last -20")?;
    writeln!(file, "    }} > \"$SNAPSHOT_FILE\"")?;
    writeln!(file, "    echo -e \"${{GREEN}}Snapshot saved: $SNAPSHOT_FILE${{NC}}\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function view_logs {{")?;
    writeln!(file, "    echo -e \"${{YELLOW}}Recent EDR Logs:${{NC}}\"")?;
    writeln!(file, "    tail -20 /var/log/rust-edr/threats_*.jsonl 2>/dev/null | jq . || echo \"No logs found\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "function archive_session {{")?;
    writeln!(file, "    echo -e \"${{YELLOW}}Archiving session...${{NC}}\"")?;
    writeln!(file, "    ARCHIVE_NAME=\"investigation_{}_$(date +%Y%m%d_%H%M%S).tar.gz\"", threat.id)?;
    writeln!(file, "    tar -czf \"/var/log/rust-edr/archives/$ARCHIVE_NAME\" -C \"$(dirname $SESSION_DIR)\" \"$(basename $SESSION_DIR)\"")?;
    writeln!(file, "    echo -e \"${{GREEN}}Session archived: $ARCHIVE_NAME${{NC}}\"")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "# Main shell loop")?;
    writeln!(file, "banner")?;
    writeln!(file, "show_help")?;
    writeln!(file, "echo \"\"")?;
    writeln!(file)?;
    writeln!(file, "while true; do")?;
    writeln!(file, "    echo -ne \"${{GREEN}}[edr-investigate]${{NC}} \"")?;
    writeln!(file, "    read -r cmd")?;
    writeln!(file, "    ")?;
    writeln!(file, "    case \"$cmd\" in")?;
    writeln!(file, "        info) show_info ;;")?;
    writeln!(file, "        events) show_events ;;")?;
    writeln!(file, "        snapshot) take_snapshot ;;")?;
    writeln!(file, "        logs) view_logs ;;")?;
    writeln!(file, "        archive) archive_session ;;")?;
    writeln!(file, "        help) show_help ;;")?;
    writeln!(file, "        clear) banner && show_help ;;")?;
    writeln!(file, "        exit|quit) echo -e \"${{YELLOW}}Closing investigation shell...${{NC}}\" && exit 0 ;;")?;
    writeln!(file, "        \"\") ;;")?;
    writeln!(file, "        *) echo -e \"${{RED}}Unknown command: $cmd${{NC}}\" && show_help ;;")?;
    writeln!(file, "    esac")?;
    writeln!(file, "    echo \"\"")?;
    writeln!(file, "done")?;
    
    Ok(())
}

/// Spawn a new terminal window
fn spawn_terminal(script_path: &PathBuf, threat_id: &str) -> Result<()> {
    let script_str = script_path.to_str().unwrap();
    let bash_cmd_xfce = format!("bash {}", script_path.display());
    let bash_cmd_mate = format!("bash {}", script_path.display());
    
    // Try different terminal emulators (in order of preference)
    let terminals: Vec<(&str, Vec<&str>)> = vec![
        ("gnome-terminal", vec!["--", "bash", script_str]),
        ("xterm", vec!["-e", "bash", script_str]),
        ("konsole", vec!["-e", "bash", script_str]),
        ("xfce4-terminal", vec!["-e", &bash_cmd_xfce]),
        ("mate-terminal", vec!["-e", &bash_cmd_mate]),
    ];

    for (terminal, args) in terminals {
        let result = Command::new(terminal)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        if result.is_ok() {
            println!("✅ Spawned {} for threat {}", terminal, threat_id);
            return Ok(());
        }
    }

    // Fallback: print script path
    eprintln!("⚠️  No terminal emulator found. Run manually:");
    eprintln!("    bash {}", script_path.display());
    
    Ok(())
}

/// Create a quick investigation script without spawning terminal
pub fn create_investigation_artifact(
    threat: &Threat,
    session_dir: &PathBuf,
) -> Result<PathBuf> {
    fs::create_dir_all(session_dir)?;
    
    let info_path = session_dir.join("session_info.txt");
    create_session_info(&info_path, threat)?;
    
    let script_path = session_dir.join("investigate.sh");
    create_investigation_script(&script_path, threat, session_dir)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }
    
    Ok(script_path)
}
