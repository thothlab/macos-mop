use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;

pub struct UserCacheCleaner;

impl Cleaner for UserCacheCleaner {
    fn name(&self) -> &str {
        "User Caches"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();
        let cache_dir = home.join("Library").join("Caches");

        if !cache_dir.exists() {
            return targets;
        }

        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Skip Apple system caches
                    if name.starts_with("com.apple.") {
                        continue;
                    }

                    let size = dir_size(&path);
                    if size > 1_048_576 {
                        // Only show > 1MB
                        targets.push(CleanTarget {
                            path,
                            size,
                            description: format!("App cache: {}", name),
                        });
                    }
                }
            }
        }

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}
