use anyhow::Context;
use reqwest::blocking;
use serde_json::{Map, Value};

use crate::commands::{DownloadUrl, VersionInfo};

pub const VERSIONS_URL: &str = "https://ziglang.org/download/index.json";

pub fn execute() -> anyhow::Result<()> {
    let response = blocking::get(VERSIONS_URL).context("Failed to download version list")?;

    let version_list: Map<String, Value> = serde_json::from_slice(
        &response
            .bytes()
            .context("Failed to obtain bytes of version list")?,
    )
    .context("Failed to deserialize version list")?;

    let mut versions = Vec::new();
    for (version, value) in version_list {
        let Value::Object(info) = value else {
            unreachable!()
        };

        let date = info.get("date").unwrap().as_str().unwrap().to_string();

        let mut download_urls = Vec::new();
        for (arch, v) in info {
            let Value::Object(url_group) = v else {
                continue;
            };

            if arch.as_str() == "src" || arch.as_str() == "bootstrap" {
                continue;
            }

            download_urls.push(DownloadUrl {
                arch,
                url: url_group
                    .get("tarball")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            });
        }

        versions.push(VersionInfo {
            version,
            date,
            download_urls,
        })
    }

    let cache_file = std::env::current_dir()
        .context("Failed to obtain current dir")?
        .join(".zigup");
    std::fs::write(
        &cache_file,
        serde_json::to_vec_pretty(&versions).context("Failed to serialize cache")?,
    )
    .context("Failed to write cache")?;

    Ok(())
}
