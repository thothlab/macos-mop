# mop

**English** | [Русский](README.md)

Fast macOS system cleaner CLI utility written in Rust.

Clean system junk, caches, logs, app leftovers, build artifacts, and more — all from the terminal.

## Features

- **clean** — Remove system caches, logs, browser data, app leftovers, Homebrew cache, Trash, and more
- **analyze** — Analyze disk space usage with visual bars and file type breakdown
- **uninstall** — Completely remove applications with all associated files
- **purge** — Find and remove build artifacts (node_modules, target, .build, venv, etc.)
- **status** — System health overview: disk, memory, CPU, top processes
- **installer** — Find and remove old installer files (.dmg, .pkg, .zip)

## Installation

### Homebrew

```bash
brew tap thothlab/macos-mop
brew install mop
```

### Install script

```bash
curl -fsSL https://raw.githubusercontent.com/thothlab/macos-mop/main/install.sh | bash
```

### From source

```bash
cargo install --git https://github.com/thothlab/macos-mop
```

### From GitHub Releases

Download the latest binary from [Releases](https://github.com/thothlab/macos-mop/releases) and place it in your PATH.

## Usage

```bash
# Show what can be cleaned (dry run, nothing deleted)
mop clean --dry-run --all

# Clean all categories
mop clean --all

# Clean specific categories
mop clean --category user-cache --category browser-cache

# Analyze disk space
mop analyze ~/Documents --depth 3

# Uninstall an app completely
mop uninstall "Slack"

# Remove build artifacts from projects
mop purge ~/Projects

# System health status
mop status

# Find installer files
mop installer
```

## Clean categories

| Category | Description |
|---|---|
| `user-cache` | Application caches in ~/Library/Caches |
| `system-logs` | System and user logs |
| `crash-reports` | Crash and diagnostic reports |
| `browser-cache` | Chrome, Safari, Firefox, Arc, Brave, Edge caches |
| `app-leftovers` | Orphaned files from uninstalled apps |
| `dev-tools` | Xcode, npm, yarn, pip, cargo, CocoaPods, Gradle, Maven, Go caches |
| `homebrew` | Homebrew download cache and old formula versions |
| `trash` | macOS Trash |
| `downloads` | Old files in ~/Downloads |
| `docker` | Docker Desktop data and buildx cache |

## Configuration

Config file: `~/.config/mop/config.toml`

```toml
# Paths to scan for build artifacts
purge_paths = ["~/Projects", "~/Developer"]

# Paths that should never be cleaned
whitelist = []

# Default categories for 'clean' command
default_categories = ["user-cache", "system-logs", "crash-reports", "browser-cache", "trash"]

# Age threshold for downloads cleanup (days)
download_age_days = 30

# Age threshold for purge command (days)
purge_age_days = 7
```

## Safety

- **Dry run** mode (`--dry-run`) to preview changes before deleting
- Protected system paths (SIP-protected) are never touched
- Protected system apps cannot be uninstalled
- Whitelisting support for paths you want to keep
- All operations logged to `~/.config/mop/operations.log`
- Deletion confirmation requires explicit uppercase `Y` — Enter and lowercase `y` are rejected

## License

MIT
