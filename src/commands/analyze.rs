use anyhow::Result;
use console::style;
use serde_json::json;
use std::path::PathBuf;

use crate::core::scanner;
use crate::core::size::format_size;
use crate::ui::progress;
use crate::ui::table::{self, TableRow};

pub fn run(path: Option<String>, depth: usize, top: usize, json_output: bool) -> Result<()> {
    let target_path = match path {
        Some(p) => PathBuf::from(p),
        None => dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
    };

    if !target_path.exists() {
        anyhow::bail!("Path does not exist: {}", target_path.display());
    }

    if !json_output {
        println!(
            "\n{}",
            style(format!("  mop — Analyzing {}", target_path.display()))
                .bold()
                .cyan()
        );
    }

    let spinner = if !json_output {
        Some(progress::spinner("Scanning..."))
    } else {
        None
    };

    let entries = scanner::scan_directory(&target_path, depth);

    if let Some(s) = spinner {
        s.finish_and_clear();
    }

    let top_entries: Vec<_> = entries.into_iter().take(top).collect();
    let total_size: u64 = top_entries.iter().map(|e| e.size).sum();

    if json_output {
        let json_entries: Vec<_> = top_entries
            .iter()
            .map(|e| {
                json!({
                    "path": e.path.to_string_lossy(),
                    "size": e.size,
                    "size_human": format_size(e.size),
                    "is_dir": e.is_dir,
                })
            })
            .collect();

        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "target": target_path.to_string_lossy(),
                "total_scanned": total_size,
                "entries": json_entries,
            }))?
        );
        return Ok(());
    }

    // Display results
    progress::print_header("Largest items");

    let rows: Vec<TableRow> = top_entries
        .iter()
        .map(|e| {
            let name = e.path.to_string_lossy().to_string();
            let type_indicator = if e.is_dir { "[DIR]" } else { "[FILE]" };
            TableRow {
                name: format!("{} {}", type_indicator, name),
                size: e.size,
                detail: None,
            }
        })
        .collect();

    table::print_size_table(&rows, total_size);

    // File type breakdown
    let spinner = progress::spinner("Analyzing file types...");
    let extensions = scanner::scan_by_extension(&target_path);
    spinner.finish_and_clear();

    let mut ext_sorted: Vec<_> = extensions.into_iter().collect();
    ext_sorted.sort_by_key(|b| std::cmp::Reverse(b.1));

    progress::print_header("By file type (top 10)");

    let ext_total: u64 = ext_sorted.iter().map(|(_, s)| s).sum();
    let ext_rows: Vec<TableRow> = ext_sorted
        .into_iter()
        .take(10)
        .map(|(ext, size)| TableRow {
            name: format!(".{}", ext),
            size,
            detail: None,
        })
        .collect();

    table::print_size_table(&ext_rows, ext_total);
    table::print_summary("Total analyzed", total_size);

    Ok(())
}
