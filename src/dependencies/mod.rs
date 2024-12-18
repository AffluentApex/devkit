use serde::Deserialize;
use std::path::Path;
use std::fs;

pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze<P: AsRef<Path>>(&self, path: P) -> DependencyStats {
        let mut stats = DependencyStats::default();
        
        let path = path.as_ref();
        if path.join("Cargo.toml").exists() {
            if let Ok(content) = fs::read_to_string(path.join("Cargo.toml")) {
                if let Ok(cargo_toml) = toml::from_str::<CargoToml>(&content) {
                    stats.total_deps = cargo_toml.dependencies.len();
                }
            }
        } else if path.join("package.json").exists() {
            if let Ok(content) = fs::read_to_string(path.join("package.json")) {
                if let Ok(package_json) = serde_json::from_str::<PackageJson>(&content) {
                    stats.total_deps = package_json.dependencies.len();
                }
            }
        }
        
        stats
    }
}

#[derive(Default)]
pub struct DependencyStats {
    pub total_deps: usize,
}

#[derive(Deserialize)]
struct CargoToml {
    dependencies: toml::Table,
}

#[derive(Deserialize)]
struct PackageJson {
    dependencies: serde_json::Map<String, serde_json::Value>,
}
