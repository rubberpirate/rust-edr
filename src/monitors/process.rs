// Process monitoring module
// Tracks process creation, execution, and termination

use crate::types::{SystemEvent, EventType, EventDetails, ProcessEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use procfs::process::all_processes;
use chrono::Utc;
use std::collections::HashSet;
use std::time::Duration;

pub struct ProcessMonitor {
    enabled: bool,
    known_pids: HashSet<i32>,
    event_tx: mpsc::Sender<SystemEvent>,
}

impl ProcessMonitor {
    pub fn new(event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            known_pids: HashSet::new(),
            event_tx,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting process monitor...");
        self.enabled = true;

        // Initialize with current processes
        self.scan_processes().await?;

        // Start monitoring loop
        self.monitor_loop().await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping process monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        while self.enabled {
            interval.tick().await;
            self.scan_processes().await?;
        }

        Ok(())
    }

    /// Scan for new/terminated processes
    async fn scan_processes(&mut self) -> Result<()> {
        let mut current_pids = HashSet::new();

        // Get all current processes
        if let Ok(processes) = all_processes() {
            for proc_result in processes {
                if let Ok(process) = proc_result {
                    let pid = process.pid;
                    current_pids.insert(pid);

                    // New process detected
                    if !self.known_pids.contains(&pid) {
                        if let Ok(stat) = process.stat() {
                            if let Ok(cmdline) = process.cmdline() {
                                self.report_process_created(pid, stat, cmdline).await;
                            }
                        }
                    }
                }
            }
        }

        // Detect terminated processes
        for pid in &self.known_pids {
            if !current_pids.contains(pid) {
                self.report_process_terminated(*pid).await;
            }
        }

        self.known_pids = current_pids;
        Ok(())
    }

    /// Report new process creation
    async fn report_process_created(
        &self,
        pid: i32,
        stat: procfs::process::Stat,
        cmdline: Vec<String>,
    ) {
        let exe_path = procfs::process::Process::new(pid)
            .and_then(|p| p.exe())
            .unwrap_or_else(|_| std::path::PathBuf::from("unknown"));

        let uid = procfs::process::Process::new(pid)
            .and_then(|p| p.uid())
            .unwrap_or(0);

        let username = users::get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Determine severity based on characteristics
        let severity = self.assess_process_severity(&exe_path, &cmdline, uid);

        let event = SystemEvent {
            id: format!("proc_create_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::ProcessCreated,
            severity,
            source: "process_monitor".to_string(),
            details: EventDetails::Process(ProcessEvent {
                pid: pid as u32,
                ppid: Some(stat.ppid as u32),
                name: stat.comm,
                path: exe_path,
                cmdline,
                user: username,
                uid,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report process termination
    async fn report_process_terminated(&self, pid: i32) {
        let event = SystemEvent {
            id: format!("proc_term_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::ProcessTerminated,
            severity: Severity::Info,
            source: "process_monitor".to_string(),
            details: EventDetails::Process(ProcessEvent {
                pid: pid as u32,
                ppid: None,
                name: "terminated".to_string(),
                path: std::path::PathBuf::from("unknown"),
                cmdline: vec![],
                user: "unknown".to_string(),
                uid: 0,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Assess process severity based on characteristics
    fn assess_process_severity(
        &self,
        exe_path: &std::path::Path,
        cmdline: &[String],
        uid: u32,
    ) -> Severity {
        let path_str = exe_path.to_string_lossy();
        let cmd_str = cmdline.join(" ");

        // Critical: Running from suspicious locations
        if path_str.contains("/tmp/") || path_str.contains("/dev/shm/") {
            return Severity::High;
        }

        // High: Suspicious command patterns
        if cmd_str.contains("wget") && cmd_str.contains("http") ||
           cmd_str.contains("curl") && cmd_str.contains("bash") ||
           cmd_str.contains("nc -") ||
           cmd_str.contains("/dev/tcp/") {
            return Severity::High;
        }

        // Medium: Root process
        if uid == 0 {
            return Severity::Medium;
        }

        Severity::Info
    }
}

