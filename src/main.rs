mod types;
mod monitors;
mod detection;
mod response;
mod telemetry;
mod forensics;

use clap::{Parser, Subcommand};
use anyhow::Result;
use tokio::sync::mpsc;
use std::path::PathBuf;

use monitors::{ProcessMonitor, FileMonitor, NetworkMonitor, MemoryMonitor, UserMonitor, RootkitMonitor};
use detection::DetectionEngine;
use response::ResponseEngine;
use telemetry::{TelemetryLogger, EventStore};

#[derive(Parser)]
#[command(name = "rust-edr")]
#[command(about = "A lightweight EDR system for Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the EDR agent
    Start {
        /// Run in foreground mode
        #[arg(short, long)]
        foreground: bool,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Specific modules to enable (comma-separated)
        #[arg(short, long)]
        modules: Option<String>,
        
        /// Threat score threshold (0.0-10.0)
        #[arg(short, long, default_value = "7.0")]
        threshold: f32,
        
        /// Enable auto-response
        #[arg(short, long)]
        auto_response: bool,
    },
    /// Stop the EDR agent
    Stop,
    /// Check agent status
    Status,
    /// View alerts
    Alerts {
        /// Number of recent alerts to show
        #[arg(short, long, default_value = "10")]
        recent: usize,
    },
    /// Manage configuration
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
    },
    /// Forensics tools
    Forensics {
        #[command(subcommand)]
        action: ForensicsAction,
    },
}

#[derive(Subcommand)]
enum ForensicsAction {
    /// Archive threat logs
    Archive {
        /// Threat ID to archive
        threat_id: String,
    },
    /// List all archives
    List,
    /// Extract an archive
    Extract {
        /// Archive file path
        archive: String,
        /// Extract to directory
        #[arg(short, long)]
        output: String,
    },
    /// Compress old logs
    Compress {
        /// Days old to compress
        #[arg(short, long, default_value = "7")]
        days: u64,
    },
    /// Cleanup old archives
    Cleanup {
        /// Keep archives newer than days
        #[arg(short, long, default_value = "90")]
        days: u64,
    },
    /// Capture system snapshot
    Snapshot {
        /// Threat ID
        threat_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { foreground: _, verbose, modules, threshold, auto_response } => {
            println!("🛡️  Starting Rust EDR System...");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            if verbose {
                println!("Verbose logging enabled");
            }
            
            println!("Threat threshold: {}", threshold);
            println!("Auto-response: {}", if auto_response { "enabled" } else { "disabled" });
            
            // Create channels
            let (event_tx, event_rx) = mpsc::channel(1000);
            let (threat_tx, mut threat_rx) = mpsc::channel(100);
            let (response_tx, mut response_rx) = mpsc::channel(100);
            
            // Initialize telemetry
            let log_dir = PathBuf::from("/var/log/rust-edr");
            let mut telemetry = TelemetryLogger::new(log_dir.clone())?;
            let event_store = EventStore::new("/var/lib/rust-edr/events.db")?;
            
            // Initialize detection engine
            let mut detection_engine = DetectionEngine::new(threshold);
            detection_engine.initialize()?;
            
            // Initialize response engine
            let mut response_engine = ResponseEngine::new(auto_response, threshold);
            response_engine.set_response_channel(response_tx.clone());
            
            // Determine which modules to start
            let enabled_modules = modules.unwrap_or_else(|| "process,file,network,user,rootkit".to_string());
            println!("Enabled modules: {}", enabled_modules);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            
            // Start monitoring agents
            if enabled_modules.contains("process") {
                let mut process_monitor = ProcessMonitor::new(event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = process_monitor.start().await {
                        eprintln!("Process monitor error: {}", e);
                    }
                });
            }
            
            if enabled_modules.contains("file") {
                let watch_paths = vec![
                    "/etc".to_string(),
                    "/usr/bin".to_string(),
                    "/home".to_string(),
                ];
                let mut file_monitor = FileMonitor::new(watch_paths, event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = file_monitor.start().await {
                        eprintln!("File monitor error: {}", e);
                    }
                });
            }
            
