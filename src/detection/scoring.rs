// Threat Scoring System
// Calculates threat scores based on various factors

use crate::types::{SystemEvent, Severity, Threat, ThreatType};
use crate::detection::ioc::IOCMatcher;
use crate::detection::rules::RuleEngine;

pub struct ThreatScorer {
    base_scores: std::collections::HashMap<Severity, f32>,
}

impl ThreatScorer {
    pub fn new() -> Self {
        let mut base_scores = std::collections::HashMap::new();
        base_scores.insert(Severity::Info, 1.0);
        base_scores.insert(Severity::Low, 2.5);
        base_scores.insert(Severity::Medium, 5.0);
        base_scores.insert(Severity::High, 7.5);
        base_scores.insert(Severity::Critical, 10.0);

        Self { base_scores }
    }

    /// Calculate threat score for an event
    pub fn score_event(
        &self,
        event: &SystemEvent,
        ioc_matches: &[String],
        rule_matches: &[String],
        ioc_matcher: &IOCMatcher,
        rule_engine: &RuleEngine,
    ) -> f32 {
        let mut score = 0.0;

        // Base score from event severity
        score += self.base_scores.get(&event.severity).unwrap_or(&1.0);

        // Add points for IOC matches
        for ioc_id in ioc_matches {
            if let Some(ioc) = ioc_matcher.get_ioc(ioc_id) {
                score += self.base_scores.get(&ioc.severity).unwrap_or(&2.0);
            }
        }

        // Add points for rule matches
        for rule_id in rule_matches {
            if let Some(rule) = rule_engine.get_rule(rule_id) {
                score += self.base_scores.get(&rule.severity).unwrap_or(&2.0);
            }
        }

        // Multiplier for multiple matches (correlation)
        if !ioc_matches.is_empty() && !rule_matches.is_empty() {
            score *= 1.5;
        }

        // Cap at 10.0
        score.min(10.0)
    }

    /// Calculate overall threat score from multiple events
    pub fn score_threat(&self, events: &[SystemEvent], threat_type: &ThreatType) -> f32 {
        let mut total_score = 0.0;

        for event in events {
            total_score += self.base_scores.get(&event.severity).unwrap_or(&1.0);
        }

        // Apply threat type multiplier
        let type_multiplier = match threat_type {
            ThreatType::Ransomware => 2.0,
            ThreatType::Rootkit => 1.8,
            ThreatType::DataExfiltration => 1.5,
            ThreatType::PrivilegeEscalation => 1.5,
            ThreatType::Malware => 1.3,
            ThreatType::LateralMovement => 1.2,
            _ => 1.0,
        };

        total_score *= type_multiplier;

        // Normalize to 0-10 scale
        (total_score / events.len() as f32).min(10.0)
    }

    /// Determine severity from score
    pub fn score_to_severity(&self, score: f32) -> Severity {
        match score {
            s if s < 2.0 => Severity::Info,
            s if s < 4.0 => Severity::Low,
            s if s < 6.0 => Severity::Medium,
            s if s < 8.0 => Severity::High,
            _ => Severity::Critical,
        }
    }

    /// Check if score exceeds threshold for action
    pub fn exceeds_threshold(&self, score: f32, threshold: f32) -> bool {
        score >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring() {
        let scorer = ThreatScorer::new();
        assert_eq!(scorer.score_to_severity(1.5), Severity::Info);
        assert_eq!(scorer.score_to_severity(5.5), Severity::Medium);
        assert_eq!(scorer.score_to_severity(9.0), Severity::Critical);
    }
}
