// Detection Engine module

pub mod ioc;
pub mod rules;
pub mod scoring;
pub mod correlator;
pub mod engine;

pub use engine::DetectionEngine;
pub use ioc::IOCMatcher;
pub use rules::RuleEngine;
pub use scoring::ThreatScorer;
pub use correlator::EventCorrelator;
