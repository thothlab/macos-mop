use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Entry found during scanning
#[derive(Debug, Clone)]
pub struct ScanEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
}

/// Scan a directory and return entries sorted by size (largest first)
pub fn scan_directory(path: &Path, max_depth: usize) -> Vec<ScanEntry> {
    let mut entries: Vec<ScanEntry> = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(max_depth)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = if meta.is_dir() {
            super::size::dir_size(entry.path())
        } else {
            meta.len()
        };

        entries.push(ScanEntry {
            path: entry.path().to_path_buf(),
            size,
            is_dir: meta.is_dir(),
        });
    }

    entries.sort_by(|a, b| b.size.cmp(&a.size));
    entries
}

/// Scan and group by file extension
pub fn scan_by_extension(path: &Path) -> HashMap<String, u64> {
    let mut extensions: HashMap<String, u64> = HashMap::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let ext = entry
            .path()
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_else(|| "no extension".to_string());

        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        *extensions.entry(ext).or_insert(0) += size;
    }

    extensions
}

/// Find directories matching a pattern name (e.g. "node_modules", "target")
pub fn find_directories_by_name(root: &Path, names: &[&str]) -> Vec<ScanEntry> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        if let Some(dir_name) = entry.path().file_name() {
            let name_str = dir_name.to_string_lossy();
            if names.iter().any(|&n| name_str == n) {
                let size = super::size::dir_size(entry.path());
                results.push(ScanEntry {
                    path: entry.path().to_path_buf(),
                    size,
                    is_dir: true,
                });
            }
        }
    }

    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
}

/// Find files matching extensions
pub fn find_files_by_extension(root: &Path, extensions: &[&str]) -> Vec<ScanEntry> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Some(ext) = entry.path().extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if extensions.iter().any(|&e| ext_str == e) {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                results.push(ScanEntry {
                    path: entry.path().to_path_buf(),
                    size,
                    is_dir: false,
                });
            }
        }
    }

    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
}
