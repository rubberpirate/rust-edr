// EDR Agent - Runs on endpoints (VMs)
use edr_common::types::*;
use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use sysinfo::System;
use tokio::time::{interval, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Parser)]
#[command(name = "edr-agent")]
#[command(about = "EDR Agent - Runs on endpoints")]
struct Cli {
    /// Server address
    #[arg(short, long, default_value = "ws://192.168.1.100:8080")]
    server: String,

    /// Endpoint ID
    #[arg(short, long)]
    endpoint_id: Option<String>,

    /// Heartbeat interval in seconds
    #[arg(long, default_value = "5")]
    heartbeat: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let endpoint_id = cli.endpoint_id.unwrap_or_else(|| {
        format!("{}_{}", whoami::hostname(), whoami::username())
    });
    
    println!("🛡️  EDR Agent Starting...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Endpoint ID: {}", endpoint_id);
    println!("Server: {}", cli.server);
    println!("Heartbeat: {}s", cli.heartbeat);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Connect to server
    println!("🔌 Connecting to server...");
    let (ws_stream, _) = connect_async(&cli.server).await?;
    println!("✅ Connected!");
    
    let (mut write, mut read) = ws_stream.split();
    
    // Spawn heartbeat task
    let endpoint_id_clone = endpoint_id.clone();
    let heartbeat_interval = cli.heartbeat;
    tokio::spawn(async move {
        let mut sys = System::new_all();
        let mut interval = interval(Duration::from_secs(heartbeat_interval));
        
        loop {
            interval.tick().await;
            
            // Collect stats
            let stats = collect_system_stats(&mut sys);
            
            let message = AgentMessage {
                endpoint_id: endpoint_id_clone.clone(),
                hostname: whoami::hostname(),
                timestamp: Utc::now(),
                message_type: MessageType::Heartbeat(stats),
            };
            
            if let Ok(json) = serde_json::to_string(&message) {
                if let Err(e) = write.send(Message::Text(json)).await {
                    eprintln!("Failed to send heartbeat: {}", e);
                    break;
                }
            }
        }
    });
    
    // Handle server messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("📥 Server: {}", text);
            }
            Ok(Message::Close(_)) => {
                println!("🔌 Server closed connection");
                break;
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

fn collect_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_all();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage() as f64;
    let memory_used_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let memory_total_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    
    // Disk stats - simplified for now (full disk monitoring requires additional deps)
    let disk_total_gb = 0.0;
    let disk_used_gb = 0.0;
    
    SystemStats {
        cpu_usage,
        memory_used_gb,
        memory_total_gb,
        disk_used_gb,
        disk_total_gb,
        process_count: sys.processes().len(),
        network_connections: 0, // TODO: Implement if needed
        uptime_seconds: System::uptime(),
    }
}
