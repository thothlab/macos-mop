#![allow(dead_code)]

mod cleaners;
mod cli;
mod commands;
mod config;
mod core;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use console::style;

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if cli.no_color {
        console::set_colors_enabled(false);
    }

    // Check NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        console::set_colors_enabled(false);
    }

    let result = match cli.command {
        Some(Commands::Clean {
            dry_run,
            all,
            category,
            force,
            verbose,
        }) => commands::clean::run(dry_run, all, category, force, verbose),

        Some(Commands::Analyze {
            path,
            depth,
            top,
            json,
        }) => commands::analyze::run(path, depth, top, json),

        Some(Commands::Uninstall {
            name,
            dry_run,
            force,
        }) => commands::uninstall::run(name, dry_run, force),

        Some(Commands::Purge {
            paths,
            older_than,
            dry_run,
            force,
        }) => commands::purge::run(paths, older_than, dry_run, force),

        Some(Commands::Status { json }) => commands::status::run(json),

        Some(Commands::Installer { dry_run, force }) => commands::installer::run(dry_run, force),

        None => {
            print_welcome();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("  {} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }
}

fn print_welcome() {
    println!(
        r#"
  {}

  {}

  Commands:
    {}    Clean system junk, caches, and logs
    {}  Analyze disk space usage
    {}  Uninstall apps and their leftovers
    {}    Remove build artifacts from projects
    {}   Show system health status
    {}  Find and remove installer files

  Run '{} --help' for more details.
"#,
        style("mop — Fast macOS System Cleaner").bold().cyan(),
        style("v0.1.0").dim(),
        style("clean").green().bold(),
        style("analyze").green().bold(),
        style("uninstall").green().bold(),
        style("purge").green().bold(),
        style("status").green().bold(),
        style("installer").green().bold(),
        style("mop <command>").bold(),
    );
}
