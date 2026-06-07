use super::{CleanTarget, Cleaner};
use crate::core::size::dir_size;

pub struct DockerCleaner;

impl Cleaner for DockerCleaner {
    fn name(&self) -> &str {
        "Docker"
    }

    fn scan(&self) -> Vec<CleanTarget> {
        let mut targets = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();

        // Docker Desktop data
        let docker_data = home
            .join("Library")
            .join("Containers")
            .join("com.docker.docker")
            .join("Data");
        if docker_data.exists() {
            let size = dir_size(&docker_data);
            if size > 104_857_600 {
                // > 100MB
                targets.push(CleanTarget {
                    path: docker_data,
                    size,
                    description: "Docker Desktop data".to_string(),
                });
            }
        }

        // Docker images/layers cache
        let docker_cache = home.join(".docker").join("buildx");
        if docker_cache.exists() {
            let size = dir_size(&docker_cache);
            if size > 1_048_576 {
                targets.push(CleanTarget {
                    path: docker_cache,
                    size,
                    description: "Docker buildx cache".to_string(),
                });
            }
        }

        targets
    }
}
