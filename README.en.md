# mop

**English** | [Русский](README.md)

**mop** is a fast macOS system cleaner CLI utility written in Rust.

It frees up disk space by removing system junk, caches, logs, leftover files from uninstalled apps, and build artifacts — all from the terminal, with no GUI and no subscription required.

In seconds, mop finds everything that can be safely removed and shows you the full list with file sizes before deleting anything. Nothing is ever removed without your explicit confirmation.

---

## Features

### `mop clean` — System Cleanup

Scans macOS across 10 categories and removes junk that silently accumulates over time:

| Category | What it cleans |
|---|---|
| `user-cache` | Application caches in `~/Library/Caches` |
| `system-logs` | System and user logs in `/var/log` and `~/Library/Logs` |
| `crash-reports` | Crash reports in `~/Library/Logs/DiagnosticReports` |
| `browser-cache` | Chrome, Safari, Firefox, Arc, Brave, Edge caches |
| `app-leftovers` | Orphaned files from uninstalled apps (Preferences, Application Support, Containers) |
| `dev-tools` | Xcode DerivedData, CoreSimulator, npm, yarn, pip, cargo, CocoaPods, Gradle, Maven, Go caches |
| `homebrew` | Old formula versions and Homebrew download cache |
| `trash` | macOS Trash |
| `downloads` | Old files in `~/Downloads` (older than a configurable threshold) |
| `docker` | Unused Docker images, containers, and volumes |

```bash
# Preview what will be removed — nothing is deleted
mop clean --dry-run --all

# Clean everything
mop clean --all

# Clean specific categories only
mop clean --category user-cache --category browser-cache --category trash

# Verbose output
mop clean --all --verbose
```

---

### `mop analyze` — Disk Space Analysis

Finds what's taking up space on your disk. Shows top directories and files with visual bars and groups results by file type.

```bash
# Analyze home directory
mop analyze

# Analyze a specific directory
mop analyze ~/Documents

# Limit traversal depth
mop analyze ~/Projects --depth 3

# JSON output for scripting
mop analyze --json
```

---

### `mop uninstall` — Complete App Removal

Removes not just the `.app` bundle, but **all associated files** that macOS leaves behind after a standard drag-to-trash uninstall:

- `~/Library/Application Support/<app>`
- `~/Library/Preferences/com.<bundle>.*`
- `~/Library/Caches/<app>`
- `~/Library/Containers/<bundle>`
- `~/Library/Group Containers`
- `~/Library/Saved Application State`
- `~/Library/WebKit/<bundle>`
- `~/Library/LaunchAgents`

```bash
# Preview what will be removed
mop uninstall --dry-run "Slack"

# Remove app and all its associated files
mop uninstall "Charles"

# Interactive selection from a list of installed apps
mop uninstall
```

Before deleting, mop shows the full file list with sizes and requires explicit uppercase `Y` confirmation.

---

### `mop purge` — Build Artifact Cleanup

For developers: recursively scans project directories and finds folders that can be safely deleted and rebuilt:

- `node_modules/` (Node.js)
- `target/` (Rust)
- `.build/` (Swift)
- `build/`, `dist/` (general)
- `venv/`, `.venv/`, `__pycache__/` (Python)
- `.gradle/`, `.m2/` (Java)

```bash
# Find artifacts in your projects folder
mop purge --dry-run ~/Projects

# Remove all found artifacts
mop purge ~/Projects

# Only artifacts older than 30 days
mop purge ~/Projects --older-than 30
```

---

### `mop status` — System Health Overview

A quick snapshot of your system's current state: disk usage, memory, CPU, and top resource-consuming processes.

```bash
mop status

# JSON output
mop status --json
```

---

### `mop installer` — Find Installer Files

Locates old `.dmg`, `.pkg`, and `.zip` files in `~/Downloads` and other standard locations — files you no longer need after installing apps.

```bash
mop installer --dry-run
mop installer
```

---

## Installation

### Homebrew (recommended)

```bash
brew tap thothlab/macos-mop
brew install mop
```

Or in a single command:

```bash
brew install thothlab/macos-mop/mop
```

### Install script

```bash
curl -fsSL https://raw.githubusercontent.com/thothlab/macos-mop/main/install.sh | bash
```

### From source (requires Rust)

```bash
cargo install --git https://github.com/thothlab/macos-mop
```

### Manually from GitHub Releases

Download the binary for your architecture from [Releases](https://github.com/thothlab/macos-mop/releases):

```bash
# Apple Silicon (M1/M2/M3/M4)
curl -L https://github.com/thothlab/macos-mop/releases/latest/download/mop-arm64.tar.gz | tar xz
sudo mv mop-arm64 /usr/local/bin/mop

# Intel
curl -L https://github.com/thothlab/macos-mop/releases/latest/download/mop-x86_64.tar.gz | tar xz
sudo mv mop-x86_64 /usr/local/bin/mop
```

---

## Configuration

The config file is created automatically on first run: `~/.config/mop/config.toml`

```toml
# Directories to scan for build artifacts with 'purge'
purge_paths = ["~/Projects", "~/Developer"]

# Paths that should never be touched
whitelist = []

# Default categories for 'mop clean' (without --all)
default_categories = ["user-cache", "system-logs", "crash-reports", "browser-cache", "trash"]

# Age threshold for Downloads cleanup (days)
download_age_days = 30

# Age threshold for purge (days)
purge_age_days = 7
```

---

## Safety

mop is designed to prevent accidental data loss:

- **Dry run** — `--dry-run` shows the file list without deleting anything
- **Explicit confirmation** — deletion requires uppercase `Y`; Enter and lowercase `y` are rejected
- **SIP protection** — macOS system paths protected by SIP are never touched
- **Whitelist** — paths listed in config `whitelist` are never removed
- **System app protection** — built-in macOS apps cannot be removed via `uninstall`
- **Operation log** — all actions are recorded in `~/.config/mop/operations.log`

---

## Requirements

- macOS 12 Monterey or later
- Apple Silicon (arm64) or Intel (x86_64)

---

## License

MIT
