// Event Store
// Persistent storage for events using embedded database

use crate::types::{SystemEvent, Threat};
use anyhow::Result;
use sled::Db;
use std::path::Path;

pub struct EventStore {
    db: Db,
}

impl EventStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)?;
        println!("Event store initialized");
        Ok(Self { db })
    }

    /// Store a system event
    pub fn store_event(&self, event: &SystemEvent) -> Result<()> {
        let key = format!("event_{}", event.id);
        let value = serde_json::to_vec(event)?;
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Store a threat
    pub fn store_threat(&self, threat: &Threat) -> Result<()> {
        let key = format!("threat_{}", threat.id);
        let value = serde_json::to_vec(threat)?;
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Get event by ID
    pub fn get_event(&self, id: &str) -> Result<Option<SystemEvent>> {
        let key = format!("event_{}", id);
        if let Some(data) = self.db.get(key.as_bytes())? {
            let event: SystemEvent = serde_json::from_slice(&data)?;
            return Ok(Some(event));
        }
        Ok(None)
    }

    /// Get threat by ID
    pub fn get_threat(&self, id: &str) -> Result<Option<Threat>> {
        let key = format!("threat_{}", id);
        if let Some(data) = self.db.get(key.as_bytes())? {
            let threat: Threat = serde_json::from_slice(&data)?;
            return Ok(Some(threat));
        }
        Ok(None)
    }

    /// Get recent events (last N)
    pub fn get_recent_events(&self, count: usize) -> Result<Vec<SystemEvent>> {
        let mut events = Vec::new();
        
        for item in self.db.scan_prefix(b"event_").take(count) {
            if let Ok((_, value)) = item {
                if let Ok(event) = serde_json::from_slice::<SystemEvent>(&value) {
                    events.push(event);
                }
            }
        }

        Ok(events)
    }

    /// Get recent threats (last N)
    pub fn get_recent_threats(&self, count: usize) -> Result<Vec<Threat>> {
        let mut threats = Vec::new();
        
        for item in self.db.scan_prefix(b"threat_").take(count) {
            if let Ok((_, value)) = item {
                if let Ok(threat) = serde_json::from_slice::<Threat>(&value) {
                    threats.push(threat);
                }
            }
        }

        Ok(threats)
    }

    /// Get count of stored events
    pub fn event_count(&self) -> usize {
        self.db.scan_prefix(b"event_").count()
    }

    /// Get count of stored threats
    pub fn threat_count(&self) -> usize {
        self.db.scan_prefix(b"threat_").count()
    }

    /// Clear old data (retention policy)
    pub fn cleanup_old_data(&self, days: u64) -> Result<usize> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
        let mut deleted = 0;

        // Clean events
        for item in self.db.scan_prefix(b"event_") {
            if let Ok((key, value)) = item {
                if let Ok(event) = serde_json::from_slice::<SystemEvent>(&value) {
                    if event.timestamp < cutoff {
                        self.db.remove(&key)?;
                        deleted += 1;
                    }
                }
            }
        }

        // Clean threats
        for item in self.db.scan_prefix(b"threat_") {
            if let Ok((key, value)) = item {
                if let Ok(threat) = serde_json::from_slice::<Threat>(&value) {
                    if threat.timestamp < cutoff {
                        self.db.remove(&key)?;
                        deleted += 1;
                    }
                }
            }
        }

        self.db.flush()?;
        Ok(deleted)
    }
}
