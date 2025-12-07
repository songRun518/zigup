use std::io::{Read, Seek, Write};

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
pub struct Cache {
        inner: std::fs::File,
}
impl Cache {
        pub const FILENAME: &str = ".zigup";

        pub fn new() -> anyhow::Result<Self> {
                let path = std::env::current_dir()
                        .context("Failed to obtain current dir")?
                        .join(Self::FILENAME);

                let inner = if path.exists() {
                        std::fs::File::options()
                                .read(true)
                                .write(true)
                                .open(&path)
                                .context("Failed to open cache")?
                } else {
                        std::fs::File::create_new(&path).context("Failed to create cache")?
                };

                Ok(Self { inner })
        }

        pub fn restore_offset(&mut self) -> anyhow::Result<()> {
                self.inner
                        .seek(std::io::SeekFrom::Start(0))
                        .context("Failed to seek cache")?;
                Ok(())
        }

        pub fn read(&mut self) -> anyhow::Result<Vec<u8>> {
                self.restore_offset()?;

                let mut buf = Vec::new();
                self.inner
                        .read_to_end(&mut buf)
                        .context("Failed to read cache")?;
                Ok(buf)
        }

        pub fn deserialize(&mut self) -> anyhow::Result<Vec<VersionInfo>> {
                let bytes = self.read()?;
                serde_json::from_slice(&bytes).context("Failed to deserialize cache")
        }

        pub fn write(&mut self, bytes: impl AsRef<[u8]>) -> anyhow::Result<()> {
                self.restore_offset()?;

                self.inner
                        .set_len(0)
                        .context("Failed to set length of cache")?;
                self.inner
                        .write_all(bytes.as_ref())
                        .context("Failed to write cache")
        }

        pub fn serialize(&mut self, versions_info: &[VersionInfo]) -> anyhow::Result<()> {
                let bytes = serde_json::to_vec_pretty(&versions_info)
                        .context("Failed to serialize cache")?;
                self.write(&bytes)
        }
}
