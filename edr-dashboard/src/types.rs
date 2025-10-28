// Shared types between server and agent
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub endpoint_id: String,
    pub hostname: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Heartbeat(SystemStats),
    ProcessEvent(ProcessEvent),
    FileEvent(FileEvent),
    NetworkEvent(NetworkEvent),
    ThreatAlert(ThreatAlert),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_usage: f64,
    pub memory_used_gb: f64,
    pub memory_total_gb: f64,
    pub disk_used_gb: f64,
    pub disk_total_gb: f64,
    pub process_count: usize,
    pub network_connections: usize,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEvent {
    pub event_type: ProcessEventType,
    pub pid: u32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub uid: u32,
    pub parent_pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessEventType {
    Created,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    pub event_type: FileEventType,
    pub path: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
    Accessed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub event_type: NetworkEventType,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEventType {
    ConnectionOpened,
    ConnectionClosed,
    DataTransferred { bytes: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub severity: Severity,
    pub threat_type: String,
    pub description: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}
