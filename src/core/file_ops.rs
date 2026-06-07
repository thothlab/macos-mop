use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::protection::is_protected;

/// Result of a file operation
pub struct CleanResult {
    pub path: String,
    pub size: u64,
    pub removed: bool,
    pub error: Option<String>,
}

/// Safely remove a file or directory
/// Returns the size that was freed
pub fn safe_remove(path: &Path, dry_run: bool) -> Result<CleanResult> {
    let size = super::size::entry_size(path);
    let path_str = path.to_string_lossy().to_string();

    if is_protected(path) {
        return Ok(CleanResult {
            path: path_str,
            size,
            removed: false,
            error: Some("Protected path, skipping".to_string()),
        });
    }

    if dry_run {
        return Ok(CleanResult {
            path: path_str,
            size,
            removed: false,
            error: None,
        });
    }

    let result = if path.is_dir() {
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove directory: {}", path.display()))
    } else {
        fs::remove_file(path).with_context(|| format!("Failed to remove file: {}", path.display()))
    };

    match result {
        Ok(()) => Ok(CleanResult {
            path: path_str,
            size,
            removed: true,
            error: None,
        }),
        Err(e) => Ok(CleanResult {
            path: path_str,
            size,
            removed: false,
            error: Some(e.to_string()),
        }),
    }
}

/// Remove multiple paths, collecting results
pub fn batch_remove(paths: &[&Path], dry_run: bool) -> Vec<CleanResult> {
    paths
        .iter()
        .filter_map(|p| safe_remove(p, dry_run).ok())
        .collect()
}

/// Move to trash instead of deleting (macOS)
pub fn move_to_trash(path: &Path) -> Result<()> {
    let status = std::process::Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Finder\" to delete POSIX file \"{}\"",
            path.display()
        ))
        .output()
        .context("Failed to run osascript")?;

    if status.status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "Failed to move to trash: {}",
            String::from_utf8_lossy(&status.stderr)
        )
    }
}
