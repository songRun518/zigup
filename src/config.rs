use std::{path::PathBuf, sync::LazyLock};

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub versions_info: Option<Vec<VersionInfo>>,
}

impl Config {
    pub fn load() -> Self {
        let result = std::fs::read(&*PATH);

        if let Err(err) = &result
            && err.kind() == std::io::ErrorKind::NotFound
        {
            return Self::default();
        }

        let bytes = result.expect("Failed to read config");
        serde_json::from_slice(&bytes).expect("Failed to deserialize config")
    }

    pub fn save(&self) {
        let bytes = serde_json::to_vec_pretty(self).expect("Failed to serialize config");
        std::fs::write(&*PATH, &bytes).expect("Failed to write config")
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub date: String,
    pub arch_specific: Vec<ArchSpecific>,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ArchSpecific {
    pub arch: String,
    pub url: String,
}

pub const FILENAME: &str = ".zigup";

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_dir()
        .expect("Failed to obtain current dir")
        .join(FILENAME)
});
