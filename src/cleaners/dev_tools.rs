use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;

pub struct DevToolsCleaner;

impl Cleaner for DevToolsCleaner {
    fn name(&self) -> &str {
        "Developer Tools Cache"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        // Xcode DerivedData
        let derived_data = home
            .join("Library")
            .join("Developer")
            .join("Xcode")
            .join("DerivedData");
        add_if_exists(&derived_data, "Xcode DerivedData", &mut targets);

        // Xcode Archives
        let archives = home
            .join("Library")
            .join("Developer")
            .join("Xcode")
            .join("Archives");
        add_if_exists(&archives, "Xcode Archives", &mut targets);

        // CoreSimulator
        let core_sim = home.join("Library").join("Developer").join("CoreSimulator");
        add_if_exists(&core_sim, "CoreSimulator data", &mut targets);

        // Xcode iOS DeviceSupport
        let device_support = home
            .join("Library")
            .join("Developer")
            .join("Xcode")
            .join("iOS DeviceSupport");
        add_if_exists(&device_support, "iOS DeviceSupport", &mut targets);

        // npm cache
        let npm_cache = home.join(".npm");
        add_if_exists(&npm_cache, "npm cache", &mut targets);

        // yarn cache
        let yarn_cache = home.join("Library").join("Caches").join("Yarn");
        add_if_exists(&yarn_cache, "Yarn cache", &mut targets);

        // pnpm cache
        let pnpm_store = home.join("Library").join("pnpm").join("store");
        add_if_exists(&pnpm_store, "pnpm store", &mut targets);

        // pip cache
        let pip_cache = home.join("Library").join("Caches").join("pip");
        add_if_exists(&pip_cache, "pip cache", &mut targets);

        // cargo cache
        let cargo_registry = home.join(".cargo").join("registry");
        add_if_exists(&cargo_registry, "Cargo registry cache", &mut targets);

        // CocoaPods cache
        let cocoapods_cache = home.join("Library").join("Caches").join("CocoaPods");
        add_if_exists(&cocoapods_cache, "CocoaPods cache", &mut targets);

        // Gradle cache
        let gradle_cache = home.join(".gradle").join("caches");
        add_if_exists(&gradle_cache, "Gradle caches", &mut targets);

        // Maven cache
        let maven_cache = home.join(".m2").join("repository");
        add_if_exists(&maven_cache, "Maven repository cache", &mut targets);

        // Go module cache
        let go_cache = home.join("go").join("pkg").join("mod").join("cache");
        add_if_exists(&go_cache, "Go module cache", &mut targets);

        // Ruby gems cache
        let gem_cache = home.join(".gem");
        add_if_exists(&gem_cache, "Ruby gem cache", &mut targets);

        targets.sort_by(|a, b| b.size.cmp(&a.size));
        targets
    }
}

fn add_if_exists(path: &std::path::Path, desc: &str, targets: &mut Vec<CleanTarget>) {
    if path.exists() {
        let size = dir_size(path);
        if size > 1_048_576 {
            targets.push(CleanTarget {
                path: path.to_path_buf(),
                size,
                description: desc.to_string(),
            });
        }
    }
}
