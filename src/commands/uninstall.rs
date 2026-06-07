use anyhow::Result;
use console::style;
use std::path::{Path, PathBuf};

use crate::core::file_ops;
use crate::core::protection;
use crate::core::size::{dir_size, entry_size, format_size};
use crate::ui::progress;
use crate::ui::prompt;
use crate::ui::table::{self, TableRow};

struct AppInfo {
    name: String,
    path: PathBuf,
    bundle_id: Option<String>,
    size: u64,
}

pub fn run(name: Option<String>, dry_run: bool, force: bool) -> Result<()> {
    println!("\n{}", style("  mop — App Uninstaller").bold().cyan());

    // List installed applications
    let apps = list_applications();

    if apps.is_empty() {
        println!("  {}", style("No applications found.").dim());
        return Ok(());
    }

    let app = match name {
        Some(ref n) => {
            let found = apps
                .iter()
                .find(|a| a.name.to_lowercase().contains(&n.to_lowercase()));
            match found {
                Some(a) => a,
                None => {
                    println!("  Application '{}' not found.", n);
                    println!("\n  Available applications:");
                    for app in &apps {
                        println!("    - {} ({})", app.name, format_size(app.size));
                    }
                    return Ok(());
                }
            }
        }
        None => {
            // Interactive selection
            let items: Vec<String> = apps
                .iter()
                .map(|a| format!("{} ({})", a.name, format_size(a.size)))
                .collect();

            let selection = dialoguer::FuzzySelect::new()
                .with_prompt("Select application to uninstall")
                .items(&items)
                .interact_opt()?;

            match selection {
                Some(idx) => &apps[idx],
                None => {
                    println!("  {}", style("Cancelled.").dim());
                    return Ok(());
                }
            }
        }
    };

    if protection::is_protected_app(&app.name) {
        println!(
            "  {} '{}' is a protected system application and cannot be uninstalled.",
            style("Error:").red().bold(),
            app.name
        );
        return Ok(());
    }

    println!("\n  Uninstalling: {}", style(&app.name).bold());

    if let Some(ref bid) = app.bundle_id {
        println!("  Bundle ID: {}", style(bid).dim());
    }

    // Find associated files
    let spinner = progress::spinner("Finding associated files...");
    let associated = find_associated_files(&app.name, &app.bundle_id);
    spinner.finish_and_clear();

    progress::print_header("Files to remove");

    let mut all_paths: Vec<(PathBuf, u64, String)> = Vec::new();

    // The app bundle itself
    all_paths.push((app.path.clone(), app.size, "Application bundle".to_string()));

    for (path, size, desc) in &associated {
        all_paths.push((path.clone(), *size, desc.clone()));
    }

    let total_size: u64 = all_paths.iter().map(|(_, s, _)| *s).sum();

    let rows: Vec<TableRow> = all_paths
        .iter()
        .map(|(path, size, desc)| TableRow {
            name: desc.clone(),
            size: *size,
            detail: Some(path.to_string_lossy().to_string()),
        })
        .collect();

    table::print_size_table(&rows, total_size);
    table::print_summary("Total", total_size);

    if dry_run {
        println!(
            "\n  {}",
            style("Dry run — nothing will be deleted.").yellow()
        );
        return Ok(());
    }

    if !force {
        println!();
        if !prompt::confirm(&format!("Remove '{}' and all associated files?", app.name)) {
            println!("  {}", style("Cancelled.").dim());
            return Ok(());
        }
    }

    // Remove files
    let mut freed: u64 = 0;
    for (path, _, desc) in &all_paths {
        let result = file_ops::safe_remove(path, false);
        match result {
            Ok(r) if r.removed => {
                freed += r.size;
                progress::print_success(&format!("Removed: {}", desc));
            }
            Ok(r) => {
                if let Some(err) = r.error {
                    progress::print_error(&format!("{}: {}", desc, err));
                }
            }
            Err(e) => {
                progress::print_error(&format!("{}: {}", desc, e));
            }
        }
    }

    println!(
        "\n  {} {}",
        style("Space freed:").bold(),
        style(format_size(freed)).bold().green()
    );

    Ok(())
}

fn list_applications() -> Vec<AppInfo> {
    let mut apps = Vec::new();

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
                        .to_string();

                    let bundle_id = get_bundle_id(&path);
                    let size = dir_size(&path);

                    apps.push(AppInfo {
                        name,
                        path,
                        bundle_id,
                        size,
                    });
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn get_bundle_id(app_path: &Path) -> Option<String> {
    let plist_path = app_path.join("Contents").join("Info.plist");
    if let Ok(plist) = plist::Value::from_file(&plist_path) {
        if let Some(dict) = plist.as_dictionary() {
            if let Some(bundle_id) = dict.get("CFBundleIdentifier") {
                return bundle_id.as_string().map(|s| s.to_string());
            }
        }
    }
    None
}

fn find_associated_files(
    app_name: &str,
    bundle_id: &Option<String>,
) -> Vec<(PathBuf, u64, String)> {
    let mut files = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();

    let search_terms: Vec<String> = {
        let mut terms = vec![app_name.to_lowercase()];
        if let Some(ref bid) = bundle_id {
            terms.push(bid.to_lowercase());
            // Also search by parts of bundle ID
            for part in bid.split('.') {
                if part.len() > 3 && part.to_lowercase() != "com" && part.to_lowercase() != "app" {
                    terms.push(part.to_lowercase());
                }
            }
        }
        terms
    };

    let search_dirs = [
        (
            home.join("Library").join("Application Support"),
            "App Support",
        ),
        (home.join("Library").join("Caches"), "Cache"),
        (home.join("Library").join("Preferences"), "Preferences"),
        (home.join("Library").join("Containers"), "Container"),
        (
            home.join("Library").join("Group Containers"),
            "Group Container",
        ),
        (
            home.join("Library").join("Saved Application State"),
            "Saved State",
        ),
        (home.join("Library").join("HTTPStorages"), "HTTP Storage"),
        (home.join("Library").join("WebKit"), "WebKit data"),
        (home.join("Library").join("LaunchAgents"), "Launch Agent"),
    ];

    for (dir, desc) in &search_dirs {
        if !dir.exists() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let entry_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                let matches = search_terms.iter().any(|term| entry_name.contains(term));

                if matches {
                    let size = if path.is_dir() {
                        dir_size(&path)
                    } else {
                        entry_size(&path)
                    };

                    if size > 0 {
                        files.push((path, size, desc.to_string()));
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));
    files
}
