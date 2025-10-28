// User action monitoring module
// Tracks user logins, logouts, and privilege escalations

use crate::types::{SystemEvent, EventType, EventDetails, UserEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use chrono::Utc;
use std::time::Duration;
use std::collections::HashSet;

pub struct UserMonitor {
    enabled: bool,
    event_tx: mpsc::Sender<SystemEvent>,
    logged_in_users: HashSet<String>,
}

impl UserMonitor {
    pub fn new(event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            event_tx,
            logged_in_users: HashSet::new(),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting user monitor...");
        self.enabled = true;

        // Initialize with current users
        self.scan_users().await?;

        // Start monitoring loop
        self.monitor_loop().await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping user monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        while self.enabled {
            interval.tick().await;
            self.scan_users().await?;
            self.check_auth_logs().await?;
        }

        Ok(())
    }

    /// Scan currently logged in users
    async fn scan_users(&mut self) -> Result<()> {
        let mut current_users = HashSet::new();

        // Parse /var/run/utmp or use 'who' command output
        if let Ok(output) = tokio::process::Command::new("who")
            .output()
            .await
        {
            if let Ok(who_output) = String::from_utf8(output.stdout) {
                for line in who_output.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(username) = parts.first() {
                        let user = username.to_string();
                        current_users.insert(user.clone());

                        // New login detected
                        if !self.logged_in_users.contains(&user) {
                            self.report_user_login(&user, parts.get(1).map(|s| s.to_string())).await;
                        }
                    }
                }
            }
        }

        // Detect logouts
        for user in &self.logged_in_users {
            if !current_users.contains(user) {
                self.report_user_logout(user).await;
            }
        }

        self.logged_in_users = current_users;
        Ok(())
    }

    /// Check auth logs for privilege escalations
    async fn check_auth_logs(&self) -> Result<()> {
        // Monitor /var/log/auth.log for sudo usage
        // In production, use file tailing or audit subsystem
        
        // For now, check recent sudo usage from journal
        if let Ok(output) = tokio::process::Command::new("journalctl")
            .args(&["-n", "10", "-u", "sudo", "--no-pager"])
            .output()
            .await
        {
            if let Ok(log_output) = String::from_utf8(output.stdout) {
                if log_output.contains("COMMAND") {
                    self.report_privilege_escalation("sudo").await;
                }
            }
        }

        Ok(())
    }

    /// Report user login
    async fn report_user_login(&self, username: &str, terminal: Option<String>) {
        let uid = users::get_user_by_name(username)
            .map(|u| u.uid())
            .unwrap_or(65534);

        let severity = if uid == 0 {
            Severity::High
        } else {
            Severity::Info
        };

        let event = SystemEvent {
            id: format!("user_login_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::UserLogin,
            severity,
            source: "user_monitor".to_string(),
            details: EventDetails::User(UserEvent {
                username: username.to_string(),
                uid,
                action: "login".to_string(),
                terminal,
                remote_ip: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report user logout
    async fn report_user_logout(&self, username: &str) {
        let uid = users::get_user_by_name(username)
            .map(|u| u.uid())
            .unwrap_or(65534);

        let event = SystemEvent {
            id: format!("user_logout_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::UserLogout,
            severity: Severity::Info,
            source: "user_monitor".to_string(),
            details: EventDetails::User(UserEvent {
                username: username.to_string(),
                uid,
                action: "logout".to_string(),
                terminal: None,
                remote_ip: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Report privilege escalation
    async fn report_privilege_escalation(&self, method: &str) {
        let username = whoami::username();
        let uid = users::get_user_by_name(&username)
            .map(|u| u.uid())
            .unwrap_or(1000);

        let event = SystemEvent {
            id: format!("user_escalation_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::UserElevation,
            severity: Severity::High,
            source: "user_monitor".to_string(),
            details: EventDetails::User(UserEvent {
                username,
                uid,
                action: format!("privilege_escalation_{}", method),
                terminal: None,
                remote_ip: None,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }
}
