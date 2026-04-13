use bytesize::ByteSize;
use std::path::Path;
use walkdir::WalkDir;

/// Calculate total size of a directory recursively
pub fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

/// Calculate size of a single file or directory
pub fn entry_size(path: &Path) -> u64 {
    if path.is_file() {
        path.metadata().map(|m| m.len()).unwrap_or(0)
    } else if path.is_dir() {
        dir_size(path)
    } else {
        0
    }
}

/// Format bytes as human-readable string
pub fn format_size(bytes: u64) -> String {
    ByteSize(bytes).to_string()
}

/// Format size with color based on magnitude
pub fn format_size_colored(bytes: u64) -> String {
    let size_str = format_size(bytes);
    if bytes >= 1_073_741_824 {
        // >= 1 GB - red
        format!("\x1b[31m{}\x1b[0m", size_str)
    } else if bytes >= 104_857_600 {
        // >= 100 MB - yellow
        format!("\x1b[33m{}\x1b[0m", size_str)
    } else if bytes >= 1_048_576 {
        // >= 1 MB - green
        format!("\x1b[32m{}\x1b[0m", size_str)
    } else {
        size_str
    }
}
