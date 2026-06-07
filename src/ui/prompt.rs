use dialoguer::MultiSelect;
use std::io::{self, Write};

/// Ask for confirmation — requires explicit uppercase 'Y' to proceed.
/// Enter alone and lowercase 'y' are rejected.
pub fn confirm(message: &str) -> bool {
    loop {
        print!("  {} [Y/n]: ", message);
        io::stdout().flush().unwrap_or(());

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        match input.trim() {
            "Y" => return true,
            "n" | "N" => return false,
            "" => eprintln!("  Please type 'Y' to confirm or 'n' to cancel."),
            _ => eprintln!("  Use uppercase 'Y' to confirm or 'n' to cancel."),
        }
    }
}

/// Multi-select from a list of items
pub fn multi_select(prompt: &str, items: &[String]) -> Vec<usize> {
    if items.is_empty() {
        return vec![];
    }

    MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap_or_default()
}

/// Multi-select with all items pre-selected
pub fn multi_select_all(prompt: &str, items: &[String]) -> Vec<usize> {
    if items.is_empty() {
        return vec![];
    }

    let defaults: Vec<bool> = vec![true; items.len()];
    MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
        .defaults(&defaults)
        .interact()
        .unwrap_or_default()
}
