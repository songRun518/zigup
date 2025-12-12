use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub date: String,
    pub download_urls: Vec<DownloadUrl>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DownloadUrl {
    pub arch: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Cache;
impl Cache {
    pub const NAME: &str = ".zigup";

    pub fn path() -> PathBuf {
        std::env::current_dir()
            .expect("Failed to obtain current dir")
            .join(Self::NAME)
    }

    pub fn deserialize() -> Vec<VersionInfo> {
        let bytes = std::fs::read(Self::path()).expect("Failed to read cache");
        serde_json::from_slice(&bytes).expect("Failed to deserialize cache")
    }

    pub fn serialize(versions_info: &[VersionInfo]) {
        let bytes = serde_json::to_vec_pretty(&versions_info).expect("Failed to serialize cache");
        std::fs::write(Self::path(), &bytes).expect("Failed to write cache")
    }
}
