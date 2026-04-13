use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;

pub struct TrashCleaner;

impl Cleaner for TrashCleaner {
    fn name(&self) -> &str {
        "Trash"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        let trash_dir = home.join(".Trash");
        if trash_dir.exists() {
            let size = dir_size(&trash_dir);
            if size > 0 {
                targets.push(CleanTarget {
                    path: trash_dir,
                    size,
                    description: "Trash contents".to_string(),
                });
            }
        }

        targets
    }
}
