use dialoguer::{Confirm, MultiSelect};

/// Ask for confirmation
pub fn confirm(message: &str) -> bool {
    Confirm::new()
        .with_prompt(message)
        .default(false)
        .interact()
        .unwrap_or(false)
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
