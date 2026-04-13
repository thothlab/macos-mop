use super::{CleanTarget, Cleaner};
use crate::core::size::{dir_size, entry_size};
use std::path::PathBuf;

pub struct SystemLogsCleaner;

impl Cleaner for SystemLogsCleaner {
    fn name(&self) -> &str {
        "System Logs"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        // User logs
        let user_logs = home.join("Library").join("Logs");
        if user_logs.exists() {
            scan_log_dir(&user_logs, "User log", &mut targets);
        }

        // System logs (may need sudo)
        let system_log = PathBuf::from("/var/log");
        if system_log.exists() {
            // Only scan readable files
            scan_log_dir(&system_log, "System log", &mut targets);
        }

        // ASL logs
        let asl_logs = PathBuf::from("/private/var/log/asl");
        if asl_logs.exists() {
            let size = dir_size(&asl_logs);
            if size > 1_048_576 {
                targets.push(CleanTarget {
                    path: asl_logs,
                    size,
                    description: "ASL logs".to_string(),
                });
            }
        }

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}

pub struct CrashReportsCleaner;

impl Cleaner for CrashReportsCleaner {
    fn name(&self) -> &str {
        "Crash Reports"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        let diagnostic_reports = home.join("Library").join("Logs").join("DiagnosticReports");
        if diagnostic_reports.exists() {
            let size = dir_size(&diagnostic_reports);
            if size > 0 {
                targets.push(CleanTarget {
                    path: diagnostic_reports,
                    size,
                    description: "Diagnostic reports".to_string(),
                });
            }
        }

        // Retired crash reports
        let retired = home
            .join("Library")
            .join("Logs")
            .join("DiagnosticReports")
            .join("Retired");
        if retired.exists() {
            let size = dir_size(&retired);
            if size > 0 {
                targets.push(CleanTarget {
                    path: retired,
                    size,
                    description: "Retired crash reports".to_string(),
                });
            }
        }

        targets
    }
}

fn scan_log_dir(dir: &PathBuf, prefix: &str, targets: &mut Vec<CleanTarget>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let size = if path.is_dir() {
                dir_size(&path)
            } else {
                entry_size(&path)
            };

            if size > 1_048_576 {
                targets.push(CleanTarget {
                    path,
                    size,
                    description: format!("{}: {}", prefix, name),
                });
            }
        }
    }
}
