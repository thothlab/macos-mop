pub mod app_leftovers;
pub mod browser_cache;
pub mod dev_tools;
pub mod docker;
pub mod downloads;
pub mod homebrew;
pub mod system_logs;
pub mod trash;
pub mod user_cache;

use std::path::PathBuf;

/// Result from scanning a cleaning category
#[derive(Debug)]
pub struct CleanTarget {
    pub path: PathBuf,
    pub size: u64,
    pub description: String,
}

/// A cleaning category
pub trait Cleaner {
    /// Human-readable name of this cleaner
    fn name(&self) -> &str;

    /// Scan and return targets that can be cleaned
    fn scan(&self) -> Vec<CleanTarget>;
}
