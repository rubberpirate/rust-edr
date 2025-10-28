// Log Archiver - Compress and rotate forensic logs
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ForensicArchiver {
    base_dir: PathBuf,
    archive_dir: PathBuf,
    max_log_size_mb: u64,
    retention_days: u64,
}

impl ForensicArchiver {
    pub fn new(base_dir: &str) -> Self {
        let base = PathBuf::from(base_dir);
        let archive = base.join("archives");
        
        Self {
            base_dir: base,
            archive_dir: archive,
            max_log_size_mb: 100, // Compress logs over 100MB
            retention_days: 90,    // Keep archives for 90 days
        }
    }

    /// Initialize forensics directories
    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.archive_dir)
            .context("Failed to create archive directory")?;
        
        // Create subdirectories for organization
        fs::create_dir_all(self.archive_dir.join("threats"))?;
        fs::create_dir_all(self.archive_dir.join("snapshots"))?;
        fs::create_dir_all(self.archive_dir.join("sessions"))?;
        
        println!("📁 Forensic archive initialized at {:?}", self.archive_dir);
        Ok(())
    }

    /// Compress a threat investigation session
    pub fn archive_threat_session(
        &self,
        threat_id: &str,
        log_files: Vec<PathBuf>,
    ) -> Result<PathBuf> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let archive_name = format!("threat_{}_{}.tar.gz", threat_id, timestamp);
        let archive_path = self.archive_dir.join("threats").join(&archive_name);

        println!("🗜️  Compressing threat session: {}", threat_id);
        
        // Create gzip encoder
        let tar_gz = File::create(&archive_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::best());
        let mut tar = tar::Builder::new(enc);

        // Add all log files to archive
        for log_file in log_files {
            if log_file.exists() {
                let file_name = log_file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                tar.append_path_with_name(&log_file, file_name)?;
                println!("  ✓ Added: {}", file_name);
            }
        }

        // Finalize archive
        tar.finish()?;
        
        println!("✅ Archive created: {:?} ({} MB)", 
                 archive_path, 
                 self.get_file_size_mb(&archive_path)?);
        
        Ok(archive_path)
    }

    /// Compress logs by date
    pub fn compress_old_logs(&self, days_old: u64) -> Result<Vec<PathBuf>> {
        let mut compressed = Vec::new();
        let cutoff_date = Utc::now() - chrono::Duration::days(days_old as i64);

        println!("🗜️  Compressing logs older than {} days", days_old);

        // Find JSONL files in log directory
        for entry in WalkDir::new(&self.base_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Only process .jsonl files
            if !path.is_file() || !path.extension().map_or(false, |e| e == "jsonl") {
                continue;
            }

            // Check file age
            let metadata = fs::metadata(path)?;
            let modified: DateTime<Utc> = metadata.modified()?.into();

            if modified < cutoff_date {
                // Compress this file
                let compressed_path = self.compress_single_file(path)?;
                compressed.push(compressed_path);
                
                // Remove original
                fs::remove_file(path)?;
                println!("  ✓ Compressed and removed: {:?}", path);
            }
        }

        println!("✅ Compressed {} log files", compressed.len());
        Ok(compressed)
    }

    /// Compress a single file with gzip
    fn compress_single_file(&self, file_path: &Path) -> Result<PathBuf> {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file name")?;
        
        let compressed_name = format!("{}.gz", file_name);
        let compressed_path = self.archive_dir.join(&compressed_name);

        // Read original file
        let input = fs::read(file_path)?;
        
        // Write compressed
        let output = File::create(&compressed_path)?;
        let mut encoder = GzEncoder::new(output, Compression::best());
        encoder.write_all(&input)?;
        encoder.finish()?;

        Ok(compressed_path)
    }

    /// Clean up old archives based on retention policy
    pub fn cleanup_old_archives(&self) -> Result<usize> {
        let mut removed = 0;
        let cutoff_date = Utc::now() - chrono::Duration::days(self.retention_days as i64);

        println!("🧹 Cleaning archives older than {} days", self.retention_days);

        for entry in WalkDir::new(&self.archive_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }

            let metadata = fs::metadata(path)?;
            let created: DateTime<Utc> = metadata.created()?.into();

            if created < cutoff_date {
                fs::remove_file(path)?;
                removed += 1;
                println!("  ✓ Removed old archive: {:?}", path);
            }
        }

        println!("✅ Removed {} old archives", removed);
        Ok(removed)
    }

    /// Get file size in MB
    fn get_file_size_mb(&self, path: &Path) -> Result<f64> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len() as f64 / 1_048_576.0)
    }

    /// Extract an archived threat session
    pub fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        println!("📦 Extracting archive: {:?}", archive_path);
        
        let tar_gz = File::open(archive_path)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        
        archive.unpack(extract_to)?;
        
        println!("✅ Extracted to: {:?}", extract_to);
        Ok(())
    }

    /// List all archived threat sessions
    pub fn list_archives(&self) -> Result<Vec<ArchiveInfo>> {
        let mut archives = Vec::new();
        let threats_dir = self.archive_dir.join("threats");

        if !threats_dir.exists() {
            return Ok(archives);
        }

        for entry in fs::read_dir(&threats_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }

            let metadata = fs::metadata(&path)?;
            let size_mb = metadata.len() as f64 / 1_048_576.0;
            let created: DateTime<Utc> = metadata.created()?.into();
            
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            archives.push(ArchiveInfo {
                name,
                path,
                size_mb,
                created,
            });
        }

        // Sort by creation date (newest first)
        archives.sort_by(|a, b| b.created.cmp(&a.created));
        
        Ok(archives)
    }

    /// Create a forensic report for a threat
    pub fn create_threat_report(
        &self,
        threat_id: &str,
        events: Vec<String>,
    ) -> Result<PathBuf> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let report_name = format!("report_{}_{}.txt", threat_id, timestamp);
        let report_path = self.archive_dir.join("threats").join(&report_name);

        let mut report = File::create(&report_path)?;
        
        writeln!(report, "═══════════════════════════════════════════════")?;
        writeln!(report, "  RUST EDR - FORENSIC THREAT REPORT")?;
        writeln!(report, "═══════════════════════════════════════════════")?;
        writeln!(report, "Threat ID: {}", threat_id)?;
        writeln!(report, "Generated: {}", Utc::now().to_rfc3339())?;
        writeln!(report, "Event Count: {}", events.len())?;
        writeln!(report, "═══════════════════════════════════════════════\n")?;
        
        writeln!(report, "TIMELINE OF EVENTS:\n")?;
        for (i, event) in events.iter().enumerate() {
            writeln!(report, "{}. {}", i + 1, event)?;
        }
        
        writeln!(report, "\n═══════════════════════════════════════════════")?;
        writeln!(report, "End of Report")?;
        writeln!(report, "═══════════════════════════════════════════════")?;

        println!("📝 Report created: {:?}", report_path);
        Ok(report_path)
    }
}

#[derive(Debug)]
pub struct ArchiveInfo {
    pub name: String,
    pub path: PathBuf,
    pub size_mb: f64,
    pub created: DateTime<Utc>,
}

impl std::fmt::Display for ArchiveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({:.2} MB) - Created: {}",
            self.name,
            self.size_mb,
            self.created.format("%Y-%m-%d %H:%M:%S")
        )
    }
}
