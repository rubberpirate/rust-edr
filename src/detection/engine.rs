// Detection Engine - Main coordinator
// Combines IOC matching, rule evaluation, scoring, and correlation

use crate::types::{SystemEvent, Threat};
use crate::detection::{IOCMatcher, RuleEngine, ThreatScorer, EventCorrelator};
use anyhow::Result;
use tokio::sync::mpsc;

pub struct DetectionEngine {
    ioc_matcher: IOCMatcher,
    rule_engine: RuleEngine,
    threat_scorer: ThreatScorer,
    event_correlator: EventCorrelator,
    threat_threshold: f32,
}

impl DetectionEngine {
    pub fn new(threat_threshold: f32) -> Self {
        Self {
            ioc_matcher: IOCMatcher::new(),
            rule_engine: RuleEngine::new(),
            threat_scorer: ThreatScorer::new(),
            event_correlator: EventCorrelator::new(),
            threat_threshold,
        }
    }

    /// Initialize the detection engine
    pub fn initialize(&mut self) -> Result<()> {
        println!("Initializing Detection Engine...");
        
        // Load IOCs
        self.ioc_matcher.load_iocs()?;
        
        // Load behavioral rules
        self.rule_engine.load_rules()?;
        
        println!("Detection Engine initialized successfully");
        Ok(())
    }

    /// Process an incoming event
    pub fn process_event(&mut self, event: SystemEvent) -> Option<Threat> {
        // Check IOCs
        let ioc_matches = self.ioc_matcher.check_event(&event);
        
        // Check behavioral rules
        let rule_matches = self.rule_engine.check_event(&event);
        
        // Calculate threat score
        let score = self.threat_scorer.score_event(
            &event,
            &ioc_matches,
            &rule_matches,
            &self.ioc_matcher,
            &self.rule_engine,
        );

        // Add to correlator for pattern detection
        self.event_correlator.add_event(event.clone());

        // If score exceeds threshold, create threat
        if self.threat_scorer.exceeds_threshold(score, self.threat_threshold) {
            let severity = self.threat_scorer.score_to_severity(score);
            
            return Some(Threat {
                id: format!("threat_{}", uuid::Uuid::new_v4()),
                timestamp: event.timestamp,
                threat_type: self.determine_threat_type(&event, &rule_matches),
                severity,
                score,
                description: self.generate_threat_description(&event, &ioc_matches, &rule_matches),
                events: vec![event],
                ioc_matches,
                rule_matches,
            });
        }

        None
    }

    /// Check for correlated threats
    pub fn check_correlations(&mut self) -> Vec<Threat> {
        self.event_correlator.correlate()
    }

    /// Determine threat type based on event and matches
    fn determine_threat_type(
        &self,
        event: &SystemEvent,
        rule_matches: &[String],
    ) -> crate::types::ThreatType {
        use crate::types::{ThreatType, EventType};

        // Check rule matches first
        for rule_id in rule_matches {
            if rule_id.contains("ransomware") {
                return ThreatType::Ransomware;
            }
            if rule_id.contains("rootkit") {
                return ThreatType::Rootkit;
            }
            if rule_id.contains("privilege_escalation") {
                return ThreatType::PrivilegeEscalation;
            }
            if rule_id.contains("lateral_movement") {
                return ThreatType::LateralMovement;
            }
            if rule_id.contains("exfiltration") {
                return ThreatType::DataExfiltration;
            }
        }

        // Fall back to event type
        match event.event_type {
            EventType::ProcessCreated | EventType::ProcessModified => ThreatType::SuspiciousProcess,
            EventType::NetworkConnection | EventType::NetworkDnsQuery => ThreatType::SuspiciousNetwork,
            EventType::RootkitDetected => ThreatType::Rootkit,
            EventType::MemoryInjection => ThreatType::Malware,
            _ => ThreatType::AnomalousBehavior,
        }
    }

    /// Generate human-readable threat description
    fn generate_threat_description(
        &self,
        event: &SystemEvent,
        ioc_matches: &[String],
        rule_matches: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!("Event: {:?}", event.event_type));

        if !ioc_matches.is_empty() {
            parts.push(format!("IOC matches: {}", ioc_matches.len()));
        }

        if !rule_matches.is_empty() {
            parts.push(format!("Rule matches: {}", rule_matches.join(", ")));
        }

        parts.join(" | ")
    }

    /// Start detection engine in async mode
    pub async fn run(
        mut self,
        mut event_rx: mpsc::Receiver<SystemEvent>,
        threat_tx: mpsc::Sender<Threat>,
    ) -> Result<()> {
        println!("Detection Engine running...");

        // Correlation check interval
        let mut correlation_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

        loop {
            tokio::select! {
                // Process incoming events
                Some(event) = event_rx.recv() => {
                    if let Some(threat) = self.process_event(event) {
                        let _ = threat_tx.send(threat).await;
                    }
                }
                
                // Periodic correlation checks
                _ = correlation_interval.tick() => {
                    let correlated_threats = self.check_correlations();
                    for threat in correlated_threats {
                        let _ = threat_tx.send(threat).await;
                    }
                }
            }
        }
    }
}
