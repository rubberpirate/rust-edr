// EDR Server - Central monitoring dashboard with TUI
mod tui;

use edr_common::types::*;
use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

pub type EndpointState = Arc<RwLock<HashMap<String, EndpointInfo>>>;

#[derive(Debug, Clone)]
pub struct EndpointInfo {
    pub endpoint_id: String,
    pub hostname: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub stats: SystemStats,
    pub recent_processes: Vec<ProcessEvent>,
    pub recent_files: Vec<FileEvent>,
    pub recent_network: Vec<NetworkEvent>,
    pub recent_threats: Vec<ThreatAlert>,
}

#[derive(Parser)]
#[command(name = "edr-server")]
#[command(about = "EDR Central Monitoring Dashboard")]
struct Cli {
    /// Server bind address
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    bind: String,

    /// Disable TUI (log to console)
    #[arg(long)]
    no_tui: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let state: EndpointState = Arc::new(RwLock::new(HashMap::new()));
    
    println!("🛡️  EDR Central Monitoring Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Binding to: {}", cli.bind);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let listener = TcpListener::bind(&cli.bind).await?;
    println!("✅ Server started!");
    println!("📡 Waiting for agents to connect...");
    println!();
    
    // Spawn TUI if enabled
    if !cli.no_tui {
        let state_clone = state.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            if let Err(e) = tui::run_tui(state_clone).await {
                eprintln!("TUI error: {}", e);
            }
        });
    }
    
    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        println!("🔌 New connection from: {}", addr);
        
        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, state_clone).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
    
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, state: EndpointState) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();
    
    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                if let Ok(agent_msg) = serde_json::from_str::<AgentMessage>(&text) {
                    process_agent_message(agent_msg, &state).await;
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                break;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

async fn process_agent_message(msg: AgentMessage, state: &EndpointState) {
    let mut endpoints = state.write().await;
    
    let entry = endpoints.entry(msg.endpoint_id.clone()).or_insert_with(|| {
        EndpointInfo {
            endpoint_id: msg.endpoint_id.clone(),
            hostname: msg.hostname.clone(),
            last_seen: msg.timestamp,
            stats: SystemStats {
                cpu_usage: 0.0,
                memory_used_gb: 0.0,
                memory_total_gb: 0.0,
                disk_used_gb: 0.0,
                disk_total_gb: 0.0,
                process_count: 0,
                network_connections: 0,
                uptime_seconds: 0,
            },
            recent_processes: Vec::new(),
            recent_files: Vec::new(),
            recent_network: Vec::new(),
            recent_threats: Vec::new(),
        }
    });
    
    entry.last_seen = msg.timestamp;
    entry.hostname = msg.hostname;
    
    match msg.message_type {
        MessageType::Heartbeat(stats) => {
            entry.stats = stats;
        }
        MessageType::ProcessEvent(event) => {
            entry.recent_processes.push(event);
            if entry.recent_processes.len() > 100 {
                entry.recent_processes.remove(0);
            }
        }
        MessageType::FileEvent(event) => {
            entry.recent_files.push(event);
            if entry.recent_files.len() > 100 {
                entry.recent_files.remove(0);
            }
        }
        MessageType::NetworkEvent(event) => {
            entry.recent_network.push(event);
            if entry.recent_network.len() > 100 {
                entry.recent_network.remove(0);
            }
        }
        MessageType::ThreatAlert(alert) => {
            entry.recent_threats.push(alert);
            if entry.recent_threats.len() > 50 {
                entry.recent_threats.remove(0);
            }
        }
    }
}
