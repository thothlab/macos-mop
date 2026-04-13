use super::{CleanTarget, Cleaner};
use chrono::{Duration, Utc};
use std::time::SystemTime;

pub struct DownloadsCleaner {
    pub age_days: u64,
}

impl DownloadsCleaner {
    pub fn new(age_days: u64) -> Self {
        Self { age_days }
    }
}

impl Cleaner for DownloadsCleaner {
    fn name(&self) -> &str {
        "Old Downloads"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();
        let downloads = home.join("Downloads");

        if !downloads.exists() {
            return targets;
        }

        let cutoff = Utc::now() - Duration::days(self.age_days as i64);
        let cutoff_system = SystemTime::from(cutoff);

        if let Ok(entries) = std::fs::read_dir(&downloads) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let meta = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                if modified < cutoff_system {
                    let size = if meta.is_dir() {
                        crate::core::size::dir_size(&path)
                    } else {
                        meta.len()
                    };

                    if size > 0 {
                        let name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let days_old = SystemTime::now()
                            .duration_since(modified)
                            .map(|d| d.as_secs() / 86400)
                            .unwrap_or(0);

                        targets.push(CleanTarget {
                            path,
                            size,
                            description: format!("{} ({} days old)", name, days_old),
                        });
                    }
                }
            }
        }

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}
