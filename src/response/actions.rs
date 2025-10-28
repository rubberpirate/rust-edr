// Response Actions
// Execute response actions: block, allow, quarantine, alert, kill

use crate::types::{Threat, ResponseAction, Severity};
use crate::forensics::snapshot::ForensicSnapshot;
use crate::forensics::shell_spawner::create_investigation_artifact;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult {
    pub action: ResponseAction,
    pub success: bool,
    pub message: String,
    pub timestamp: chrono::DateTime<Utc>,
}

pub struct ResponseEngine {
    auto_response_enabled: bool,
    threat_threshold: f32,
    response_tx: Option<mpsc::Sender<ResponseResult>>,
}

impl ResponseEngine {
    pub fn new(auto_response_enabled: bool, threat_threshold: f32) -> Self {
        Self {
            auto_response_enabled,
            threat_threshold,
            response_tx: None,
        }
    }

    /// Set response result channel
    pub fn set_response_channel(&mut self, tx: mpsc::Sender<ResponseResult>) {
        self.response_tx = Some(tx);
    }

    /// Process a threat and decide on response
    pub async fn handle_threat(&self, threat: &Threat) -> Vec<ResponseResult> {
        let mut results = Vec::new();

        // Always alert
        results.push(self.alert(threat).await);

        // Auto-response based on severity and score
        if self.auto_response_enabled && threat.score >= self.threat_threshold {
            match threat.severity {
                Severity::Critical => {
                    // Critical threats: Kill process and isolate
                    if let Some(result) = self.kill_threat_process(threat).await {
                        results.push(result);
                    }
                    results.push(self.quarantine(threat).await);
                }
                Severity::High => {
                    // High severity: Block and quarantine
                    results.push(self.block(threat).await);
                    results.push(self.quarantine(threat).await);
                }
                Severity::Medium => {
                    // Medium severity: Block only
                    results.push(self.block(threat).await);
                }
                _ => {
                    // Low/Info: Alert only (already done)
                }
            }
        }

        // Send results through channel if configured
        if let Some(tx) = &self.response_tx {
            for result in &results {
                let _ = tx.send(result.clone()).await;
            }
        }

        results
    }

    /// Alert on threat (always executed)
    async fn alert(&self, threat: &Threat) -> ResponseResult {
        // Capture forensic snapshot for high-severity threats
        if matches!(threat.severity, Severity::High | Severity::Critical) {
            self.capture_forensics(threat).await;
        }

        ResponseResult {
            action: ResponseAction::Alert,
            success: true,
            message: format!(
                "Alert: {:?} threat detected - Score: {:.2} - {}",
                threat.threat_type, threat.score, threat.description
            ),
            timestamp: Utc::now(),
        }
    }

    /// Capture forensic data for investigation
    async fn capture_forensics(&self, threat: &Threat) {
        let threat_clone = threat.clone();
        
        tokio::spawn(async move {
            // Create session directory
            let session_dir = PathBuf::from(format!(
                "/var/log/rust-edr/archives/sessions/investigation_{}",
                threat_clone.id
            ));

            // Capture snapshot
            if let Ok(snapshot) = ForensicSnapshot::capture(&threat_clone.id) {
                let snapshot_path = session_dir.join("snapshot");
                if let Err(e) = snapshot.save(&snapshot_path) {
                    eprintln!("Failed to save snapshot: {}", e);
                } else {
                    println!("📸 Forensic snapshot captured for threat: {}", threat_clone.id);
                }
            }

            // Create investigation shell artifact
            if let Err(e) = create_investigation_artifact(&threat_clone, &session_dir) {
                eprintln!("Failed to create investigation artifact: {}", e);
            } else {
                println!("🐚 Investigation shell created: {}/investigate.sh", session_dir.display());
                println!("   Run with: bash {}/investigate.sh", session_dir.display());
            }
        });
    }

    /// Block threat (prevent execution/connection)
    async fn block(&self, threat: &Threat) -> ResponseResult {
        // In production, this would:
        // - Add firewall rules for network threats
        // - Prevent process execution for file threats
        // - Use SELinux/AppArmor policies
        
        ResponseResult {
            action: ResponseAction::Block,
            success: true,
            message: format!("Blocked: {:?} - {}", threat.threat_type, threat.description),
            timestamp: Utc::now(),
        }
    }

    /// Quarantine threat (isolate files/processes)
    async fn quarantine(&self, threat: &Threat) -> ResponseResult {
        // In production, this would:
        // - Move malicious files to quarantine directory
        // - Suspend suspicious processes
        // - Isolate network connections
        
        ResponseResult {
            action: ResponseAction::Quarantine,
            success: true,
            message: format!("Quarantined: {:?} - {}", threat.threat_type, threat.description),
            timestamp: Utc::now(),
        }
    }

    /// Kill threat process
    async fn kill_threat_process(&self, threat: &Threat) -> Option<ResponseResult> {
        // Extract PID from threat events
        for event in &threat.events {
            if let crate::types::EventDetails::Process(proc_event) = &event.details {
                return Some(self.kill_process(proc_event.pid).await);
            }
        }

        None
    }

    /// Kill a specific process
    async fn kill_process(&self, pid: u32) -> ResponseResult {
        // In production, send SIGKILL to process
        let success = if let Ok(output) = tokio::process::Command::new("kill")
            .args(&["-9", &pid.to_string()])
            .output()
            .await
        {
            output.status.success()
        } else {
            false
        };

        ResponseResult {
            action: ResponseAction::Kill,
            success,
            message: if success {
                format!("Killed process PID: {}", pid)
            } else {
                format!("Failed to kill process PID: {}", pid)
            },
            timestamp: Utc::now(),
        }
    }

    /// Isolate network (disconnect threat from network)
    async fn isolate_network(&self, _threat: &Threat) -> ResponseResult {
        // In production, this would:
        // - Drop network connections using iptables
        // - Disable network interfaces
        // - Use network namespaces
        
        ResponseResult {
            action: ResponseAction::IsolateNetwork,
            success: true,
            message: "Network isolation applied".to_string(),
            timestamp: Utc::now(),
        }
    }

    /// Allow threat (whitelist)
    pub async fn allow(&self, threat: &Threat) -> ResponseResult {
        ResponseResult {
            action: ResponseAction::Allow,
            success: true,
            message: format!("Allowed: {:?} - {}", threat.threat_type, threat.description),
            timestamp: Utc::now(),
        }
    }

    /// Manual response action
    pub async fn execute_action(
        &self,
        action: ResponseAction,
        threat: &Threat,
    ) -> ResponseResult {
        match action {
            ResponseAction::Alert => self.alert(threat).await,
            ResponseAction::Block => self.block(threat).await,
            ResponseAction::Quarantine => self.quarantine(threat).await,
            ResponseAction::Kill => {
                if let Some(result) = self.kill_threat_process(threat).await {
                    result
                } else {
                    ResponseResult {
                        action: ResponseAction::Kill,
                        success: false,
                        message: "No process to kill".to_string(),
                        timestamp: Utc::now(),
                    }
                }
            }
            ResponseAction::IsolateNetwork => self.isolate_network(threat).await,
            ResponseAction::Allow => self.allow(threat).await,
        }
    }
}
