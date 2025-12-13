use reqwest::blocking;
use serde_json::{Map, Value};

use crate::config::{ArchSpecific, Config, VersionInfo};

pub const URL: &str = "https://ziglang.org/download/index.json";

pub fn download_blocking(url: &str) -> Vec<u8> {
    let panic_msg = format!("Failed to get {url}");
    let response = blocking::get(URL).expect(&panic_msg);
    response.bytes().expect("Failed to download bytes").to_vec()
}

pub fn deserialize(content: &[u8]) -> Vec<VersionInfo> {
    let version_list: Map<String, Value> =
        serde_json::from_slice(content).expect("Failed to deserialize content");

    let mut versions_info = Vec::new();
    for (mut version, info) in version_list {
        let info = info.as_object().unwrap();

        let mut date = String::new();
        let mut arch_specific = vec![];

        for (key, val) in info {
            match key.as_str() {
                "version" => {
                    version = val.as_str().unwrap().to_string();
                }

                "date" => {
                    date = val.as_str().unwrap().to_string();
                }

                _ => {
                    if !val.is_object() {
                        continue;
                    }

                    arch_specific.push(ArchSpecific {
                        arch: key.clone(),
                        url: val
                            .as_object()
                            .unwrap()
                            .get("tarball")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    });
                }
            }
        }

        versions_info.push(VersionInfo {
            version,
            date,
            arch_specific,
        })
    }

    versions_info
}

pub fn execute() -> Config {
    let versions_info = deserialize(&download_blocking(URL));

    let mut config = Config::load();
    config.versions_info = Some(versions_info);
    config.save();

    config
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize() {
        let ctt = std::fs::read("tests/index.json").unwrap();
        dbg!(&super::deserialize(&ctt));
    }
}
