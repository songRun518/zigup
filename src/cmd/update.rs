use reqwest::blocking;
use serde_json::{Map, Value};

use crate::cache::{self, ArchAndUrl, VersionInfo};

pub const URL: &str = "https://ziglang.org/download/index.json";

pub fn download_blocking(url: &str) -> Vec<u8> {
    let response = blocking::get(URL).unwrap_or_else(|err| panic!("Failed to get {url}: {err:?}"));
    response.bytes().expect("Failed to download bytes").to_vec()
}

pub fn execute() -> Vec<VersionInfo> {
    let version_list: Map<String, Value> = serde_json::from_slice(&download_blocking(URL))
        .expect("Failed to deserialize version list");

    let mut versions_info = Vec::new();
    for (version, info) in version_list {
        let Value::Object(info) = info else {
            unreachable!()
        };

        let date = info.get("date").unwrap().as_str().unwrap().to_string();

        let mut arch_and_url = Vec::new();
        for (arch, urls) in info {
            let Value::Object(urls) = urls else {
                continue;
            };

            if arch.as_str() == "src" || arch.as_str() == "bootstrap" {
                continue;
            }

            arch_and_url.push(ArchAndUrl {
                arch,
                url: urls.get("tarball").unwrap().as_str().unwrap().to_string(),
            });
        }

        versions_info.push(VersionInfo {
            version,
            date,
            arch_and_url,
        })
    }

    cache::serialize(&versions_info);

    versions_info
}
