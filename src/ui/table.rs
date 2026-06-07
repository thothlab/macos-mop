use crate::core::size::format_size;
use crate::ui::colors;
use console::style;

/// A row in a display table
pub struct TableRow {
    pub name: String,
    pub size: u64,
    pub detail: Option<String>,
}

/// Print a table of items with sizes and bar visualization
pub fn print_size_table(rows: &[TableRow], total: u64) {
    if rows.is_empty() {
        println!("  {}", style("Nothing found.").dim());
        return;
    }

    let max_name_len = rows
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(20)
        .min(50);
    let bar_width = 20;

    for row in rows {
        let name = if row.name.len() > 50 {
            format!("...{}", &row.name[row.name.len() - 47..])
        } else {
            row.name.clone()
        };

        let fraction = if total > 0 {
            (row.size as f64 / total as f64).min(1.0)
        } else {
            0.0
        };
        let filled = (fraction * bar_width as f64) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_width - filled));

        let size_str = colors::colored_size(row.size);

        if let Some(ref detail) = row.detail {
            println!(
                "  {:<width$}  {}  {}  {}",
                name,
                style(&bar).cyan(),
                size_str,
                style(detail).dim(),
                width = max_name_len
            );
        } else {
            println!(
                "  {:<width$}  {}  {}",
                name,
                style(&bar).cyan(),
                size_str,
                width = max_name_len
            );
        }
    }
}

/// Print a summary line
pub fn print_summary(label: &str, size: u64) {
    println!(
        "\n  {} {}",
        style(format!("{}: {}", label, format_size(size))).bold(),
        if size >= 1_073_741_824 {
            style("(!)").red().bold().to_string()
        } else {
            String::new()
        }
    );
}
