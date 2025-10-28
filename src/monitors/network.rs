// Network monitoring module
// Tracks network connections and suspicious activity

use crate::types::{SystemEvent, EventType, EventDetails, NetworkEvent, Severity};
use anyhow::Result;
use tokio::sync::mpsc;
use chrono::Utc;
use std::net::IpAddr;
use std::time::Duration;
use std::collections::HashSet;

pub struct NetworkMonitor {
    enabled: bool,
    interface: String,
    event_tx: mpsc::Sender<SystemEvent>,
    known_connections: HashSet<String>,
}

impl NetworkMonitor {
    pub fn new(interface: String, event_tx: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            enabled: false,
            interface,
            event_tx,
            known_connections: HashSet::new(),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting network monitor on interface: {}", self.interface);
        self.enabled = true;

        // Start monitoring loop
        self.monitor_loop().await?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping network monitor...");
        self.enabled = false;
        Ok(())
    }

    /// Main monitoring loop
    async fn monitor_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        while self.enabled {
            interval.tick().await;
            self.scan_connections().await?;
        }

        Ok(())
    }

    /// Scan /proc/net for active connections
    async fn scan_connections(&mut self) -> Result<()> {
        let mut current_connections = HashSet::new();

        // Read TCP connections
        if let Ok(tcp) = std::fs::read_to_string("/proc/net/tcp") {
            for line in tcp.lines().skip(1) {
                if let Some(conn_info) = self.parse_tcp_line(line) {
                    let conn_key = format!("{:?}", conn_info);
                    current_connections.insert(conn_key.clone());

                    // New connection detected
                    if !self.known_connections.contains(&conn_key) {
                        self.report_connection(conn_info, "tcp").await;
                    }
                }
            }
        }

        // Read UDP connections
        if let Ok(udp) = std::fs::read_to_string("/proc/net/udp") {
            for line in udp.lines().skip(1) {
                if let Some(conn_info) = self.parse_tcp_line(line) {
                    let conn_key = format!("{:?}", conn_info);
                    current_connections.insert(conn_key.clone());

                    if !self.known_connections.contains(&conn_key) {
                        self.report_connection(conn_info, "udp").await;
                    }
                }
            }
        }

        self.known_connections = current_connections;
        Ok(())
    }

    /// Parse /proc/net/tcp line
    fn parse_tcp_line(&self, line: &str) -> Option<(IpAddr, u16, IpAddr, u16)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        // Parse local address
        let local_parts: Vec<&str> = parts[1].split(':').collect();
        if local_parts.len() != 2 {
            return None;
        }

        // Parse remote address
        let remote_parts: Vec<&str> = parts[2].split(':').collect();
        if remote_parts.len() != 2 {
            return None;
        }

        let src_ip = self.parse_hex_ip(local_parts[0])?;
        let src_port = u16::from_str_radix(local_parts[1], 16).ok()?;
        let dst_ip = self.parse_hex_ip(remote_parts[0])?;
        let dst_port = u16::from_str_radix(remote_parts[1], 16).ok()?;

        Some((src_ip, src_port, dst_ip, dst_port))
    }

    /// Parse hex IP address from /proc/net format
    fn parse_hex_ip(&self, hex: &str) -> Option<IpAddr> {
        if hex.len() == 8 {
            // IPv4
            let ip_num = u32::from_str_radix(hex, 16).ok()?;
            let octets = [
                (ip_num & 0xFF) as u8,
                ((ip_num >> 8) & 0xFF) as u8,
                ((ip_num >> 16) & 0xFF) as u8,
                ((ip_num >> 24) & 0xFF) as u8,
            ];
            Some(IpAddr::from(octets))
        } else {
            None
        }
    }

    /// Report new network connection
    async fn report_connection(
        &self,
        conn: (IpAddr, u16, IpAddr, u16),
        protocol: &str,
    ) {
        let (src_ip, src_port, dst_ip, dst_port) = conn;

        // Assess severity
        let severity = self.assess_connection_severity(&dst_ip, dst_port);

        let event = SystemEvent {
            id: format!("net_{}", uuid::Uuid::new_v4()),
            timestamp: Utc::now(),
            event_type: EventType::NetworkConnection,
            severity,
            source: "network_monitor".to_string(),
            details: EventDetails::Network(NetworkEvent {
                protocol: protocol.to_string(),
                src_ip,
                src_port,
                dst_ip,
                dst_port,
                process_pid: None,
                process_name: None,
                bytes_sent: 0,
                bytes_received: 0,
            }),
        };

        let _ = self.event_tx.send(event).await;
    }

    /// Assess connection severity
    fn assess_connection_severity(&self, dst_ip: &IpAddr, dst_port: u16) -> Severity {
        // Suspicious ports
        let suspicious_ports = vec![4444, 31337, 1337, 8888, 9999, 6666, 12345];
        if suspicious_ports.contains(&dst_port) {
            return Severity::High;
        }

        // Localhost connections
        if dst_ip.to_string().starts_with("127.") {
            return Severity::Info;
        }

        // Default
        Severity::Low
    }
}

