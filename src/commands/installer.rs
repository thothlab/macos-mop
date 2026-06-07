use anyhow::Result;
use console::style;

use crate::core::file_ops;
use crate::core::scanner;
use crate::core::size::format_size;
use crate::ui::progress;
use crate::ui::prompt;
use crate::ui::table::{self, TableRow};

const INSTALLER_EXTENSIONS: &[&str] = &["dmg", "pkg", "zip", "iso", "tar", "gz", "bz2", "xz"];

pub fn run(dry_run: bool, force: bool) -> Result<()> {
    println!("\n{}", style("  mop — Installer Finder").bold().cyan());

    if dry_run {
        println!(
            "  {}",
            style("(Dry run — nothing will be deleted)").yellow()
        );
    }

    let home = dirs::home_dir().unwrap_or_default();
    let search_dirs = vec![home.join("Downloads"), home.join("Desktop")];

    // Also check Homebrew cache
    let brew_cache = home.join("Library").join("Caches").join("Homebrew");

    let spinner = progress::spinner("Searching for installer files...");

    let mut all_entries = Vec::new();

    for dir in &search_dirs {
        if dir.exists() {
            let entries = scanner::find_files_by_extension(dir, INSTALLER_EXTENSIONS);
            all_entries.extend(entries);
        }
    }

    if brew_cache.exists() {
        let entries = scanner::find_files_by_extension(&brew_cache, INSTALLER_EXTENSIONS);
        all_entries.extend(entries);
    }

    spinner.finish_and_clear();

    if all_entries.is_empty() {
        println!("\n  {}", style("No installer files found.").green());
        return Ok(());
    }

    all_entries.sort_by(|a, b| b.size.cmp(&a.size));
    let total_size: u64 = all_entries.iter().map(|e| e.size).sum();

    progress::print_header("Installer files found");

    let rows: Vec<TableRow> = all_entries
        .iter()
        .map(|e| {
            let name = e
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let location = e
                .path
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            TableRow {
                name,
                size: e.size,
                detail: Some(location),
            }
        })
        .collect();

    table::print_size_table(&rows, total_size);
    table::print_summary("Total", total_size);

    if dry_run {
        println!("\n  {}", style("Run without --dry-run to delete.").dim());
        return Ok(());
    }

    // Interactive selection
    let items: Vec<String> = all_entries
        .iter()
        .map(|e| {
            let name = e
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            format!("{} ({})", name, format_size(e.size))
        })
        .collect();

    let selected = if force {
        (0..all_entries.len()).collect()
    } else {
        println!();
        prompt::multi_select("Select files to remove", &items)
    };

    if selected.is_empty() {
        println!("  {}", style("Nothing selected.").dim());
        return Ok(());
    }

    let mut freed: u64 = 0;
    for idx in &selected {
        let entry = &all_entries[*idx];
        let result = file_ops::safe_remove(&entry.path, false);
        if let Ok(r) = result {
            if r.removed {
                freed += r.size;
                progress::print_success(&format!(
                    "Removed: {}",
                    entry.path.file_name().unwrap_or_default().to_string_lossy()
                ));
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
