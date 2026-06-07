use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;

pub struct BrowserCacheCleaner;

impl Cleaner for BrowserCacheCleaner {
    fn name(&self) -> &str {
        "Browser Caches"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();
        let app_support = home.join("Library").join("Application Support");
        let caches = home.join("Library").join("Caches");

        // Chrome
        let chrome_cache = app_support
            .join("Google")
            .join("Chrome")
            .join("Default")
            .join("Cache");
        add_if_exists(&chrome_cache, "Chrome cache", &mut targets);

        let chrome_code_cache = app_support
            .join("Google")
            .join("Chrome")
            .join("Default")
            .join("Code Cache");
        add_if_exists(&chrome_code_cache, "Chrome code cache", &mut targets);

        let chrome_service_worker = app_support
            .join("Google")
            .join("Chrome")
            .join("Default")
            .join("Service Worker");
        add_if_exists(
            &chrome_service_worker,
            "Chrome Service Worker cache",
            &mut targets,
        );

        // Safari
        let safari_cache = caches.join("com.apple.Safari");
        add_if_exists(&safari_cache, "Safari cache", &mut targets);

        let safari_webpage = home.join("Library").join("Safari").join("LocalStorage");
        add_if_exists(&safari_webpage, "Safari local storage", &mut targets);

        // Firefox
        let firefox_profiles = app_support.join("Firefox").join("Profiles");
        if firefox_profiles.exists() {
            if let Ok(entries) = std::fs::read_dir(&firefox_profiles) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let cache_path = entry.path().join("cache2");
                    if cache_path.exists() {
                        let size = dir_size(&cache_path);
                        if size > 1_048_576 {
                            targets.push(CleanTarget {
                                path: cache_path,
                                size,
                                description: "Firefox cache".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Arc
        let arc_cache = app_support
            .join("Arc")
            .join("User Data")
            .join("Default")
            .join("Cache");
        add_if_exists(&arc_cache, "Arc browser cache", &mut targets);

        // Brave
        let brave_cache = app_support
            .join("BraveSoftware")
            .join("Brave-Browser")
            .join("Default")
            .join("Cache");
        add_if_exists(&brave_cache, "Brave browser cache", &mut targets);

        // Edge
        let edge_cache = app_support
            .join("Microsoft Edge")
            .join("Default")
            .join("Cache");
        add_if_exists(&edge_cache, "Microsoft Edge cache", &mut targets);

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}

fn add_if_exists(path: &std::path::Path, desc: &str, targets: &mut Vec<CleanTarget>) {
    if path.exists() {
        let size = dir_size(path);
        if size > 1_048_576 {
            targets.push(CleanTarget {
                path: path.to_path_buf(),
                size,
                description: desc.to_string(),
            });
        }
    }
}
