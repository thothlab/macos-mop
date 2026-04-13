use console::Style;
use once_cell::sync::Lazy;

pub static TITLE: Lazy<Style> = Lazy::new(|| Style::new().bold().cyan());
pub static SUCCESS: Lazy<Style> = Lazy::new(|| Style::new().green());
pub static WARNING: Lazy<Style> = Lazy::new(|| Style::new().yellow());
pub static ERROR: Lazy<Style> = Lazy::new(|| Style::new().red());
pub static DIM: Lazy<Style> = Lazy::new(|| Style::new().dim());
pub static BOLD: Lazy<Style> = Lazy::new(|| Style::new().bold());
pub static SIZE_LARGE: Lazy<Style> = Lazy::new(|| Style::new().red().bold());
pub static SIZE_MEDIUM: Lazy<Style> = Lazy::new(|| Style::new().yellow());
pub static SIZE_SMALL: Lazy<Style> = Lazy::new(|| Style::new().green());

/// Format size with appropriate color
pub fn colored_size(bytes: u64) -> String {
    let size_str = crate::core::size::format_size(bytes);
    if bytes >= 1_073_741_824 {
        SIZE_LARGE.apply_to(&size_str).to_string()
    } else if bytes >= 104_857_600 {
        SIZE_MEDIUM.apply_to(&size_str).to_string()
    } else {
        SIZE_SMALL.apply_to(&size_str).to_string()
    }
}
