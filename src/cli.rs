use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mop", about = "Fast macOS system cleaner", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clean system junk, caches, and logs
    Clean {
        /// Show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,

        /// Clean all categories
        #[arg(long)]
        all: bool,

        /// Clean specific categories (user-cache, system-logs, crash-reports, browser-cache, app-leftovers, dev-tools, homebrew, trash, downloads, docker)
        #[arg(long, short)]
        category: Vec<String>,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,

        /// Show detailed output
        #[arg(long, short)]
        verbose: bool,
    },

    /// Analyze disk space usage
    Analyze {
        /// Directory to analyze (default: home directory)
        path: Option<String>,

        /// Maximum depth for analysis
        #[arg(long, short, default_value = "5")]
        depth: usize,

        /// Number of top entries to show
        #[arg(long, short = 'n', default_value = "20")]
        top: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Uninstall applications and their associated files
    Uninstall {
        /// Application name to uninstall
        name: Option<String>,

        /// Show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Remove build artifacts from project directories
    Purge {
        /// Directories to scan (default: ~/Projects, ~/Developer, ~/Documents)
        paths: Vec<String>,

        /// Minimum age in days for projects (default: 7)
        #[arg(long, default_value = "7")]
        older_than: u64,

        /// Show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Show system status and health
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find and remove installer files (.dmg, .pkg, .zip)
    Installer {
        /// Show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },
}
