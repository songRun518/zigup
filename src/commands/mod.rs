pub mod check;
pub mod update;

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
