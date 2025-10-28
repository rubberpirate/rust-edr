// Forensics module - Log compression, archiving, and analysis
pub mod archiver;
pub mod snapshot;
pub mod shell_spawner;

pub use archiver::ForensicArchiver;
pub use snapshot::ForensicSnapshot;
pub use shell_spawner::spawn_investigation_shell;
