// IOC (Indicator of Compromise) Matcher
// Matches system events against known IOCs

use crate::types::{SystemEvent, EventDetails, IOC, IOCType, Severity};
use std::collections::HashMap;
use anyhow::Result;

pub struct IOCMatcher {
    iocs: HashMap<String, IOC>,
    hash_iocs: Vec<IOC>,
    path_iocs: Vec<IOC>,
    ip_iocs: Vec<IOC>,
    domain_iocs: Vec<IOC>,
    process_iocs: Vec<IOC>,
}

impl IOCMatcher {
    pub fn new() -> Self {
        Self {
            iocs: HashMap::new(),
            hash_iocs: Vec::new(),
            path_iocs: Vec::new(),
            ip_iocs: Vec::new(),
            domain_iocs: Vec::new(),
            process_iocs: Vec::new(),
        }
    }

    /// Load IOCs from configuration
    pub fn load_iocs(&mut self) -> Result<()> {
        // Load default known malicious indicators
        self.add_default_iocs();
        
        // TODO: Load from external threat intelligence feeds
        // TODO: Load from local IOC database
        
        println!("Loaded {} IOCs", self.iocs.len());
        Ok(())
    }

    /// Add a new IOC to the matcher
    pub fn add_ioc(&mut self, ioc: IOC) {
        let id = ioc.id.clone();
        
        // Add to type-specific lists for faster matching
        match ioc.ioc_type {
            IOCType::FileHash => self.hash_iocs.push(ioc.clone()),
            IOCType::FilePath => self.path_iocs.push(ioc.clone()),
            IOCType::IpAddress => self.ip_iocs.push(ioc.clone()),
            IOCType::Domain => self.domain_iocs.push(ioc.clone()),
            IOCType::ProcessName => self.process_iocs.push(ioc.clone()),
            _ => {}
        }
        
        self.iocs.insert(id, ioc);
    }

    /// Check if an event matches any IOCs
    pub fn check_event(&self, event: &SystemEvent) -> Vec<String> {
        let mut matches = Vec::new();

        match &event.details {
            EventDetails::Process(proc_event) => {
                // Check process name and path against IOCs
                for ioc in &self.process_iocs {
                    if proc_event.name.contains(&ioc.value) || 
                       proc_event.path.to_string_lossy().contains(&ioc.value) {
                        matches.push(ioc.id.clone());
                    }
                }
                
                // Check path IOCs
                for ioc in &self.path_iocs {
                    if proc_event.path.to_string_lossy().contains(&ioc.value) {
                        matches.push(ioc.id.clone());
                    }
                }
            }
            EventDetails::File(file_event) => {
                // Check file paths
                for ioc in &self.path_iocs {
                    if file_event.path.to_string_lossy().contains(&ioc.value) {
                        matches.push(ioc.id.clone());
                    }
                }
            }
            EventDetails::Network(net_event) => {
                // Check IP addresses
                for ioc in &self.ip_iocs {
                    if net_event.dst_ip.to_string() == ioc.value ||
                       net_event.src_ip.to_string() == ioc.value {
                        matches.push(ioc.id.clone());
                    }
                }
            }
            _ => {}
        }

        matches
    }

    /// Add default known malicious indicators
    fn add_default_iocs(&mut self) {
        // Suspicious process names
        let suspicious_processes = vec![
            ("mimikatz", "Credential dumping tool"),
            ("nc.exe", "Netcat reverse shell"),
            ("psexec", "Remote execution tool"),
            ("whoami", "Reconnaissance command"),
            ("curl", "Potential data exfiltration"),
        ];

        for (name, desc) in suspicious_processes {
            self.add_ioc(IOC {
                id: format!("proc_{}", name),
                ioc_type: IOCType::ProcessName,
                value: name.to_string(),
                description: desc.to_string(),
                severity: Severity::High,
                tags: vec!["process".to_string(), "suspicious".to_string()],
            });
        }

        // Suspicious file paths
        let suspicious_paths = vec![
            ("/tmp/", "Temporary directory execution"),
            ("/dev/shm/", "Shared memory execution"),
            (".ssh/authorized_keys", "SSH key modification"),
            ("/etc/passwd", "Password file access"),
            ("/etc/shadow", "Shadow file access"),
        ];

        for (path, desc) in suspicious_paths {
            self.add_ioc(IOC {
                id: format!("path_{}", path.replace('/', "_")),
                ioc_type: IOCType::FilePath,
                value: path.to_string(),
                description: desc.to_string(),
                severity: Severity::Medium,
                tags: vec!["file".to_string(), "suspicious".to_string()],
            });
        }

        // Known malicious IPs (examples)
        let malicious_ips = vec![
            ("0.0.0.0", "Null route"),
            ("127.0.0.1", "Localhost suspicious connection"),
        ];

        for (ip, desc) in malicious_ips {
            self.add_ioc(IOC {
                id: format!("ip_{}", ip.replace('.', "_")),
                ioc_type: IOCType::IpAddress,
                value: ip.to_string(),
                description: desc.to_string(),
                severity: Severity::Low,
                tags: vec!["network".to_string()],
            });
        }
    }

    /// Get IOC by ID
    pub fn get_ioc(&self, id: &str) -> Option<&IOC> {
        self.iocs.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioc_matcher() {
        let mut matcher = IOCMatcher::new();
        matcher.load_iocs().unwrap();
        assert!(matcher.iocs.len() > 0);
    }
}
