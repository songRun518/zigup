use std::path::{Path, PathBuf};

use crate::config::Config;

fn ensure_dir(path: impl AsRef<Path>) {
    let path = path.as_ref();

    if path.exists() && path.is_dir() {
        return;
    }

    let panic_msg = format!("Failed to create dir {}", path.display());
    std::fs::create_dir_all(path).expect(&panic_msg);
}

pub const DIRNAME: &str = "versions";

pub fn versions_path() -> PathBuf {
    std::env::current_dir()
        .expect("Failed to obtain current dir")
        .join(DIRNAME)
}

pub fn download(version: String, platform: String) {
    let versions_path = versions_path();
    ensure_dir(&versions_path);

    let mut config = Config::load();
    if config.versions_info.is_none() {
        config = super::update::execute();
    }

    let Some(versions_info) = &config.versions_info else {
        unreachable!()
    };

    let panic_msg_v = format!("{version} is not found");
    let panic_msg_p = format!("{platform} is not found");
    let vi = versions_info
        .iter()
        .find(|vi| vi.version == version)
        .expect(&panic_msg_v);
    let url = vi
        .platform_specific
        .iter()
        .find_map(|ps| {
            if ps.platform == platform {
                Some(ps.url.clone())
            } else {
                None
            }
        })
        .expect(&panic_msg_p);

    println!("Downloading {version} {platform} via {url}");

    todo!()
}