            if enabled_modules.contains("network") {
                let mut network_monitor = NetworkMonitor::new("any".to_string(), event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = network_monitor.start().await {
                        eprintln!("Network monitor error: {}", e);
                    }
                });
            }
            
            if enabled_modules.contains("memory") {
                let mut memory_monitor = MemoryMonitor::new(event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = memory_monitor.start().await {
                        eprintln!("Memory monitor error: {}", e);
                    }
                });
            }
            
            if enabled_modules.contains("user") {
                let mut user_monitor = UserMonitor::new(event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = user_monitor.start().await {
                        eprintln!("User monitor error: {}", e);
                    }
                });
            }
            
            if enabled_modules.contains("rootkit") {
                let mut rootkit_monitor = RootkitMonitor::new(event_tx.clone());
                tokio::spawn(async move {
                    if let Err(e) = rootkit_monitor.start().await {
                        eprintln!("Rootkit monitor error: {}", e);
                    }
                });
            }
            
            // Start detection engine
            tokio::spawn(async move {
                if let Err(e) = detection_engine.run(event_rx, threat_tx).await {
                    eprintln!("Detection engine error: {}", e);
                }
            });
            
            // Main event loop - handle threats and responses
            println!("✅ EDR System running. Press Ctrl+C to stop.\n");
            
            loop {
                tokio::select! {
                    Some(threat) = threat_rx.recv() => {
                        // Log threat
                        telemetry.log_threat(&threat)?;
                        event_store.store_threat(&threat)?;
                        
                        // Handle with response engine
                        let responses = response_engine.handle_threat(&threat).await;
                        for response in responses {
                            telemetry.log_response(&response)?;
                        }
                    }
                    
                    Some(response) = response_rx.recv() => {
                        telemetry.log_response(&response)?;
                    }
                    
                    _ = tokio::signal::ctrl_c() => {
                        println!("\n🛑 Shutting down EDR system...");
                        break;
                    }
                }
            }
            
            println!("EDR agent stopped successfully");
        }
        Commands::Stop => {
            println!("Stopping EDR agent...");
            // In production, this would send signal to daemon
            println!("EDR agent stopped");
        }
        Commands::Status => {
            println!("EDR Agent Status:");
            println!("  State: Running");
            
            // Try to read event store
            if let Ok(store) = EventStore::new("/var/lib/rust-edr/events.db") {
                println!("  Events Processed: {}", store.event_count());
                println!("  Threats Detected: {}", store.threat_count());
            }
        }
        Commands::Alerts { recent } => {
            println!("Recent {} threats:", recent);
            
            if let Ok(store) = EventStore::new("/var/lib/rust-edr/events.db") {
                if let Ok(threats) = store.get_recent_threats(recent) {
                    for threat in threats {
                        println!("\n🚨 {:?} - Score: {:.2}", threat.threat_type, threat.score);
                        println!("   Time: {}", threat.timestamp);
                        println!("   Description: {}", threat.description);
                        println!("   Severity: {:?}", threat.severity);
                    }
                } else {
                    println!("No threats to display");
                }
            }
        }
        Commands::Config { show } => {
            if show {
                println!("Current configuration:");
                println!("  Config file: /etc/rust-edr/config.toml");
                println!("  Data dir: /var/lib/rust-edr");
                println!("  Log dir: /var/log/rust-edr");
            }
        }
        Commands::Forensics { action } => {
            use forensics::{ForensicArchiver, ForensicSnapshot};
            use std::path::Path;

            let archiver = ForensicArchiver::new("/var/log/rust-edr");
            archiver.init()?;

            match action {
                ForensicsAction::Archive { threat_id } => {
                    println!("📦 Archiving threat: {}", threat_id);
                    
                    // Find related log files
                    let log_files = vec![
                        PathBuf::from(format!("/var/log/rust-edr/threats_{}.jsonl", threat_id)),
                        PathBuf::from(format!("/var/log/rust-edr/events_{}.jsonl", threat_id)),
                    ];
                    
                    archiver.archive_threat_session(&threat_id, log_files)?;
                }
                ForensicsAction::List => {
                    println!("📚 Available Archives:\n");
                    let archives = archiver.list_archives()?;
                    
                    if archives.is_empty() {
                        println!("No archives found");
                    } else {
                        for archive in archives {
                            println!("  📦 {}", archive);
                        }
                    }
                }
                ForensicsAction::Extract { archive, output } => {
                    println!("📤 Extracting archive...");
                    archiver.extract_archive(
                        Path::new(&archive),
                        Path::new(&output)
                    )?;
                }
                ForensicsAction::Compress { days } => {
                    println!("🗜️  Compressing logs older than {} days", days);
                    let compressed = archiver.compress_old_logs(days)?;
                    println!("✅ Compressed {} files", compressed.len());
                }
                ForensicsAction::Cleanup { days } => {
                    println!("🧹 Cleaning up archives older than {} days", days);
                    let removed = archiver.cleanup_old_archives()?;
                    println!("✅ Removed {} archives", removed);
                }
                ForensicsAction::Snapshot { threat_id } => {
                    println!("📸 Capturing forensic snapshot for threat: {}", threat_id);
                    let snapshot = ForensicSnapshot::capture(&threat_id)?;
                    
                    let snapshot_path = PathBuf::from(format!(
                        "/var/log/rust-edr/archives/snapshots/snapshot_{}",
                        threat_id
                    ));
                    
                    snapshot.save(&snapshot_path)?;
                    println!("✅ Snapshot saved");
                }
            }
        }
    }

    Ok(())
}
