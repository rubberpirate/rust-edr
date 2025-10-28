// Event Correlator
// Correlates multiple events to detect complex attack patterns

use crate::types::{SystemEvent, EventType, ThreatType, Threat, Severity};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use chrono::Utc;

pub struct EventCorrelator {
    // Store recent events for correlation
    event_window: VecDeque<(Instant, SystemEvent)>,
    window_duration: Duration,
    
    // Track event sequences by process
    process_events: HashMap<u32, Vec<SystemEvent>>,
    
    // Track event patterns
    pattern_cache: HashMap<String, Vec<SystemEvent>>,
}

impl EventCorrelator {
    pub fn new() -> Self {
        Self {
            event_window: VecDeque::new(),
            window_duration: Duration::from_secs(300), // 5 minute window
            process_events: HashMap::new(),
            pattern_cache: HashMap::new(),
        }
    }

    /// Add an event to the correlation engine
    pub fn add_event(&mut self, event: SystemEvent) {
        let now = Instant::now();
        
        // Clean old events
        self.clean_old_events(now);
        
        // Add to window
        self.event_window.push_back((now, event.clone()));
        
        // Track by process if applicable
        if let Some(pid) = self.extract_pid(&event) {
            self.process_events.entry(pid)
                .or_insert_with(Vec::new)
                .push(event.clone());
        }
    }

    /// Correlate events and detect attack patterns
    pub fn correlate(&mut self) -> Vec<Threat> {
        let mut threats = Vec::new();

        // Pattern 1: Privilege escalation chain
        if let Some(threat) = self.detect_privilege_escalation_chain() {
            threats.push(threat);
        }

        // Pattern 2: Data exfiltration
        if let Some(threat) = self.detect_data_exfiltration() {
            threats.push(threat);
        }

        // Pattern 3: Lateral movement
        if let Some(threat) = self.detect_lateral_movement() {
            threats.push(threat);
        }

        // Pattern 4: Ransomware behavior
        if let Some(threat) = self.detect_ransomware_behavior() {
            threats.push(threat);
        }

        // Pattern 5: Rootkit installation
        if let Some(threat) = self.detect_rootkit_installation() {
            threats.push(threat);
        }

        threats
    }

    /// Detect privilege escalation attack chain
    fn detect_privilege_escalation_chain(&self) -> Option<Threat> {
        // Look for: user action -> process spawn -> file modification -> elevated process
        let mut chain_events = Vec::new();
        
        for (_, event) in &self.event_window {
            match event.event_type {
                EventType::UserElevation | 
                EventType::ProcessCreated | 
                EventType::FileModified => {
                    chain_events.push(event.clone());
                }
                _ => {}
            }
        }

        if chain_events.len() >= 3 {
            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                threat_type: ThreatType::PrivilegeEscalation,
                severity: Severity::High,
                score: 7.5,
                description: "Privilege escalation chain detected".to_string(),
                events: chain_events,
                ioc_matches: vec![],
                rule_matches: vec!["privilege_escalation_chain".to_string()],
            });
        }

        None
    }

    /// Detect data exfiltration
    fn detect_data_exfiltration(&self) -> Option<Threat> {
        // Look for: file access -> large network transfer
        let mut file_events = Vec::new();
        let mut network_events = Vec::new();
        
        for (_, event) in &self.event_window {
            match event.event_type {
                EventType::FileAccessed => file_events.push(event.clone()),
                EventType::NetworkConnection => network_events.push(event.clone()),
                _ => {}
            }
        }

        if !file_events.is_empty() && !network_events.is_empty() {
            let mut combined_events = file_events;
            combined_events.extend(network_events);

            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                threat_type: ThreatType::DataExfiltration,
                severity: Severity::High,
                score: 8.0,
                description: "Potential data exfiltration detected".to_string(),
                events: combined_events,
                ioc_matches: vec![],
                rule_matches: vec!["data_exfiltration".to_string()],
            });
        }

        None
    }

    /// Detect lateral movement
    fn detect_lateral_movement(&self) -> Option<Threat> {
        // Look for: network connections + process spawns + authentication
        let mut relevant_events = Vec::new();
        
        for (_, event) in &self.event_window {
            match event.event_type {
                EventType::NetworkConnection |
                EventType::ProcessCreated |
                EventType::UserLogin => {
                    relevant_events.push(event.clone());
                }
                _ => {}
            }
        }

        if relevant_events.len() >= 3 {
            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                threat_type: ThreatType::LateralMovement,
                severity: Severity::High,
                score: 7.0,
                description: "Lateral movement detected".to_string(),
                events: relevant_events,
                ioc_matches: vec![],
                rule_matches: vec!["lateral_movement".to_string()],
            });
        }

        None
    }

    /// Detect ransomware behavior
    fn detect_ransomware_behavior(&self) -> Option<Threat> {
        // Look for: rapid file modifications + file deletions
        let mut file_mod_count = 0;
        let mut file_del_count = 0;
        let mut relevant_events = Vec::new();
        
        for (_, event) in &self.event_window {
            match event.event_type {
                EventType::FileModified => {
                    file_mod_count += 1;
                    relevant_events.push(event.clone());
                }
                EventType::FileDeleted => {
                    file_del_count += 1;
                    relevant_events.push(event.clone());
                }
                _ => {}
            }
        }

        // Ransomware typically modifies many files quickly
        if file_mod_count > 10 || file_del_count > 5 {
            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                threat_type: ThreatType::Ransomware,
                severity: Severity::Critical,
                score: 9.5,
                description: format!("Ransomware behavior: {} files modified, {} deleted", 
                                   file_mod_count, file_del_count),
                events: relevant_events,
                ioc_matches: vec![],
                rule_matches: vec!["ransomware_behavior".to_string()],
            });
        }

        None
    }

    /// Detect rootkit installation
    fn detect_rootkit_installation(&self) -> Option<Threat> {
        // Look for: system file modification + kernel module loading + hidden processes
        let mut relevant_events = Vec::new();
        
        for (_, event) in &self.event_window {
            match event.event_type {
                EventType::RootkitDetected |
                EventType::FileModified => {
                    relevant_events.push(event.clone());
                }
                _ => {}
            }
        }

        if !relevant_events.is_empty() {
            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: Utc::now(),
                threat_type: ThreatType::Rootkit,
                severity: Severity::Critical,
                score: 9.0,
                description: "Rootkit installation detected".to_string(),
                events: relevant_events,
                ioc_matches: vec![],
                rule_matches: vec!["rootkit_installation".to_string()],
            });
        }

        None
    }

    /// Clean events older than the window duration
    fn clean_old_events(&mut self, now: Instant) {
        while let Some((timestamp, _)) = self.event_window.front() {
            if now.duration_since(*timestamp) > self.window_duration {
                self.event_window.pop_front();
            } else {
                break;
            }
        }

        // Clean process events cache
        self.process_events.retain(|_, events| !events.is_empty());
    }

    /// Extract PID from event if available
    fn extract_pid(&self, event: &SystemEvent) -> Option<u32> {
        match &event.details {
            crate::types::EventDetails::Process(p) => Some(p.pid),
            crate::types::EventDetails::File(f) => Some(f.process_pid),
            crate::types::EventDetails::Network(n) => n.process_pid,
            crate::types::EventDetails::Memory(m) => Some(m.process_pid),
            _ => None,
        }
    }
}
