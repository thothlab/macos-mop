use anyhow::Result;
use console::style;

use crate::cleaners::app_leftovers::AppLeftoversCleaner;
use crate::cleaners::browser_cache::BrowserCacheCleaner;
use crate::cleaners::dev_tools::DevToolsCleaner;
use crate::cleaners::docker::DockerCleaner;
use crate::cleaners::downloads::DownloadsCleaner;
use crate::cleaners::homebrew::HomebrewCleaner;
use crate::cleaners::system_logs::{CrashReportsCleaner, SystemLogsCleaner};
use crate::cleaners::trash::TrashCleaner;
use crate::cleaners::user_cache::UserCacheCleaner;
use crate::cleaners::{CleanTarget, Cleaner};
use crate::config::Config;
use crate::core::file_ops;
use crate::core::size::format_size;
use crate::ui::progress;
use crate::ui::prompt;
use crate::ui::table::{self, TableRow};

const ALL_CATEGORIES: &[&str] = &[
    "user-cache",
    "system-logs",
    "crash-reports",
    "browser-cache",
    "app-leftovers",
    "dev-tools",
    "homebrew",
    "trash",
    "downloads",
    "docker",
];

pub fn run(
    dry_run: bool,
    all: bool,
    categories: Vec<String>,
    force: bool,
    verbose: bool,
) -> Result<()> {
    let config = Config::load().unwrap_or_default();

    let active_categories = if all {
        ALL_CATEGORIES.iter().map(|s| s.to_string()).collect()
    } else if categories.is_empty() {
        config.default_categories.clone()
    } else {
        categories
    };

    println!("\n{}", style("  mop — System Cleanup").bold().cyan());

    if dry_run {
        println!(
            "  {}",
            style("(Dry run — nothing will be deleted)").yellow()
        );
    }

    let mut total_size: u64 = 0;
    let mut all_targets: Vec<(String, Vec<CleanTarget>)> = Vec::new();

    for category in &active_categories {
        let spinner = progress::spinner(&format!("Scanning {}...", category));

        let targets: Vec<CleanTarget> = match category.as_str() {
            "user-cache" => UserCacheCleaner.scan(),
            "system-logs" => SystemLogsCleaner.scan(),
            "crash-reports" => CrashReportsCleaner.scan(),
            "browser-cache" => BrowserCacheCleaner.scan(),
            "app-leftovers" => AppLeftoversCleaner.scan(),
            "dev-tools" => DevToolsCleaner.scan(),
            "homebrew" => HomebrewCleaner.scan(),
            "trash" => TrashCleaner.scan(),
            "downloads" => DownloadsCleaner::new(config.download_age_days).scan(),
            "docker" => DockerCleaner.scan(),
            other => {
                spinner.finish_and_clear();
                progress::print_warning(&format!("Unknown category: {}", other));
                continue;
            }
        };

        spinner.finish_and_clear();

        let cat_size: u64 = targets.iter().map(|t| t.size).sum();
        total_size += cat_size;

        if !targets.is_empty() {
            progress::print_header(category);

            let rows: Vec<TableRow> = targets
                .iter()
                .map(|t| TableRow {
                    name: t.description.clone(),
                    size: t.size,
                    detail: if verbose {
                        Some(t.path.to_string_lossy().to_string())
                    } else {
                        None
                    },
                })
                .collect();

            table::print_size_table(&rows, cat_size);
            table::print_summary(&format!("{} total", category), cat_size);
        } else {
            progress::print_success(&format!("{}: clean", category));
        }

        all_targets.push((category.clone(), targets));
    }

    println!("\n{}", style("─".repeat(50)).dim());
    println!(
        "  {} {}",
        style("Total reclaimable space:").bold(),
        style(format_size(total_size)).bold().green()
    );

    if total_size == 0 {
        println!("\n  {}", style("System is already clean!").green());
        return Ok(());
    }

    if dry_run {
        println!("\n  {}", style("Run without --dry-run to clean.").dim());
        return Ok(());
    }

    // Confirm before cleaning
    if !force {
        println!();
        if !prompt::confirm("Proceed with cleanup?") {
            println!("  {}", style("Cancelled.").dim());
            return Ok(());
        }
    }

    // Perform cleanup
    let mut freed: u64 = 0;
    let mut errors: u64 = 0;

    for (category, targets) in &all_targets {
        if targets.is_empty() {
            continue;
        }

        let spinner = progress::spinner(&format!("Cleaning {}...", category));

        for target in targets {
            if config.is_whitelisted(&target.path.to_string_lossy()) {
                continue;
            }

            let result = file_ops::safe_remove(&target.path, false);
            match result {
                Ok(r) if r.removed => freed += r.size,
                Ok(r) if r.error.is_some() => {
                    errors += 1;
                    if verbose {
                        progress::print_error(&r.error.unwrap_or_default().to_string());
                    }
                }
                _ => errors += 1,
            }
        }

        spinner.finish_and_clear();
        progress::print_success(&format!("{}: cleaned", category));
    }

    println!("\n{}", style("─".repeat(50)).dim());
    println!(
        "  {} {}",
        style("Space freed:").bold(),
        style(format_size(freed)).bold().green()
    );

    if errors > 0 {
        println!(
            "  {} {}",
            style("Errors:").bold(),
            style(errors.to_string()).yellow()
        );
    }

    Ok(())
}
