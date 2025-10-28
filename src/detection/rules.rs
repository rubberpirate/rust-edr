// Behavioral Rules Engine
// Detects threats based on behavioral patterns and rules

use crate::types::{SystemEvent, EventType, EventDetails, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub enabled: bool,
    pub conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

pub struct RuleEngine {
    rules: Vec<BehavioralRule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    /// Load behavioral rules
    pub fn load_rules(&mut self) -> Result<()> {
        // Load default rules
        self.add_default_rules();
        
        // TODO: Load custom rules from configuration
        println!("Loaded {} behavioral rules", self.rules.len());
        Ok(())
    }

    /// Check if an event matches any rules
    pub fn check_event(&self, event: &SystemEvent) -> Vec<String> {
        let mut matched_rules = Vec::new();

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if self.evaluate_rule(rule, event) {
                matched_rules.push(rule.id.clone());
            }
        }

        matched_rules
    }

    /// Evaluate if an event matches a rule
    fn evaluate_rule(&self, rule: &BehavioralRule, event: &SystemEvent) -> bool {
        // Simple rule evaluation logic
        // In production, this would be much more sophisticated
        
        match &event.details {
            EventDetails::Process(proc_event) => {
                // Check for suspicious process behaviors
                if rule.id == "suspicious_process_location" {
                    let path_str = proc_event.path.to_string_lossy();
                    return path_str.contains("/tmp/") || 
                           path_str.contains("/dev/shm/") ||
                           path_str.starts_with("/var/tmp/");
                }
                
                if rule.id == "root_process_spawn" {
                    // Only flag if:
                    // 1. Running as root (uid 0)
                    // 2. Parent is NOT init/systemd (ppid != 1)
                    // 3. Process name is suspicious or from unusual location
                    if proc_event.uid == 0 {
                        let suspicious_names = vec!["nc", "ncat", "bash", "sh", "python", "perl", "ruby"];
                        let name_lower = proc_event.name.to_lowercase();
                        
                        // Check if process name is suspicious
                        let is_suspicious_name = suspicious_names.iter().any(|s| name_lower.contains(s));
                        
                        // Check if running from suspicious location
                        let cmdline_str = proc_event.cmdline.join(" ");
                        let is_suspicious_location = cmdline_str.contains("/tmp/") || 
                                                      cmdline_str.contains("/dev/shm/") ||
                                                      cmdline_str.contains("/var/tmp/");
                        
                        // Only alert if suspicious AND parent is not init/systemd
                        return (is_suspicious_name || is_suspicious_location) && 
                               proc_event.ppid != Some(1);
                    }
                    return false;
                }

                if rule.id == "suspicious_cmdline" {
                    let cmdline = proc_event.cmdline.join(" ");
                    return cmdline.contains("wget") && cmdline.contains("http") ||
                           cmdline.contains("curl") && cmdline.contains("bash") ||
                           cmdline.contains("nc -") ||
                           cmdline.contains("/dev/tcp/");
                }
            }
            EventDetails::File(file_event) => {
                // Check for suspicious file operations
                if rule.id == "critical_file_modification" {
                    let path_str = file_event.path.to_string_lossy();
                    return path_str.contains("/etc/passwd") ||
                           path_str.contains("/etc/shadow") ||
                           path_str.contains("/etc/sudoers") ||
                           path_str.contains(".ssh/authorized_keys");
                }

                if rule.id == "hidden_file_execution" {
                    let filename = file_event.path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    return filename.starts_with('.') && 
                           file_event.operation == "execute";
                }
            }
            EventDetails::Network(net_event) => {
                // Check for suspicious network activity
                if rule.id == "uncommon_port_connection" {
                    let suspicious_ports = vec![4444, 31337, 1337, 8888, 9999];
                    return suspicious_ports.contains(&net_event.dst_port);
                }

                if rule.id == "high_volume_transfer" {
                    return net_event.bytes_sent > 100_000_000; // 100MB
                }
            }
            EventDetails::User(user_event) => {
                // Check for suspicious user actions
                if rule.id == "privilege_escalation" {
                    return user_event.action.contains("sudo") ||
                           user_event.action.contains("su");
                }

                if rule.id == "remote_root_login" {
                    return user_event.uid == 0 && 
                           user_event.remote_ip.is_some() &&
                           user_event.action == "login";
                }
            }
            EventDetails::Memory(mem_event) => {
                // Check for suspicious memory operations
                if rule.id == "memory_injection" {
                    return mem_event.operation.contains("inject") ||
                           mem_event.permissions.contains("rwx");
                }
            }
            _ => {}
        }

        false
    }

    /// Add default behavioral rules
    fn add_default_rules(&mut self) {
        self.rules.push(BehavioralRule {
            id: "suspicious_process_location".to_string(),
            name: "Process Started from Suspicious Location".to_string(),
            description: "Detects processes starting from /tmp or /dev/shm".to_string(),
            severity: Severity::High,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "critical_file_modification".to_string(),
            name: "Critical System File Modified".to_string(),
            description: "Detects modifications to /etc/passwd, /etc/shadow, or SSH keys".to_string(),
            severity: Severity::Critical,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "uncommon_port_connection".to_string(),
            name: "Connection to Uncommon Port".to_string(),
            description: "Detects connections to commonly used hacker ports".to_string(),
            severity: Severity::Medium,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "privilege_escalation".to_string(),
            name: "Privilege Escalation Attempt".to_string(),
            description: "Detects sudo or su usage".to_string(),
            severity: Severity::High,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "remote_root_login".to_string(),
            name: "Remote Root Login".to_string(),
            description: "Detects root login from remote IP".to_string(),
            severity: Severity::Critical,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "suspicious_cmdline".to_string(),
            name: "Suspicious Command Line".to_string(),
            description: "Detects suspicious command patterns (reverse shells, downloads)".to_string(),
            severity: Severity::High,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "memory_injection".to_string(),
            name: "Memory Injection Detected".to_string(),
            description: "Detects memory injection or RWX permissions".to_string(),
            severity: Severity::Critical,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "hidden_file_execution".to_string(),
            name: "Hidden File Execution".to_string(),
            description: "Detects execution of hidden files (starting with .)".to_string(),
            severity: Severity::Medium,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "high_volume_transfer".to_string(),
            name: "High Volume Data Transfer".to_string(),
            description: "Detects large data transfers (potential exfiltration)".to_string(),
            severity: Severity::High,
            enabled: true,
            conditions: vec![],
        });

        self.rules.push(BehavioralRule {
            id: "root_process_spawn".to_string(),
            name: "Suspicious Root Process".to_string(),
            description: "Detects suspicious processes running as root (shells, scripts from /tmp)".to_string(),
            severity: Severity::High,
            enabled: true,
            conditions: vec![],
        });
    }

    pub fn get_rule(&self, id: &str) -> Option<&BehavioralRule> {
        self.rules.iter().find(|r| r.id == id)
    }
}
