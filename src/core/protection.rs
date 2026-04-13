use std::path::Path;

/// Paths that should never be deleted
const PROTECTED_PATHS: &[&str] = &[
    "/System",
    "/usr",
    "/bin",
    "/sbin",
    "/Library/Apple",
    "/private/var/db",
    "/Applications/Utilities",
];

/// Applications that should never be uninstalled
const PROTECTED_APPS: &[&str] = &[
    "Finder",
    "Safari",
    "App Store",
    "System Preferences",
    "System Settings",
    "Terminal",
    "Activity Monitor",
    "Disk Utility",
    "Console",
    "Keychain Access",
    "Migration Assistant",
    "Font Book",
];

/// Check if a path is protected and should not be deleted
pub fn is_protected(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Never delete protected system paths
    for protected in PROTECTED_PATHS {
        if path_str.starts_with(protected) && !path_str.contains("/Caches/") {
            return true;
        }
    }

    // SIP-protected check
    if is_sip_protected(path) {
        return true;
    }

    false
}

/// Check if path is SIP protected
fn is_sip_protected(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    let sip_paths = [
        "/System/",
        "/usr/bin/",
        "/usr/lib/",
        "/usr/sbin/",
        "/usr/share/",
    ];
    sip_paths.iter().any(|p| path_str.starts_with(p))
}

/// Check if an app is protected
pub fn is_protected_app(name: &str) -> bool {
    PROTECTED_APPS
        .iter()
        .any(|&app| name.eq_ignore_ascii_case(app))
}
