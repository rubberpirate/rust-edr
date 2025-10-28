// File system monitoring module
// Monitors file operations in critical directories

use crate::types::{SystemEvent, EventType, EventDetails, FileEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use inotify::{Inotify, WatchMask, Event};
use chrono::Utc;
use std::path::PathBuf;

pub struct FileMonitor {
    enabled: bool,
    watch_paths: Vec<String>,
    event_tx: mpsc::Sender<SystemEvent>,
}

impl FileMonitor {
    pub fn new(watch_paths: Vec<String>, event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            watch_paths,
            event_tx,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting file monitor...");
        println!("Watching paths: {:?}", self.watch_paths);
        self.enabled = true;

        // Initialize inotify
        let mut inotify = Inotify::init()?;

        // Add watches for each path
        for path in &self.watch_paths {
            if let Ok(_) = inotify.watches().add(
                path,
                WatchMask::CREATE
                    | WatchMask::DELETE
                    | WatchMask::MODIFY
                    | WatchMask::ATTRIB
                    | WatchMask::MOVED_TO
                    | WatchMask::MOVED_FROM,
            ) {
                println!("Watching: {}", path);
            }
        }

        // Start monitoring
        self.monitor_loop(inotify).await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping file monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self, mut inotify: Inotify) -> Result<()> {
        let mut buffer = [0; 4096];

        while self.enabled {
            // Read events (blocking)
            if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                for event in events {
                    self.process_inotify_event(event).await;
                }
            }

            // Small yield to prevent 100% CPU
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        Ok(())
    }

    /// Process inotify event
    async fn process_inotify_event(&self, event: Event<&std::ffi::OsStr>) {
        if let Some(name) = event.name {
            let path = PathBuf::from(name);
            
            let (event_type, operation) = if event.mask.contains(inotify::EventMask::CREATE) {
                (EventType::FileCreated, "create")
            } else if event.mask.contains(inotify::EventMask::DELETE) {
                (EventType::FileDeleted, "delete")
            } else if event.mask.contains(inotify::EventMask::MODIFY) {
                (EventType::FileModified, "modify")
            } else if event.mask.contains(inotify::EventMask::ATTRIB) {
                (EventType::FileModified, "chmod")
            } else if event.mask.contains(inotify::EventMask::MOVED_TO) {
                (EventType::FileModified, "move_to")
            } else if event.mask.contains(inotify::EventMask::MOVED_FROM) {
                (EventType::FileModified, "move_from")
            } else {
                return;
            };

            // Determine severity
            let severity = self.assess_file_severity(&path, operation);

            // Get process info (best effort)
            let (process_pid, process_name) = self.get_current_process_info();

            let sys_event = SystemEvent {
                id: format!("file_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                event_type,
                severity,
                source: "file_monitor".to_string(),
                details: EventDetails::File(FileEvent {
                    path,
                    operation: operation.to_string(),
                    process_pid,
                    process_name,
                    user: whoami::username(),
                    permissions: None,
                }),
            };

            let _ = self.event_tx.send(sys_event).await;
        }
    }

    /// Assess file operation severity
    fn assess_file_severity(&self, path: &PathBuf, operation: &str) -> Severity {
        let path_str = path.to_string_lossy();

        // Critical files
        if path_str.contains("/etc/passwd") ||
           path_str.contains("/etc/shadow") ||
           path_str.contains("/etc/sudoers") ||
           path_str.contains(".ssh/authorized_keys") {
            return Severity::Critical;
        }

        // System directories
        if path_str.starts_with("/etc/") ||
           path_str.starts_with("/usr/bin/") ||
           path_str.starts_with("/usr/sbin/") {
            if operation == "modify" || operation == "delete" {
                return Severity::High;
            }
            return Severity::Medium;
        }

        // Temporary directories (suspicious if executable)
        if path_str.contains("/tmp/") || path_str.contains("/dev/shm/") {
            return Severity::Medium;
        }

        Severity::Info
    }

    /// Get current process info (simplified)
    fn get_current_process_info(&self) -> (u32, String) {
        // In a real implementation, we'd track which process triggered the event
        // For now, return placeholder
        (std::process::id(), "unknown".to_string())
    }
}

