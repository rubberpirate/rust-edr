// Forensic Snapshot - Capture system state for analysis
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ForensicSnapshot {
    pub timestamp: String,
    pub threat_id: String,
    pub processes: Vec<ProcessInfo>,
    pub network_connections: Vec<NetworkConnection>,
    pub open_files: Vec<OpenFile>,
    pub memory_maps: Vec<MemoryMap>,
    pub system_info: SystemInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub uid: u32,
    pub parent_pid: Option<u32>,
    pub status: String,
    pub memory_kb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub state: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenFile {
    pub pid: u32,
    pub path: String,
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryMap {
    pub pid: u32,
    pub address: String,
    pub permissions: String,
    pub path: Option<String>,
    pub size_kb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub kernel: String,
    pub uptime_seconds: u64,
    pub load_average: Vec<f64>,
    pub total_processes: usize,
}

impl ForensicSnapshot {
    /// Capture a complete system snapshot
    pub fn capture(threat_id: &str) -> Result<Self> {
        println!("📸 Capturing forensic snapshot for threat: {}", threat_id);

        let snapshot = Self {
            timestamp: Utc::now().to_rfc3339(),
            threat_id: threat_id.to_string(),
            processes: Self::capture_processes()?,
            network_connections: Self::capture_network()?,
            open_files: Self::capture_open_files()?,
            memory_maps: Self::capture_memory_maps()?,
            system_info: Self::capture_system_info()?,
        };

        println!("✅ Snapshot captured: {} processes, {} connections",
                 snapshot.processes.len(),
                 snapshot.network_connections.len());

        Ok(snapshot)
    }

    /// Save snapshot to file
    pub fn save(&self, path: &Path) -> Result<()> {
        // Save as JSON
        let json_path = path.with_extension("json");
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(&json_path, json)?;
        println!("💾 Snapshot saved: {:?}", json_path);

        // Also save human-readable version
        let txt_path = path.with_extension("txt");
        self.save_text(&txt_path)?;
        println!("💾 Text version saved: {:?}", txt_path);

        Ok(())
    }

    /// Save human-readable text version
    fn save_text(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "═══════════════════════════════════════════════")?;
        writeln!(file, "  FORENSIC SNAPSHOT")?;
        writeln!(file, "═══════════════════════════════════════════════")?;
        writeln!(file, "Threat ID: {}", self.threat_id)?;
        writeln!(file, "Timestamp: {}", self.timestamp)?;
        writeln!(file, "═══════════════════════════════════════════════\n")?;

        writeln!(file, "SYSTEM INFORMATION:")?;
        writeln!(file, "  Hostname: {}", self.system_info.hostname)?;
        writeln!(file, "  Kernel: {}", self.system_info.kernel)?;
        writeln!(file, "  Uptime: {} seconds", self.system_info.uptime_seconds)?;
        writeln!(file, "  Load Average: {:?}", self.system_info.load_average)?;
        writeln!(file, "  Total Processes: {}\n", self.system_info.total_processes)?;

        writeln!(file, "PROCESSES ({}):", self.processes.len())?;
        for proc in &self.processes {
            writeln!(file, "  PID {}: {} (UID: {}, MEM: {} KB)",
                     proc.pid, proc.name, proc.uid, proc.memory_kb)?;
            if !proc.cmdline.is_empty() {
                writeln!(file, "    CMD: {}", proc.cmdline.join(" "))?;
            }
        }
        writeln!(file)?;

        writeln!(file, "NETWORK CONNECTIONS ({}):", self.network_connections.len())?;
        for conn in &self.network_connections {
            writeln!(file, "  {} {}:{} -> {}:{} [{}]",
                     conn.protocol,
                     conn.local_addr, conn.local_port,
                     conn.remote_addr, conn.remote_port,
                     conn.state)?;
        }
        writeln!(file)?;

        writeln!(file, "OPEN FILES ({}):", self.open_files.len())?;
        for file_info in &self.open_files {
            writeln!(file, "  PID {}: {} ({})", file_info.pid, file_info.path, file_info.mode)?;
        }
        writeln!(file)?;

        writeln!(file, "MEMORY MAPS ({}):", self.memory_maps.len())?;
        for map in &self.memory_maps {
            writeln!(file, "  PID {}: {} [{}] - {} KB",
                     map.pid, map.address, map.permissions, map.size_kb)?;
            if let Some(ref path) = map.path {
                writeln!(file, "    Path: {}", path)?;
            }
        }

        Ok(())
    }

    /// Capture running processes
    fn capture_processes() -> Result<Vec<ProcessInfo>> {
        let mut processes = Vec::new();
        
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Ok(proc_info) = Self::read_process_info(pid) {
                            processes.push(proc_info);
                        }
                    }
                }
            }
        }

        Ok(processes)
    }

    /// Read process information from /proc
    fn read_process_info(pid: u32) -> Result<ProcessInfo> {
        let proc_path = format!("/proc/{}", pid);

        // Read cmdline
        let cmdline_path = format!("{}/cmdline", proc_path);
        let cmdline = fs::read_to_string(&cmdline_path)
            .unwrap_or_default()
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        // Read status
        let status_path = format!("{}/status", proc_path);
        let status_content = fs::read_to_string(&status_path)?;
        
        let mut name = String::from("unknown");
        let mut uid = 0;
        let mut ppid = None;
        let mut memory_kb = 0;

        for line in status_content.lines() {
            if line.starts_with("Name:") {
                name = line.split_whitespace().nth(1).unwrap_or("unknown").to_string();
            } else if line.starts_with("Uid:") {
                uid = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("PPid:") {
                ppid = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse().ok());
            } else if line.starts_with("VmRSS:") {
                memory_kb = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            }
        }

        Ok(ProcessInfo {
            pid,
            name,
            cmdline,
            uid,
            parent_pid: ppid,
            status: "running".to_string(),
            memory_kb,
        })
    }

    /// Capture network connections
    fn capture_network() -> Result<Vec<NetworkConnection>> {
        let mut connections = Vec::new();

        // Parse /proc/net/tcp
        if let Ok(tcp_conns) = Self::parse_network_file("/proc/net/tcp") {
            connections.extend(tcp_conns);
        }

        // Parse /proc/net/tcp6
        if let Ok(tcp6_conns) = Self::parse_network_file("/proc/net/tcp6") {
            connections.extend(tcp6_conns);
        }

        Ok(connections)
    }

    /// Parse network file (/proc/net/tcp or /proc/net/tcp6)
    fn parse_network_file(path: &str) -> Result<Vec<NetworkConnection>> {
        let mut connections = Vec::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                continue; // Skip header
            }

            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() < 10 {
                continue;
            }

            // Parse local address
            let local_parts: Vec<&str> = parts[1].split(':').collect();
            let local_addr = Self::parse_hex_ip(local_parts[0]);
            let local_port = u16::from_str_radix(local_parts[1], 16).unwrap_or(0);

            // Parse remote address
            let remote_parts: Vec<&str> = parts[2].split(':').collect();
            let remote_addr = Self::parse_hex_ip(remote_parts[0]);
            let remote_port = u16::from_str_radix(remote_parts[1], 16).unwrap_or(0);

            // Parse state
            let state = match parts[3] {
                "01" => "ESTABLISHED",
                "02" => "SYN_SENT",
                "03" => "SYN_RECV",
                "0A" => "LISTEN",
                _ => "UNKNOWN",
            }.to_string();

            connections.push(NetworkConnection {
                protocol: "TCP".to_string(),
                local_addr,
                local_port,
                remote_addr,
                remote_port,
                state,
                pid: None,
            });
        }

        Ok(connections)
    }

    /// Parse hexadecimal IP address
    fn parse_hex_ip(hex: &str) -> String {
        if hex.len() == 8 {
            // IPv4
            let bytes = (0..4)
                .map(|i| u8::from_str_radix(&hex[i*2..i*2+2], 16).unwrap_or(0))
                .collect::<Vec<_>>();
            format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
        } else {
            // IPv6 or invalid
            hex.to_string()
        }
    }

    /// Capture open files (limited to first 100 for performance)
    fn capture_open_files() -> Result<Vec<OpenFile>> {
        let mut open_files = Vec::new();
        let mut count = 0;

        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.filter_map(|e| e.ok()) {
                if count >= 100 {
                    break; // Limit to avoid huge output
                }

                let path = entry.path();
                if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        let fd_dir = format!("/proc/{}/fd", pid);
                        if let Ok(fd_entries) = fs::read_dir(&fd_dir) {
                            for fd_entry in fd_entries.filter_map(|e| e.ok()) {
                                if let Ok(link) = fs::read_link(fd_entry.path()) {
                                    open_files.push(OpenFile {
                                        pid,
                                        path: link.to_string_lossy().to_string(),
                                        mode: "r".to_string(),
                                    });
                                    count += 1;
                                    if count >= 100 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(open_files)
    }

    /// Capture memory maps (limited sample)
    fn capture_memory_maps() -> Result<Vec<MemoryMap>> {
        let mut maps = Vec::new();
        let mut count = 0;

        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.filter_map(|e| e.ok()) {
                if count >= 50 {
                    break; // Limit to avoid huge output
                }

                let path = entry.path();
                if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        let maps_path = format!("/proc/{}/maps", pid);
                        if let Ok(content) = fs::read_to_string(&maps_path) {
                            for line in content.lines().take(5) {
                                let parts: Vec<&str> = line.split_whitespace().collect();
                                if parts.len() >= 5 {
                                    let address = parts[0].to_string();
                                    let permissions = parts[1].to_string();
                                    let path_opt = if parts.len() > 5 {
                                        Some(parts[5..].join(" "))
                                    } else {
                                        None
                                    };

                                    maps.push(MemoryMap {
                                        pid,
                                        address,
                                        permissions,
                                        path: path_opt,
                                        size_kb: 0,
                                    });
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(maps)
    }

    /// Capture system information
    fn capture_system_info() -> Result<SystemInfo> {
        let hostname = whoami::hostname();
        
        // Read kernel version
        let kernel = fs::read_to_string("/proc/version")
            .unwrap_or_else(|_| "Unknown".to_string())
            .lines()
            .next()
            .unwrap_or("Unknown")
            .to_string();

        // Read uptime
        let uptime_str = fs::read_to_string("/proc/uptime")?;
        let uptime_seconds = uptime_str.split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) as u64;

        // Read load average
        let loadavg_str = fs::read_to_string("/proc/loadavg")?;
        let load_average: Vec<f64> = loadavg_str.split_whitespace()
            .take(3)
            .filter_map(|s| s.parse().ok())
            .collect();

        // Count processes
        let total_processes = fs::read_dir("/proc")?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .and_then(|s| s.parse::<u32>().ok())
                    .is_some()
            })
            .count();

        Ok(SystemInfo {
            hostname,
            kernel,
            uptime_seconds,
            load_average,
            total_processes,
        })
    }
}
