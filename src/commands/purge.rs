use anyhow::Result;
use console::style;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::config::Config;
use crate::core::file_ops;
use crate::core::size::{dir_size, format_size};
use crate::ui::progress;
use crate::ui::prompt;
use crate::ui::table::{self, TableRow};

const BUILD_ARTIFACT_DIRS: &[&str] = &[
    "node_modules",
    "target",
    ".build",
    "build",
    "dist",
    "venv",
    ".venv",
    "env",
    "__pycache__",
    ".gradle",
    "Pods",
    ".next",
    ".nuxt",
    ".cache",
];

struct ArtifactEntry {
    path: PathBuf,
    project: String,
    artifact_type: String,
    size: u64,
    days_old: u64,
}

pub fn run(
    paths: Vec<String>,
    older_than: u64,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    let config = Config::load().unwrap_or_default();

    let scan_paths: Vec<PathBuf> = if paths.is_empty() {
        config.purge_paths.iter().map(PathBuf::from).collect()
    } else {
        paths.iter().map(PathBuf::from).collect()
    };

    println!(
        "\n{}",
        style("  mop — Build Artifact Purge").bold().cyan()
    );

    if dry_run {
        println!("  {}", style("(Dry run — nothing will be deleted)").yellow());
    }

    let mut all_artifacts: Vec<ArtifactEntry> = Vec::new();

    for scan_path in &scan_paths {
        if !scan_path.exists() {
            progress::print_warning(&format!(
                "Path does not exist: {}",
                scan_path.display()
            ));
            continue;
        }

        let spinner = progress::spinner(&format!("Scanning {}...", scan_path.display()));
        let artifacts = find_artifacts(scan_path, older_than);
        spinner.finish_and_clear();

        all_artifacts.extend(artifacts);
    }

    if all_artifacts.is_empty() {
        println!("\n  {}", style("No build artifacts found.").green());
        return Ok(());
    }

    all_artifacts.sort_by(|a, b| b.size.cmp(&a.size));

    let total_size: u64 = all_artifacts.iter().map(|a| a.size).sum();

    progress::print_header("Build artifacts found");

    let rows: Vec<TableRow> = all_artifacts
        .iter()
        .map(|a| TableRow {
            name: format!("{} ({})", a.project, a.artifact_type),
            size: a.size,
            detail: Some(format!("{} days old", a.days_old)),
        })
        .collect();

    table::print_size_table(&rows, total_size);
    table::print_summary("Total reclaimable", total_size);

    if dry_run {
        println!(
            "\n  {}",
            style("Run without --dry-run to purge.").dim()
        );
        return Ok(());
    }

    // Interactive selection or force
    let selected_indices = if force {
        (0..all_artifacts.len()).collect()
    } else {
        let items: Vec<String> = all_artifacts
            .iter()
            .map(|a| {
                format!(
                    "{} ({}) — {}",
                    a.project,
                    a.artifact_type,
                    format_size(a.size)
                )
            })
            .collect();

        println!();
        prompt::multi_select_all("Select artifacts to remove", &items)
    };

    if selected_indices.is_empty() {
        println!("  {}", style("Nothing selected.").dim());
        return Ok(());
    }

    let mut freed: u64 = 0;
    let spinner = progress::spinner("Removing artifacts...");

    for idx in &selected_indices {
        let artifact = &all_artifacts[*idx];
        let result = file_ops::safe_remove(&artifact.path, false);
        if let Ok(r) = result {
            if r.removed {
                freed += r.size;
            }
        }
    }

    spinner.finish_and_clear();

    println!(
        "\n  {} {}",
        style("Space freed:").bold(),
        style(format_size(freed)).bold().green()
    );
    println!(
        "  {} {} artifacts removed",
        style("✓").green().bold(),
        selected_indices.len()
    );

    Ok(())
}

fn find_artifacts(root: &PathBuf, _min_age_days: u64) -> Vec<ArtifactEntry> {
    let mut artifacts = Vec::new();

    // Walk top-level project directories
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return artifacts,
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let project_path = entry.path();
        if !project_path.is_dir() {
            continue;
        }

        let project_name = project_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Skip hidden directories
        if project_name.starts_with('.') {
            continue;
        }

        // Check project modification time
        let modified = project_path
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let days_old = SystemTime::now()
            .duration_since(modified)
            .map(|d| d.as_secs() / 86400)
            .unwrap_or(0);

        // Search for build artifacts within the project
        for artifact_name in BUILD_ARTIFACT_DIRS {
            let artifact_path = project_path.join(artifact_name);
            if artifact_path.exists() && artifact_path.is_dir() {
                let size = dir_size(&artifact_path);
                if size > 1_048_576 {
                    // > 1MB
                    artifacts.push(ArtifactEntry {
                        path: artifact_path,
                        project: project_name.clone(),
                        artifact_type: artifact_name.to_string(),
                        size,
                        days_old,
                    });
                }
            }
        }
    }

    artifacts
}
