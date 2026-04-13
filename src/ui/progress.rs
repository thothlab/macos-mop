use console::style;
use indicatif::{ProgressBar, ProgressStyle};

/// Create a spinner with a message
pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

/// Create a progress bar with known total
pub fn progress_bar(total: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:30.cyan/dim}] {pos}/{len} {eta}")
            .unwrap()
            .progress_chars("━░"),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Print a success checkmark
pub fn print_success(msg: &str) {
    println!("  {} {}", style("✓").green().bold(), msg);
}

/// Print a warning
pub fn print_warning(msg: &str) {
    println!("  {} {}", style("⚠").yellow().bold(), msg);
}

/// Print an error
pub fn print_error(msg: &str) {
    println!("  {} {}", style("✗").red().bold(), msg);
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("  {} {}", style("ℹ").cyan().bold(), msg);
}

/// Print a section header
pub fn print_header(msg: &str) {
    println!("\n{}", style(msg).bold().cyan());
    println!("{}", style("─".repeat(50)).dim());
}
