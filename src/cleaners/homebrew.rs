use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;
use std::path::PathBuf;

pub struct HomebrewCleaner;

impl Cleaner for HomebrewCleaner {
    fn name(&self) -> &str {
        "Homebrew Cache"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        // Homebrew cache directory
        let brew_cache = home.join("Library").join("Caches").join("Homebrew");
        if brew_cache.exists() {
            let size = dir_size(&brew_cache);
            if size > 1_048_576 {
                targets.push(CleanTarget {
                    path: brew_cache,
                    size,
                    description: "Homebrew download cache".to_string(),
                });
            }
        }

        // Homebrew logs
        let brew_logs = home.join("Library").join("Logs").join("Homebrew");
        if brew_logs.exists() {
            let size = dir_size(&brew_logs);
            if size > 1_048_576 {
                targets.push(CleanTarget {
                    path: brew_logs,
                    size,
                    description: "Homebrew logs".to_string(),
                });
            }
        }

        // Old formula versions in Cellar
        // Check common Homebrew paths
        for cellar_path in &[
            PathBuf::from("/opt/homebrew/Cellar"),
            PathBuf::from("/usr/local/Cellar"),
        ] {
            if cellar_path.exists() {
                scan_old_versions(cellar_path, &mut targets);
            }
        }

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}

fn scan_old_versions(cellar: &PathBuf, targets: &mut Vec<CleanTarget>) {
    if let Ok(formulas) = std::fs::read_dir(cellar) {
        for formula in formulas.filter_map(|e| e.ok()) {
            let formula_path = formula.path();
            if !formula_path.is_dir() {
                continue;
            }

            if let Ok(versions) = std::fs::read_dir(&formula_path) {
                let mut version_dirs: Vec<_> = versions
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .collect();

                // Sort by modification time (newest first)
                version_dirs.sort_by(|a, b| {
                    let a_time = a
                        .metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let b_time = b
                        .metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    b_time.cmp(&a_time)
                });

                // Skip the newest, mark old versions for cleanup
                for old_version in version_dirs.iter().skip(1) {
                    let path = old_version.path();
                    let size = dir_size(&path);
                    if size > 1_048_576 {
                        let formula_name = formula_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let version_name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        targets.push(CleanTarget {
                            path,
                            size,
                            description: format!(
                                "Old Homebrew version: {} {}",
                                formula_name, version_name
                            ),
                        });
                    }
                }
            }
        }
    }
}
