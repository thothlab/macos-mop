use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct AppLeftoversCleaner;

impl Cleaner for AppLeftoversCleaner {
    fn name(&self) -> &str {
        "Application Leftovers"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        // Get currently installed app bundle IDs
        let installed_apps = get_installed_app_identifiers();

        // Scan Application Support for orphaned dirs
        let app_support = home.join("Library").join("Application Support");
        scan_orphaned_entries(
            &app_support,
            &installed_apps,
            "App Support leftover",
            &mut targets,
        );

        // Scan Containers
        let containers = home.join("Library").join("Containers");
        scan_orphaned_entries(
            &containers,
            &installed_apps,
            "Container leftover",
            &mut targets,
        );

        // Scan Group Containers
        let group_containers = home.join("Library").join("Group Containers");
        scan_orphaned_entries(
            &group_containers,
            &installed_apps,
            "Group Container leftover",
            &mut targets,
        );

        // Scan Saved Application State
        let saved_state = home.join("Library").join("Saved Application State");
        scan_orphaned_entries(
            &saved_state,
            &installed_apps,
            "Saved state leftover",
            &mut targets,
        );

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}

fn get_installed_app_identifiers() -> HashSet<String> {
    let mut identifiers = HashSet::new();

    for app_dir in &[
        PathBuf::from("/Applications"),
        dirs::home_dir().unwrap_or_default().join("Applications"),
    ] {
        if let Ok(entries) = std::fs::read_dir(app_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "app") {
                    let name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_lowercase();
                    identifiers.insert(name);

                    // Try to get bundle identifier from Info.plist
                    let plist_path = path.join("Contents").join("Info.plist");
                    if let Ok(plist) = plist::Value::from_file(&plist_path) {
                        if let Some(dict) = plist.as_dictionary() {
                            if let Some(bundle_id) = dict.get("CFBundleIdentifier") {
                                if let Some(id) = bundle_id.as_string() {
                                    identifiers.insert(id.to_lowercase());
                                    // Also add parts of the bundle ID
                                    for part in id.split('.') {
                                        if part.len() > 3 {
                                            identifiers.insert(part.to_lowercase());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    identifiers
}

fn scan_orphaned_entries(
    dir: &PathBuf,
    installed: &HashSet<String>,
    prefix: &str,
    targets: &mut Vec<CleanTarget>,
) {
    if !dir.exists() {
        return;
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            // Skip Apple system entries
            if name.starts_with("com.apple.") || name.starts_with("apple") {
                continue;
            }

            // Check if this matches any installed app
            let is_orphaned = !installed
                .iter()
                .any(|app| name.contains(app) || app.contains(&name));

            if is_orphaned {
                let size = dir_size(&path);
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
}
