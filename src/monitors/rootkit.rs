// Rootkit detection module
// Detects kernel module manipulations and hidden processes/files

use crate::types::{SystemEvent, EventType, EventDetails, RootkitEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use chrono::Utc;
use std::time::Duration;
use std::collections::HashSet;

pub struct RootkitMonitor {
    enabled: bool,
    event_tx: mpsc::Sender<SystemEvent>,
    known_modules: HashSet<String>,
}

impl RootkitMonitor {
    pub fn new(event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            event_tx,
            known_modules: HashSet::new(),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting rootkit monitor...");
        self.enabled = true;

        // Initialize with current kernel modules
        self.scan_modules().await?;

        // Start monitoring loop
        self.monitor_loop().await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping rootkit monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(15));

        while self.enabled {
            interval.tick().await;
            self.scan_modules().await?;
            self.check_hidden_processes().await?;
            self.check_hidden_files().await?;
        }

        Ok(())
    }

    /// Scan for kernel module changes
    async fn scan_modules(&mut self) -> Result<()> {
        let mut current_modules = HashSet::new();

        // Read /proc/modules
        if let Ok(modules) = tokio::fs::read_to_string("/proc/modules").await {
            for line in modules.lines() {
                if let Some(module_name) = line.split_whitespace().next() {
                    let name = module_name.to_string();
                    current_modules.insert(name.clone());

                    // New module loaded
                    if !self.known_modules.contains(&name) {
                        self.report_module_loaded(&name).await;
                    }
                }
            }
        }

        // Detect unloaded modules
        for module in &self.known_modules {
            if !current_modules.contains(module) {
                self.report_module_unloaded(module).await;
            }
        }

        self.known_modules = current_modules;
        Ok(())
    }

    /// Check for hidden processes (compare different enumeration methods)
    async fn check_hidden_processes(&self) -> Result<()> {
        // Compare /proc enumeration with system calls
        // This is a simplified check
        
        let proc_pids = self.get_proc_pids().await?;
        let ps_pids = self.get_ps_pids().await?;

        // If there's a discrepancy, potential rootkit
        if proc_pids.len() != ps_pids.len() {
            self.report_hidden_processes(proc_pids.len(), ps_pids.len()).await;
        }

        Ok(())
    }

    /// Get PIDs from /proc
    async fn get_proc_pids(&self) -> Result<HashSet<u32>> {
        let mut pids = HashSet::new();

        if let Ok(entries) = tokio::fs::read_dir("/proc").await {
            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(pid) = file_name.parse::<u32>() {
                        pids.insert(pid);
                    }
                }
            }
        }

        Ok(pids)
    }

    /// Get PIDs from ps command
    async fn get_ps_pids(&self) -> Result<HashSet<u32>> {
        let mut pids = HashSet::new();

        if let Ok(output) = tokio::process::Command::new("ps")
            .args(&["-e", "-o", "pid="])
            .output()
            .await
        {
            if let Ok(ps_output) = String::from_utf8(output.stdout) {
                for line in ps_output.lines() {
                    if let Ok(pid) = line.trim().parse::<u32>() {
                        pids.insert(pid);
                    }
                }
            }
        }

        Ok(pids)
    }

    /// Check for hidden files
    async fn check_hidden_files(&self) -> Result<()> {
        // Check for suspicious files in common rootkit locations
        let suspicious_paths = vec![
            "/dev/.hidden",
            "/usr/lib/.hidden",
            "/tmp/.hidden",
            "/var/tmp/.hidden",
        ];

        for path in suspicious_paths {
            if tokio::fs::metadata(path).await.is_ok() {
                self.report_hidden_file(path).await;
            }
        }

        Ok(())
    }

    /// Report kernel module loaded
    async fn report_module_loaded(&self, module_name: &str) {
        // Check for known malicious module names
        let suspicious_modules = vec!["diamorphine", "suterusu", "rootkit"];
        let severity = if suspicious_modules.iter().any(|m| module_name.contains(m)) {
            Severity::Critical
        } else {
            Severity::Medium
        };

        let event = SystemEvent {
            id: format!("rootkit_module_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::RootkitDetected,
            severity,
            source: "rootkit_monitor".to_string(),
            details: EventDetails::Rootkit(RootkitEvent {
                detection_type: "kernel_module_loaded".to_string(),
                description: format!("Kernel module loaded: {}", module_name),
                affected_path: None,
                affected_process: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report kernel module unloaded
    async fn report_module_unloaded(&self, module_name: &str) {
        let event = SystemEvent {
            id: format!("rootkit_module_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::RootkitDetected,
            severity: Severity::Info,
            source: "rootkit_monitor".to_string(),
            details: EventDetails::Rootkit(RootkitEvent {
                detection_type: "kernel_module_unloaded".to_string(),
                description: format!("Kernel module unloaded: {}", module_name),
                affected_path: None,
                affected_process: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report hidden processes detected
    async fn report_hidden_processes(&self, proc_count: usize, ps_count: usize) {
        let event = SystemEvent {
            id: format!("rootkit_hidden_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::RootkitDetected,
            severity: Severity::Critical,
            source: "rootkit_monitor".to_string(),
            details: EventDetails::Rootkit(RootkitEvent {
                detection_type: "hidden_processes".to_string(),
                description: format!(
                    "Process count mismatch: /proc={}, ps={}",
                    proc_count, ps_count
                ),
                affected_path: None,
                affected_process: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report hidden file detected
    async fn report_hidden_file(&self, path: &str) {
        let event = SystemEvent {
            id: format!("rootkit_file_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::RootkitDetected,
            severity: Severity::High,
            source: "rootkit_monitor".to_string(),
            details: EventDetails::Rootkit(RootkitEvent {
                detection_type: "hidden_file".to_string(),
                description: format!("Suspicious hidden file detected: {}", path),
                affected_path: Some(std::path::PathBuf::from(path)),
                affected_process: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }
}
