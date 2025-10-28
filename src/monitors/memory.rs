// Memory analysis module
// Detects suspicious memory operations

use crate::types::{SystemEvent, EventType, EventDetails, MemoryEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use chrono::Utc;
use std::time::Duration;
use procfs::process::all_processes;

pub struct MemoryMonitor {
    enabled: bool,
    event_tx: mpsc::Sender<SystemEvent>,
}

impl MemoryMonitor {
    pub fn new(event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            event_tx,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting memory monitor...");
        self.enabled = true;

        // Start monitoring loop
        self.monitor_loop().await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping memory monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        while self.enabled {
            interval.tick().await;
            self.scan_memory().await?;
        }

        Ok(())
    }

    /// Scan process memory for suspicious patterns
    async fn scan_memory(&mut self) -> Result<()> {
        if let Ok(processes) = all_processes() {
            for proc_result in processes {
                if let Ok(process) = proc_result {
                    // Check memory maps for suspicious permissions
                    if let Ok(maps) = process.maps() {
                        for map in maps {
                            // Check for RWX (read-write-execute) permissions
                            let perms = format!("{:?}", map.perms);
                            if perms.contains("r") && perms.contains("w") && perms.contains("x") {
                                self.report_suspicious_memory(
                                    process.pid,
                                    map.address.0 as u64,
                                    (map.address.1 - map.address.0) as u64,
                                    perms,
                                ).await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Report suspicious memory operation
    async fn report_suspicious_memory(
        &self,
        pid: i32,
        address: u64,
        size: u64,
        permissions: String,
    ) {
        let process_name = procfs::process::Process::new(pid)
            .and_then(|p| p.stat())
            .map(|s| s.comm)
            .unwrap_or_else(|_| "unknown".to_string());

        let event = SystemEvent {
            id: format!("mem_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::MemoryInjection,
            severity: Severity::High,
            source: "memory_monitor".to_string(),
            details: EventDetails::Memory(MemoryEvent {
                process_pid: pid as u32,
                process_name,
                operation: "rwx_memory".to_string(),
                address,
                size,
                permissions,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }
}

