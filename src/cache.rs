use std::path::PathBuf;

use anyhow::Context;

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

        pub fn path() -> anyhow::Result<PathBuf> {
                Ok(std::env::current_dir()
                        .context("Failed to obtain current dir")?
                        .join(Self::NAME))
        }

        pub fn deserialize() -> anyhow::Result<Vec<VersionInfo>> {
                let bytes = std::fs::read(Self::path()?).context("Failed to read cache")?;
                serde_json::from_slice(&bytes).context("Failed to deserialize cache")
        }

        pub fn serialize(versions_info: &[VersionInfo]) -> anyhow::Result<()> {
                let bytes = serde_json::to_vec_pretty(&versions_info)
                        .context("Failed to serialize cache")?;
                std::fs::write(Self::path()?, &bytes).context("Failed to write cache")
        }
}
