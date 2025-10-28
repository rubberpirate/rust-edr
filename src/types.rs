// Common types and structures used across the EDR system

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Severity level for events and threats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Type of event detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    ProcessCreated,
    ProcessTerminated,
    ProcessModified,
    FileCreated,
    FileModified,
    FileDeleted,
    FileAccessed,
    NetworkConnection,
    NetworkDnsQuery,
    NetworkHttpRequest,
    MemoryInjection,
    MemoryAllocation,
    UserLogin,
    UserLogout,
    UserElevation,
    RootkitDetected,
    SuspiciousBehavior,
}

/// System event captured by monitoring agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: Severity,
    pub source: String,
    pub details: EventDetails,
}

/// Detailed information about an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventDetails {
    Process(ProcessEvent),
    File(FileEvent),
    Network(NetworkEvent),
    Memory(MemoryEvent),
    User(UserEvent),
    Rootkit(RootkitEvent),
}

/// Process-related event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEvent {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: String,
    pub path: PathBuf,
    pub cmdline: Vec<String>,
    pub user: String,
    pub uid: u32,
}

/// File-related event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    pub path: PathBuf,
    pub operation: String,
    pub process_pid: u32,
    pub process_name: String,
    pub user: String,
    pub permissions: Option<u32>,
}

/// Network-related event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub protocol: String,
    pub src_ip: IpAddr,
    pub src_port: u16,
    pub dst_ip: IpAddr,
    pub dst_port: u16,
    pub process_pid: Option<u32>,
    pub process_name: Option<String>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

/// Memory-related event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub process_pid: u32,
    pub process_name: String,
    pub operation: String,
    pub address: u64,
    pub size: u64,
    pub permissions: String,
}

/// User action event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub username: String,
    pub uid: u32,
    pub action: String,
    pub terminal: Option<String>,
    pub remote_ip: Option<IpAddr>,
}

/// Rootkit detection event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootkitEvent {
    pub detection_type: String,
    pub description: String,
    pub affected_path: Option<PathBuf>,
    pub affected_process: Option<String>,
}

/// Threat detected by the detection engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub threat_type: ThreatType,
    pub severity: Severity,
    pub score: f32,
    pub description: String,
    pub events: Vec<SystemEvent>,
    pub ioc_matches: Vec<String>,
    pub rule_matches: Vec<String>,
}

/// Type of threat detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    Malware,
    Ransomware,
    Rootkit,
    PrivilegeEscalation,
    LateralMovement,
    DataExfiltration,
    SuspiciousProcess,
    SuspiciousNetwork,
    AnomalousBehavior,
}

/// Indicator of Compromise (IOC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOC {
    pub id: String,
    pub ioc_type: IOCType,
    pub value: String,
    pub description: String,
    pub severity: Severity,
    pub tags: Vec<String>,
}

/// Types of IOCs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IOCType {
    FileHash,
    FilePath,
    IpAddress,
    Domain,
    Url,
    ProcessName,
    RegistryKey,
    Mutex,
}

/// Response action to take
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseAction {
    Allow,
    Block,
    Quarantine,
    Alert,
    Kill,
    IsolateNetwork,
}

/// Result of a response action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult {
    pub action: ResponseAction,
    pub success: bool,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}
