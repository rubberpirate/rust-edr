// Telemetry Logger
// Comprehensive logging of all EDR activities

use crate::types::{SystemEvent, Threat};
use crate::response::ResponseResult;
use anyhow::Result;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Utc;

pub struct TelemetryLogger {
    log_dir: PathBuf,
    event_log: Option<File>,
    threat_log: Option<File>,
    response_log: Option<File>,
}

impl TelemetryLogger {
    pub fn new(log_dir: PathBuf) -> Result<Self> {
        // Create log directory if it doesn't exist
        std::fs::create_dir_all(&log_dir)?;

        let mut logger = Self {
            log_dir,
            event_log: None,
            threat_log: None,
            response_log: None,
        };

        logger.initialize_logs()?;

        Ok(logger)
    }

    /// Initialize log files
    fn initialize_logs(&mut self) -> Result<()> {
        let timestamp = Utc::now().format("%Y%m%d");

        // Event log
        let event_log_path = self.log_dir.join(format!("events_{}.jsonl", timestamp));
        self.event_log = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(event_log_path)?
        );

        // Threat log
        let threat_log_path = self.log_dir.join(format!("threats_{}.jsonl", timestamp));
        self.threat_log = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(threat_log_path)?
        );

        // Response log
        let response_log_path = self.log_dir.join(format!("responses_{}.jsonl", timestamp));
        self.response_log = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(response_log_path)?
        );

        println!("Telemetry logging initialized at: {:?}", self.log_dir);
        Ok(())
    }

    /// Log a system event
    pub fn log_event(&mut self, event: &SystemEvent) -> Result<()> {
        if let Some(file) = &mut self.event_log {
            let json = serde_json::to_string(event)?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }
        Ok(())
    }

    /// Log a detected threat
    pub fn log_threat(&mut self, threat: &Threat) -> Result<()> {
        if let Some(file) = &mut self.threat_log {
            let json = serde_json::to_string(threat)?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }

        // Also print to console
        println!("🚨 THREAT DETECTED: {:?} - Score: {:.2} - {}",
                 threat.threat_type, threat.score, threat.description);

        Ok(())
    }

    /// Log a response action
    pub fn log_response(&mut self, response: &ResponseResult) -> Result<()> {
        if let Some(file) = &mut self.response_log {
            let json = serde_json::to_string(response)?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }

        // Also print to console
        println!("⚡ RESPONSE: {:?} - Success: {} - {}",
                 response.action, response.success, response.message);

        Ok(())
    }

    /// Log general message
    pub fn log_message(&self, level: &str, message: &str) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] {}", timestamp, level, message);
    }

    /// Generate summary statistics
    pub fn generate_summary(&self) -> Result<String> {
        // In production, this would read the log files and generate statistics
        Ok(format!("Telemetry Summary:\nLog Directory: {:?}", self.log_dir))
    }
}
